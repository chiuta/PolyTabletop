//! Animale (variantă proprie de mini-șah cu reintroducere pe tablă 3×4): fiecare jucător are un
//! leu (L), o girafă (G), un elefant (E) și un pui (P). Leul merge un pas în orice direcție,
//! girafa un pas ortogonal, elefantul un pas diagonal, puiul un pas înainte; puiul ajuns
//! pe ultimul rând devine cocoș (C), care merge ca leul fără cele două diagonale din spate.
//! Piesele luate trec în mâna celui care le-a luat (cocoșul redevine pui) și pot fi puse
//! mai târziu pe orice celulă goală. Câștigi luând leul adversarului sau ajungând cu leul pe
//! ultimul rând, într-o celulă pe care adversarul nu o poate lua. Remiză după 100 de semi-mutări.
//! Mutări: `b1-b2`, drop: `G@b2` (pune din mână).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 3;
const H: usize = 4;
// 1 leu, 2 girafă, 3 elefant, 4 pui, 5 cocoș
const NAMES: [char; 6] = [' ', 'L', 'G', 'E', 'P', 'C'];

#[derive(Clone)]
pub struct Animale {
    b: [u8; 12], // 0 gol, owner<<3 | tip
    hand: [[u8; 3]; 2],
    side: Player,
    win: Option<Player>,
    ply: u32,
}

impl Default for Animale {
    fn default() -> Self {
        Self::new()
    }
}

fn steps(t: u8, fwd: i32) -> Vec<(i32, i32)> {
    match t {
        1 => vec![
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ],
        2 => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        3 => vec![(1, 1), (-1, 1), (1, -1), (-1, -1)],
        4 => vec![(0, 1)],
        _ => vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1)],
    }
    .into_iter()
    .map(|(dx, dy)| (dx, dy * fwd))
    .collect()
}

impl Animale {
    pub fn new() -> Self {
        let o = 1u8 << 3;
        let mut g = Animale {
            b: [0; 12],
            hand: [[0; 3]; 2],
            side: Player::One,
            win: None,
            ply: 0,
        };
        g.b[0] = 3;
        g.b[1] = 1;
        g.b[2] = 2;
        g.b[4] = 0;
        g.b[W + 1] = 4;
        g.b[3 * W] = o | 2;
        g.b[3 * W + 1] = o | 1;
        g.b[3 * W + 2] = o | 3;
        g.b[2 * W + 1] = o | 4;
        g
    }
    fn attacked_by(&self, cell: usize, p: usize) -> bool {
        let fwd = if p == 0 { 1 } else { -1 };
        for i in 0..12 {
            let v = self.b[i];
            if v == 0 || (v >> 3) as usize != p {
                continue;
            }
            let (x, y) = ((i % W) as i32, (i / W) as i32);
            for (dx, dy) in steps(v & 7, fwd) {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0
                    && ny >= 0
                    && nx < W as i32
                    && ny < H as i32
                    && (ny as usize * W + nx as usize) == cell
                {
                    return true;
                }
            }
        }
        false
    }
}

impl Game for Animale {
    type Move = (u8, u8, u8); // (de la sau 100+tip pentru drop, la, 0)
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.win.is_some() || self.ply >= 100 {
            return vec![];
        }
        let p = self.side.idx();
        let fwd = if p == 0 { 1 } else { -1 };
        let mut v = vec![];
        for i in 0..12 {
            let c = self.b[i];
            if c == 0 || (c >> 3) as usize != p {
                continue;
            }
            let (x, y) = ((i % W) as i32, (i / W) as i32);
            for (dx, dy) in steps(c & 7, fwd) {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= W as i32 || ny >= H as i32 {
                    continue;
                }
                let t = ny as usize * W + nx as usize;
                if self.b[t] != 0 && (self.b[t] >> 3) as usize == p {
                    continue;
                }
                v.push((i as u8, t as u8, 0));
            }
        }
        for k in 0..3 {
            if self.hand[p][k] > 0 {
                for t in 0..12 {
                    if self.b[t] == 0 {
                        v.push((100 + k as u8 + 2, t as u8, 0));
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let p = self.side.idx();
        let me = (p as u8) << 3;
        if m.0 >= 100 {
            let k = (m.0 - 100) as usize; // 2,3,4
            self.hand[p][k - 2] -= 1;
            self.b[m.1 as usize] = me | k as u8;
        } else {
            let c = self.b[m.0 as usize];
            let t = self.b[m.1 as usize];
            if t != 0 {
                let ty = t & 7;
                if ty == 1 {
                    self.win = Some(self.side);
                } else {
                    let base = if ty == 5 { 4 } else { ty };
                    self.hand[p][base as usize - 2] += 1;
                }
            }
            self.b[m.0 as usize] = 0;
            let mut nc = c;
            let ty = c & 7;
            let ny = m.1 as usize / W;
            if ty == 4 && ((p == 0 && ny == H - 1) || (p == 1 && ny == 0)) {
                nc = me | 5;
            }
            self.b[m.1 as usize] = nc;
            if ty == 1 && self.win.is_none() {
                let last = if p == 0 { H - 1 } else { 0 };
                if ny == last && !self.attacked_by(m.1 as usize, 1 - p) {
                    self.win = Some(self.side);
                }
            }
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.ply >= 100 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let val = |t: u8| match t {
            1 => 0,
            2 => 30,
            3 => 30,
            4 => 12,
            _ => 40,
        };
        let mut s = 0;
        for i in 0..12 {
            let c = self.b[i];
            if c == 0 {
                continue;
            }
            let o = (c >> 3) as usize;
            let adv = if o == 0 {
                (i / W) as i32
            } else {
                (H - 1 - i / W) as i32
            };
            let v = val(c & 7) + adv * 2;
            s += if o == me { v } else { -v };
        }
        for k in 0..3 {
            s += (self.hand[me][k] as i32 - self.hand[1 - me][k] as i32) * 25;
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(W, H, |x, y| {
            let c = self.b[y * W + x];
            if c == 0 {
                '.'
            } else {
                let ch = NAMES[(c & 7) as usize];
                if c >> 3 == 0 {
                    ch
                } else {
                    ch.to_ascii_lowercase()
                }
            }
        });
        s.push_str(
            "Majuscule = J1, minuscule = J2 (L leu, G girafă, E elefant, P pui, C cocoș).\n",
        );
        for p in 0..2 {
            s.push_str(&format!(
                "În mână J{}: G×{} E×{} P×{}\n",
                p + 1,
                self.hand[p][0],
                self.hand[p][1],
                self.hand[p][2]
            ));
        }
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        let to = sq(m.1 as usize % W, m.1 as usize / W);
        if m.0 >= 100 {
            format!("{}@{}", NAMES[(m.0 - 100) as usize], to)
        } else {
            format!("{}-{}", sq(m.0 as usize % W, m.0 as usize / W), to)
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 9,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = hkey(
            22,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        );
        for p in 0..2 {
            for k in 0..3 {
                h ^= polytabletop_core::hash_mix(
                    5000 + (p * 3 + k) as u64 * 10 + self.hand[p][k] as u64,
                );
            }
        }
        Some(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn capture_goes_to_hand_and_drop() {
        let mut g = Animale::new();
        g.b = [0; 12];
        g.b[0] = 1;
        g.b[1] = (1 << 3) | 4; // pui J2 pe b1
        g.b[11] = (1 << 3) | 1;
        g.apply(&(0, 1, 0));
        assert_eq!(g.hand[0][2], 1);
        g.apply(&(11, 10, 0));
        let m: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(m.contains(&"P@c3".to_string()));
    }
    #[test]
    fn lion_capture_wins() {
        let mut g = Animale::new();
        g.b = [0; 12];
        g.b[4] = 2; // girafă J1 pe b2
        g.b[7] = (1 << 3) | 1;
        g.b[0] = 1;
        g.b[W] = 0;
        g.apply(&(4, 7, 0));
        // b2 -> b3 nu e leu; mută girafa spre leu: (1,1)->(1,2)=7 este chiar leul J2
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Animale::new(), 150);
    }
}
