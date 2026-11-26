//! Cărți în Cascadă: cursă pe ture. Fiecare jucător are un pachet de 40 de cărți (4 culori × 1..10),
//! trase la întâmplare, și 3 cărți deschise la vedere («posturi»). Pe masă sunt maximum 4 stive
//! comune, câte una pe culoare, care cresc 1,2,3…10. La rând: joci un post pe o stivă («joc i») —
//! 1 deschide o stivă nouă, altă carte trebuie să urmeze exact vârful stivei de aceeași culoare — și
//! în locul lui apare o carte nouă; sau schimbi un post cu o carte nouă din pachet («schimb i»).
//! Cine își golește pachetul și posturile câștigă imediat; altfel, după 400 de mutări,
//! câștigă cine a jucat mai multe cărți în stive minus cele rămase. Variantă originală, pe ture.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Play(u8),
    Swap(u8),
}

#[derive(Clone)]
pub struct Cascada {
    stock: [Vec<u8>; 2],
    post: [[Option<u8>; 3]; 2],
    /// pentru fiecare culoare: ultima valoare jucată (0 = stiva nu există)
    top: [u8; 4],
    played: [i32; 2],
    side: Player,
    plies: u32,
    winner: Option<Player>,
    dice: Dice,
}

impl Default for Cascada {
    fn default() -> Self {
        Self::new()
    }
}

fn name(c: u8) -> String {
    format!("{}{}", ['r', 'v', 'a', 'g'][(c / 10) as usize], c % 10 + 1)
}

impl Cascada {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Cascada {
            stock: [(0..40).collect(), (0..40).collect()],
            post: [[None; 3]; 2],
            top: [0; 4],
            played: [0; 2],
            side: Player::One,
            plies: 0,
            winner: None,
            dice: Dice::new(seed),
        };
        for p in 0..2 {
            for i in 0..3 {
                g.post[p][i] = g.pull(p);
            }
        }
        g
    }

    fn pull(&mut self, p: usize) -> Option<u8> {
        let n = self.stock[p].len();
        if n == 0 {
            return None;
        }
        let i = self.dice.below(n);
        Some(self.stock[p].swap_remove(i))
    }

    fn can_play(&self, c: u8) -> bool {
        let (col, v) = ((c / 10) as usize, c % 10 + 1);
        if v == 1 {
            self.top[col] == 0 && self.top.iter().filter(|&&t| t > 0).count() < 4
        } else {
            self.top[col] + 1 == v
        }
    }

    fn left(&self, p: usize) -> usize {
        self.stock[p].len() + self.post[p].iter().flatten().count()
    }
}

impl Game for Cascada {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        for i in 0..3 {
            if let Some(c) = self.post[p][i] {
                if self.can_play(c) {
                    v.push(Mv::Play(i as u8));
                }
                if !self.stock[p].is_empty() {
                    v.push(Mv::Swap(i as u8));
                }
            }
        }
        if v.is_empty() {
            // postul gol se reumple; dacă nu e nimic de făcut, mutarea nulă e un schimb fictiv
            for i in 0..3 {
                if self.post[p][i].is_some() {
                    v.push(Mv::Swap(i as u8));
                    break;
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match *m {
            Mv::Play(i) => {
                if let Some(c) = self.post[p][i as usize] {
                    let col = (c / 10) as usize;
                    self.top[col] = c % 10 + 1;
                    if self.top[col] == 10 {
                        self.top[col] = 0;
                        self.played[p] += 2;
                    }
                    self.played[p] += 1;
                    self.post[p][i as usize] = self.pull(p);
                }
            }
            Mv::Swap(i) => {
                if let Some(c) = self.post[p][i as usize] {
                    if let Some(n) = self.pull(p) {
                        self.stock[p].push(c);
                        self.post[p][i as usize] = Some(n);
                    }
                }
            }
        }
        if self.left(p) == 0 {
            self.winner = Some(self.side);
        }
        self.plies += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.plies >= 400 {
            let f = |p: usize| self.played[p] - self.left(p) as i32;
            return Some(match f(0).cmp(&f(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| {
            self.played[q] * 4 - self.left(q) as i32
                + self.post[q]
                    .iter()
                    .flatten()
                    .filter(|&&c| self.can_play(c))
                    .count() as i32
        };
        f(p.idx()) - f(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::from("Stive: ");
        for (c, t) in self.top.iter().enumerate() {
            s.push_str(&format!("{}{} ", ['r', 'v', 'a', 'g'][c], t));
        }
        s.push('\n');
        for q in 0..2 {
            s.push_str(&format!(
                "J{}{}: posturi ",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                }
            ));
            for i in 0..3 {
                s.push_str(&match self.post[q][i] {
                    Some(c) => format!("{}={} ", i + 1, name(c)),
                    None => format!("{}=- ", i + 1),
                });
            }
            s.push_str(&format!(
                "| pachet {} | jucate {}\n",
                self.stock[q].len(),
                self.played[q]
            ));
        }
        s.push_str("Mutări: «joc 1», «schimb 2».\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Play(i) => format!("joc {}", i + 1),
            Mv::Swap(i) => format!("schimb {}", i + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Cascada::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    #[test]
    fn ai_plays_valid_cards_and_wins() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=6u64 {
            let mut g = Cascada::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                if g.side == Player::One {
                    let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
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
        assert!(wins >= 4, "{wins}");
    }
}
