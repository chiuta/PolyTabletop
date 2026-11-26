//! Războiul (War / Bătălia, global), 2 jucători, din regulile publice ale jocului de cărți.
//!
//! * Pachet de 52 amestecat, 26 de cărți fiecăruia, cu fața în jos. La fiecare mutare ambii întorc cartea
//!   de sus; cea mai mare (2 < … < A) ia ambele cărți, care merg la baza teancului câștigătorului, într-o
//!   ordine aleatorie (ca să nu apară bucle).
//! * Egalitate = **război**: fiecare pune 3 cărți cu fața în jos și apoi una cu fața în sus; cea mai mare ia
//!   toată grămada; la egalitate războiul se repetă. Dacă cuiva nu-i ajung cărțile, pune ce are (ultima
//!   carte e cea cu fața în sus); dacă rămâne fără cărți, pierde.
//! * Câștigă cine are toate cărțile. Limita noastră: după 1000 de mâini câștigă cine are mai multe cărți.
//! * Nu există decizii: singura mutare este `joc` (AI-ul nu are ce alege).
//!
//! Mutări: `joc`.
#![allow(clippy::manual_is_multiple_of, clippy::while_let_loop)]

use crate::cards::{full_deck, shuffle, Card};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

const MAX_HANDS: u32 = 1000;

#[derive(Clone)]
pub struct War {
    rng: Rng,
    piles: [VecDeque<Card>; 2],
    hands: u32,
    wars: u32,
    log: String,
    winner: Option<Outcome>,
}

impl Default for War {
    fn default() -> Self {
        Self::new()
    }
}

impl War {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d = full_deck(2);
        shuffle(&mut d, &mut rng);
        let mut piles = [VecDeque::new(), VecDeque::new()];
        for (i, c) in d.into_iter().enumerate() {
            piles[i % 2].push_back(c);
        }
        War {
            rng,
            piles,
            hands: 0,
            wars: 0,
            log: String::new(),
            winner: None,
        }
    }

    pub fn counts(&self) -> [usize; 2] {
        [self.piles[0].len(), self.piles[1].len()]
    }

    fn check_end(&mut self) {
        let (a, b) = (self.piles[0].len(), self.piles[1].len());
        if a == 0 && b == 0 {
            self.winner = Some(Outcome::Draw);
        } else if a == 0 {
            self.winner = Some(Outcome::Win(Player::Two));
        } else if b == 0 {
            self.winner = Some(Outcome::Win(Player::One));
        } else if self.hands >= MAX_HANDS {
            self.winner = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }
}

impl Game for War {
    type Move = ();

    fn to_move(&self) -> Player {
        if self.hands % 2 == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn legal_moves(&self) -> Vec<()> {
        if self.winner.is_some() {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn apply(&mut self, _m: &()) {
        let mut pot: Vec<Card> = Vec::new();
        let mut log = String::new();
        let mut loser_out = false;
        loop {
            let (Some(a), Some(b)) = (self.piles[0].pop_front(), self.piles[1].pop_front()) else {
                break;
            };
            pot.push(a);
            pot.push(b);
            log.push_str(&format!("{} vs {}", a.text(), b.text()));
            if a.rank != b.rank {
                let w = if a.rank > b.rank { 0 } else { 1 };
                shuffle(&mut pot, &mut self.rng);
                for c in pot.drain(..) {
                    self.piles[w].push_back(c);
                }
                log.push_str(&format!(" → J{} ia", w + 1));
                break;
            }
            // război
            self.wars += 1;
            log.push_str(" — RĂZBOI! ");
            for p in 0..2 {
                for _ in 0..3 {
                    if self.piles[p].len() > 1 {
                        pot.push(self.piles[p].pop_front().unwrap());
                    }
                }
            }
            if self.piles[0].is_empty() || self.piles[1].is_empty() {
                // cine nu mai are cărți pentru ultima carte descoperită pierde
                loser_out = true;
                let w = if self.piles[0].is_empty() { 1 } else { 0 };
                for c in pot.drain(..) {
                    self.piles[w].push_back(c);
                }
                break;
            }
        }
        let _ = loser_out;
        self.hands += 1;
        self.log = log;
        self.check_end();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.piles[p.idx()].len() as i32 - self.piles[p.other().idx()].len() as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Războiul — mâna {}; cărți: J1 {}, J2 {} (războaie: {})\n",
            self.hands + 1,
            self.piles[0].len(),
            self.piles[1].len(),
            self.wars
        );
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.winner {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str("Scrie «joc».\n"),
        }
        s
    }

    fn format_move(&self, _m: &()) -> String {
        "joc".into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_is_conserved_and_game_ends() {
        for seed in 1..=10u64 {
            let mut g = War::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                g.apply(&());
                let c = g.counts();
                assert_eq!(c[0] + c[1], 52);
                n += 1;
                assert!(n <= MAX_HANDS + 1);
            }
        }
    }

    #[test]
    fn war_on_equal_ranks() {
        let mut g = War::with_seed(1);
        let c = |rank, suit| Card { rank, suit };
        g.piles[0] = VecDeque::from(vec![c(5, 0), c(2, 0), c(3, 0), c(4, 0), c(9, 0), c(2, 1)]);
        g.piles[1] = VecDeque::from(vec![c(5, 1), c(2, 2), c(3, 1), c(4, 1), c(7, 1), c(3, 2)]);
        g.apply(&());
        // 5 vs 5 -> război: 3 jos fiecare, apoi 9 vs 7 -> J1 ia 10 cărți
        assert_eq!(g.piles[0].len(), 11);
        assert_eq!(g.piles[1].len(), 1);
        assert_eq!(g.wars, 1);
    }
}
