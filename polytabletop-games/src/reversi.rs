//! Reversi: 8×8, flank to flip. Black (One) moves first.
//! (Sold under the trademark "Othello"; the game itself is free.)

use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Place(u8),
    Pass,
}

#[derive(Clone)]
pub struct Reversi {
    cells: [u8; 64],
    side: Player,
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[rustfmt::skip]
const WEIGHTS: [i32; 64] = [
    100, -20, 10,  5,  5, 10, -20, 100,
    -20, -50, -2, -2, -2, -2, -50, -20,
     10,  -2, -1, -1, -1, -1,  -2,  10,
      5,  -2, -1, -1, -1, -1,  -2,   5,
      5,  -2, -1, -1, -1, -1,  -2,   5,
     10,  -2, -1, -1, -1, -1,  -2,  10,
    -20, -50, -2, -2, -2, -2, -50, -20,
    100, -20, 10,  5,  5, 10, -20, 100,
];

impl Default for Reversi {
    fn default() -> Self {
        Self::new()
    }
}

impl Reversi {
    pub fn new() -> Self {
        let mut cells = [0u8; 64];
        // d4, e5 = White(2); e4, d5 = Black(1)
        cells[3 * 8 + 3] = 2;
        cells[4 * 8 + 4] = 2;
        cells[3 * 8 + 4] = 1;
        cells[4 * 8 + 3] = 1;
        Reversi {
            cells,
            side: Player::One,
        }
    }

    fn flips(&self, idx: usize, color: u8) -> Vec<usize> {
        let mut out = Vec::new();
        if self.cells[idx] != 0 {
            return out;
        }
        let opp = 3 - color;
        let (r0, c0) = ((idx / 8) as i32, (idx % 8) as i32);
        for (dr, dc) in DIRS {
            let mut line = Vec::new();
            let (mut r, mut c) = (r0 + dr, c0 + dc);
            while (0..8).contains(&r) && (0..8).contains(&c) {
                let v = self.cells[(r * 8 + c) as usize];
                if v == opp {
                    line.push((r * 8 + c) as usize);
                } else {
                    if v == color && !line.is_empty() {
                        out.extend(line);
                    }
                    break;
                }
                r += dr;
                c += dc;
            }
        }
        out
    }

    fn placements(&self, color: u8) -> Vec<u8> {
        (0..64usize)
            .filter(|&i| !self.flips(i, color).is_empty())
            .map(|i| i as u8)
            .collect()
    }

    pub fn counts(&self) -> (u32, u32) {
        let b = self.cells.iter().filter(|&&c| c == 1).count() as u32;
        let w = self.cells.iter().filter(|&&c| c == 2).count() as u32;
        (b, w)
    }
}

impl Game for Reversi {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let p = self.placements(me);
        if !p.is_empty() {
            return p.into_iter().map(Mv::Place).collect();
        }
        if !self.placements(3 - me).is_empty() {
            return vec![Mv::Pass];
        }
        Vec::new()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        if let Mv::Place(i) = m {
            let f = self.flips(*i as usize, me);
            self.cells[*i as usize] = me;
            for x in f {
                self.cells[x] = me;
            }
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.placements(1).is_empty() || !self.placements(2).is_empty() {
            return None;
        }
        let (b, w) = self.counts();
        Some(if b > w {
            Outcome::Win(Player::One)
        } else if w > b {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        for (i, &w) in WEIGHTS.iter().enumerate() {
            if self.cells[i] == me {
                s += w;
            } else if self.cells[i] != 0 {
                s -= w;
            }
        }
        let mob = self.placements(me).len() as i32 - self.placements(3 - me).len() as i32;
        s + 4 * mob
    }

    fn render(&self) -> String {
        let (b, w) = self.counts();
        let mut s = String::from("    a b c d e f g h\n");
        for row in (0..8).rev() {
            s.push_str(&format!(" {}  ", row + 1));
            for col in 0..8 {
                s.push(match self.cells[row * 8 + col] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", row + 1));
        }
        s.push_str(&format!(
            "    a b c d e f g h\n  X (Negru) {b}  –  O (Alb) {w}\n"
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Pass => "pass".into(),
            Mv::Place(i) => format!("{}{}", (b'a' + i % 8) as char, i / 8 + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 14,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // Zobrist-style key computed on demand: XOR of one mixed value per piece.
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        match m {
            Mv::Pass => 0,
            Mv::Place(i) => {
                let (r, c) = (*i as usize / 8, *i as usize % 8);
                let edge = |x: usize| x == 0 || x == 7;
                let near = |x: usize| x == 1 || x == 6;
                if edge(r) && edge(c) {
                    100
                } else if near(r) && near(c) {
                    -20
                } else if (edge(r) && near(c)) || (near(r) && edge(c)) {
                    -5
                } else if edge(r) || edge(c) {
                    10
                } else {
                    0
                }
            }
        }
    }

    fn move_key(&self, m: &Mv) -> Option<u32> {
        Some(match m {
            Mv::Place(i) => *i as u32,
            Mv::Pass => 64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_has_four_moves() {
        let g = Reversi::new();
        let mut m: Vec<String> = g.legal_moves().iter().map(|x| g.format_move(x)).collect();
        m.sort();
        assert_eq!(m, vec!["c4", "d3", "e6", "f5"]);
    }

    #[test]
    fn flipping_works() {
        let mut g = Reversi::new();
        g.apply(&Mv::Place(2 * 8 + 3)); // d3
        assert_eq!(g.counts(), (4, 1));
    }
}
