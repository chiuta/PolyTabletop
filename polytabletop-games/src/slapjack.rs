//! Slapjack (SUA), joc de reflexe cu cărți pentru copii — adaptare pentru doi jucători pe ture.
//!
//! * Pachetul de 52 de cărți se împarte în două (26 fiecare, cu fața în jos). Pe rând, fiecare întoarce
//!   (`joc`) cartea de sus a pachetului său pe teancul din mijloc.
//! * Când apare un **valet** (J), celălalt jucător poate să dea cu palma (`palma`) și să ia tot teancul.
//!   Reflexele se simulează: palma celui care răspunde reușește cu probabilitate 65% (altfel o prinde
//!   întâi cel care a întors valetul și ia el teancul); rezolvarea e o mutare separată (`rezolv`).
//!   Poți să nu dai palma (`joc`): valetul e acoperit și nu mai poate fi luat.
//! * Palma pe altceva decât un valet e greșeală: dai cartea de sus din pachetul tău adversarului.
//! * Cine rămâne fără cărți la rândul lui pierde (dacă pe masă e un valet mai are o ultimă palmă).
//!   După 600 de mutări câștigă cine are mai multe cărți.
//!   Mutări: `joc`, `palma`, `rezolv`.

use crate::cards52::shuffle;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Flip,
    Slap,
    Resolve,
}

#[derive(Clone)]
pub struct Slapjack {
    piles: [VecDeque<u8>; 2],
    centre: Vec<u8>,
    side: Player,
    pending: bool,
    plies: u16,
    rng: Rng,
    log: String,
}

impl Default for Slapjack {
    fn default() -> Self {
        Self::new()
    }
}

const LABELS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];
const SLAP_WIN: usize = 65;

impl Slapjack {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d: Vec<u8> = (0..52u8).collect();
        shuffle(&mut d, &mut rng);
        let a: VecDeque<u8> = d[..26].iter().copied().collect();
        let b: VecDeque<u8> = d[26..].iter().copied().collect();
        Slapjack {
            piles: [a, b],
            centre: Vec::new(),
            side: Player::One,
            pending: false,
            plies: 0,
            rng,
            log: String::new(),
        }
    }

    pub fn cards(&self, p: Player) -> usize {
        self.piles[p.idx()].len()
    }

    fn jack_on_top(&self) -> bool {
        self.centre.last().is_some_and(|&c| c % 13 == 10)
    }
}

impl Game for Slapjack {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending {
            return vec![Move::Resolve];
        }
        let mut v = Vec::new();
        if !self.piles[self.side.idx()].is_empty() {
            v.push(Move::Flip);
        }
        if !self.centre.is_empty() || !self.piles[self.side.idx()].is_empty() {
            v.push(Move::Slap);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        self.plies += 1;
        match m {
            Move::Flip => {
                let c = self.piles[me.idx()].pop_front().unwrap();
                self.centre.push(c);
                self.log = format!("J{} întoarce {}.", me.idx() + 1, LABELS[(c % 13) as usize]);
                self.side = me.other();
            }
            Move::Slap => {
                if self.jack_on_top() {
                    self.pending = true;
                    self.log = format!("J{} dă cu palma pe valet!", me.idx() + 1);
                } else {
                    // greșeală: dai cartea de sus a pachetului tău adversarului
                    if let Some(c) = self.piles[me.idx()].pop_front() {
                        self.piles[me.other().idx()].push_back(c);
                    }
                    self.log = format!("J{} dă cu palma degeaba: cedează o carte.", me.idx() + 1);
                    self.side = me.other();
                }
            }
            Move::Resolve => {
                // cel care răspunde (side) a dat palma; celălalt a întors valetul
                let slapper = me;
                let r = self.rng.below(100);
                let winner = if r < SLAP_WIN {
                    slapper
                } else {
                    slapper.other()
                };
                let n = self.centre.len();
                for c in self.centre.drain(..) {
                    self.piles[winner.idx()].push_back(c);
                }
                self.pending = false;
                self.log = format!("J{} ia teancul de {n} cărți.", winner.idx() + 1);
                self.side = winner.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.plies >= 600 {
            return Some(by_score(
                self.piles[0].len() as i32,
                self.piles[1].len() as i32,
            ));
        }
        if self.pending {
            return None;
        }
        let s = self.side.idx();
        if self.piles[s].is_empty() && !self.jack_on_top() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.piles[me].len() as i32 - self.piles[1 - me].len() as i32) * 10;
        if self.pending {
            // palmă dată pe un valet: speranța matematică a teancului
            let n = self.centre.len() as i32 * 10;
            let ev = (2 * SLAP_WIN as i32 - 100) * n / 100;
            v += if self.side == p { ev } else { -ev };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Slapjack — pachete: J1 {}, J2 {}; în mijloc: {} cărți{}.\n",
            self.piles[0].len(),
            self.piles[1].len(),
            self.centre.len(),
            match self.centre.last() {
                Some(&c) => format!(" (sus: {})", LABELS[(c % 13) as usize]),
                None => String::new(),
            }
        );
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — {}\n",
                self.side.idx() + 1,
                if self.pending {
                    "rezolv"
                } else {
                    "joc / palma"
                }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Flip => "joc".into(),
            Move::Slap => "palma".into(),
            Move::Resolve => "rezolv".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn slap_on_a_jack_resolves_and_false_slap_costs_a_card() {
        let mut g = Slapjack::with_seed(1);
        g.centre = vec![3, 10]; // 4 și valet
        g.side = Player::Two;
        g.apply(&Move::Slap);
        assert_eq!(g.legal_moves(), vec![Move::Resolve]);
        let before: usize = g.piles[0].len() + g.piles[1].len();
        g.apply(&Move::Resolve);
        assert!(g.centre.is_empty());
        assert_eq!(g.piles[0].len() + g.piles[1].len(), before + 2);
        let mut h = Slapjack::with_seed(2);
        h.centre = vec![5];
        let (a, b) = (h.piles[0].len(), h.piles[1].len());
        h.apply(&Move::Slap);
        assert_eq!((h.piles[0].len(), h.piles[1].len()), (a - 1, b + 1));
    }

    #[test]
    fn ai_slaps_jacks_and_never_slaps_otherwise() {
        let mut g = Slapjack::with_seed(3);
        g.centre = vec![2, 10];
        g.side = Player::Two;
        assert_eq!(polytabletop_core::ai::best_move(&g, 1), Some(Move::Slap));
        let mut h = Slapjack::with_seed(3);
        h.centre = vec![2, 7];
        assert_eq!(polytabletop_core::ai::best_move(&h, 1), Some(Move::Flip));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Slapjack::with_seed, 6, 700);
    }
}
