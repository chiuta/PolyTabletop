//! Asalt pe 7 rânduri (variantă proprie, pioni pe tablă 7×7): fiecare jucător are
//! 14 pioni pe primele două rânduri ale sale. Un pion merge un pas înainte (drept sau în
//! diagonală) pe o celulă goală; ia doar în diagonală înainte. Câștigă cine ajunge primul
//! pe ultimul rând al adversarului sau cine rămâne cu piese când adversarul le pierde pe toate.
//! Mutări: a2-a3 (mers), b2xc3 (captură).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 7;

#[derive(Clone)]
pub struct Asalt {
    b: [u8; N * N],
    side: Player,
    win: Option<Player>,
}

impl Default for Asalt {
    fn default() -> Self {
        Self::new()
    }
}

impl Asalt {
    pub fn new() -> Self {
        let mut b = [0u8; N * N];
        for x in 0..N {
            b[x] = 1;
            b[N + x] = 1;
            b[(N - 1) * N + x] = 2;
            b[(N - 2) * N + x] = 2;
        }
        Asalt {
            b,
            side: Player::One,
            win: None,
        }
    }
}

impl Game for Asalt {
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.win.is_some() {
            return vec![];
        }
        let me = self.side.idx() as u8 + 1;
        let dy: i32 = if me == 1 { 1 } else { -1 };
        let mut v = vec![];
        for i in 0..N * N {
            if self.b[i] != me {
                continue;
            }
            let (x, y) = ((i % N) as i32, (i / N) as i32);
            for dx in [-1, 0, 1] {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= N as i32 || ny >= N as i32 {
                    continue;
                }
                let t = self.b[ny as usize * N + nx as usize];
                if (dx == 0 && t == 0) || (dx != 0 && t != me) {
                    v.push((i as u8, (ny as usize * N + nx as usize) as u8));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let me = self.side.idx() as u8 + 1;
        self.b[m.0 as usize] = 0;
        self.b[m.1 as usize] = me;
        let ty = m.1 as usize / N;
        if (me == 1 && ty == N - 1) || (me == 2 && ty == 0) {
            self.win = Some(self.side);
        }
        self.side = self.side.other();
        if self.win.is_none() && !self.b.contains(&(3 - me)) {
            self.win = Some(self.side.other());
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        self.win.map(Outcome::Win)
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        for i in 0..N * N {
            let v = self.b[i];
            if v == 0 {
                continue;
            }
            let adv = if v == 1 { i / N } else { N - 1 - i / N } as i32;
            let val = 100 + adv * adv * 3;
            s += if v == me { val } else { -val };
        }
        s
    }
    fn render(&self) -> String {
        grid(N, N, |x, y| match self.b[y * N + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        })
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        let (f, t) = (m.0 as usize, m.1 as usize);
        let cap = self.b[t] != 0;
        format!(
            "{}{}{}",
            sq(f % N, f / N),
            if cap { "x" } else { "-" },
            sq(t % N, t / N)
        )
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 8,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            12,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        ))
    }
    fn move_score(&self, m: &(u8, u8)) -> i32 {
        if self.b[m.1 as usize] != 0 {
            10
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn rules() {
        let g = Asalt::new();
        assert_eq!(g.legal_moves().len(), 19);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Asalt::new(), 400);
    }
}
