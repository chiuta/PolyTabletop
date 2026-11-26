//! Banchiza — pinguini pe sloiuri de gheață (variantă originală de colectare pe hexagoane).
//!
//! Tablă de 60 de plăci hexagonale (8 rânduri, rândurile impare sunt decalate), cu 1–3 pești.
//! Faza 1: fiecare pune alternativ 4 pinguini pe plăci cu exact un pește (`pune x,y`).
//! Faza 2: muți un pinguin în linie dreaptă, peste plăci libere, oricât (`mut x,y>x2,y2`);
//! placa părăsită dispare și peștii ei sunt ai tăi. Cine nu poate muta pasează (`pas`).
//! Când nimeni nu mai poate muta, câștigă cine are mai mulți pești (egal: remiză).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const DIRS: [(i8, i8); 6] = [(2, 0), (-2, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];

#[derive(Clone)]
pub struct Banchiza {
    fish: [[u8; 16]; 8],
    pen: [[u8; 16]; 8], // 0 liber, 1 = J1, 2 = J2
    placed: [u8; 2],
    score: [i32; 2],
    side: Player,
    passes: u8,
}

impl Banchiza {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut v: Vec<u8> = Vec::new();
        v.extend([1u8; 30]);
        v.extend([2u8; 20]);
        v.extend([3u8; 10]);
        shuf(&mut v, &mut rng);
        let mut fish = [[0u8; 16]; 8];
        let mut i = 0;
        for y in 0..8 {
            let n = if y % 2 == 0 { 8 } else { 7 };
            for c in 0..n {
                fish[y][2 * c + (y % 2)] = v[i];
                i += 1;
            }
        }
        Banchiza {
            fish,
            pen: [[0; 16]; 8],
            placed: [0; 2],
            score: [0; 2],
            side: Player::One,
            passes: 0,
        }
    }
    fn free(&self, x: i8, y: i8) -> bool {
        x >= 0
            && y >= 0
            && x < 16
            && y < 8
            && self.fish[y as usize][x as usize] > 0
            && self.pen[y as usize][x as usize] == 0
    }
    fn slides(&self, p: usize) -> Vec<(i8, i8, i8, i8)> {
        let mut v = Vec::new();
        for y in 0..8i8 {
            for x in 0..16i8 {
                if self.pen[y as usize][x as usize] as usize == p + 1 {
                    for (dx, dy) in DIRS {
                        let (mut cx, mut cy) = (x + dx, y + dy);
                        while self.free(cx, cy) {
                            v.push((x, y, cx, cy));
                            cx += dx;
                            cy += dy;
                        }
                    }
                }
            }
        }
        v
    }
    fn mobility(&self, p: usize) -> i32 {
        self.slides(p).len() as i32
    }
}

impl Default for Banchiza {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Banchiza {
    fn depth(&self) -> u32 {
        2
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let p = pi(self.side);
        if self.placed[p] < 4 {
            let mut v = Vec::new();
            for y in 0..8 {
                for x in 0..16 {
                    if self.fish[y][x] == 1 && self.pen[y][x] == 0 {
                        v.push(format!("pune {},{}", x, y));
                    }
                }
            }
            return v;
        }
        let m: Vec<String> = self
            .slides(p)
            .iter()
            .map(|&(a, b, c, d)| format!("mut {},{}>{},{}", a, b, c, d))
            .collect();
        if m.is_empty() {
            vec!["pas".into()]
        } else {
            m
        }
    }
    fn step(&mut self, m: &str) {
        let p = pi(self.side);
        if m == "pas" {
            self.passes += 1;
        } else if let Some(r) = m.strip_prefix("pune ") {
            let (x, y) = r.split_once(',').unwrap();
            self.pen[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = p as u8 + 1;
            self.placed[p] += 1;
            self.passes = 0;
        } else {
            let r = m.strip_prefix("mut ").unwrap();
            let (a, b) = r.split_once('>').unwrap();
            let pa: Vec<usize> = a.split(',').map(|s| s.parse().unwrap()).collect();
            let pb: Vec<usize> = b.split(',').map(|s| s.parse().unwrap()).collect();
            self.score[p] += self.fish[pa[1]][pa[0]] as i32;
            self.fish[pa[1]][pa[0]] = 0;
            self.pen[pa[1]][pa[0]] = 0;
            self.pen[pb[1]][pb[0]] = p as u8 + 1;
            self.passes = 0;
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.placed == [4, 4] && self.passes >= 2 {
            // pinguinii rămași își ia fiecare peștii plăcii de sub ei
            let mut s = self.score;
            for y in 0..8 {
                for x in 0..16 {
                    if self.pen[y][x] > 0 {
                        s[self.pen[y][x] as usize - 1] += self.fish[y][x] as i32;
                    }
                }
            }
            return Some(by_score(s[0], s[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let (a, b) = (pi(p), 1 - pi(p));
        10 * (self.score[a] - self.score[b]) + self.mobility(a) - self.mobility(b)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Banchiza — pești: J1 {} | J2 {} | pinguini puși: {}/{}, {}/{}\n   ",
            self.score[0], self.score[1], self.placed[0], 4, self.placed[1], 4
        );
        for x in 0..16 {
            s.push_str(&format!("{:<3}", x));
        }
        s.push('\n');
        for y in 0..8 {
            s.push_str(&format!("{:>2} ", y));
            for x in 0..16 {
                let f = self.fish[y][x];
                if f == 0 && !(y % 2 == x % 2 && x <= if y % 2 == 0 { 14 } else { 13 }) {
                    s.push_str("   ");
                } else if f == 0 {
                    s.push_str(" . ");
                } else {
                    let mk = match self.pen[y][x] {
                        1 => '*',
                        2 => 'o',
                        _ => ' ',
                    };
                    s.push_str(&format!("{}{} ", f, mk));
                }
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "La mutare: {} (J1 = *, J2 = o). Mutări: pune x,y | mut x,y>x2,y2 | pas\n",
            pname(self.side)
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Banchiza::with_seed, 300);
    }
    #[test]
    fn sixty_tiles() {
        let g = Banchiza::with_seed(3);
        let n: usize = g
            .fish
            .iter()
            .map(|r| r.iter().filter(|&&f| f > 0).count())
            .sum();
        assert_eq!(n, 60);
    }
}
