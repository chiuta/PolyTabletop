//! Expediții — joc de cărți pentru doi, variantă originală (mecanică de „expediții în culori”).
//!
//! Cinci masive muntoase (B Bucegi, F Făgăraș, R Retezat, D Rodna, C Ceahlău), fiecare cu
//! cărți 2–10 și trei cărți „angajament” (x). Pachet de 60, 8 cărți în mână.
//!
//! O tură: (1) joci o carte pe expediția ta (valori strict crescătoare; angajamentele doar
//! înaintea numerelor) sau o arunci pe grămada culorii ei; (2) tragi din pachet sau iei
//! vârful unei grămezi de aruncate (nu cea pe care tocmai ai aruncat). Jocul se termină când
//! se trage ultima carte din pachet. Scor pe expediție începută:
//! (suma − 20) × (1 + angajamente) și +20 dacă are cel puțin 8 cărți.
//!
//! Informație ascunsă: vezi doar mâna Jucătorului 1. AI-ul evaluează doar ce vede (nu
//! citește pachetul, nici mâna adversarului; cartea trasă din pachet nu intră în evaluare).
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// După atâtea ture la rând fără tras din pachet, grămezile se „îngheață” și
/// se trage obligatoriu din pachet (jocul avansează spre final). Regulă anti-blocaj.
const STALL_LIMIT: u8 = 8;

pub const COLORS: [char; 5] = ['B', 'F', 'R', 'D', 'C'];
pub const COLOR_NAMES: [&str; 5] = ["Bucegi", "Făgăraș", "Retezat", "Rodna", "Ceahlău"];

/// Carte: culoare 0..5, valoare 0 (angajament) sau 2..=10.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct C {
    pub c: u8,
    pub v: u8,
}

impl C {
    pub fn text(self) -> String {
        let v = if self.v == 0 {
            "x".to_string()
        } else {
            self.v.to_string()
        };
        format!("{}{}", COLORS[self.c as usize], v)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(C),
    Discard(C),
    DrawDeck,
    DrawPile(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Act,
    Draw,
}

#[derive(Clone)]
pub struct Expeditii {
    deck: Vec<C>,
    hands: [Vec<C>; 2],
    exp: [[Vec<C>; 5]; 2],
    piles: [Vec<C>; 5],
    side: Player,
    phase: Phase,
    just_discarded: Option<u8>,
    /// Cartea trasă din pachet în ultima mutare (necunoscută la evaluare).
    fresh: [bool; 2],
    /// Ture consecutive încheiate fără tras din pachet (regulă anti-blocaj).
    stall: u8,
    over: bool,
    /// `Some(p)` = render shows `p`'s private cards; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Expeditii {
    fn default() -> Self {
        Self::new()
    }
}

pub fn exp_score(cards: &[C]) -> i32 {
    if cards.is_empty() {
        return 0;
    }
    let wagers = cards.iter().filter(|c| c.v == 0).count() as i32;
    let sum: i32 = cards.iter().map(|c| c.v as i32).sum();
    let mut s = (sum - 20) * (1 + wagers);
    if cards.len() >= 8 {
        s += 20;
    }
    s
}

fn can_play_on(cards: &[C], c: C) -> bool {
    let top = cards.iter().filter(|x| x.v != 0).map(|x| x.v).max();
    match top {
        None => true,
        Some(t) => c.v != 0 && c.v > t,
    }
}

impl Expeditii {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = Vec::new();
        for c in 0..5u8 {
            for _ in 0..3 {
                deck.push(C { c, v: 0 });
            }
            for v in 2..=10u8 {
                deck.push(C { c, v });
            }
        }
        shuffle(&mut deck, &mut rng);
        let mut h0 = Vec::new();
        let mut h1 = Vec::new();
        for _ in 0..8 {
            h0.push(deck.pop().unwrap());
            h1.push(deck.pop().unwrap());
        }
        let mut g = Expeditii {
            deck,
            hands: [h0, h1],
            exp: Default::default(),
            piles: Default::default(),
            side: Player::One,
            phase: Phase::Act,
            just_discarded: None,
            fresh: [false; 2],
            stall: 0,
            over: false,
            viewer: Some(Player::One),
        };
        g.sort_hands();
        g
    }

    fn sort_hands(&mut self) {
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.c, c.v));
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        let f = |p: usize| (0..5).map(|c| exp_score(&self.exp[p][c])).sum::<i32>();
        (f(0), f(1))
    }

    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.hands.iter().map(|h| h.len()).sum::<usize>()
            + self.exp.iter().flatten().map(|v| v.len()).sum::<usize>()
            + self.piles.iter().map(|v| v.len()).sum::<usize>()
    }

    fn heuristic(&self, p: usize) -> i32 {
        let o = 1 - p;
        let mut s = 0;
        let known = self.hands[p].len() - usize::from(self.fresh[p]);
        for c in 0..5usize {
            let cur = exp_score(&self.exp[p][c]);
            // Potențial: ce am în mână și se poate juca în ordine crescătoare.
            let top = self.exp[p][c]
                .iter()
                .filter(|x| x.v != 0)
                .map(|x| x.v)
                .max()
                .unwrap_or(0);
            let mut extra: Vec<C> = self.hands[p][..known]
                .iter()
                .copied()
                .filter(|x| x.c as usize == c && x.v > top)
                .collect();
            extra.sort_by_key(|x| x.v);
            let mut all = self.exp[p][c].clone();
            all.extend(extra.iter().copied());
            let pot = exp_score(&all);
            if self.exp[p][c].is_empty() {
                // Expediție neînceput: o încep doar dacă mâna o susține.
                if pot > 0 {
                    s += pot / 3;
                }
            } else {
                s += cur + (pot - cur).max(0) / 2;
            }
            // Cărți din mână pe care adversarul le-ar putea valorifica (aruncate = cadou).
            let _ = o;
        }
        for c in &self.hands[p][..known] {
            if self.exp[p][c.c as usize].is_empty() && c.v == 0 {
                s -= 1;
            }
        }
        if self.fresh[p] {
            s += 3;
        }
        s
    }
}

impl Game for Expeditii {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        match self.phase {
            Phase::Act => {
                let mut seen: Vec<C> = Vec::new();
                for &c in &self.hands[p] {
                    if seen.contains(&c) {
                        continue;
                    }
                    seen.push(c);
                    if can_play_on(&self.exp[p][c.c as usize], c) {
                        v.push(Move::Play(c));
                    }
                    v.push(Move::Discard(c));
                }
            }
            Phase::Draw => {
                if !self.deck.is_empty() {
                    v.push(Move::DrawDeck);
                }
                for c in 0..5u8 {
                    if self.stall < STALL_LIMIT
                        && !self.piles[c as usize].is_empty()
                        && self.just_discarded != Some(c)
                    {
                        v.push(Move::DrawPile(c));
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.fresh = [false; 2];
        match *m {
            Move::Play(c) | Move::Discard(c) => {
                let i = self.hands[p]
                    .iter()
                    .position(|x| *x == c)
                    .expect("card in hand");
                self.hands[p].remove(i);
                if let Move::Play(_) = m {
                    self.exp[p][c.c as usize].push(c);
                    self.just_discarded = None;
                } else {
                    self.piles[c.c as usize].push(c);
                    self.just_discarded = Some(c.c);
                }
                self.phase = Phase::Draw;
            }
            Move::DrawDeck => {
                let c = self.deck.pop().expect("deck");
                self.hands[p].push(c);
                self.fresh[p] = true;
                self.stall = 0;
                self.end_turn();
            }
            Move::DrawPile(col) => {
                let c = self.piles[col as usize].pop().expect("pile");
                self.hands[p].push(c);
                self.stall += 1;
                self.end_turn();
            }
        }
        self.sort_hands();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = self.scores();
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (self.heuristic(p.idx()), self.heuristic(p.other().idx()));
        // Pentru adversar nu cunoaștem mâna: contează doar expedițiile jucate.
        let _ = b;
        let opp: i32 = (0..5)
            .map(|c| exp_score(&self.exp[p.other().idx()][c]))
            .sum();
        a - opp
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let (a, b) = self.scores();
        s.push_str(&format!(
            "Expediții — pachet: {} | scor curent: J1 {a}, J2 {b} | mută J{}\n",
            self.deck.len(),
            self.side.idx() + 1
        ));
        for c in 0..5usize {
            let fmt = |p: usize| {
                if self.exp[p][c].is_empty() {
                    "—".to_string()
                } else {
                    self.exp[p][c]
                        .iter()
                        .map(|x| x.text())
                        .collect::<Vec<_>>()
                        .join(" ")
                }
            };
            let top = self.piles[c]
                .last()
                .map(|x| x.text())
                .unwrap_or_else(|| "—".into());
            s.push_str(&format!(
                "{} {:<8} J1: {:<28} J2: {:<28} aruncate: {} ({})\n",
                COLORS[c],
                COLOR_NAMES[c],
                fmt(0),
                fmt(1),
                top,
                self.piles[c].len()
            ));
        }
        let v = self.viewer.unwrap_or(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            self.hands[v.idx()]
                .iter()
                .map(|x| x.text())
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str(&format!(
            "Adversarul are {} cărți.\n",
            self.hands[v.other().idx()].len()
        ));
        if self.stall >= STALL_LIMIT {
            s.push_str("Anti-blocaj: grămezile sunt înghețate, se trage din pachet.\n");
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
        } else if self.phase == Phase::Act {
            s.push_str("Fază: joacă (juc B5) sau aruncă (arunc B5).\n");
        } else {
            s.push_str("Fază: trage (trag) sau ia de pe grămadă (ia F).\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => format!("juc {}", c.text()),
            Move::Discard(c) => format!("arunc {}", c.text()),
            Move::DrawDeck => "trag".into(),
            Move::DrawPile(c) => format!("ia {}", COLORS[*c as usize]),
        }
    }
}

impl Expeditii {
    fn end_turn(&mut self) {
        if self.deck.is_empty() {
            self.over = true;
        }
        self.side = self.side.other();
        self.phase = Phase::Act;
        self.just_discarded = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring() {
        let c = |c, v| C { c, v };
        assert_eq!(exp_score(&[]), 0);
        assert_eq!(exp_score(&[c(0, 5)]), -15);
        // un angajament, 2+10+... = (22-20)*2
        assert_eq!(exp_score(&[c(0, 0), c(0, 10), c(0, 5), c(0, 7)]), 4);
        // 8 cărți: bonus 20
        let v: Vec<C> = (2..=9).map(|x| c(0, x)).collect();
        assert_eq!(exp_score(&v), (44 - 20) + 20);
    }

    #[test]
    fn ordering_rules() {
        let c = |v| C { c: 0, v };
        assert!(can_play_on(&[], c(3)));
        assert!(can_play_on(&[C { c: 0, v: 0 }], c(3)));
        assert!(!can_play_on(&[c(5)], c(0)));
        assert!(!can_play_on(&[c(5)], c(5)));
        assert!(can_play_on(&[c(5)], c(6)));
    }

    #[test]
    fn conservation_and_end() {
        for seed in 1..20 {
            let mut g = Expeditii::with_seed(seed);
            assert_eq!(g.total_cards(), 60);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_cards(), 60);
                n += 1;
                assert!(n < 2000);
            }
            assert!(g.deck.is_empty());
        }
    }

    #[test]
    fn no_pickup_of_just_discarded() {
        let mut g = Expeditii::with_seed(3);
        let c = g.hands[0][0];
        g.apply(&Move::Discard(c));
        assert!(!g.legal_moves().contains(&Move::DrawPile(c.c)));
    }

    #[test]
    fn ai_vs_ai_terminates() {
        for seed in 1..=20u64 {
            let mut g = Expeditii::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                g.apply(&m);
                n += 1;
                assert!(n < 3000, "seed {seed} does not terminate");
            }
        }
    }

    #[test]
    fn stall_forces_deck_draw() {
        let mut g = Expeditii::with_seed(9);
        g.apply(&Move::Discard(g.hands[0][0]));
        g.apply(&Move::DrawDeck);
        g.stall = STALL_LIMIT;
        let c = g.hands[1][0];
        g.apply(&Move::Discard(c));
        assert_eq!(g.legal_moves(), vec![Move::DrawDeck]);
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=30u64 {
            let mut g = Expeditii::with_seed(seed);
            let mut rng = Rng::new(seed + 100);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 24, "wins {wins}");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    fn hand_line(g: &Expeditii, p: Player) -> String {
        format!(
            "Mâna ta (J{}): {}\n",
            p.idx() + 1,
            g.hands[p.idx()]
                .iter()
                .map(|x| x.text())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    #[test]
    fn viewer_sees_only_own_hand() {
        let mut g = Expeditii::with_seed(5);
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(r.contains(&hand_line(&g, Player::One)), "{r}");
        assert!(!r.contains("Mâna ta (J2)"));
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(r.contains(&hand_line(&g, Player::Two)), "{r}");
        assert!(!r.contains("Mâna ta (J1)"));
        Game::set_viewer(&mut g, None);
        assert!(Game::render(&g).contains("Mâna ta (J1)"));
    }
}
