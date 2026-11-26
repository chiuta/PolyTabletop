//! Conquián (rummy mexican cu pachet spaniol), 2 jucători, din regulile publice.
//!
//! * 40 de cărți: patru culori (o = oros, c = copas, e = espadas, b = bastos), valorile A 2 3 4 5 6 7 S C R
//!   (S = sota, C = caballo, R = rey; 8 și 9 lipsesc). Secvențele merg în ordinea A…7, S, C, R (7 și S sunt
//!   vecine), fără trecere de la R la A.
//! * 10 cărți fiecăruia; o carte de pe stoc se întoarce pe talon. Combinații: **set** de 3–4 cărți de aceeași
//!   valoare sau **secvență** de 3+ cărți din aceeași culoare.
//! * În tura ta: tragi din stoc (`stoc`) **sau** iei cartea de sus a talonului (`ia 3c 3e` = o legi imediat
//!   într-o combinație nouă cu cărțile numite din mână, `ia la 2` = o lipești pe combinația 2 de pe masă).
//!   Apoi poți coborî combinații (`pune 3o 3c 3e`), lipi cărți pe orice combinație de pe masă, a ta sau a
//!   adversarului (`adauga 4o la 2`), și încheii cu `descarta 5o`.
//! * Câștigă mâna cine rămâne fără cărți în mână (după ce a coborât tot sau după ce a aruncat ultima carte).
//!   Dacă stocul e gol și nu poți lua talonul, mâna e nulă (`pas`). Câștigătorul ia un punct; primul la 3.
//!
//! Informație ascunsă: vezi `p3_comun`. Talonul întreg e public (cărțile aruncate se văd).
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::type_complexity
)]

use crate::cards::{handover_banner, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 3;
const LABELS: [&str; 11] = ["", "A", "2", "3", "4", "5", "6", "7", "S", "C", "R"];
const SUITS: [char; 4] = ['o', 'c', 'e', 'b'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Act,
    HandOver,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Take(Vec<Card>),
    TakeOn(usize),
    Meld(Vec<Card>),
    Attach(Card, usize),
    Discard(Card),
    Void,
    Next,
}

#[derive(Clone)]
pub struct Conquian {
    rng: Rng,
    stock: Vec<Card>,
    discard: Vec<Card>,
    hands: [Vec<Card>; 2],
    table: Vec<(Player, Vec<Card>)>,
    dealer: Player,
    side: Player,
    phase: Phase,
    scores: [i32; 2],
    delta: [i32; 2],
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Conquian {
    fn default() -> Self {
        Self::new()
    }
}

pub fn ctext(c: Card) -> String {
    format!("{}{}", LABELS[c.rank as usize], SUITS[c.suit as usize])
}

fn join(v: &[Card]) -> String {
    if v.is_empty() {
        return "—".into();
    }
    v.iter().map(|&c| ctext(c)).collect::<Vec<_>>().join(" ")
}

fn deck40() -> Vec<Card> {
    let mut d = Vec::new();
    for suit in 0..4u8 {
        for rank in 1..=10u8 {
            d.push(Card { rank, suit });
        }
    }
    d
}

fn sorted(mut v: Vec<Card>) -> Vec<Card> {
    v.sort_by_key(|c| (c.suit, c.rank));
    v
}

pub fn is_set(v: &[Card]) -> bool {
    v.len() >= 3 && v.len() <= 4 && v.iter().all(|c| c.rank == v[0].rank)
}

pub fn is_run(v: &[Card]) -> bool {
    if v.len() < 3 || !v.iter().all(|c| c.suit == v[0].suit) {
        return false;
    }
    let mut r: Vec<u8> = v.iter().map(|c| c.rank).collect();
    r.sort_unstable();
    r.windows(2).all(|w| w[1] == w[0] + 1)
}

fn can_attach(meld: &[Card], c: Card) -> bool {
    if meld.iter().all(|m| m.rank == meld[0].rank) {
        return c.rank == meld[0].rank && meld.len() < 4;
    }
    if c.suit != meld[0].suit {
        return false;
    }
    let lo = meld.iter().map(|m| m.rank).min().unwrap();
    let hi = meld.iter().map(|m| m.rank).max().unwrap();
    (c.rank + 1 == lo) || (c.rank == hi + 1)
}

/// Toate combinațiile (seturi și secvențe) care se pot forma din `cards`; dacă `must` e dat, doar cele cu el.
fn all_melds(cards: &[Card], must: Option<Card>) -> Vec<Vec<Card>> {
    let mut out: Vec<Vec<Card>> = Vec::new();
    for r in 1..=10u8 {
        let same: Vec<Card> = cards.iter().copied().filter(|c| c.rank == r).collect();
        let n = same.len();
        if n == 3 {
            out.push(sorted(same.clone()));
        }
        if n == 4 {
            for skip in 0..4 {
                let t: Vec<Card> = (0..4).filter(|&i| i != skip).map(|i| same[i]).collect();
                out.push(sorted(t));
            }
            out.push(sorted(same));
        }
    }
    for s in 0..4u8 {
        let mut by: [Option<Card>; 11] = [None; 11];
        for &c in cards.iter().filter(|c| c.suit == s) {
            by[c.rank as usize] = Some(c);
        }
        for lo in 1..=10usize {
            let mut run = Vec::new();
            for r in lo..=10usize {
                match by[r] {
                    Some(c) => {
                        run.push(c);
                        if run.len() >= 3 {
                            out.push(run.clone());
                        }
                    }
                    None => break,
                }
            }
        }
    }
    out.sort_by_key(|v| v.iter().map(|c| (c.suit, c.rank)).collect::<Vec<_>>());
    out.dedup();
    if let Some(m) = must {
        out.retain(|v| v.contains(&m));
    }
    out
}

impl Conquian {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Conquian {
            rng: Rng::new(seed),
            stock: Vec::new(),
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Draw,
            scores: [0, 0],
            delta: [0, 0],
            fresh: [0, 0],
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        let mut d = deck40();
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..10 {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        self.discard = vec![d.pop().unwrap()];
        self.stock = d;
        self.table.clear();
        self.delta = [0, 0];
        self.fresh = [0, 0];
        self.sort_hands();
        self.phase = Phase::Draw;
        self.side = self.dealer.other();
    }

    fn sort_hands(&mut self) {
        for p in 0..2 {
            let nf = (self.fresh[p] as usize).min(self.hands[p].len());
            let k = self.hands[p].len() - nf;
            self.hands[p][..k].sort_by_key(|c| (c.suit, c.rank));
        }
    }

    fn win(&mut self, p: Player) {
        self.scores[p.idx()] += 1;
        self.delta[p.idx()] += 1;
        self.log = format!("J{} rămâne fără cărți și câștigă mâna!", p.idx() + 1);
        self.phase = if self.scores[p.idx()] >= TARGET {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }

    fn remove(&mut self, p: Player, c: Card) {
        let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
        self.hands[p.idx()].remove(pos);
    }
}

impl Game for Conquian {
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
            Phase::Draw => {
                let mut v = Vec::new();
                if !self.stock.is_empty() {
                    v.push(Move::Draw);
                }
                if let Some(&top) = self.discard.last() {
                    let h = &self.hands[self.side.idx()];
                    let mut pool = h.clone();
                    pool.push(top);
                    for m in all_melds(&pool, Some(top)) {
                        let used: Vec<Card> = m.into_iter().filter(|&c| c != top).collect();
                        v.push(Move::Take(used));
                    }
                    for (i, (_, m)) in self.table.iter().enumerate() {
                        if can_attach(m, top) {
                            v.push(Move::TakeOn(i));
                        }
                    }
                }
                if v.is_empty() {
                    v.push(Move::Void);
                }
                v
            }
            Phase::Act => {
                let h = &self.hands[self.side.idx()];
                let mut v: Vec<Move> = all_melds(h, None).into_iter().map(Move::Meld).collect();
                for (i, (_, m)) in self.table.iter().enumerate() {
                    for &c in h {
                        let mv = Move::Attach(c, i);
                        if can_attach(m, c) && !v.contains(&mv) {
                            v.push(mv);
                        }
                    }
                }
                for &c in h {
                    let mv = Move::Discard(c);
                    if !v.contains(&mv) {
                        v.push(mv);
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.fresh = [0, 0];
        self.prev = self.side;
        let p = self.side;
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Void => {
                self.log = "Stoc epuizat: mână nulă.".into();
                self.phase = Phase::HandOver;
            }
            Move::Draw => {
                let c = self.stock.pop().unwrap();
                self.hands[p.idx()].push(c);
                self.fresh[p.idx()] = 1;
                self.phase = Phase::Act;
            }
            Move::Take(used) => {
                let top = self.discard.pop().unwrap();
                for &c in used {
                    self.remove(p, c);
                }
                let mut meld = used.clone();
                meld.push(top);
                self.table.push((p, sorted(meld)));
                self.phase = Phase::Act;
                if self.hands[p.idx()].is_empty() {
                    self.win(p);
                }
            }
            Move::TakeOn(i) => {
                let top = self.discard.pop().unwrap();
                self.table[*i].1.push(top);
                self.phase = Phase::Act;
            }
            Move::Meld(cs) => {
                for &c in cs {
                    self.remove(p, c);
                }
                self.table.push((p, cs.clone()));
                if self.hands[p.idx()].is_empty() {
                    self.win(p);
                }
            }
            Move::Attach(c, i) => {
                self.remove(p, *c);
                self.table[*i].1.push(*c);
                if self.hands[p.idx()].is_empty() {
                    self.win(p);
                }
            }
            Move::Discard(c) => {
                self.remove(p, *c);
                self.discard.push(*c);
                if self.hands[p.idx()].is_empty() {
                    self.win(p);
                } else {
                    self.side = p.other();
                    self.phase = Phase::Draw;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.scores[0] >= TARGET {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 14)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && matches!(self.phase, Phase::Draw | Phase::Act) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Conquián — mâini câștigate: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
            self.scores[0],
            self.scores[1],
            TARGET,
            self.dealer.idx() + 1
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Joc încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str("Scrie «urmatoarea».\n");
                return s;
            }
            _ => {}
        }
        s.push_str(&format!(
            "Stoc: {} cărți | talon sus: {} (aruncate: {})\n",
            self.stock.len(),
            self.discard
                .last()
                .map(|&c| ctext(c))
                .unwrap_or_else(|| "—".into()),
            self.discard.len()
        ));
        for (i, (o, m)) in self.table.iter().enumerate() {
            s.push_str(&format!(
                "Combinația {} (J{}): {}\n",
                i + 1,
                o.idx() + 1,
                join(m)
            ));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join(&self.hands[v.idx()])
        ));
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
            Move::Draw => "stoc".into(),
            Move::Take(v) => format!("ia {}", join(v)),
            Move::TakeOn(i) => format!("ia la {}", i + 1),
            Move::Meld(v) => format!("pune {}", join(v)),
            Move::Attach(c, i) => format!("adauga {} la {}", ctext(*c), i + 1),
            Move::Discard(c) => format!("descarta {}", ctext(*c)),
            Move::Void => "pas".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Conquian {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (mi, oi) = (me.idx(), 1 - me.idx());
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known_mine: Vec<Card> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool = deck40();
        pool.retain(|c| {
            !known_mine.contains(c)
                && !self.discard.contains(c)
                && !self.table.iter().any(|(_, m)| m.contains(c))
        });
        shuffle(&mut pool, rng);
        let mut mine = known_mine;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        let n_opp = self.hands[oi].len();
        g.hands[mi] = mine;
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        let ns = self.stock.len();
        pool.truncate(ns.min(pool.len()));
        g.stock = pool;
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
        if self.phase == Phase::Draw || self.phase == Phase::Act {
            let q = p.other();
            return (self.hands[q.idx()].len() as f64 - self.hands[p.idx()].len() as f64) * 0.05;
        }
        (self.delta[p.idx()] - self.delta[p.other().idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0].clone();
        }
        match self.phase {
            Phase::Draw => {
                let takes: Vec<&Move> = mv
                    .iter()
                    .filter(|m| matches!(m, Move::Take(_) | Move::TakeOn(_)))
                    .collect();
                if !takes.is_empty() && rng.below(100) < 85 {
                    return takes[rng.below(takes.len())].clone();
                }
                if mv.contains(&Move::Draw) {
                    Move::Draw
                } else {
                    mv[0].clone()
                }
            }
            _ => {
                // întâi cea mai mare combinație, apoi lipiri, apoi aruncă cea mai izolată carte
                let best = mv
                    .iter()
                    .filter_map(|m| {
                        if let Move::Meld(v) = m {
                            Some((v.len(), m))
                        } else {
                            None
                        }
                    })
                    .max_by_key(|(n, _)| *n);
                if let Some((_, m)) = best {
                    return m.clone();
                }
                if let Some(m) = mv.iter().find(|m| matches!(m, Move::Attach(..))) {
                    return m.clone();
                }
                let h = &self.hands[self.side.idx()];
                let score = |c: Card| -> i32 {
                    let same = h.iter().filter(|x| x.rank == c.rank && **x != c).count() as i32;
                    let near = h
                        .iter()
                        .filter(|x| {
                            x.suit == c.suit
                                && (x.rank as i32 - c.rank as i32).abs() <= 2
                                && **x != c
                        })
                        .count() as i32;
                    same * 3 + near * 2
                };
                let c = h
                    .iter()
                    .copied()
                    .min_by_key(|&c| (score(c), rng.below(5)))
                    .unwrap();
                Move::Discard(c)
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
    fn set_and_run_validation() {
        assert!(is_set(&[c(3, 0), c(3, 1), c(3, 2)]));
        assert!(!is_set(&[c(3, 0), c(3, 1)]));
        assert!(is_run(&[c(6, 1), c(7, 1), c(8, 1)])); // 6-7-S
        assert!(is_run(&[c(8, 2), c(9, 2), c(10, 2)])); // S-C-R
        assert!(!is_run(&[c(1, 0), c(2, 0), c(10, 0)]));
        assert!(!is_run(&[c(4, 0), c(5, 1), c(6, 0)]));
    }

    #[test]
    fn attach_rules() {
        let run = [c(4, 0), c(5, 0), c(6, 0)];
        assert!(can_attach(&run, c(3, 0)) && can_attach(&run, c(7, 0)));
        assert!(!can_attach(&run, c(7, 1)) && !can_attach(&run, c(9, 0)));
        let set = [c(2, 0), c(2, 1), c(2, 2)];
        assert!(can_attach(&set, c(2, 3)));
        let king = [c(8, 3), c(9, 3), c(10, 3)];
        assert!(!can_attach(&king, c(1, 3)));
    }

    #[test]
    fn take_requires_immediate_meld() {
        let mut g = Conquian::with_seed(1);
        g.discard = vec![c(5, 1)];
        g.hands[0] = vec![c(5, 0), c(5, 2), c(9, 3), c(1, 0)];
        g.side = Player::One;
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Take(vec![c(5, 0), c(5, 2)])));
        g.hands[0] = vec![c(9, 3), c(1, 0)];
        let mv = g.legal_moves();
        assert!(!mv
            .iter()
            .any(|m| matches!(m, Move::Take(_) | Move::TakeOn(_))));
        assert!(mv.contains(&Move::Draw));
    }

    #[test]
    fn going_out_by_melding_all_and_by_discarding_last() {
        let mut g = Conquian::with_seed(2);
        g.phase = Phase::Act;
        g.side = Player::One;
        g.hands[0] = vec![c(3, 0), c(3, 1), c(3, 2)];
        g.apply(&Move::Meld(vec![c(3, 0), c(3, 1), c(3, 2)]));
        assert_eq!(g.scores[0], 1);
        assert_eq!(g.phase, Phase::HandOver);
        let mut h = Conquian::with_seed(3);
        h.phase = Phase::Act;
        h.side = Player::Two;
        h.hands[1] = vec![c(9, 0)];
        h.apply(&Move::Discard(c(9, 0)));
        assert_eq!(h.scores[1], 1);
    }

    #[test]
    fn attach_to_opponent_meld() {
        let mut g = Conquian::with_seed(4);
        g.phase = Phase::Act;
        g.side = Player::One;
        g.table = vec![(Player::Two, vec![c(4, 0), c(5, 0), c(6, 0)])];
        g.hands[0] = vec![c(7, 0), c(1, 1)];
        assert!(g.legal_moves().contains(&Move::Attach(c(7, 0), 0)));
        g.apply(&Move::Attach(c(7, 0), 0));
        assert_eq!(g.table[0].1.len(), 4);
    }

    #[test]
    fn determinize_conserves_cards() {
        let g = Conquian::with_seed(5);
        let mut rng = Rng::new(2);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 10);
        let mut all: Vec<Card> = d.hands[0]
            .iter()
            .chain(&d.hands[1])
            .chain(&d.stock)
            .chain(&d.discard)
            .copied()
            .collect();
        all.sort_by_key(|c| (c.suit, c.rank));
        assert_eq!(all, deck40());
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Conquian::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 20_000 {
                let mv = g.legal_moves();
                let m = mv[rng.below(mv.len())].clone();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} after {n}");
        }
    }
}
