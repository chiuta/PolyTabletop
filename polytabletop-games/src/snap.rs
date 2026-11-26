//! Snap (Marea Britanie) — joc de cărți pentru copii, pe ture.
//!
//! Pachet de 52 de cărți amestecat, 26 fiecăruia, cu fața în jos. Pe rând întorci cartea de
//! deasupra pe grămada din mijloc (`intorc`). Când ultimele două cărți din mijloc au aceeași
//! valoare (de ex. două dame), cine e la mutare poate striga `snap` și ia toată grămada
//! (o pune sub pachetul lui). `snap` strigat fără pereche costă o carte (merge în grămada din mijloc).
//! Fiindcă în realitate contează viteza, aici: după ce întorci o carte care face pereche,
//! adversarul are prima șansă la `snap`; dacă o ratează (întoarce în loc să strige), perechea a trecut.
//! Cine rămâne fără cărți de întors pierde; după 400 de mutări câștigă cine are mai multe cărți.
//!
//! Pachetele sunt întoarse pentru ambii jucători, deci nimic nu e ascuns de un singur jucător.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

const PLY_CAP: u16 = 400;
const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Flip,
    Snap,
}

#[derive(Clone)]
pub struct Snap {
    stacks: [VecDeque<u8>; 2],
    center: Vec<u8>,
    side: Player,
    ply: u16,
    log: String,
}

impl Default for Snap {
    fn default() -> Self {
        Self::new()
    }
}

impl Snap {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d: Vec<u8> = (0..52).collect();
        for i in (1..52).rev() {
            let j = rng.below(i + 1);
            d.swap(i, j);
        }
        let b: VecDeque<u8> = d.split_off(26).into();
        Snap {
            stacks: [d.into(), b],
            center: Vec::new(),
            side: Player::One,
            ply: 0,
            log: String::new(),
        }
    }
    pub fn cards(&self, p: Player) -> usize {
        self.stacks[p.idx()].len()
    }
    pub fn center_len(&self) -> usize {
        self.center.len()
    }
    pub fn is_match(&self) -> bool {
        let n = self.center.len();
        n >= 2 && self.center[n - 1] % 13 == self.center[n - 2] % 13
    }
}

impl Game for Snap {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        if !self.stacks[p].is_empty() {
            v.push(Mv::Flip);
        }
        v.push(Mv::Snap);
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        self.ply += 1;
        match m {
            Mv::Flip => {
                let c = self.stacks[p].pop_front().unwrap();
                self.center.push(c);
                self.log = format!("J{} întoarce {}.", p + 1, RANKS[(c % 13) as usize]);
                self.side = self.side.other();
            }
            Mv::Snap => {
                if self.is_match() {
                    let n = self.center.len();
                    self.log = format!("SNAP! J{} ia {n} cărți.", p + 1);
                    let pile: Vec<u8> = std::mem::take(&mut self.center);
                    self.stacks[p].extend(pile);
                } else if let Some(c) = self.stacks[p].pop_front() {
                    self.log = format!("Snap fals: J{} pierde o carte.", p + 1);
                    self.center.push(c);
                } else {
                    self.log = format!("Snap fals al lui J{}.", p + 1);
                }
                self.side = self.side.other();
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.stacks[p.idx()].is_empty() && self.side == p {
                // nu mai are cărți de întors (dar poate lua o grămadă prin snap valid)
                if !self.is_match() {
                    return Some(Outcome::Win(p.other()));
                }
            }
        }
        if self.ply >= PLY_CAP {
            return Some(match self.stacks[0].len().cmp(&self.stacks[1].len()) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        10 * (self.stacks[p.idx()].len() as i32 - self.stacks[p.other().idx()].len() as i32)
    }
    fn render(&self) -> String {
        let top = |k: usize| {
            self.center
                .len()
                .checked_sub(k)
                .map_or("-", |i| RANKS[(self.center[i] % 13) as usize])
        };
        format!(
            "Pachetele: J1 {} cărți, J2 {} cărți. În mijloc: {} cărți (sus: {}, sub ea: {}){}.\n{}\nLa mutare: Jucătorul {}.",
            self.stacks[0].len(),
            self.stacks[1].len(),
            self.center.len(),
            top(1),
            top(2),
            if self.is_match() { " — PERECHE! poți striga snap" } else { "" },
            self.log,
            self.side.idx() + 1
        )
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Flip => "intorc".into(),
            Mv::Snap => "snap".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn snap_takes_the_pile_on_a_match_and_penalizes_a_false_call() {
        let mut g = Snap::with_seed(1);
        g.stacks = [
            VecDeque::from(vec![0, 1, 2]),
            VecDeque::from(vec![13, 5, 6]),
        ];
        g.center = vec![];
        GameObj::play(&mut g, "intorc").unwrap(); // J1: A
        GameObj::play(&mut g, "intorc").unwrap(); // J2: A (13 % 13 = 0)
        assert!(g.is_match());
        GameObj::play(&mut g, "snap").unwrap(); // J1 ia
        assert_eq!(g.cards(Player::One), 2 + 2);
        assert_eq!(g.center_len(), 0);
        let before = g.cards(Player::Two);
        GameObj::play(&mut g, "snap").unwrap(); // J2 snap fals
        assert_eq!(g.cards(Player::Two), before - 1);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Snap::with_seed(s)), 8, 500);
        ai_vs_random(|s| Box::new(Snap::with_seed(s)), 4, Player::One, 500);
        ai_vs_random(|s| Box::new(Snap::with_seed(s)), 5, Player::Two, 500);
    }
}
