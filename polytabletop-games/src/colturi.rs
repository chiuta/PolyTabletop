//! Colțuri (variantă proprie de plasare de poliomino): tablă 10×10, doi jucători, fiecare cu cele
//! nouă forme de 1–4 pătrate (1 monomino, 1 domino, 2 triomino, 5 tetramino). Prima formă a
//! fiecărui jucător acoperă colțul lui (a1 pentru J1, j10 pentru J2). Fiecare formă următoare
//! trebuie să atingă o formă proprie doar prin colț și nu are voie să atingă nicio formă
//! proprie pe latură (formele adverse pot fi atinse). Cine nu mai poate pune trece; când ambii
//! trec, câștigă cine a acoperit mai multe pătrate (egal = remiză).
//! Mutări: `t2.3@c4` = piesa t2, orientarea 3, cu celula cea mai din stânga-jos pe c4;
//! `pas` când nu ai nicio plasare.
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 10;

type Shape = Vec<(i32, i32)>;

fn norm(mut s: Shape) -> Shape {
    let mx = s.iter().map(|c| c.0).min().unwrap();
    let my = s.iter().map(|c| c.1).min().unwrap();
    for c in s.iter_mut() {
        c.0 -= mx;
        c.1 -= my;
    }
    s.sort_by_key(|c| (c.1, c.0));
    s
}

fn rot(s: &Shape) -> Shape {
    norm(s.iter().map(|&(x, y)| (y, -x)).collect())
}
fn flip(s: &Shape) -> Shape {
    norm(s.iter().map(|&(x, y)| (-x, y)).collect())
}

/// Cele 9 forme libere de 1..4 pătrate, cu orientările lor distincte.
pub fn pieces() -> Vec<Vec<Shape>> {
    let bases: [&[(i32, i32)]; 9] = [
        &[(0, 0)],
        &[(0, 0), (1, 0)],
        &[(0, 0), (1, 0), (2, 0)],
        &[(0, 0), (1, 0), (0, 1)],
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(0, 0), (1, 0), (2, 0), (0, 1)],
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
        &[(0, 0), (1, 0), (2, 0), (1, 1)],
        &[(0, 0), (1, 0), (1, 1), (2, 1)],
    ];
    bases
        .iter()
        .map(|b| {
            let mut v: Vec<Shape> = vec![];
            let mut s = norm(b.to_vec());
            for f in 0..2 {
                for _ in 0..4 {
                    if !v.contains(&s) {
                        v.push(s.clone());
                    }
                    s = rot(&s);
                }
                if f == 0 {
                    s = flip(&s);
                }
            }
            v
        })
        .collect()
}

#[derive(Clone)]
pub struct Colturi {
    b: [u8; N * N],
    used: [u16; 2],
    side: Player,
    passes: u8,
    placed: [u8; 2],
    shapes: std::rc::Rc<Vec<Vec<Shape>>>,
}

impl Default for Colturi {
    fn default() -> Self {
        Self::new()
    }
}

impl Colturi {
    pub fn new() -> Self {
        Colturi {
            b: [0; N * N],
            used: [0; 2],
            side: Player::One,
            passes: 0,
            placed: [0; 2],
            shapes: std::rc::Rc::new(pieces()),
        }
    }
    fn gen(&self, p: usize) -> Vec<(u8, u8, u8)> {
        let me = p as u8 + 1;
        let first = self.placed[p] == 0;
        let start = if p == 0 { 0 } else { N * N - 1 };
        let mut v = vec![];
        for (pi, orients) in self.shapes.iter().enumerate() {
            if self.used[p] >> pi & 1 == 1 {
                continue;
            }
            for (oi, sh) in orients.iter().enumerate() {
                let (w, h) = (
                    sh.iter().map(|c| c.0).max().unwrap() as usize + 1,
                    sh.iter().map(|c| c.1).max().unwrap() as usize + 1,
                );
                for oy in 0..=(N - h) {
                    for ox in 0..=(N - w) {
                        let mut ok = true;
                        let mut corner = false;
                        let mut covers = false;
                        for &(cx, cy) in sh {
                            let (x, y) = (ox as i32 + cx, oy as i32 + cy);
                            let i = y as usize * N + x as usize;
                            if self.b[i] != 0 {
                                ok = false;
                                break;
                            }
                            if i == start {
                                covers = true;
                            }
                            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                                let (nx, ny) = (x + dx, y + dy);
                                if nx >= 0
                                    && ny >= 0
                                    && nx < N as i32
                                    && ny < N as i32
                                    && self.b[ny as usize * N + nx as usize] == me
                                {
                                    ok = false;
                                }
                            }
                            for (dx, dy) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                                let (nx, ny) = (x + dx, y + dy);
                                if nx >= 0
                                    && ny >= 0
                                    && nx < N as i32
                                    && ny < N as i32
                                    && self.b[ny as usize * N + nx as usize] == me
                                {
                                    corner = true;
                                }
                            }
                            if !ok {
                                break;
                            }
                        }
                        if ok && ((first && covers) || (!first && corner)) {
                            v.push((pi as u8, oi as u8, (oy * N + ox) as u8));
                        }
                    }
                }
            }
        }
        v
    }
}

impl Game for Colturi {
    type Move = (u8, u8, u8); // piesă, orientare, ancoră; (255,..) = pas
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.passes >= 2 {
            return vec![];
        }
        let v = self.gen(self.side.idx());
        if v.is_empty() {
            vec![(255, 0, 0)]
        } else {
            v
        }
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let p = self.side.idx();
        if m.0 == 255 {
            self.passes += 1;
        } else {
            self.passes = 0;
            let sh = &self.shapes[m.0 as usize][m.1 as usize];
            let (ox, oy) = (m.2 as usize % N, m.2 as usize / N);
            for &(cx, cy) in sh {
                self.b[(oy + cy as usize) * N + ox + cx as usize] = p as u8 + 1;
            }
            self.used[p] |= 1 << m.0;
            self.placed[p] += sh.len() as u8;
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.passes < 2 {
            return None;
        }
        Some(match self.placed[0].cmp(&self.placed[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (self.placed[p.idx()] as i32, self.placed[1 - p.idx()] as i32);
        (a - b) * 10 + (self.gen(p.idx()).len() as i32 - self.gen(1 - p.idx()).len() as i32) / 4
    }
    fn render(&self) -> String {
        let mut s = grid(N, N, |x, y| match self.b[y * N + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        });
        for p in 0..2 {
            s.push_str(&format!(
                "J{} are acoperite {} pătrate; piese rămase: ",
                p + 1,
                self.placed[p]
            ));
            for i in 0..9 {
                if self.used[p] >> i & 1 == 0 {
                    s.push_str(&format!("t{} ", i + 1));
                }
            }
            s.push('\n');
        }
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        if m.0 == 255 {
            return "pas".into();
        }
        format!(
            "t{}.{}@{}",
            m.0 + 1,
            m.1 + 1,
            sq(m.2 as usize % N, m.2 as usize / N)
        )
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::Mcts(120)
    }
    fn playout_cap(&self) -> usize {
        40
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            17,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn piece_set() {
        let p = pieces();
        let counts: Vec<usize> = p.iter().map(|o| o.len()).collect();
        assert_eq!(counts, [1, 2, 2, 4, 2, 8, 1, 4, 4]);
    }
    #[test]
    fn first_move_covers_corner() {
        let g = Colturi::new();
        let m = g.legal_moves();
        assert!(!m.is_empty());
        let text = g.format_move(&m[0]);
        assert!(text.contains('@'));
        // monomino în a1
        assert!(m.contains(&(0, 0, 0)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Colturi::new(), 120);
    }
}
