//! Discuri (variantă proprie de fotbal cu discuri pe ture): teren de 9×13 celule, porți late de 3
//! celule la mijlocul capetelor. Fiecare echipă are 5 discuri; mingea e la centru. O tură:
//! alegi un disc propriu, o direcție (8) și o putere 1–3: discul alunecă până la atâtea celule,
//! oprindu-se înaintea altui disc sau a marginii; dacă drumul lui ajunge la minge, discul se
//! oprește lângă ea și mingea e lovită în aceeași direcție cu atâtea celule cât puterea,
//! alunecând până înaintea unui disc sau a marginii. Mingea care iese pe la capăt prin poartă
//! e gol (discurile revin la poziții). Primul la 2 goluri câștigă; după 160 de semi-mutări
//! câștigă cine are mai multe goluri. Mutări: `c3ne2` (discul de pe c3, direcția ne, puterea 2).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 9;
const H: i32 = 13;
const DIRS: [(&str, i32, i32); 8] = [
    ("n", 0, 1),
    ("s", 0, -1),
    ("e", 1, 0),
    ("v", -1, 0),
    ("ne", 1, 1),
    ("nv", -1, 1),
    ("se", 1, -1),
    ("sv", -1, -1),
];

#[derive(Clone)]
pub struct Discuri {
    discs: [[(i32, i32); 5]; 2],
    ball: (i32, i32),
    score: [u8; 2],
    side: Player,
    ply: u32,
}

impl Default for Discuri {
    fn default() -> Self {
        Self::new()
    }
}

fn start() -> [[(i32, i32); 5]; 2] {
    [
        [(4, 1), (2, 3), (6, 3), (3, 5), (5, 5)],
        [(4, 11), (2, 9), (6, 9), (3, 7), (5, 7)],
    ]
}

impl Discuri {
    pub fn new() -> Self {
        Discuri {
            discs: start(),
            ball: (4, 6),
            score: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn occupied(&self, c: (i32, i32)) -> bool {
        self.discs.iter().flatten().any(|&d| d == c)
    }
    fn goal(&self) -> Option<usize> {
        None
    }
}

impl Game for Discuri {
    type Move = (u8, u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        let _ = self.goal();
        for i in 0..5 {
            let (x, y) = self.discs[p][i];
            for (d, &(_, dx, dy)) in DIRS.iter().enumerate() {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= W || ny >= H || self.occupied((nx, ny)) {
                    continue;
                }
                for pw in 1..=3 {
                    v.push((i as u8, d as u8, pw));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let p = self.side.idx();
        let (i, d, pw) = (m.0 as usize, m.1 as usize, m.2 as i32);
        let (_, dx, dy) = DIRS[d];
        let mut cur = self.discs[p][i];
        let mut touched = false;
        for k in 0..pw {
            let nx = (cur.0 + dx, cur.1 + dy);
            if nx.0 < 0 || nx.1 < 0 || nx.0 >= W || nx.1 >= H || self.occupied(nx) {
                break;
            }
            if nx == self.ball {
                touched = true;
                let _ = k;
                break;
            }
            cur = nx;
        }
        self.discs[p][i] = cur;
        let mut scored = None;
        if touched {
            let mut b = self.ball;
            for _ in 0..pw {
                let nb = (b.0 + dx, b.1 + dy);
                if nb.1 < 0 || nb.1 >= H {
                    if (3..=5).contains(&b.0) && (3..=5).contains(&nb.0) {
                        scored = Some(if nb.1 >= H { 0 } else { 1 });
                    }
                    break;
                }
                if nb.0 < 0 || nb.0 >= W || self.occupied(nb) {
                    break;
                }
                b = nb;
            }
            self.ball = b;
        }
        if let Some(s) = scored {
            self.score[s] += 1;
            self.discs = start();
            self.ball = (4, 6);
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.score[0] >= 2 {
            return Some(Outcome::Win(Player::One));
        }
        if self.score[1] >= 2 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.ply >= 160 {
            return Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.score[me] as i32 - self.score[1 - me] as i32) * 1000;
        let adv = if me == 0 {
            self.ball.1
        } else {
            H - 1 - self.ball.1
        };
        s += adv * 12 - (self.ball.0 - 4).abs() * 2;
        let near = |q: usize| {
            self.discs[q]
                .iter()
                .map(|d| (d.0 - self.ball.0).abs().max((d.1 - self.ball.1).abs()))
                .min()
                .unwrap()
        };
        s + (near(1 - me) - near(me)) * 5
    }
    fn render(&self) -> String {
        let mut s = grid(W as usize, H as usize, |x, y| {
            let c = (x as i32, y as i32);
            if c == self.ball {
                '*'
            } else if self.discs[0].contains(&c) {
                'A'
            } else if self.discs[1].contains(&c) {
                'B'
            } else if (y == 0 || y == H as usize - 1) && (3..=5).contains(&x) {
                '='
            } else {
                '.'
            }
        });
        s.push_str(&format!(
            "A = J1 (jos), B = J2 (sus), * = mingea ({}), scor {}–{}\n",
            sq(self.ball.0 as usize, self.ball.1 as usize),
            self.score[0],
            self.score[1]
        ));
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        let c = self.discs[self.side.idx()][m.0 as usize];
        format!(
            "{}{}{}",
            sq(c.0 as usize, c.1 as usize),
            DIRS[m.1 as usize].0,
            m.2
        )
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 4,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = hkey(28, self.side, std::iter::empty());
        for (p, ds) in self.discs.iter().enumerate() {
            for d in ds {
                h ^= polytabletop_core::hash_mix(
                    ((p as u64) << 16) | ((d.0 as u64) << 8) | d.1 as u64 | 1 << 40,
                );
            }
        }
        Some(
            h ^ polytabletop_core::hash_mix(
                ((self.ball.0 as u64) << 8 | self.ball.1 as u64)
                    + (self.score[0] as u64 * 7 + self.score[1] as u64) * 65536
                    + 5 * (1 << 41),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn kick_and_goal() {
        let mut g = Discuri::new();
        g.discs[0][0] = (4, 5);
        g.ball = (4, 6);
        g.apply(&(0, 0, 2)); // n, puterea 2: atinge mingea imediat
        assert_eq!(g.discs[0][0], (4, 5));
        assert_eq!(g.ball, (4, 8));
        let mut h = Discuri::new();
        h.ball = (4, 12);
        h.discs[0][0] = (4, 11);
        h.discs[1][0] = (0, 0);
        h.apply(&(0, 0, 1));
        assert_eq!(h.score[0], 1);
        assert_eq!(h.ball, (4, 6));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Discuri::new(), 300);
    }
}
