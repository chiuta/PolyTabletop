//! Popa prostul (Old Maid), 2 jucători, pachet de 52 de cărți fără Dama de treflă.
//!
//! * Se împart toate cărțile; fiecare aruncă perechile de aceeași valoare (automat, la început
//!   și după fiecare carte trasă).
//! * Pe rând, tragi o carte din mâna adversarului, la alegere după poziție (`trage 3`); dacă face
//!   pereche cu o carte a ta, perechea se aruncă. Cel care trage își amestecă apoi mâna.
//! * Cine rămâne cu ultima carte (o Damă nepereche, „popa”) pierde.
//! * Încheierea e o mutare formală (`sfarsit`), ca AI-ul să nu poată deduce din rezultat ce carte
//!   a tras (poziția aleasă nu dezvăluie nimic). AI-ul alege deci poziția la întâmplare.
//!
//! Reguli: Old Maid, joc tradițional (domeniu public, cunoscut în România ca „Popa prostul”).

use crate::cards::{full_deck, handover_banner, join_cards, pl, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Trage cartea de pe poziția dată (0-based) din mâna adversarului.
    Draw(usize),
    /// Formal: încheie jocul când o mână s-a golit.
    Finish,
}

#[derive(Clone)]
pub struct Popa {
    hands: [Vec<Card>; 2],
    side: Player,
    rng: Rng,
    ending: bool,
    finished: bool,
    log: String,
    viewer: Viewer,
    handover: bool,
}

impl Default for Popa {
    fn default() -> Self {
        Self::new()
    }
}

/// Aruncă perechile (câte două de aceeași valoare); returnează valorile aruncate.
fn discard_pairs(h: &mut Vec<Card>) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < h.len() {
        if let Some(j) = (i + 1..h.len()).find(|&j| h[j].rank == h[i].rank) {
            out.push(h[i].rank);
            h.remove(j);
            h.remove(i);
        } else {
            i += 1;
        }
    }
    out
}

impl Popa {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = full_deck(2);
        deck.retain(|c| !(c.rank == 12 && c.suit == 3));
        shuffle(&mut deck, &mut rng);
        let mut hands = [Vec::new(), Vec::new()];
        for (i, c) in deck.into_iter().enumerate() {
            hands[i % 2].push(c);
        }
        for h in hands.iter_mut() {
            discard_pairs(h);
        }
        let mut g = Popa {
            hands,
            side: Player::One,
            rng,
            ending: false,
            finished: false,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        g.ending = g.hands.iter().any(|h| h.is_empty());
        g
    }

    pub fn hand_sizes(&self) -> (usize, usize) {
        (self.hands[0].len(), self.hands[1].len())
    }
}

impl Game for Popa {
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
        if self.finished {
            return Vec::new();
        }
        if self.ending {
            return vec![Move::Finish];
        }
        (0..self.hands[self.side.other().idx()].len())
            .map(Move::Draw)
            .collect()
    }

    fn apply(&mut self, m: &Move) {
        self.handover = false;
        match *m {
            Move::Finish => {
                self.finished = true;
            }
            Move::Draw(i) => {
                let me = self.side.idx();
                let c = self.hands[1 - me].remove(i);
                self.hands[me].push(c);
                let gone = discard_pairs(&mut self.hands[me]);
                self.log = if gone.is_empty() {
                    format!("J{} a tras poziția {}.", me + 1, i + 1)
                } else {
                    format!(
                        "J{} a tras poziția {} și a aruncat o pereche.",
                        me + 1,
                        i + 1
                    )
                };
                let mut h = std::mem::take(&mut self.hands[me]);
                shuffle(&mut h, &mut self.rng);
                self.hands[me] = h;
                if self.hands.iter().any(|h| h.is_empty()) {
                    self.ending = true;
                }
                self.side = self.side.other();
                self.handover = true;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.finished {
            return None;
        }
        // pierde cel care mai are carte (popa)
        Some(if self.hands[0].is_empty() {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    /// Nimic nu se poate deduce legitim despre poziția aleasă: evaluarea e constantă.
    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && !self.finished {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str("Popa prostul\n");
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        if self.finished {
            let loser = if self.hands[0].is_empty() { 2 } else { 1 };
            s.push_str(&format!(
                "Joc încheiat: Jucătorul {loser} rămâne cu popa ({}).\n",
                join_cards(&self.hands[loser - 1])
            ));
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        if self.ending {
            s.push_str("O mână s-a golit: scrie «sfarsit».\n");
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw(i) => format!("trage {}", i + 1),
            Move::Finish => "sfarsit".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn deal_removes_pairs_and_keeps_one_odd_queen() {
        for seed in 1..30 {
            let g = Popa::with_seed(seed);
            let all: Vec<Card> = g.hands.iter().flatten().copied().collect();
            // after pairing, each rank appears at most once per hand; odd count of queens overall
            for h in &g.hands {
                for r in 2..=14u8 {
                    assert!(h.iter().filter(|x| x.rank == r).count() <= 1);
                }
            }
            assert_eq!(all.iter().filter(|x| x.rank == 12).count() % 2, 1);
            for r in (2..=14u8).filter(|&r| r != 12) {
                assert_eq!(all.iter().filter(|x| x.rank == r).count() % 2, 0);
            }
        }
    }

    #[test]
    fn drawing_pairs_and_ending() {
        let mut g = Popa::with_seed(1);
        g.hands = [vec![c(5, 0), c(12, 0)], vec![c(5, 1)]];
        g.ending = false;
        g.apply(&Move::Draw(0)); // J1 draws 5d, pairs with 5h
        assert_eq!(g.hands[0], vec![c(12, 0)]);
        assert!(g.hands[1].is_empty());
        assert!(g.outcome().is_none());
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn render_hides_opponent_hand_and_hotseat_works() {
        let g = Popa::with_seed(9);
        let r = Game::render(&g);
        for x in &g.hands[1] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
        let mut h = g.clone();
        Game::set_viewer(&mut h, None);
        let m = h.legal_moves()[0];
        h.apply(&m);
        let r = Game::render(&h);
        assert!(r.contains("Predă ecranul"));
        for x in &h.hands[0] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
    }

    #[test]
    fn ai_is_blind() {
        for seed in 1..30u64 {
            let g = Popa::with_seed(seed);
            let mut h = g.clone();
            let mut rng = Rng::new(seed);
            let n = h.hands[1].len();
            shuffle(&mut h.hands[1], &mut rng);
            assert_eq!(h.hands[1].len(), n);
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 5),
                polytabletop_core::ai::best_move(&h, 5)
            );
            // outcome of a child never reveals what was drawn
            for m in g.legal_moves() {
                let mut a = g.clone();
                a.apply(&m);
                if !a.ending {
                    assert!(a.outcome().is_none());
                }
                assert!(a.outcome().is_none());
            }
        }
    }

    #[test]
    fn random_games_finish_with_exactly_one_card_left() {
        for seed in 1..80u64 {
            let mut g = Popa::with_seed(seed);
            let mut rng = Rng::new(seed ^ 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 200);
            }
            let (a, b) = g.hand_sizes();
            assert_eq!(a + b, 1);
            assert_eq!(g.hands.iter().flatten().next().unwrap().rank, 12);
        }
    }
}
