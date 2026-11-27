//! Tablanet (Table), joc românesc de capturat cărți, 2 jucători, din regulile publice.
//!
//! * Pachet de 52 de cărți. Se pun 4 cărți pe masă, apoi se dau câte 6 cărți fiecăruia; când
//!   mâinile se golesc se mai dau câte 6, până se termină pachetul (4 runde de 6).
//! * Valori: A = 1 sau 11, 2–10 = valoarea numerică. J, Q, K nu se adună: capturează doar
//!   cărți de același rang.
//! * La rândul tău joci o carte. Dacă poate captura (carte de aceeași valoare sau grupuri de
//!   cărți de pe masă care se adună la valoarea ei), ești obligat să captezi, și anume numărul
//!   maxim posibil de cărți (puteți alege între capturi de aceeași mărime). Dacă nu poți
//!   captura, lași cartea pe masă.
//! * Masa curățată printr-o captură = „tablă” (1 punct), dar nu la ultima captură a jocului.
//!   Ultimul care a capturat ia cărțile rămase pe masă la sfârșit.
//! * Puncte: cele mai multe cărți 3 (egalitate: nimic), cele mai multe treflă 1, 10♦ = 2,
//!   2♣ = 1, fiecare As = 1, fiecare tablă 1. Câștigă cine are mai multe puncte.
//!
//! Mutări: `7h` (lași cartea, dacă nu poți captura), `7h x 3c 4d` (capturezi), `imparte`.
//! Informație ascunsă: AI-ul citește mâna proprie, masa și cărțile capturate (publice).

use crate::cards::Viewer;
use crate::cards::{full_deck, join_cards, shuffle, Card};
use crate::cardsb_util::sort_hand;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Play,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Place(Card),
    Capture(Card, Vec<Card>),
}

#[derive(Clone)]
pub struct Tablanet {
    stock: Vec<Card>,
    hands: [Vec<Card>; 2],
    table: Vec<Card>,
    captured: [Vec<Card>; 2],
    tablas: [u8; 2],
    last_capture: Option<Player>,
    side: Player,
    first: Player,
    phase: Phase,
    log: String,
    viewer: Viewer,
}

impl Default for Tablanet {
    fn default() -> Self {
        Self::new()
    }
}

fn is_face(c: Card) -> bool {
    (11..=13).contains(&c.rank)
}

fn options(c: Card) -> &'static [u8] {
    match c.rank {
        14 => &[1, 11],
        2..=10 => match c.rank {
            2 => &[2],
            3 => &[3],
            4 => &[4],
            5 => &[5],
            6 => &[6],
            7 => &[7],
            8 => &[8],
            9 => &[9],
            _ => &[10],
        },
        _ => &[],
    }
}

/// Toate submulțimile nevide ale mesei (măști) capturabile de `c`.
fn capture_masks(c: Card, table: &[Card]) -> Vec<u32> {
    let n = table.len();
    if n == 0 || n > 24 {
        return Vec::new();
    }
    let mut out: HashSet<u32> = HashSet::new();
    if is_face(c) {
        // figuri: doar aceeași valoare, orice submulțime nevidă
        let same: Vec<usize> = (0..n).filter(|&i| table[i].rank == c.rank).collect();
        for sub in 1u32..(1 << same.len()) {
            let mut m = 0u32;
            for (k, &i) in same.iter().enumerate() {
                if sub & (1 << k) != 0 {
                    m |= 1 << i;
                }
            }
            out.insert(m);
        }
    } else {
        for &target in options(c) {
            // grupuri care se adună la `target`
            let mut groups: Vec<u32> = Vec::new();
            fn dfs(
                table: &[Card],
                i: usize,
                sum_opts: &[u32],
                mask: u32,
                target: u32,
                groups: &mut Vec<u32>,
            ) {
                if mask != 0 && sum_opts.contains(&target) {
                    groups.push(mask);
                }
                for j in i..table.len() {
                    let mut next: Vec<u32> = Vec::new();
                    let c = table[j];
                    if is_face(c) {
                        continue;
                    }
                    for &s in sum_opts {
                        for &o in options(c) {
                            let v = s + o as u32;
                            if v <= target && !next.contains(&v) {
                                next.push(v);
                            }
                        }
                    }
                    if !next.is_empty() {
                        dfs(table, j + 1, &next, mask | (1 << j), target, groups);
                    }
                }
            }
            dfs(table, 0, &[0], 0, target as u32, &mut groups);
            // uniuni disjuncte de grupuri
            let mut reach: HashSet<u32> = HashSet::new();
            reach.insert(0);
            for &g in &groups {
                let cur: Vec<u32> = reach.iter().copied().collect();
                for r in cur {
                    if r & g == 0 {
                        reach.insert(r | g);
                    }
                }
                if reach.len() > 20000 {
                    break;
                }
            }
            for r in reach {
                if r != 0 {
                    out.insert(r);
                }
            }
        }
    }
    let mut v: Vec<u32> = out.into_iter().collect();
    v.sort_unstable();
    v
}

/// Capturile permise (numărul maxim de cărți).
fn best_captures(c: Card, table: &[Card]) -> Vec<u32> {
    let all = capture_masks(c, table);
    let max = all.iter().map(|m| m.count_ones()).max().unwrap_or(0);
    all.into_iter().filter(|m| m.count_ones() == max).collect()
}

fn mask_cards(table: &[Card], mask: u32) -> Vec<Card> {
    (0..table.len())
        .filter(|&i| mask & (1 << i) != 0)
        .map(|i| table[i])
        .collect()
}

/// Punctele finale pentru ambii jucători.
pub fn final_points(cap: &[Vec<Card>; 2], tablas: [u8; 2]) -> [i32; 2] {
    let mut p = [0i32; 2];
    for k in 0..2 {
        for c in &cap[k] {
            if c.rank == 14 {
                p[k] += 1;
            }
            if c.rank == 10 && c.suit == 1 {
                p[k] += 2;
            }
            if c.rank == 2 && c.suit == 3 {
                p[k] += 1;
            }
        }
        p[k] += tablas[k] as i32;
    }
    match cap[0].len().cmp(&cap[1].len()) {
        std::cmp::Ordering::Greater => p[0] += 3,
        std::cmp::Ordering::Less => p[1] += 3,
        _ => {}
    }
    let cl = |k: usize| cap[k].iter().filter(|c| c.suit == 3).count();
    match cl(0).cmp(&cl(1)) {
        std::cmp::Ordering::Greater => p[0] += 1,
        std::cmp::Ordering::Less => p[1] += 1,
        _ => {}
    }
    p
}

impl Tablanet {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d = full_deck(2);
        shuffle(&mut d, &mut rng);
        let table: Vec<Card> = (0..4).map(|_| d.pop().unwrap()).collect();
        let mut g = Tablanet {
            stock: d,
            hands: [Vec::new(), Vec::new()],
            table,
            captured: [Vec::new(), Vec::new()],
            tablas: [0, 0],
            last_capture: None,
            side: Player::One,
            first: Player::One,
            phase: Phase::Deal,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn points(&self) -> (i32, i32) {
        let p = final_points(&self.captured, self.tablas);
        (p[0], p[1])
    }

    fn deal(&mut self) {
        for _ in 0..6 {
            for p in 0..2 {
                let c = self.stock.pop().unwrap();
                self.hands[p].push(c);
            }
        }
        sort_hand(&mut self.hands[0]);
        sort_hand(&mut self.hands[1]);
        self.phase = Phase::Play;
        self.side = self.first;
    }

    fn take_cards(&mut self, p: Player, cards: &[Card]) {
        self.table.retain(|c| !cards.contains(c));
        self.captured[p.idx()].extend_from_slice(cards);
    }
}

impl Game for Tablanet {
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
            Phase::Deal => vec![Move::Deal],
            Phase::Play => {
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    let caps = best_captures(c, &self.table);
                    if caps.is_empty() {
                        v.push(Move::Place(c));
                    } else {
                        for m in caps {
                            v.push(Move::Capture(c, mask_cards(&self.table, m)));
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Deal => self.deal(),
            Move::Place(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.table.push(*c);
                self.advance();
            }
            Move::Capture(c, cards) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.captured[p.idx()].push(*c);
                self.take_cards(p, cards);
                self.last_capture = Some(p);
                if self.table.is_empty() {
                    self.log = format!("Tablă pentru J{}!", p.idx() + 1);
                    let last = self.stock.is_empty()
                        && self.hands[0].is_empty()
                        && self.hands[1].is_empty();
                    if !last {
                        self.tablas[p.idx()] += 1;
                    }
                }
                self.advance();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let p = final_points(&self.captured, self.tablas);
        Some(match p[0].cmp(&p[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let pts = final_points(&self.captured, self.tablas);
        let mut s = 100 * (pts[p.idx()] - pts[q.idx()]);
        // cărți / treflă brute (mai multe = mai bine)
        s += 3 * (self.captured[p.idx()].len() as i32 - self.captured[q.idx()].len() as i32);
        // pericol: cărțile de pe masă pe care le poate lua adversarul (valori necunoscute, 30%)
        if self.phase == Phase::Play && self.side != p && self.table.len() <= 14 {
            let mut threat = 0;
            for r in 2..=14u8 {
                let probe = Card { rank: r, suit: 0 };
                let caps = best_captures(probe, &self.table);
                if let Some(&m) = caps.first() {
                    let cs = mask_cards(&self.table, m);
                    let v: i32 = cs
                        .iter()
                        .map(|c| {
                            2 + if c.rank == 14 { 8 } else { 0 }
                                + if c.rank == 10 && c.suit == 1 { 16 } else { 0 }
                                + if c.rank == 2 && c.suit == 3 { 8 } else { 0 }
                        })
                        .sum();
                    threat += v;
                }
            }
            s -= threat * 3 / 10;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Tablanet\n");
        let pts = final_points(&self.captured, self.tablas);
        s.push_str(&format!(
            "Capturate: J1 {} cărți ({} puncte provizorii, {} table), J2 {} cărți ({} puncte, {} table) | stoc: {}\n",
            self.captured[0].len(),
            pts[0],
            self.tablas[0],
            self.captured[1].len(),
            pts[1],
            self.tablas[1],
            self.stock.len()
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!("Masa: {}\n", join_cards(&self.table)));
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Place(c) => c.text(),
            Move::Capture(c, cs) => format!(
                "{} x {}",
                c.text(),
                cs.iter().map(|c| c.text()).collect::<Vec<_>>().join(" ")
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

impl Tablanet {
    fn advance(&mut self) {
        if self.hands[0].is_empty() && self.hands[1].is_empty() {
            if self.stock.is_empty() {
                if let Some(l) = self.last_capture {
                    let rest = std::mem::take(&mut self.table);
                    self.captured[l.idx()].extend(rest);
                }
                self.phase = Phase::Over;
                return;
            }
            self.phase = Phase::Deal;
        }
        self.side = self.side.other();
        if self.phase == Phase::Deal {
            self.first = self.first.other();
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
    fn sums_and_face_captures() {
        let table = [c(3, 0), c(4, 1), c(7, 2), c(12, 3), c(2, 0), c(5, 1)];
        let caps = best_captures(c(7, 0), &table);
        // 7 | 3+4 | 2+5 -> maxim: {3,4,7,2,5} = 5 cărți
        assert_eq!(caps.len(), 1);
        assert_eq!(caps[0].count_ones(), 5);
        // J nu se adună
        assert!(best_captures(c(11, 0), &table).is_empty());
        assert_eq!(best_captures(c(12, 0), &table).len(), 1);
    }

    #[test]
    fn ace_is_one_or_eleven() {
        let table = [c(5, 0), c(6, 1)];
        // A (11) ia 5+6
        assert_eq!(best_captures(c(14, 2), &table).len(), 1);
        let table2 = [c(14, 0), c(9, 1)];
        // A(1) ia A-ul de pe masă (1) ; 9 nu
        let caps = best_captures(c(14, 2), &table2);
        assert_eq!(caps.len(), 1);
        assert_eq!(caps[0].count_ones(), 1);
    }

    #[test]
    fn scoring() {
        let mut cap = [Vec::new(), Vec::new()];
        cap[0] = vec![c(10, 1), c(14, 0), c(2, 3), c(3, 3)];
        cap[1] = vec![c(14, 1)];
        let p = final_points(&cap, [1, 0]);
        // J1: 10♦=2, A=1, 2♣=1, tablă 1, cărți 3, treflă 1 -> 9; J2: A 1
        assert_eq!(p, [9, 1]);
    }

    #[test]
    fn conserve_52() {
        let g = Tablanet::with_seed(2);
        assert_eq!(
            g.table.len() + g.stock.len() + g.hands[0].len() + g.hands[1].len(),
            52
        );
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..=12u64 {
            let mut g = Tablanet::with_seed(seed);
            let mut rng = Rng::new(seed + 9);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                let total = g.table.len()
                    + g.stock.len()
                    + g.hands[0].len()
                    + g.hands[1].len()
                    + g.captured[0].len()
                    + g.captured[1].len();
                assert_eq!(total, 52);
                n += 1;
            }
            assert!(g.outcome().is_some());
            assert!(g.table.is_empty());
            assert_eq!(g.captured[0].len() + g.captured[1].len(), 52);
        }
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_hand_and_stock_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "tablanet",
            Tablanet::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                crate::p3_comun::mix_unseen(&mut g.hands[h], &mut g.stock, 0, s);
                sort_hand(&mut g.hands[h]);
            },
            6,
        );
    }
}
