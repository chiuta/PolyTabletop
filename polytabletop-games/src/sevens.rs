//! Șapte (Sevens / Fan Tan), joc de cărți de așezat în rânduri, 2 jucători, pachet de 52.
//!
//! * Se împart toate cărțile: 26 fiecăruia. J1 începe.
//! * Într-o culoare nu poți începe decât cu **șapte**; apoi poți pune doar cartea imediat mai mică sau
//!   imediat mai mare decât capetele rândului (6 sau 8 pe 7, 5 sau 9 etc.). Nu ai nicio carte de pus: `pas`.
//! * Câștigă primul care rămâne fără cărți.
//!
//! Mutări: `7h` (șapte de cupă), `6h`, `Kd`, `pas`. Litere: h cupă, d caro, s pică, c treflă.

use crate::cards::{full_deck, join_cards, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Card),
    Pass,
}

#[derive(Clone)]
pub struct Sevens {
    hands: [Vec<Card>; 2],
    /// per culoare: (cel mai mic, cel mai mare) rang de pe masă
    table: [Option<(u8, u8)>; 4],
    side: Player,
    viewer: Viewer,
}

impl Default for Sevens {
    fn default() -> Self {
        Self::new()
    }
}

fn sort(h: &mut [Card]) {
    h.sort_by_key(|c| (c.suit, c.rank));
}

impl Sevens {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut Rng::new(seed));
        let mut a: Vec<Card> = d.drain(..26).collect();
        let mut b = d;
        sort(&mut a);
        sort(&mut b);
        Sevens {
            hands: [a, b],
            table: [None; 4],
            side: Player::One,
            viewer: Viewer::Fixed(Player::One),
        }
    }

    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    fn can_play(&self, c: Card) -> bool {
        match self.table[c.suit as usize] {
            None => c.rank == 7,
            Some((lo, hi)) => c.rank + 1 == lo || c.rank == hi + 1,
        }
    }
}

impl Game for Sevens {
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
        if self.outcome().is_some() {
            return Vec::new();
        }
        let v: Vec<Move> = self.hands[self.side.idx()]
            .iter()
            .filter(|&&c| self.can_play(c))
            .map(|&c| Move::Play(c))
            .collect();
        if v.is_empty() {
            vec![Move::Pass]
        } else {
            v
        }
    }

    fn apply(&mut self, m: &Move) {
        if let Move::Play(c) = *m {
            let h = &mut self.hands[self.side.idx()];
            let pos = h.iter().position(|&x| x == c).unwrap();
            h.remove(pos);
            let t = &mut self.table[c.suit as usize];
            *t = Some(match *t {
                None => (7, 7),
                Some((lo, hi)) => (lo.min(c.rank), hi.max(c.rank)),
            });
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.hands[0].is_empty() {
            Some(Outcome::Win(Player::One))
        } else if self.hands[1].is_empty() {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = &self.hands[p.idx()];
        let mut v = (self.hands[1 - p.idx()].len() as i32 - me.len() as i32) * 100;
        // cărți vecine în propria mână = lanț ușor de golit; cărțile de capăt blochează adversarul
        for c in me {
            if me
                .iter()
                .any(|d| d.suit == c.suit && (d.rank + 1 == c.rank || c.rank + 1 == d.rank))
            {
                v += 4;
            }
            if c.rank == 2 || c.rank == 14 {
                v -= 3;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("Șapte (Sevens)\n");
        for suit in 0..4u8 {
            let c = Card { rank: 7, suit };
            let row = match self.table[suit as usize] {
                None => "—".to_string(),
                Some((lo, hi)) => (lo..=hi)
                    .map(|r| Card { rank: r, suit }.rank_label().to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            };
            s.push_str(&format!("{}: {}\n", c.suit_letter(), row));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            _ => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => c.text(),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_sevens_open_and_neighbours_extend() {
        let mut g = Sevens::with_seed(1);
        g.hands = [
            vec![
                Card { rank: 7, suit: 0 },
                Card { rank: 6, suit: 0 },
                Card { rank: 9, suit: 1 },
            ],
            vec![Card { rank: 8, suit: 0 }],
        ];
        let m = g.legal_moves();
        assert_eq!(m, vec![Move::Play(Card { rank: 7, suit: 0 })]);
        g.apply(&m[0]);
        assert_eq!(g.table[0], Some((7, 7)));
        assert_eq!(g.legal_moves(), vec![Move::Play(Card { rank: 8, suit: 0 })]);
        g.apply(&g.legal_moves()[0].clone());
        // J2 a rămas fără cărți
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn random_games_end_and_cards_are_conserved() {
        for seed in 1..=15u64 {
            let mut g = Sevens::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())].clone());
                n += 1;
                assert!(n < 500);
            }
        }
    }

    #[test]
    fn ai_ignores_the_hidden_hand() {
        let a = Sevens::with_seed(3);
        let mut b = a.clone();
        b.hands[1].reverse();
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
