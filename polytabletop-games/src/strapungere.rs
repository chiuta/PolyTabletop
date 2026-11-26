//! "Străpungere": 8×8 pawn race. Each side starts with 16 pieces on its two
//! home rows. A piece steps one square straight ahead onto an empty square, or
//! one square diagonally ahead onto an empty square or an enemy piece (capture).
//! First to reach the far row wins; capturing every enemy piece also wins; a
//! side with no legal move loses. Public-domain style abstract game.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 8;

#[derive(Clone)]
pub struct Strapungere {
    cells: [u8; N * N], // index = rank*8 + file, rank 0 is One's home row
    side: Player,
    winner: Option<Player>,
    count: [u8; 2],
}

impl Default for Strapungere {
    fn default() -> Self {
        Self::new()
    }
}

impl Strapungere {
    pub fn new() -> Self {
        let mut cells = [0u8; N * N];
        for f in 0..N {
            cells[f] = 1;
            cells[N + f] = 1;
            cells[6 * N + f] = 2;
            cells[7 * N + f] = 2;
        }
        Strapungere {
            cells,
            side: Player::One,
            winner: None,
            count: [16, 16],
        }
    }

    fn dir(p: Player) -> i32 {
        if p == Player::One {
            1
        } else {
            -1
        }
    }

    fn gen(&self, p: Player, out: &mut Vec<(u8, u8)>, first_only: bool) {
        let me = p.idx() as u8 + 1;
        let d = Self::dir(p);
        for i in 0..N * N {
            if self.cells[i] != me {
                continue;
            }
            let (r, f) = ((i / N) as i32, (i % N) as i32);
            let nr = r + d;
            if !(0..N as i32).contains(&nr) {
                continue;
            }
            for df in [-1i32, 0, 1] {
                let nf = f + df;
                if !(0..N as i32).contains(&nf) {
                    continue;
                }
                let t = (nr as usize) * N + nf as usize;
                let ok = if df == 0 {
                    self.cells[t] == 0
                } else {
                    self.cells[t] != me
                };
                if ok {
                    out.push((i as u8, t as u8));
                    if first_only {
                        return;
                    }
                }
            }
        }
    }

    fn sq(i: u8) -> String {
        polytabletop_core::square_name(i as usize % N, i as usize / N)
    }

    pub fn pieces(&self, p: Player) -> u8 {
        self.count[p.idx()]
    }
}

impl Game for Strapungere {
    type Move = (u8, u8);

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<(u8, u8)> {
        let mut v = Vec::new();
        if self.winner.is_none() {
            self.gen(self.side, &mut v, false);
            v.sort_by_key(|m| -self.move_score(m));
        }
        v
    }

    fn apply(&mut self, m: &(u8, u8)) {
        let (a, b) = (m.0 as usize, m.1 as usize);
        let me = self.cells[a];
        if self.cells[b] != 0 {
            let o = (self.cells[b] - 1) as usize;
            self.count[o] -= 1;
            if self.count[o] == 0 {
                self.winner = Some(self.side);
            }
        }
        self.cells[a] = 0;
        self.cells[b] = me;
        let goal = if self.side == Player::One { N - 1 } else { 0 };
        if b / N == goal {
            self.winner = Some(self.side);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        let mut v = Vec::new();
        self.gen(self.side, &mut v, true);
        if v.is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0i32;
        for i in 0..N * N {
            let c = self.cells[i];
            if c == 0 {
                continue;
            }
            let owner = if c == 1 { Player::One } else { Player::Two };
            let rank = (i / N) as i32;
            let adv = if owner == Player::One { rank } else { 7 - rank };
            let mut v = 100 + adv * adv;
            if adv >= 5 {
                v += (adv - 4) * 60;
            }
            // home-row guards are worth a little
            if adv == 0 {
                v += 12;
            }
            s += if owner == p { v } else { -v };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..N).rev() {
            s.push_str(&format!("{} ", r + 1));
            for f in 0..N {
                s.push(match self.cells[r * N + f] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g h\n");
        s.push_str(&format!(
            "X (J1, urcă) {} piese, O (J2, coboară) {} piese\n",
            self.count[0], self.count[1]
        ));
        s
    }

    fn format_move(&self, m: &(u8, u8)) -> String {
        let cap = self.cells[m.1 as usize] != 0;
        format!(
            "{}{}{}",
            Self::sq(m.0),
            if cap { 'x' } else { '-' },
            Self::sq(m.1)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 12,
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

    fn move_score(&self, m: &(u8, u8)) -> i32 {
        let cap = if self.cells[m.1 as usize] != 0 { 50 } else { 0 };
        let rank = m.1 as usize / N;
        let adv = if self.side == Player::One {
            rank
        } else {
            7 - rank
        } as i32;
        cap + adv * adv
    }

    fn move_key(&self, m: &(u8, u8)) -> Option<u32> {
        Some(m.0 as u32 * 64 + m.1 as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn initial_moves() {
        let g = Strapungere::new();
        // front row: 8 straight + 14 diagonal = 22
        assert_eq!(g.legal_moves().len(), 22);
    }

    #[test]
    fn forward_cannot_capture_but_diagonal_can() {
        let mut g = Strapungere::new();
        g.cells = [0; 64];
        g.cells[3 * N + 3] = 1; // d4
        g.cells[4 * N + 3] = 2; // d5 blocks straight
        g.cells[4 * N + 4] = 2; // e5 capturable
        g.cells[7 * N] = 2; // keep Two alive
        g.count = [1, 3];
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(!t.contains(&"d4-d5".to_string()) && !t.contains(&"d4xd5".to_string()));
        assert!(t.contains(&"d4xe5".to_string()));
        assert!(t.contains(&"d4-c5".to_string()));
        g.play_text("d4xe5");
        assert_eq!(g.pieces(Player::Two), 2);
    }

    impl Strapungere {
        fn play_text(&mut self, t: &str) {
            let m = *self
                .legal_moves()
                .iter()
                .find(|m| self.format_move(m) == t)
                .unwrap();
            self.apply(&m);
        }
    }

    #[test]
    fn reaching_last_row_wins() {
        let mut g = Strapungere::new();
        g.cells = [0; 64];
        g.cells[6 * N + 2] = 1;
        g.cells[N] = 2;
        g.count = [1, 1];
        g.play_text("c7-c8");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn no_moves_loses() {
        let mut g = Strapungere::new();
        g.cells = [0; 64];
        g.cells[0] = 2; // Two at a1 on rank 0 cannot move down
        g.cells[N * 7] = 1;
        g.count = [1, 1];
        g.side = Player::Two;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn capturing_everything_wins() {
        let mut g = Strapungere::new();
        g.cells = [0; 64];
        g.cells[3 * N + 3] = 1;
        g.cells[4 * N + 4] = 2;
        g.count = [1, 1];
        g.play_text("d4xe5");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_playouts_end() {
        for seed in 1..=30 {
            let mut g = Strapungere::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let l = g.legal_moves();
                assert!(!l.is_empty());
                g.apply(&l[rng.below(l.len())]);
                n += 1;
                assert!(n < 400, "too long");
            }
        }
    }
}
