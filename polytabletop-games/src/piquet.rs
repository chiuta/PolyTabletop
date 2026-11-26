//! Piquet (Franța), 2 jucători, din regulile publice ale partidei clasice (șase maini).
//!
//! * 32 de cărți (7–A). Fiecare primește 12; restul de 8 formează talonul. Cel care nu e dealer = „elder”
//!   (mâna întâi). O partie = 6 maini, dealerul alternează; câștigă cine are mai multe puncte.
//! * **Schimbul**: elderul aruncă 1–5 cărți (`d7h`, câte una, apoi `gata`) și trage tot atâtea din talon; apoi
//!   dealerul aruncă 0–(cărțile rămase în talon, max. 5) și trage.
//! * **Declarațiile** (calculate automat din mâinile definitive; câștigătorul fiecărei categorii punctează):
//!   *punct* (cele mai multe cărți într-o culoare, la egalitate suma valorilor A 11, figuri 10, restul pip;
//!   punctează numărul de cărți), *secvență* (serii de cel puțin 3 cărți consecutive într-o culoare: 3→3, 4→4,
//!   5→15, 6→16, 7→17, 8→18; cea mai lungă / mai mare decide, câștigătorul punctează toate secvențele lui), *set*
//!   (trei sau patru cărți de 10, J, Q, K sau A: trio 3, quatorze 14; câștigătorul punctează toate seturile lui).
//!   La egal fără câștigător, nimeni nu punctează. Variantă implementată: fără carte blanche și fără carte rouge.
//! * **Jocul**: elderul conduce, se răspunde obligatoriu la culoare, ia cea mai mare carte din culoarea condusă.
//!   Puncte în joc (variantă documentată, sursele diferă): cine conduce cu o carte de 10 sau mai mare ia 1 punct;
//!   cine ia levata răspunzând ia 1 punct; ultima levată +1. Cărțile: cine ia mai multe levate +10; capot (toate cele
//!   12) +40 în loc de 10. **Repique**: 30 de puncte din declarații înainte ca adversarul să aibă vreunul → +60;
//!   **pique**: elderul ajunge la 30 (declarații + joc) înainte ca dealerul să aibă vreun punct → +30.
//!
//! Mutări: `d7h` (aruncă 7♥), `gata`, carte (`Ah`, `10c`), `urmatoarea`. Informație ascunsă: vezi `p3_comun`.
#![allow(clippy::if_same_then_else)]
#![allow(clippy::needless_range_loop, clippy::type_complexity)]

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const DEALS: u32 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    ExchE,
    ExchY,
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Discard(Card),
    Done,
    Play(Card),
    Next,
}

#[derive(Clone)]
pub struct Piquet {
    rng: Rng,
    hands: [Vec<Card>; 2],
    talon: Vec<Card>,
    disc: [Vec<Card>; 2],
    discarded: [Vec<Card>; 2],
    dealer: Player,
    phase: Phase,
    side: Player,
    led: Option<Card>,
    leader: Player,
    tricks: [u8; 2],
    pts: [i32; 2],
    total: [i32; 2],
    gain: [i32; 2],
    deal_no: u32,
    pique_done: bool,
    played: Vec<Card>,
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Piquet {
    fn default() -> Self {
        Self::new()
    }
}

fn sort_hand(h: &mut [Card]) {
    h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
}

fn val(c: Card) -> i32 {
    match c.rank {
        14 => 11,
        11..=13 => 10,
        r => r as i32,
    }
}

/// Punct: (număr de cărți, suma valorilor) în cea mai bună culoare.
fn best_point(h: &[Card]) -> (i32, i32) {
    let mut best = (0, 0);
    for s in 0..4u8 {
        let v: Vec<&Card> = h.iter().filter(|c| c.suit == s).collect();
        let k = (v.len() as i32, v.iter().map(|c| val(**c)).sum());
        if k > best {
            best = k;
        }
    }
    best
}

/// Secvențe maximale (lungime, rang maxim) din mână.
fn sequences(h: &[Card]) -> Vec<(i32, u8)> {
    let mut out = Vec::new();
    for s in 0..4u8 {
        let mut present = [false; 15];
        for c in h.iter().filter(|c| c.suit == s) {
            present[c.rank as usize] = true;
        }
        let mut run = 0;
        for r in 7..=15usize {
            if r <= 14 && present[r] {
                run += 1;
            } else {
                if run >= 3 {
                    out.push((run, (r - 1) as u8));
                }
                run = 0;
            }
        }
    }
    out
}

fn seq_points(len: i32) -> i32 {
    match len {
        3 => 3,
        4 => 4,
        5 => 15,
        6 => 16,
        7 => 17,
        _ => 18,
    }
}

/// Seturi: (număr de cărți, rang) pentru 10,J,Q,K,A cu cel puțin 3 cărți.
fn sets(h: &[Card]) -> Vec<(i32, u8)> {
    let mut out = Vec::new();
    for r in 10..=14u8 {
        let n = h.iter().filter(|c| c.rank == r).count() as i32;
        if n >= 3 {
            out.push((n, r));
        }
    }
    out
}

fn set_points(n: i32) -> i32 {
    if n >= 4 {
        14
    } else {
        3
    }
}

/// Valoare euristică a unei mâini pentru schimb.
fn hand_potential(h: &[Card]) -> i32 {
    let (n, _) = best_point(h);
    let mut v = n * 3;
    for (l, _) in sequences(h) {
        v += seq_points(l);
    }
    for (n, _) in sets(h) {
        v += set_points(n);
    }
    v + h
        .iter()
        .map(|c| {
            if c.rank >= 13 {
                2
            } else if c.rank >= 11 {
                1
            } else {
                0
            }
        })
        .sum::<i32>()
}

impl Piquet {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Piquet {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            talon: Vec::new(),
            disc: [Vec::new(), Vec::new()],
            discarded: [Vec::new(), Vec::new()],
            dealer: Player::Two,
            phase: Phase::ExchE,
            side: Player::One,
            led: None,
            leader: Player::One,
            tricks: [0, 0],
            pts: [0, 0],
            total: [0, 0],
            gain: [0, 0],
            deal_no: 0,
            pique_done: false,
            played: Vec::new(),
            fresh: [0, 0],
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }

    fn elder(&self) -> Player {
        self.dealer.other()
    }

    fn deal(&mut self) {
        let mut d = full_deck(7);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.drain(..12).collect();
            sort_hand(&mut self.hands[p]);
            self.disc[p].clear();
            self.discarded[p].clear();
        }
        self.talon = d;
        self.phase = Phase::ExchE;
        self.side = self.elder();
        self.led = None;
        self.tricks = [0, 0];
        self.pts = [0, 0];
        self.gain = [0, 0];
        self.pique_done = false;
        self.played.clear();
        self.fresh = [0, 0];
    }

    fn add(&mut self, p: usize, n: i32) {
        self.pts[p] += n;
    }

    fn declarations(&mut self) {
        let e = self.elder().idx();
        let y = 1 - e;
        let mut log = String::new();
        // punct
        let (pe, py) = (best_point(&self.hands[e]), best_point(&self.hands[y]));
        match pe.cmp(&py) {
            std::cmp::Ordering::Greater => {
                self.add(e, pe.0);
                log.push_str(&format!("Punct J{} ({}) ", e + 1, pe.0));
            }
            std::cmp::Ordering::Less => {
                self.add(y, py.0);
                log.push_str(&format!("Punct J{} ({}) ", y + 1, py.0));
            }
            _ => log.push_str("Punct egal "),
        }
        // secvențe
        let (se, sy) = (sequences(&self.hands[e]), sequences(&self.hands[y]));
        let (be, by) = (se.iter().max().copied(), sy.iter().max().copied());
        let winner = match (be, by) {
            (Some(a), Some(b)) => match a.cmp(&b) {
                std::cmp::Ordering::Greater => Some(e),
                std::cmp::Ordering::Less => Some(y),
                _ => None,
            },
            (Some(_), None) => Some(e),
            (None, Some(_)) => Some(y),
            _ => None,
        };
        if let Some(w) = winner {
            let list = if w == e { &se } else { &sy };
            let t: i32 = list.iter().map(|x| seq_points(x.0)).sum();
            self.add(w, t);
            log.push_str(&format!("| secvențe J{} ({t}) ", w + 1));
        }
        // seturi
        let (te, ty) = (sets(&self.hands[e]), sets(&self.hands[y]));
        let (be, by) = (te.iter().max().copied(), ty.iter().max().copied());
        let winner = match (be, by) {
            (Some(a), Some(b)) => match a.cmp(&b) {
                std::cmp::Ordering::Greater => Some(e),
                std::cmp::Ordering::Less => Some(y),
                _ => None,
            },
            (Some(_), None) => Some(e),
            (None, Some(_)) => Some(y),
            _ => None,
        };
        if let Some(w) = winner {
            let list = if w == e { &te } else { &ty };
            let t: i32 = list.iter().map(|x| set_points(x.0)).sum();
            self.add(w, t);
            log.push_str(&format!("| seturi J{} ({t}) ", w + 1));
        }
        // repique
        for p in 0..2 {
            if self.pts[p] >= 30 && self.pts[1 - p] == 0 {
                self.add(p, 60);
                log.push_str(&format!("| REPIQUE J{} (+60) ", p + 1));
                self.pique_done = true;
            }
        }
        self.log = log;
    }

    fn check_pique(&mut self) {
        if self.pique_done {
            return;
        }
        let e = self.elder().idx();
        if self.pts[e] >= 30 && self.pts[1 - e] == 0 {
            self.add(e, 30);
            self.pique_done = true;
            self.log.push_str(&format!(" PIQUE J{} (+30)", e + 1));
        }
    }

    fn follow(&self) -> Vec<Card> {
        let h = &self.hands[self.side.idx()];
        if let Some(l) = self.led {
            if h.iter().any(|c| c.suit == l.suit) {
                return h.iter().copied().filter(|c| c.suit == l.suit).collect();
            }
        }
        h.clone()
    }

    fn max_discards(&self) -> usize {
        if self.phase == Phase::ExchE {
            5.min(self.talon.len())
        } else {
            5.min(self.talon.len())
        }
    }

    fn finish_hand(&mut self) {
        // cărți
        let (a, b) = (self.tricks[0], self.tricks[1]);
        if a == 12 {
            self.add(0, 40);
        } else if b == 12 {
            self.add(1, 40);
        } else if a > b {
            self.add(0, 10);
        } else if b > a {
            self.add(1, 10);
        }
        for p in 0..2 {
            self.total[p] += self.pts[p];
            self.gain[p] = self.pts[p];
        }
        self.deal_no += 1;
        self.log.push_str(&format!(
            " — mână încheiată: J1 +{}, J2 +{}",
            self.pts[0], self.pts[1]
        ));
        self.phase = if self.deal_no >= DEALS {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for Piquet {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::HandOver => vec![Move::Next],
            Phase::ExchE | Phase::ExchY => {
                let p = self.side.idx();
                let mut v = Vec::new();
                if self.disc[p].len() < self.max_discards() {
                    for &c in &self.hands[p] {
                        v.push(Move::Discard(c));
                    }
                }
                if self.phase == Phase::ExchY || !self.disc[p].is_empty() {
                    v.push(Move::Done);
                }
                v
            }
            Phase::Play => self.follow().into_iter().map(Move::Play).collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = [0, 0];
        self.prev = self.side;
        match *m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.log.clear();
                self.deal();
            }
            Move::Discard(c) => {
                let p = self.side.idx();
                let pos = self.hands[p].iter().position(|&x| x == c).unwrap();
                self.hands[p].remove(pos);
                self.disc[p].push(c);
                self.prev = self.side;
            }
            Move::Done => {
                let p = self.side.idx();
                let k = self.disc[p].len();
                for _ in 0..k {
                    let c = self.talon.remove(0);
                    self.hands[p].push(c);
                }
                self.fresh[p] = k as u8;
                let d = std::mem::take(&mut self.disc[p]);
                self.discarded[p].extend(d);
                if self.phase == Phase::ExchE {
                    self.phase = Phase::ExchY;
                    self.side = self.dealer;
                } else {
                    for q in 0..2 {
                        sort_hand(&mut self.hands[q]);
                    }
                    self.fresh = [0, 0];
                    self.declarations();
                    self.phase = Phase::Play;
                    self.side = self.elder();
                    self.leader = self.side;
                }
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.played.push(c);
                match self.led {
                    None => {
                        self.led = Some(c);
                        self.leader = p;
                        if c.rank >= 10 {
                            self.add(p.idx(), 1);
                        }
                        self.side = p.other();
                    }
                    Some(l) => {
                        let win_resp = c.suit == l.suit && c.rank > l.rank;
                        let winner = if win_resp { p } else { p.other() };
                        self.tricks[winner.idx()] += 1;
                        if win_resp {
                            self.add(p.idx(), 1);
                        }
                        let last = self.hands[0].is_empty();
                        if last {
                            self.add(winner.idx(), 1);
                        }
                        self.led = None;
                        self.side = winner;
                        self.log = format!("{} vs {} → J{}", l.text(), c.text(), winner.idx() + 1);
                        self.check_pique();
                        if last {
                            self.finish_hand();
                        }
                    }
                }
                if self.phase == Phase::Play && self.led.is_some() {
                    self.check_pique();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.total[0].cmp(&self.total[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        match self.phase {
            Phase::Over => 0,
            Phase::ExchE | Phase::ExchY if self.side == p => {
                let k = self.disc[p.idx()].len() as i32;
                hand_potential(&self.hands[p.idx()]) * 10 + k * 25
            }
            _ => honest_eval(self, p, 10),
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !matches!(self.phase, Phase::Over | Phase::HandOver) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Piquet — mâna {}/{DEALS}; total J1 {}, J2 {}\n",
            (self.deal_no + 1).min(DEALS),
            self.total[0],
            self.total[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Partida s-a încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n");
                return s;
            }
            _ => {}
        }
        s.push_str(&format!(
            "Elder (conduce): J{} | talon: {} cărți | puncte mână: J1 {}, J2 {}\n",
            self.elder().idx() + 1,
            self.talon.len(),
            self.pts[0],
            self.pts[1]
        ));
        if let Some(l) = self.led {
            s.push_str(&format!("Pe masă: {}\n", l.text()));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        if matches!(self.phase, Phase::ExchE | Phase::ExchY) && !self.disc[v.idx()].is_empty() {
            s.push_str(&format!(
                "Aruncate (tragi {} din talon la «gata»): {}\n",
                self.disc[v.idx()].len(),
                join_cards(&self.disc[v.idx()])
            ));
        }
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Next => "urmatoarea".into(),
            Move::Done => "gata".into(),
            Move::Discard(c) => format!("d{}", c.text()),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Piquet {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let oi = 1 - mi;
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known: Vec<Card> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool = full_deck(7);
        pool.retain(|c| {
            !known.contains(c)
                && !self.played.contains(c)
                && !self.disc[mi].contains(c)
                && !self.discarded[mi].contains(c)
        });
        shuffle(&mut pool, rng);
        let n_opp = self.hands[oi].len();
        let mut mine = known;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        g.hands[mi] = mine;
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        g.talon = pool;
        g.fresh = [0, 0];
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn seed_state(&self, me: Player) -> Self {
        let mut c = self.clone();
        let nf = (c.fresh[me.idx()] as usize).min(c.hands[me.idx()].len());
        let k = c.hands[me.idx()].len() - nf;
        c.hands[me.idx()].truncate(k);
        c.fresh = [0, 0];
        c
    }

    fn forget_fresh(&mut self) {
        self.fresh = [0, 0];
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        let q = p.other();
        if self.rollout_done() {
            return (self.gain[p.idx()] - self.gain[q.idx()]) as f64;
        }
        (self.pts[p.idx()] - self.pts[q.idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 || rng.below(100) < 10 {
            return mv[rng.below(mv.len())];
        }
        match self.phase {
            Phase::ExchE | Phase::ExchY => {
                // aruncă cărțile care ajută cel mai puțin hand_potential, până la 4 (sau 3) cărți
                let p = self.side.idx();
                let base = hand_potential(&self.hands[p]);
                let want = if self.phase == Phase::ExchE { 4 } else { 3 };
                if self.disc[p].len() >= want.min(self.max_discards()) && mv.contains(&Move::Done) {
                    return Move::Done;
                }
                let mut best = None;
                let mut bs = i32::MIN;
                for &m in &mv {
                    if let Move::Discard(c) = m {
                        let mut h = self.hands[p].clone();
                        let k = h.iter().position(|&x| x == c).unwrap();
                        h.remove(k);
                        let sc = hand_potential(&h) - base;
                        if sc > bs {
                            bs = sc;
                            best = Some(m);
                        }
                    }
                }
                if bs < 0 && mv.contains(&Move::Done) {
                    return Move::Done;
                }
                best.unwrap_or(Move::Done)
            }
            _ => {
                let mut best = mv[0];
                let mut bs = i32::MIN;
                for &m in &mv {
                    if let Move::Play(c) = m {
                        let sc = if let Some(l) = self.led {
                            if c.suit == l.suit && c.rank > l.rank {
                                30 - c.rank as i32
                            } else {
                                -(c.rank as i32)
                            }
                        } else {
                            c.rank as i32
                        };
                        if sc > bs {
                            bs = sc;
                            best = m;
                        }
                    }
                }
                best
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn declaration_helpers() {
        let h = vec![
            c(14, 0),
            c(13, 0),
            c(12, 0),
            c(11, 0),
            c(10, 1),
            c(10, 2),
            c(10, 3),
            c(7, 2),
        ];
        assert_eq!(best_point(&h).0, 4);
        assert_eq!(sequences(&h), vec![(4, 14)]);
        assert_eq!(sets(&h), vec![(3, 10)]);
        assert_eq!(seq_points(5), 15);
        assert_eq!(set_points(4), 14);
    }

    #[test]
    fn exchange_flow() {
        let mut g = Piquet::with_seed(5);
        assert_eq!(g.to_move(), g.elder());
        // elderul nu poate spune gata fără să arunce
        assert!(!g.legal_moves().contains(&Move::Done));
        let c0 = g.hands[g.side.idx()][0];
        g.apply(&Move::Discard(c0));
        assert!(g.legal_moves().contains(&Move::Done));
        g.apply(&Move::Done);
        assert_eq!(g.hands[g.elder().idx()].len(), 12);
        assert_eq!(g.talon.len(), 7);
        g.apply(&Move::Done); // dealerul nu schimbă
        assert_eq!(g.phase, Phase::Play);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=3u64 {
            let mut g = Piquet::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
            assert_eq!(g.deal_no, 6);
        }
    }
}
