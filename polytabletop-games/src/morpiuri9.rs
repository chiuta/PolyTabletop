//! Morpion în nouă (variantă proprie, X și 0 pe 9 table mici): 9×9 = nouă table 3×3.
//! Locul unde joci în tabla mică îi impune adversarului în ce tablă mică joacă
//! (dacă tabla trimisă e câștigată sau plină, alege liber). Cine câștigă trei table mici
//! în linie pe tabla mare câștigă; tablă plină = remiză. Mutări: e5 (coloană a–i, rând 1–9).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const L: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Clone)]
pub struct Morpiuri9 {
    c: [u8; 81],
    big: [u8; 9], // 0 deschisă, 1/2 câștigată, 3 plină
    next: Option<usize>,
    side: Player,
    winner: Option<u8>,
    left: u32,
}

impl Default for Morpiuri9 {
    fn default() -> Self {
        Self::new()
    }
}

fn board_of(i: usize) -> usize {
    let (x, y) = (i % 9, i / 9);
    (y / 3) * 3 + x / 3
}
fn cell_in(i: usize) -> usize {
    let (x, y) = (i % 9, i / 9);
    (y % 3) * 3 + x % 3
}
fn at(b: usize, k: usize) -> usize {
    let (bx, by, cx, cy) = (b % 3, b / 3, k % 3, k / 3);
    (by * 3 + cy) * 9 + bx * 3 + cx
}

impl Morpiuri9 {
    pub fn new() -> Self {
        Morpiuri9 {
            c: [0; 81],
            big: [0; 9],
            next: None,
            side: Player::One,
            winner: None,
            left: 81,
        }
    }
    fn line_won(v: &[u8], who: u8) -> bool {
        L.iter().any(|l| l.iter().all(|&k| v[k] == who))
    }
    fn small(&self, b: usize) -> [u8; 9] {
        let mut s = [0u8; 9];
        for (k, v) in s.iter_mut().enumerate() {
            *v = self.c[at(b, k)];
        }
        s
    }
}

impl Game for Morpiuri9 {
    type Move = u8;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<u8> {
        if self.winner.is_some() || self.left == 0 {
            return vec![];
        }
        let boards: Vec<usize> = match self.next {
            Some(b) if self.big[b] == 0 => vec![b],
            _ => (0..9).filter(|&b| self.big[b] == 0).collect(),
        };
        let mut v = vec![];
        for b in boards {
            for k in 0..9 {
                if self.c[at(b, k)] == 0 {
                    v.push(at(b, k) as u8);
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &u8) {
        let i = *m as usize;
        let me = self.side.idx() as u8 + 1;
        self.c[i] = me;
        self.left -= 1;
        let b = board_of(i);
        let s = self.small(b);
        if Self::line_won(&s, me) {
            self.big[b] = me;
        } else if s.iter().all(|&x| x != 0) {
            self.big[b] = 3;
        }
        let bw: Vec<u8> = self
            .big
            .iter()
            .map(|&x| if x == me { me } else { 0 })
            .collect();
        if Self::line_won(&bw, me) {
            self.winner = Some(me);
        }
        let nb = cell_in(i);
        self.next = if self.big[nb] == 0 { Some(nb) } else { None };
        if self.big.iter().all(|&x| x != 0) && self.winner.is_none() {
            self.left = 0;
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.winner {
            Some(w) => Some(Outcome::Win(if w == 1 { Player::One } else { Player::Two })),
            None if self.left == 0 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        for b in 0..9 {
            let w = if b == 4 {
                30
            } else if b % 2 == 0 {
                20
            } else {
                15
            };
            if self.big[b] == me {
                s += w * 4;
            } else if self.big[b] == 3 - me {
                s -= w * 4;
            } else {
                let sm = self.small(b);
                for l in L {
                    let (a, o) = (
                        l.iter().filter(|&&k| sm[k] == me).count(),
                        l.iter().filter(|&&k| sm[k] == 3 - me).count(),
                    );
                    if o == 0 {
                        s += (a * a) as i32;
                    }
                    if a == 0 {
                        s -= (o * o) as i32;
                    }
                }
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(9, 9, |x, y| match self.c[y * 9 + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        });
        s.push_str("Table mici: ");
        for b in 0..9 {
            s.push(['.', 'X', 'O', '='][self.big[b] as usize]);
        }
        match self.next {
            Some(b) if self.big[b] == 0 => s.push_str(&format!(
                "\nJoci în tabla {} (rând {}, coloană {}).\n",
                b + 1,
                b / 3 + 1,
                b % 3 + 1
            )),
            _ => s.push_str("\nAlegi liber orice tablă deschisă.\n"),
        }
        s
    }
    fn format_move(&self, m: &u8) -> String {
        sq(*m as usize % 9, *m as usize / 9)
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 9,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            11,
            self.side,
            self.c
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v as u64))
                .chain(std::iter::once((
                    200,
                    self.next.map_or(0, |n| n as u64 + 1),
                ))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;

    #[test]
    fn forced_board_and_win() {
        let mut g = Morpiuri9::new();
        assert_eq!(g.legal_moves().len(), 81);
        g.apply(&(at(0, 4) as u8)); // centrul tablei 0 -> adversarul joacă în tabla 4
        assert!(g.legal_moves().iter().all(|&m| board_of(m as usize) == 4));
        // tablă mică câștigată
        let mut h = Morpiuri9::new();
        h.c[at(0, 0)] = 1;
        h.c[at(0, 1)] = 1;
        h.next = Some(0);
        h.apply(&(at(0, 2) as u8));
        assert_eq!(h.big[0], 1);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Morpiuri9::new(), 100);
    }
}
