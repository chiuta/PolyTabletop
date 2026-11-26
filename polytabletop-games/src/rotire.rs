//! "Rotire": 6×6 board in four 3×3 quadrants. A turn = place a marble on an
//! empty cell, then rotate one quadrant a quarter turn. Five in a row
//! (any direction) after the rotation wins; if both players have five at once,
//! or the board fills with no five, the game is drawn.

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const N: usize = 6;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rot {
    pub cell: u8,
    /// 1 = top-left, 2 = top-right, 3 = bottom-left, 4 = bottom-right
    pub quad: u8,
    pub cw: bool,
}

#[derive(Clone)]
pub struct Rotire {
    cells: [u8; N * N], // index = row*6 + col, row 0 is rank 1 (bottom)
    side: Player,
    result: Option<Outcome>,
    count: u8,
}

impl Default for Rotire {
    fn default() -> Self {
        Self::new()
    }
}

fn windows() -> &'static Vec<[u8; 5]> {
    static W: OnceLock<Vec<[u8; 5]>> = OnceLock::new();
    W.get_or_init(|| {
        let mut w = Vec::new();
        for r in 0..N as i32 {
            for c in 0..N as i32 {
                for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
                    let (er, ec) = (r + 4 * dr, c + 4 * dc);
                    if !(0..N as i32).contains(&er) || !(0..N as i32).contains(&ec) {
                        continue;
                    }
                    let mut a = [0u8; 5];
                    for (k, x) in a.iter_mut().enumerate() {
                        *x =
                            ((r + dr * k as i32) as usize * N + (c + dc * k as i32) as usize) as u8;
                    }
                    w.push(a);
                }
            }
        }
        w
    })
}

fn quad_origin(q: u8) -> (usize, usize) {
    match q {
        1 => (3, 0),
        2 => (3, 3),
        3 => (0, 0),
        _ => (0, 3),
    }
}

impl Rotire {
    pub fn new() -> Self {
        Rotire {
            cells: [0; N * N],
            side: Player::One,
            result: None,
            count: 0,
        }
    }

    fn rotate(&mut self, q: u8, cw: bool) {
        let (r0, c0) = quad_origin(q);
        let old = self.cells;
        for y in 0..3i32 {
            for x in 0..3i32 {
                let (dx, dy) = (x - 1, y - 1);
                let (nx, ny) = if cw {
                    (1 + dy, 1 - dx)
                } else {
                    (1 - dy, 1 + dx)
                };
                let from = (r0 + y as usize) * N + c0 + x as usize;
                let to = (r0 + ny as usize) * N + c0 + nx as usize;
                self.cells[to] = old[from];
            }
        }
    }

    fn has_five(&self, who: u8) -> bool {
        windows()
            .iter()
            .any(|w| w.iter().all(|&i| self.cells[i as usize] == who))
    }

    fn sq(i: u8) -> String {
        polytabletop_core::square_name(i as usize % N, i as usize / N)
    }

    pub fn cell(&self, row: usize, col: usize) -> u8 {
        self.cells[row * N + col]
    }
}

impl Game for Rotire {
    type Move = Rot;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Rot> {
        let mut v = Vec::new();
        if self.result.is_some() {
            return v;
        }
        for cell in 0..(N * N) as u8 {
            if self.cells[cell as usize] != 0 {
                continue;
            }
            for quad in 1..=4u8 {
                for cw in [true, false] {
                    v.push(Rot { cell, quad, cw });
                }
            }
        }
        v.sort_by_key(|m| -self.move_score(m));
        v
    }

    fn apply(&mut self, m: &Rot) {
        self.cells[m.cell as usize] = self.side.idx() as u8 + 1;
        self.count += 1;
        self.rotate(m.quad, m.cw);
        let (a, b) = (self.has_five(1), self.has_five(2));
        self.result = match (a, b) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::One)),
            (false, true) => Some(Outcome::Win(Player::Two)),
            _ if self.count as usize == N * N => Some(Outcome::Draw),
            _ => None,
        };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        const W: [i32; 6] = [0, 1, 6, 40, 400, 0];
        let me = p.idx() as u8 + 1;
        let you = 3 - me;
        let mut s = 0;
        for w in windows() {
            let (mut a, mut b) = (0, 0);
            for &i in w {
                match self.cells[i as usize] {
                    x if x == me => a += 1,
                    x if x == you => b += 1,
                    _ => {}
                }
            }
            if b == 0 {
                s += W[a];
            } else if a == 0 {
                s -= W[b];
            }
        }
        // Quadrant centres never move under rotation.
        for i in [7usize, 10, 25, 28] {
            match self.cells[i] {
                x if x == me => s += 6,
                x if x == you => s -= 6,
                _ => {}
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..N).rev() {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..N {
                if c == 3 {
                    s.push_str("| ");
                }
                s.push(match self.cells[r * N + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
            if r == 3 {
                s.push_str("  ------+------\n");
            }
        }
        s.push_str("  a b c | d e f\nCadrane: 1 stânga-sus, 2 dreapta-sus, 3 stânga-jos, 4 dreapta-jos; d = spre dreapta (orar), s = spre stânga.\n");
        s
    }

    fn format_move(&self, m: &Rot) -> String {
        format!(
            "{}/{}{}",
            Self::sq(m.cell),
            m.quad,
            if m.cw { 'd' } else { 's' }
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 800,
            max_depth: 8,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &c) in self.cells.iter().enumerate() {
            if c != 0 {
                h ^= polytabletop_core::hash_mix((c as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Rot) -> i32 {
        let (r, c) = (m.cell as i32 / N as i32, m.cell as i32 % N as i32);
        6 - ((2 * r - 5).abs() + (2 * c - 5).abs()) / 2
    }

    fn move_key(&self, m: &Rot) -> Option<u32> {
        Some(m.cell as u32 * 8 + (m.quad as u32 - 1) * 2 + m.cw as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn move_count_and_unique_text() {
        let g = Rotire::new();
        let l = g.legal_moves();
        assert_eq!(l.len(), 36 * 8);
        let mut t: Vec<String> = l.iter().map(|m| g.format_move(m)).collect();
        t.sort();
        t.dedup();
        assert_eq!(t.len(), 288);
    }

    #[test]
    fn rotation_four_times_is_identity_and_cw_ccw_cancel() {
        let mut g = Rotire::new();
        for (i, c) in g.cells.iter_mut().enumerate() {
            *c = (i % 3) as u8;
        }
        let orig = g.cells;
        for q in 1..=4 {
            for _ in 0..4 {
                g.rotate(q, true);
            }
            assert_eq!(g.cells, orig);
            g.rotate(q, true);
            g.rotate(q, false);
            assert_eq!(g.cells, orig);
        }
    }

    #[test]
    fn clockwise_moves_top_left_corner_to_top_right() {
        let mut g = Rotire::new();
        // quadrant 3 (bottom-left): local top-left = row 2, col 0 -> row 2, col 2 after cw
        g.cells[2 * N] = 1;
        g.rotate(3, true);
        assert_eq!(g.cells[2 * N], 0);
        assert_eq!(g.cells[2 * N + 2], 1);
    }

    #[test]
    fn five_in_a_row_wins_after_rotation() {
        let mut g = Rotire::new();
        // One has a1..d1 (row 0) and e1 empty; place e1 then rotate empty quadrant 1.
        for c in 0..4 {
            g.cells[c] = 1;
        }
        g.count = 4;
        let m = Rot {
            cell: 4,
            quad: 1,
            cw: true,
        };
        g.apply(&m);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn rotation_can_break_own_line() {
        let mut g = Rotire::new();
        for c in 0..4 {
            g.cells[c] = 1;
        }
        g.count = 4;
        // placing e1 and rotating quadrant 3 (contains a1..c1) breaks the five
        g.apply(&Rot {
            cell: 4,
            quad: 3,
            cw: true,
        });
        assert_eq!(g.outcome(), None);
    }

    #[test]
    fn full_board_ends_the_game() {
        let mut g = Rotire::new();
        let pat = [1u8, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2];
        for i in 0..36 {
            g.cells[i] = pat[(i + i / 6 * 2) % 12];
        }
        g.cells[35] = 0;
        g.count = 35;
        g.side = Player::One;
        g.apply(&Rot {
            cell: 35,
            quad: 4,
            cw: true,
        });
        assert!(g.outcome().is_some());
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn random_playouts_end() {
        for seed in 1..=30 {
            let mut g = Rotire::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let l = g.legal_moves();
                assert!(!l.is_empty());
                g.apply(&l[rng.below(l.len())]);
                n += 1;
                assert!(n <= 36);
            }
        }
    }
}
