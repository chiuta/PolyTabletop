//! "Patru în linie": 7×6 grid, drop discs, first to align four wins.
//! (The commercial product name is a trademark; the rules are not.)

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 7;
const H: usize = 6;

#[derive(Clone)]
pub struct Connect4 {
    cells: [u8; W * H], // index = row * W + col, row 0 is the bottom
    side: Player,
    count: u8,
    winner: Option<Player>,
}

impl Default for Connect4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Connect4 {
    pub fn new() -> Self {
        Connect4 {
            cells: [0; W * H],
            side: Player::One,
            count: 0,
            winner: None,
        }
    }

    fn wins_at(&self, row: usize, col: usize) -> bool {
        let me = self.cells[row * W + col];
        for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
            let mut n = 1;
            for sign in [1i32, -1] {
                let (mut r, mut c) = (row as i32 + dr * sign, col as i32 + dc * sign);
                while r >= 0 && r < H as i32 && c >= 0 && c < W as i32 {
                    if self.cells[r as usize * W + c as usize] != me {
                        break;
                    }
                    n += 1;
                    r += dr * sign;
                    c += dc * sign;
                }
            }
            if n >= 4 {
                return true;
            }
        }
        false
    }
}

impl Game for Connect4 {
    type Move = u8; // column 0..7

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.winner.is_some() {
            return Vec::new();
        }
        // Centre-first ordering helps alpha-beta.
        [3u8, 2, 4, 1, 5, 0, 6]
            .into_iter()
            .filter(|&c| self.cells[(H - 1) * W + c as usize] == 0)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        let col = *m as usize;
        let me = self.side.idx() as u8 + 1;
        for row in 0..H {
            if self.cells[row * W + col] == 0 {
                self.cells[row * W + col] = me;
                self.count += 1;
                if self.wins_at(row, col) {
                    self.winner = Some(self.side);
                }
                break;
            }
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.winner {
            Some(Outcome::Win(p))
        } else if self.count as usize == W * H {
            Some(Outcome::Draw)
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let you = p.other().idx() as u8 + 1;
        let mut score = 0;
        let weight = |n: i32| match n {
            1 => 1,
            2 => 10,
            3 => 100,
            _ => 0,
        };
        for r in 0..H as i32 {
            for c in 0..W as i32 {
                for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
                    let (er, ec) = (r + dr * 3, c + dc * 3);
                    if er < 0 || er >= H as i32 || ec < 0 || ec >= W as i32 {
                        continue;
                    }
                    let (mut a, mut b) = (0, 0);
                    for k in 0..4 {
                        match self.cells[((r + dr * k) as usize) * W + (c + dc * k) as usize] {
                            x if x == me => a += 1,
                            x if x == you => b += 1,
                            _ => {}
                        }
                    }
                    if b == 0 {
                        score += weight(a);
                    } else if a == 0 {
                        score -= weight(b);
                    }
                }
            }
        }
        // Centre column is worth more.
        for r in 0..H {
            match self.cells[r * W + 3] {
                x if x == me => score += 3,
                x if x == you => score -= 3,
                _ => {}
            }
        }
        score
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for row in (0..H).rev() {
            s.push_str("  | ");
            for col in 0..W {
                s.push(match self.cells[row * W + col] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str("|\n");
        }
        s.push_str("  +---------------+\n    1 2 3 4 5 6 7\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        (m + 1).to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 16,
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

    fn move_key(&self, m: &u8) -> Option<u32> {
        Some(*m as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_win() {
        let mut g = Connect4::new();
        for _ in 0..3 {
            g.apply(&0); // One
            g.apply(&1); // Two
        }
        g.apply(&0);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn diagonal_win() {
        let mut g = Connect4::new();
        // One builds a rising diagonal on columns 0..3.
        let seq = [0u8, 1, 1, 2, 2, 3, 2, 3, 3, 6, 3];
        for m in seq {
            g.apply(&m);
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn full_column_not_legal() {
        let mut g = Connect4::new();
        for _ in 0..6 {
            g.apply(&0);
            if g.outcome().is_some() {
                break;
            }
        }
        if g.outcome().is_none() {
            assert!(!g.legal_moves().contains(&0));
        }
    }
}
