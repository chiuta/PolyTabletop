//! Păsări pe Sârmă: pe o sârmă comună stau păsări de 5 culori. Fiecare jucător are 5 cărți
//! deschise. La rând, așezi o pasăre la capătul stâng sau drept al sârmei («st v» / «dr v», v = culoarea).
//! Dacă la acel capăt se formează un șir de cel puțin 2 păsări de aceeași culoare, le iei pe toate
//! în colecția ta. Apoi tragi o carte (din pachet, la întâmplare). La final, fiecare culoare colectată
//! valorează 1+2+…+n puncte pentru n păsări. Jocul se termină când pachetul (40 de cărți) se golește
//! și mâinile se consumă. Variantă originală (reguli de captură proprii).
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COL: usize = 5;
const SYM: [char; COL] = ['r', 'v', 'a', 'g', 'n'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    right: bool,
    color: u8,
}

#[derive(Clone)]
pub struct Pasari {
    wire: Vec<u8>,
    hand: [[u8; COL]; 2],
    pile: [[u8; COL]; 2],
    deck: [u8; COL],
    side: Player,
    dice: Dice,
}

impl Default for Pasari {
    fn default() -> Self {
        Self::new()
    }
}

impl Pasari {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pasari {
            wire: vec![],
            hand: [[0; COL]; 2],
            pile: [[0; COL]; 2],
            deck: [8; COL],
            side: Player::One,
            dice: Dice::new(seed),
        };
        for _ in 0..3 {
            if let Some(c) = g.draw() {
                g.wire.push(c);
            }
        }
        for p in 0..2 {
            for _ in 0..5 {
                if let Some(c) = g.draw() {
                    g.hand[p][c as usize] += 1;
                }
            }
        }
        g
    }

    fn draw(&mut self) -> Option<u8> {
        let tot: usize = self.deck.iter().map(|&x| x as usize).sum();
        if tot == 0 {
            return None;
        }
        let mut r = self.dice.below(tot);
        for c in 0..COL {
            if r < self.deck[c] as usize {
                self.deck[c] -= 1;
                return Some(c as u8);
            }
            r -= self.deck[c] as usize;
        }
        None
    }

    fn pts(&self, p: usize) -> i32 {
        self.pile[p]
            .iter()
            .map(|&n| n as i32 * (n as i32 + 1) / 2)
            .sum()
    }
}

impl Game for Pasari {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        let h = &self.hand[self.side.idx()];
        let mut v = vec![];
        for c in 0..COL {
            if h[c] > 0 {
                v.push(Mv {
                    right: false,
                    color: c as u8,
                });
                v.push(Mv {
                    right: true,
                    color: c as u8,
                });
            }
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        self.hand[p][m.color as usize] -= 1;
        if m.right {
            self.wire.push(m.color);
        } else {
            self.wire.insert(0, m.color);
        }
        let n = self.wire.len();
        let run = if m.right {
            self.wire
                .iter()
                .rev()
                .take_while(|&&c| c == m.color)
                .count()
        } else {
            self.wire.iter().take_while(|&&c| c == m.color).count()
        };
        if run >= 2 {
            self.pile[p][m.color as usize] += run as u8;
            if m.right {
                self.wire.truncate(n - run);
            } else {
                self.wire.drain(0..run);
            }
        }
        if let Some(c) = self.draw() {
            self.hand[p][c as usize] += 1;
        }
        self.side = self.side.other();
        if self.hand[self.side.idx()].iter().all(|&x| x == 0) {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.hand.iter().all(|h| h.iter().all(|&x| x == 0)) {
            return Some(match self.pts(0).cmp(&self.pts(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), p.other().idx());
        (self.pts(a) - self.pts(b)) * 3
            + (self.pile[a].iter().map(|&x| x as i32).max().unwrap_or(0) - 1)
    }

    fn render(&self) -> String {
        let mut s = String::from("Sârma: ");
        for &c in &self.wire {
            s.push(SYM[c as usize]);
            s.push(' ');
        }
        s.push('\n');
        for q in 0..2 {
            s.push_str(&format!(
                "J{}{}: mână ",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                }
            ));
            for c in 0..COL {
                s.push_str(&format!("{}{} ", SYM[c], self.hand[q][c]));
            }
            s.push_str("| colecție ");
            for c in 0..COL {
                s.push_str(&format!("{}{} ", SYM[c], self.pile[q][c]));
            }
            s.push_str(&format!("= {} p\n", self.pts(q)));
        }
        s.push_str("Mutări: «st r» / «dr v» (culori r,v,a,g,n).\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!(
            "{} {}",
            if m.right { "dr" } else { "st" },
            SYM[m.color as usize]
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end_and_conserve_cards() {
        for seed in 1..=30u64 {
            let mut g = Pasari::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
            }
            let in_play: u32 = g.wire.len() as u32
                + g.pile.iter().flatten().map(|&x| x as u32).sum::<u32>()
                + g.deck.iter().map(|&x| x as u32).sum::<u32>();
            assert_eq!(in_play, 40);
        }
    }

    #[test]
    fn ai_wins_most_against_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=8u64 {
            let mut g = Pasari::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                if g.side == Player::One {
                    let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                } else {
                    let mv = g.legal_moves();
                    g.apply(&mv[rng.below(mv.len())]);
                }
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 5, "{wins}");
    }
}
