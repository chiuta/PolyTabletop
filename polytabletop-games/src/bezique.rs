#![allow(clippy::doc_lazy_continuation)]
//! Bézique (Franța), joc de levate cu declarări pentru doi jucători, din regulile publice ale jocului clasic.
//!
//! * Pachet de 64 de cărți (două pachete de câte 32: 7, 8, 9, 10, valet, damă, rege, as). Ordinea în
//!   culoare, de la cea mai mare: as, 10, rege, damă, valet, 9, 8, 7. Fiecare primește 8 cărți; următoarea
//!   se întoarce și dă atuul (jucătorul care nu conduce, „dealerul”, ia 10 puncte dacă e un 7 de atu).
//! * Cât timp mai e pachet: poți juca orice carte; cine câștigă levata (carte mai mare în culoarea cerută sau
//!   atu; la două cărți identice câștigă cel care a condus) poate **declara o figură**, apoi ambii trag
//!   (câștigătorul primul) și câștigătorul conduce. Cărțile declarate rămân pe masă, vizibile, dar pot fi jucate.
//!   O carte poate intra în figuri de clase diferite (căsătorie / secvență / patru la fel / bézique), dar nu
//!   de două ori în aceeași clasă, iar figura trebuie să includă cel puțin o carte din mână.
//! * Figuri: căsătorie rege+damă 20 (40 la atu); secvență A-10-K-Q-V de atu 250; patru ași 100, patru
//!   regi 80, patru dame 60, patru valeți 40 (culori diferite); bézique (damă pică + valet caro) 40; dublu
//!   bézique (ambele dame de pică + ambii valeți de caro) 500; `dix` = schimbi 7 de atu cu atuul întors (10).
//!   Un 7 de atu jucat într-o levată dă 10.
//! * După epuizarea pachetului (ultimele 8 levate) nu se mai declară nimic, răspunzi la culoare, bați dacă
//!   poți, altfel tai cu atu. La sfârșit fiecare as și 10 ridicat în levate (brisque) dă 10; ultima levată 10.
//! * Mai multe mâini (cel mult 4); cine atinge primul 1000 câștigă pe loc, altfel câștigă cine are mai multe puncte.
//! * Informație ascunsă: AI-ul vede doar mâna lui, cărțile declarate și jucate. Trage la pasul `trag`.
//! Mutări: `Qs` (carte: 7 8 9 10 J Q K A + h/d/s/c), `mariaj s`, `secventa`, `patru K`, `bezic`, `dublubezic`,
//! `dix`, `fara` (nicio declarație), `trag`.

use crate::cards::{handover_banner, shuffle, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANKS: [&str; 8] = ["7", "8", "9", "J", "Q", "K", "10", "A"];
const SUITS: [char; 4] = ['h', 'd', 's', 'c'];
const TARGET: i32 = 1000;
const HANDS: u8 = 4;
const SHOWN: u8 = 1;
const CL_M: u8 = 2;
const CL_S: u8 = 4;
const CL_B: u8 = 8;
const CL_F: u8 = 16;

fn rank(c: u8) -> u8 {
    c % 8
}
fn suit(c: u8) -> u8 {
    c / 8
}
fn card(s: u8, r: u8) -> u8 {
    s * 8 + r
}
fn text(c: u8) -> String {
    format!("{}{}", RANKS[rank(c) as usize], SUITS[suit(c) as usize])
}
fn is_brisque(c: u8) -> bool {
    rank(c) >= 6
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Meld {
    Marriage(u8),
    Sequence,
    Four(u8),
    Bezique,
    Double,
    Dix,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Declare(Meld),
    NoMeld,
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Lead,
    Follow,
    Meld,
    Draw,
    Over,
}

type Hand = Vec<(u8, u8)>;

#[derive(Clone)]
pub struct Bezique {
    hands: [Hand; 2],
    stock: Vec<u8>,
    trump: u8,
    pts: [i32; 2],
    brisques: [i32; 2],
    double_done: [bool; 2],
    leader: Player,
    table: Vec<u8>,
    phase: Phase,
    hand_no: u8,
    rng: Rng,
    viewer: Viewer,
    log: String,
}

impl Default for Bezique {
    fn default() -> Self {
        Self::new()
    }
}

impl Bezique {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Bezique {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            trump: 0,
            pts: [0; 2],
            brisques: [0; 2],
            double_done: [false; 2],
            leader: Player::One,
            table: Vec::new(),
            phase: Phase::Lead,
            hand_no: 0,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..32u8).chain(0..32u8).collect();
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..8 {
            self.hands[0].push((d.pop().unwrap(), 0));
            self.hands[1].push((d.pop().unwrap(), 0));
        }
        let up = d.pop().unwrap();
        self.trump = suit(up);
        let mut stock = vec![up];
        stock.extend(d);
        self.stock = stock;
        self.brisques = [0; 2];
        self.double_done = [false; 2];
        self.table.clear();
        self.leader = if self.hand_no.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
        self.phase = Phase::Lead;
        if rank(up) == 0 {
            let dealer = self.leader.other().idx();
            self.pts[dealer] += 10;
            self.log = format!(
                "Atuul întors e un 7: dealerul J{} ia 10 puncte.",
                dealer + 1
            );
        }
    }

    fn follower(&self) -> Player {
        self.leader.other()
    }

    fn power(&self, c: u8) -> u8 {
        rank(c)
    }

    fn beats(&self, led: u8, reply: u8) -> bool {
        if suit(reply) == suit(led) {
            self.power(reply) > self.power(led)
        } else {
            suit(reply) == self.trump
        }
    }

    fn follow_legal(&self, hand: &Hand, led: u8) -> Vec<u8> {
        let all: Vec<u8> = hand.iter().map(|x| x.0).collect();
        if !self.stock.is_empty() {
            return all;
        }
        let same: Vec<u8> = all
            .iter()
            .copied()
            .filter(|&c| suit(c) == suit(led))
            .collect();
        if !same.is_empty() {
            let better: Vec<u8> = same
                .iter()
                .copied()
                .filter(|&c| self.power(c) > self.power(led))
                .collect();
            return if better.is_empty() { same } else { better };
        }
        let tr: Vec<u8> = all
            .iter()
            .copied()
            .filter(|&c| suit(c) == self.trump)
            .collect();
        if !tr.is_empty() {
            return tr;
        }
        all
    }

    /// Cărțile necesare unei figuri, ca (culoare, rang).
    fn required(&self, m: Meld) -> Vec<u8> {
        let t = self.trump;
        match m {
            Meld::Marriage(s) => vec![card(s, 5), card(s, 4)],
            Meld::Sequence => vec![card(t, 7), card(t, 6), card(t, 5), card(t, 4), card(t, 3)],
            Meld::Four(r) => (0..4).map(|s| card(s, r)).collect(),
            Meld::Bezique => vec![card(2, 4), card(1, 3)],
            Meld::Double => vec![card(2, 4), card(2, 4), card(1, 3), card(1, 3)],
            Meld::Dix => vec![card(t, 0)],
        }
    }

    fn class(m: Meld) -> u8 {
        match m {
            Meld::Marriage(_) => CL_M,
            Meld::Sequence => CL_S,
            Meld::Four(_) => CL_F,
            Meld::Bezique => CL_B,
            _ => 0,
        }
    }

    fn value(&self, m: Meld) -> i32 {
        match m {
            Meld::Marriage(s) => {
                if s == self.trump {
                    40
                } else {
                    20
                }
            }
            Meld::Sequence => 250,
            Meld::Four(r) => match r {
                7 => 100,
                5 => 80,
                4 => 60,
                _ => 40,
            },
            Meld::Bezique => 40,
            Meld::Double => 500,
            Meld::Dix => 10,
        }
    }

    /// Alege indicii din mână pentru figură; `None` dacă nu se poate.
    fn assign(&self, p: usize, m: Meld) -> Option<Vec<usize>> {
        let h = &self.hands[p];
        let cls = Self::class(m);
        for prefer_shown in [true, false] {
            let mut used: Vec<usize> = Vec::new();
            let mut ok = true;
            for need in self.required(m) {
                let mut cand: Vec<usize> = (0..h.len())
                    .filter(|&i| {
                        h[i].0 == need
                            && !used.contains(&i)
                            && (m == Meld::Double || cls == 0 || h[i].1 & cls == 0)
                    })
                    .collect();
                cand.sort_by_key(|&i| {
                    let shown = h[i].1 & SHOWN != 0;
                    if shown == prefer_shown {
                        0
                    } else {
                        1
                    }
                });
                match cand.first() {
                    Some(&i) => used.push(i),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok && used.iter().any(|&i| h[i].1 & SHOWN == 0) {
                return Some(used);
            }
        }
        None
    }

    fn melds(&self, p: usize) -> Vec<Meld> {
        let mut v = Vec::new();
        for s in 0..4u8 {
            v.push(Meld::Marriage(s));
        }
        v.push(Meld::Sequence);
        for r in [7u8, 5, 4, 3] {
            v.push(Meld::Four(r));
        }
        v.push(Meld::Bezique);
        if !self.double_done[p] {
            v.push(Meld::Double);
        }
        if self.stock.len() > 1 {
            v.push(Meld::Dix);
        }
        v.into_iter()
            .filter(|&m| self.assign(p, m).is_some())
            .collect()
    }

    fn check_target(&mut self) {
        if self.pts.iter().any(|&x| x >= TARGET) {
            self.phase = Phase::Over;
        }
    }

    fn end_hand(&mut self) {
        for p in 0..2 {
            self.pts[p] += self.brisques[p] * 10;
        }
        self.hand_no += 1;
        if self.pts.iter().any(|&x| x >= TARGET) || self.hand_no >= HANDS {
            self.phase = Phase::Over;
        } else {
            self.deal();
        }
    }

    fn hand_value(&self, p: Player) -> i32 {
        self.hands[p.idx()]
            .iter()
            .map(|&(c, _)| {
                let mut v = if is_brisque(c) { 5 } else { 0 };
                if suit(c) == self.trump {
                    v += 3;
                }
                v + rank(c) as i32 / 2
            })
            .sum()
    }
}

impl Game for Bezique {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Lead | Phase::Meld => self.leader,
            Phase::Follow => self.follower(),
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Draw => vec![Move::Draw],
            Phase::Lead => {
                let mut v: Vec<u8> = self.hands[self.leader.idx()].iter().map(|x| x.0).collect();
                v.sort_unstable();
                v.dedup();
                v.into_iter().map(Move::Play).collect()
            }
            Phase::Follow => {
                let mut v = self.follow_legal(&self.hands[self.follower().idx()], self.table[0]);
                v.sort_unstable();
                v.dedup();
                v.into_iter().map(Move::Play).collect()
            }
            Phase::Meld => {
                let mut v: Vec<Move> = self
                    .melds(self.leader.idx())
                    .into_iter()
                    .map(Move::Declare)
                    .collect();
                v.push(Move::NoMeld);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Play(c) => match self.phase {
                Phase::Lead => {
                    let l = self.leader.idx();
                    let k = self.hands[l].iter().position(|x| x.0 == c).unwrap();
                    self.hands[l].remove(k);
                    self.table = vec![c];
                    if suit(c) == self.trump && rank(c) == 0 {
                        self.pts[l] += 10;
                    }
                    self.phase = Phase::Follow;
                    self.check_target();
                }
                Phase::Follow => {
                    let f = self.follower().idx();
                    let k = self.hands[f].iter().position(|x| x.0 == c).unwrap();
                    self.hands[f].remove(k);
                    if suit(c) == self.trump && rank(c) == 0 {
                        self.pts[f] += 10;
                    }
                    let led = self.table[0];
                    let win = if self.beats(led, c) {
                        self.follower()
                    } else {
                        self.leader
                    };
                    let w = win.idx();
                    self.brisques[w] += is_brisque(led) as i32 + is_brisque(c) as i32;
                    self.log = format!(
                        "{} contra {}: levata e a lui J{}.",
                        text(led),
                        text(c),
                        w + 1
                    );
                    self.leader = win;
                    self.table.clear();
                    if self.phase != Phase::Over {
                        if self.hands[0].is_empty() && self.hands[1].is_empty() {
                            self.pts[w] += 10;
                            self.end_hand();
                        } else if !self.stock.is_empty() {
                            self.phase = Phase::Meld;
                        } else {
                            self.phase = Phase::Lead;
                        }
                    }
                    if self.phase != Phase::Over {
                        self.check_target();
                    }
                }
                _ => {}
            },
            Move::Declare(meld) => {
                let l = self.leader.idx();
                let idx = self.assign(l, meld).unwrap();
                match meld {
                    Meld::Dix => {
                        let up = self.stock[0];
                        let i = idx[0];
                        self.hands[l][i] = (up, 0);
                        self.stock[0] = card(self.trump, 0);
                    }
                    _ => {
                        for &i in &idx {
                            self.hands[l][i].1 |= SHOWN | Self::class(meld);
                        }
                    }
                }
                if meld == Meld::Double {
                    self.double_done[l] = true;
                }
                self.pts[l] += self.value(meld);
                self.log = format!("J{} declară {:?}: +{}.", l + 1, meld, self.value(meld));
                self.phase = Phase::Draw;
                self.check_target();
            }
            Move::NoMeld => self.phase = Phase::Draw,
            Move::Draw => {
                let w = self.leader;
                let a = self.stock.pop().unwrap();
                let b = self.stock.pop().unwrap();
                self.hands[w.idx()].push((a, 0));
                self.hands[w.other().idx()].push((b, 0));
                self.phase = Phase::Lead;
            }
        }
        if self.phase == Phase::Draw && self.stock.is_empty() {
            self.phase = Phase::Lead;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut v = (self.pts[p.idx()] - self.pts[q.idx()]) * 10
            + (self.brisques[p.idx()] - self.brisques[q.idx()]) * 80;
        v += self.hand_value(p) * 2;
        if let (Phase::Follow, Some(&led)) = (self.phase, self.table.first()) {
            if self.leader == p {
                v -= if is_brisque(led) { 60 } else { 10 }
                    + if suit(led) == self.trump { 40 } else { 0 }
                    + rank(led) as i32 * 4;
            }
        }
        v
    }

    fn render(&self) -> String {
        let me = self.viewer.who(self.to_move());
        let mut s = format!(
            "Bézique — mâna {}/{HANDS}. Atu: {}. Pachet: {}. Puncte: J1 {}, J2 {} (brisque în levate: {}–{}).\n",
            (self.hand_no + 1).min(HANDS),
            ["cupă ♥", "caro ♦", "pică ♠", "treflă ♣"][self.trump as usize],
            self.stock.len(),
            self.pts[0],
            self.pts[1],
            self.brisques[0],
            self.brisques[1]
        );
        if let Viewer::HotSeat = self.viewer {
            if self.phase == Phase::Follow {
                s.push_str(&handover_banner(self.follower()));
            }
        }
        if !self.table.is_empty() {
            s.push_str(&format!(
                "Pe masă: {} (condus de J{})\n",
                text(self.table[0]),
                self.leader.idx() + 1
            ));
        }
        for p in 0..2 {
            let mut hidden: Vec<String> = Vec::new();
            let mut shown: Vec<String> = Vec::new();
            let mut h = self.hands[p].clone();
            h.sort_unstable();
            for &(c, f) in &h {
                if f & SHOWN != 0 {
                    shown.push(text(c));
                } else {
                    hidden.push(text(c));
                }
            }
            if p == me.idx() {
                s.push_str(&format!(
                    "J{} (mâna ta): {}  | declarate pe masă: {}\n",
                    p + 1,
                    hidden.join(" "),
                    if shown.is_empty() {
                        "—".into()
                    } else {
                        shown.join(" ")
                    }
                ));
            } else {
                s.push_str(&format!(
                    "J{}: {} cărți ascunse; declarate pe masă: {}\n",
                    p + 1,
                    hidden.len(),
                    if shown.is_empty() {
                        "—".into()
                    } else {
                        shown.join(" ")
                    }
                ));
            }
        }
        match self.phase {
            Phase::Meld => s.push_str(&format!(
                "J{} poate declara o figură sau `fara`.\n",
                self.leader.idx() + 1
            )),
            Phase::Draw => s.push_str("Se trag cărți: scrie «trag».\n"),
            Phase::Over => s.push_str("Joc încheiat.\n"),
            _ => {}
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => text(*c),
            Move::Declare(d) => match d {
                Meld::Marriage(s) => format!("mariaj {}", SUITS[*s as usize]),
                Meld::Sequence => "secventa".into(),
                Meld::Four(r) => format!("patru {}", RANKS[*r as usize]),
                Meld::Bezique => "bezic".into(),
                Meld::Double => "dublubezic".into(),
                Meld::Dix => "dix".into(),
            },
            Move::NoMeld => "fara".into(),
            Move::Draw => "trag".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_is_64_cards_and_melds_need_a_hidden_card() {
        let g = Bezique::with_seed(3);
        assert_eq!(g.hands[0].len(), 8);
        assert_eq!(g.stock.len(), 64 - 16);
        let mut g = Bezique::with_seed(4);
        g.trump = 2;
        g.stock = vec![card(0, 1); 10];
        g.hands[0] = vec![(card(0, 5), 0), (card(0, 4), 0), (card(1, 5), 0)];
        assert!(g.melds(0).contains(&Meld::Marriage(0)));
        g.hands[0][0].1 = SHOWN | CL_M;
        g.hands[0][1].1 = SHOWN | CL_M;
        assert!(!g.melds(0).contains(&Meld::Marriage(0)));
    }

    #[test]
    fn meld_values_and_bezique_pairs() {
        let mut g = Bezique::with_seed(5);
        g.trump = 2; // pică
        g.stock = vec![card(0, 1); 10];
        g.hands[0] = vec![
            (card(2, 5), 0),
            (card(2, 4), 0),
            (card(1, 3), 0),
            (card(2, 4), 0),
            (card(1, 3), 0),
        ];
        g.phase = Phase::Meld;
        g.leader = Player::One;
        let ms = g.melds(0);
        assert!(
            ms.contains(&Meld::Marriage(2))
                && ms.contains(&Meld::Bezique)
                && ms.contains(&Meld::Double)
        );
        assert_eq!(g.value(Meld::Marriage(2)), 40);
        assert_eq!(g.value(Meld::Marriage(0)), 20);
        g.apply(&Move::Declare(Meld::Double));
        assert_eq!(g.pts[0], 500);
        assert!(!g.melds(0).contains(&Meld::Double));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=12u64 {
            let mut g = Bezique::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn hidden_cards_and_ai_legal() {
        let mut g = Bezique::with_seed(7);
        g.set_viewer(Some(Player::One));
        let a = g.render();
        g.set_viewer(Some(Player::Two));
        assert_ne!(a, g.render());
        let m = polytabletop_core::GameObj::ai_move(&g, 1).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }
}
