#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Ataxx (SUA, 1990) — joc de teritoriu pe tabla 7×7 pentru 2 jucători.
//!
//! * Fiecare jucător începe cu două piese în colțuri opuse. La mutare alegi o piesă proprie și o
//!   destinație liberă: la distanță 1 (inclusiv diagonal) piesa se **clonează** (rămân două), la
//!   distanță 2 **sare** (se mută). Mutări: `a1-b2` (clonare), `a1-c3` (salt).
//! * Toate piesele adverse vecine cu destinația (cele 8 pătrate din jur) devin ale tale.
//! * Fără nicio mutare legală spui `pas`; jocul se termină când tabla e plină, un jucător nu mai
//!   are piese sau niciunul nu mai poate muta (după 400 de mutări se numără piesele). Câștigă cine
//!   are mai multe piese.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 7;
const CAP: u32 = 400;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Go(u8, u8),
    Pass,
}

#[derive(Clone)]
pub struct Ataxx {
    b: [u8; N * N],
    side: Player,
    ply: u32,
}

impl Default for Ataxx {
    fn default() -> Self {
        Self::new()
    }
}

fn sq(i: usize) -> String {
    format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
}

impl Ataxx {
    pub fn new() -> Self {
        let mut b = [0u8; N * N];
        b[0] = 1;
        b[N * N - 1] = 1;
        b[N - 1] = 2;
        b[N * (N - 1)] = 2;
        Ataxx {
            b,
            side: Player::One,
            ply: 0,
        }
    }

    pub fn counts(&self) -> (usize, usize) {
        (
            self.b.iter().filter(|&&x| x == 1).count(),
            self.b.iter().filter(|&&x| x == 2).count(),
        )
    }

    fn moves_for(&self, p: u8) -> Vec<Move> {
        let mut v = Vec::new();
        for i in 0..N * N {
            if self.b[i] != p {
                continue;
            }
            let (r, f) = ((i / N) as i32, (i % N) as i32);
            for dr in -2..=2i32 {
                for df in -2..=2i32 {
                    if dr == 0 && df == 0 {
                        continue;
                    }
                    let (nr, nf) = (r + dr, f + df);
                    if nr < 0 || nf < 0 || nr >= N as i32 || nf >= N as i32 {
                        continue;
                    }
                    let to = nr as usize * N + nf as usize;
                    if self.b[to] == 0 {
                        v.push(Move::Go(i as u8, to as u8));
                    }
                }
            }
        }
        v
    }

    fn full_or_dead(&self) -> bool {
        let (a, b) = self.counts();
        a == 0 || b == 0 || !self.b.contains(&0)
    }
}

impl Game for Ataxx {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let v = self.moves_for(self.side.idx() as u8 + 1);
        if v.is_empty() {
            vec![Move::Pass]
        } else {
            v
        }
    }

    fn apply(&mut self, m: &Move) {
        self.ply += 1;
        let me = self.side.idx() as u8 + 1;
        if let Move::Go(a, b) = *m {
            let (a, b) = (a as usize, b as usize);
            let d = (a / N).abs_diff(b / N).max((a % N).abs_diff(b % N));
            if d == 2 {
                self.b[a] = 0;
            }
            self.b[b] = me;
            let (r, f) = ((b / N) as i32, (b % N) as i32);
            for dr in -1..=1i32 {
                for df in -1..=1i32 {
                    let (nr, nf) = (r + dr, f + df);
                    if nr < 0 || nf < 0 || nr >= N as i32 || nf >= N as i32 {
                        continue;
                    }
                    let i = nr as usize * N + nf as usize;
                    if self.b[i] != 0 {
                        self.b[i] = me;
                    }
                }
            }
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let dead = self.full_or_dead()
            || self.ply >= CAP
            || (self.moves_for(1).is_empty() && self.moves_for(2).is_empty());
        if !dead {
            return None;
        }
        let (a, b) = self.counts();
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = self.counts();
        let d = a as i32 - b as i32;
        if p == Player::One {
            d * 10
        } else {
            -d * 10
        }
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g\n");
        for r in (0..N).rev() {
            s.push_str(&format!(" {}  ", r + 1));
            for f in 0..N {
                s.push(match self.b[r * N + f] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        let (a, b) = self.counts();
        s.push_str(&format!("    a b c d e f g\n  X = J1: {a}   O = J2: {b}\n"));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Go(a, b) => format!("{}-{}", sq(*a as usize), sq(*b as usize)),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }

    fn move_score(&self, m: &Move) -> i32 {
        match m {
            Move::Go(a, b) => {
                let d = (*a as usize / N)
                    .abs_diff(*b as usize / N)
                    .max((*a as usize % N).abs_diff(*b as usize % N));
                if d == 1 {
                    1
                } else {
                    0
                }
            }
            Move::Pass => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_and_clone_vs_jump() {
        let g = Ataxx::new();
        assert_eq!(g.counts(), (2, 2));
        let mut h = g.clone();
        h.apply(&Move::Go(0, 1)); // clonare a1-b1
        assert_eq!(h.counts().0, 3);
        let mut j = g.clone();
        j.apply(&Move::Go(0, 2)); // salt a1-c1
        assert_eq!(j.counts().0, 2);
        assert_eq!(j.b[0], 0);
    }

    #[test]
    fn conversion() {
        let mut g = Ataxx::new();
        g.b = [0; N * N];
        g.b[0] = 1;
        g.b[N * 2 + 1] = 2; // b3
        g.b[N * 2 + 2] = 2; // c3
        g.apply(&Move::Go(0, N as u8 + 1)); // a1-b2 clonare: vecinii b3, c3 devin X
        assert_eq!(g.counts(), (4, 0));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Ataxx::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 1000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
