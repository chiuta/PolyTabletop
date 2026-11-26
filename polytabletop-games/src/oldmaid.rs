#![allow(clippy::doc_lazy_continuation)]
//! Old Maid (Bătrâna fată), joc de cărți pentru doi jucători.
//!
//! * Pachet de 52 de cărți din care se scoate o damă (♣): rămân trei dame, deci una rămâne fără
//!   pereche — „bătrâna fată”. Se împart toate cărțile; fiecare își aruncă perechile de aceeași
//!   valoare.
//! * Pe rând, jucătorul la mutare trage o carte din mâna celuilalt, fără să o vadă (`trag 3` =
//!   a treia carte din mâna lui); dacă face pereche cu o carte din mâna lui, aruncă amândouă cărțile.
//!   Mâna celui din care s-a tras se amestecă la loc, ca poziția să nu poată fi memorată.
//! * Jocul se termină când nu mai rămâne decât o carte: cine o ține (dama) pierde.
//! * Informația e ascunsă: mâna ta se vede doar de tine; AI-ul alege poziția la întâmplare (nu
//!   citește cărțile adversarului) iar rezultatul final apare abia la pasul `final` (arbitru J2).
//! Mutări: `trag N`, `final`.

use crate::cards::{handover_banner, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw(u8),
    End,
}

#[derive(Clone)]
pub struct OldMaid {
    hands: [Vec<Card>; 2],
    side: Player,
    rng: Rng,
    discarded: [u8; 2],
    ending: bool,
    closed: bool,
    viewer: Viewer,
    log: String,
}

fn pair_off(hand: &mut Vec<Card>) -> u8 {
    let mut removed = 0;
    for rank in 2..=14u8 {
        let idx: Vec<usize> = hand
            .iter()
            .enumerate()
            .filter(|(_, c)| c.rank == rank)
            .map(|(i, _)| i)
            .collect();
        let mut k = idx.len() / 2 * 2;
        while k > 0 {
            let i = idx[k - 1];
            hand.remove(i);
            k -= 1;
            removed += 1;
        }
    }
    removed / 2
}

impl Default for OldMaid {
    fn default() -> Self {
        Self::new()
    }
}

impl OldMaid {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = crate::cards::full_deck(2);
        deck.retain(|c| !(c.rank == 12 && c.suit == 3));
        shuffle(&mut deck, &mut rng);
        let mut hands = [Vec::new(), Vec::new()];
        for (i, c) in deck.into_iter().enumerate() {
            hands[i % 2].push(c);
        }
        let d0 = pair_off(&mut hands[0]);
        let d1 = pair_off(&mut hands[1]);
        shuffle(&mut hands[0], &mut rng);
        shuffle(&mut hands[1], &mut rng);
        let mut g = OldMaid {
            hands,
            side: Player::One,
            rng,
            discarded: [d0, d1],
            ending: false,
            closed: false,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.ending = g.total() <= 1;
        g
    }

    fn total(&self) -> usize {
        self.hands[0].len() + self.hands[1].len()
    }

    fn loser(&self) -> Player {
        if self.hands[0].is_empty() {
            Player::Two
        } else {
            Player::One
        }
    }
}

impl Game for OldMaid {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.ending {
            Player::Two
        } else {
            self.side
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.closed {
            return Vec::new();
        }
        if self.ending {
            return vec![Move::End];
        }
        (0..self.hands[self.side.other().idx()].len() as u8)
            .map(Move::Draw)
            .collect()
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::End => self.closed = true,
            Move::Draw(i) => {
                let me = self.side.idx();
                let you = 1 - me;
                let c = self.hands[you].remove(i as usize);
                if let Some(pos) = self.hands[me].iter().position(|x| x.rank == c.rank) {
                    self.hands[me].remove(pos);
                    self.discarded[me] += 1;
                    self.log = format!(
                        "J{} a tras o carte și a făcut pereche de {}.",
                        me + 1,
                        c.rank_label()
                    );
                } else {
                    self.hands[me].push(c);
                    self.log = format!("J{} a tras o carte, fără pereche.", me + 1);
                }
                shuffle(&mut self.hands[you], &mut self.rng);
                shuffle(&mut self.hands[me], &mut self.rng);
                if self.total() <= 1 {
                    self.ending = true;
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.closed {
            Some(Outcome::Win(self.loser().other()))
        } else {
            None
        }
    }

    /// Mâna adversarului e necunoscută: toate alegerile sunt egale pentru robot.
    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let me = self.viewer.who(self.to_move());
        let mut s = format!(
            "Old Maid — perechi aruncate: J1 {}, J2 {}.\n",
            self.discarded[0], self.discarded[1]
        );
        if let Viewer::HotSeat = self.viewer {
            if self
                .log
                .contains(&format!("J{}", self.side.other().idx() + 1))
            {
                s.push_str(&handover_banner(self.side));
            }
        }
        for p in 0..2 {
            if p == me.idx() {
                let mut h = self.hands[p].clone();
                h.sort_by_key(|c| (c.rank, c.suit));
                s.push_str(&format!(
                    "J{} (mâna ta): {}\n",
                    p + 1,
                    crate::cards::join_cards(&h)
                ));
            } else {
                s.push_str(&format!(
                    "J{}: {} cărți (ascunse)\n",
                    p + 1,
                    self.hands[p].len()
                ));
            }
        }
        if self.closed {
            s.push_str(&format!(
                "Dama a rămas la J{}: acela pierde.\n",
                self.loser().idx() + 1
            ));
        } else if self.ending {
            s.push_str("Ultima carte a fost trasă. Scrie «final».\n");
        } else {
            s.push_str(&format!(
                "J{} trage: `trag 1`…`trag {}`.\n",
                self.side.idx() + 1,
                self.hands[self.side.other().idx()].len()
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw(i) => format!("trag {}", i + 1),
            Move::End => "final".into(),
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
    fn deal_has_51_cards_and_no_internal_pairs_at_start() {
        let g = OldMaid::with_seed(5);
        let total = g.total() + 2 * (g.discarded[0] + g.discarded[1]) as usize;
        assert_eq!(total, 51);
        for h in &g.hands {
            for r in 2..=14u8 {
                assert!(h.iter().filter(|c| c.rank == r).count() <= 1);
            }
        }
    }

    #[test]
    fn random_games_end_with_the_queen_left() {
        for seed in 1..=40u64 {
            let mut g = OldMaid::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 500);
            }
            let last: Vec<Card> = g.hands.iter().flatten().copied().collect();
            assert_eq!(last.len(), 1);
            assert_eq!(last[0].rank, 12);
        }
    }

    #[test]
    fn opponent_hand_hidden() {
        let mut g = OldMaid::with_seed(2);
        g.set_viewer(Some(Player::One));
        let a = g.render();
        g.set_viewer(Some(Player::Two));
        let b = g.render();
        assert_ne!(a, b);
        assert!(a.contains("J2: ") && a.contains("cărți (ascunse)"));
    }
}
