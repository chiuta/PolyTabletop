//! Șase în linie (variantă proprie, tablă 13×13): J1 pune o piesă la prima tură, apoi fiecare
//! jucător pune câte două piese pe tură (a doua piesă e o mutare separată, a aceluiași jucător).
//! Șase sau mai multe piese proprii în linie (rând, coloană, diagonală) câștigă; tabla plină
//! = remiză. Mutări: g7.
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, run_through, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 13;

#[derive(Clone)]
pub struct Sase {
    b: [u8; N * N],
    side: Player,
    left: u8,
    filled: u32,
    win: Option<Player>,
}

impl Default for Sase {
    fn default() -> Self {
        Self::new()
    }
}

impl Sase {
    pub fn new() -> Self {
        Sase {
            b: [0; N * N],
            side: Player::One,
            left: 1,
            filled: 0,
            win: None,
        }
    }
    fn at(&self, x: i32, y: i32) -> u8 {
        self.b[y as usize * N + x as usize]
    }
}

impl Game for Sase {
    type Move = u8;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<u8> {
        if self.win.is_some() {
            return vec![];
        }
        (0..N * N)
            .filter(|&i| self.b[i] == 0)
            .map(|i| i as u8)
            .collect()
    }
    fn apply(&mut self, m: &u8) {
        let me = self.side.idx() as u8 + 1;
        let (x, y) = ((*m as usize % N) as i32, (*m as usize / N) as i32);
        self.b[*m as usize] = me;
        self.filled += 1;
        for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
            if run_through(N as i32, N as i32, x, y, dx, dy, |a, c| self.at(a, c) == me) >= 6 {
                self.win = Some(self.side);
            }
        }
        self.left -= 1;
        if self.left == 0 {
            self.side = self.side.other();
            self.left = 2;
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.filled as usize == N * N => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0i32;
        let w = [0, 1, 4, 20, 120, 900, 100000];
        for y in 0..N as i32 {
            for x in 0..N as i32 {
                let v = self.at(x, y);
                if v == 0 {
                    continue;
                }
                for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                    // numărăm doar la începutul șirului
                    let (px, py) = (x - dx, y - dy);
                    if px >= 0 && py >= 0 && px < N as i32 && py < N as i32 && self.at(px, py) == v
                    {
                        continue;
                    }
                    let mut n = 0;
                    let (mut cx, mut cy) = (x, y);
                    while cx >= 0
                        && cy >= 0
                        && cx < N as i32
                        && cy < N as i32
                        && self.at(cx, cy) == v
                    {
                        n += 1;
                        cx += dx;
                        cy += dy;
                    }
                    let open = cx >= 0
                        && cy >= 0
                        && cx < N as i32
                        && cy < N as i32
                        && self.at(cx, cy) == 0;
                    let open2 = px >= 0
                        && py >= 0
                        && px < N as i32
                        && py < N as i32
                        && self.at(px, py) == 0;
                    let val = w[(n as usize).min(6)] * (1 + open as i32 + open2 as i32);
                    s += if v == me { val } else { -val };
                }
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(N, N, |x, y| match self.b[y * N + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        });
        s.push_str(&format!("Piese de pus în această tură: {}\n", self.left));
        s
    }
    fn format_move(&self, m: &u8) -> String {
        sq(*m as usize % N, *m as usize / N)
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 4,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            15,
            self.side,
            self.b
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v as u64))
                .chain(std::iter::once((300, self.left as u64))),
        ))
    }
    fn move_score(&self, m: &u8) -> i32 {
        let (x, y) = ((*m as usize % N) as i32, (*m as usize / N) as i32);
        let mut s = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let (a, c) = (x + dx, y + dy);
                if a >= 0 && c >= 0 && a < N as i32 && c < N as i32 && self.at(a, c) != 0 {
                    s += 10;
                }
            }
        }
        s - ((x - 6).abs() + (y - 6).abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn two_stones_per_turn_and_six_wins() {
        let mut g = Sase::new();
        assert_eq!(g.to_move(), Player::One);
        g.apply(&80);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&0);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&1);
        assert_eq!(g.to_move(), Player::One);
        let mut h = Sase::new();
        for i in 0..5 {
            h.b[i] = 1;
        }
        h.apply(&5);
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Sase::new(), 400);
    }
}
