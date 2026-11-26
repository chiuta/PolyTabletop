//! Helperi comuni pentru motoarele de miniaturi / skirmish (copia w7):
//! zaruri cu sămânță care se schimbă la `clone()` și geometrie pe grilă pătrată.

use polytabletop_core::rng::Rng;
use std::cell::Cell;

/// Zarurile stării. La `clone()` sămânța se schimbă, ca o căutare AI să nu poată
/// „prevedea” aruncarea reală (care se face doar la mutarea forțată `arunca`).
pub struct Dice {
    rng: Rng,
    ctr: Cell<u64>,
}

impl Dice {
    pub fn new(seed: u64) -> Self {
        Dice {
            rng: Rng::new(seed),
            ctr: Cell::new(0),
        }
    }
    pub fn d6(&mut self) -> u8 {
        self.rng.d6()
    }
    pub fn d20(&mut self) -> u8 {
        (self.rng.below(20) + 1) as u8
    }
    pub fn below(&mut self, n: usize) -> usize {
        self.rng.below(n)
    }
    pub fn two_d6(&mut self) -> u8 {
        self.d6() + self.d6()
    }
}

impl Clone for Dice {
    fn clone(&self) -> Self {
        let k = self.ctr.get().wrapping_add(1);
        self.ctr.set(k);
        let mut r = self.rng.clone();
        Dice {
            rng: Rng::new(r.next_u64() ^ k.wrapping_mul(0x9E37_79B9_7F4A_7C15)),
            ctr: Cell::new(0),
        }
    }
}

/// Numele pătratului (x, y) ca „d4” (coloană literă, rând număr, de la 1).
pub fn cell_name(x: i32, y: i32) -> String {
    format!("{}{}", (b'a' + x as u8) as char, y + 1)
}

/// Parsează „d4” într-un pătrat din grila `w`×`h`.
pub fn parse_cell(s: &str, w: i32, h: i32) -> Option<(i32, i32)> {
    let mut ch = s.chars();
    let c = ch.next()?;
    if !c.is_ascii_lowercase() {
        return None;
    }
    let r: i32 = ch.as_str().parse().ok()?;
    let x = (c as u8 - b'a') as i32;
    let y = r - 1;
    if x < w && y >= 0 && y < h {
        Some((x, y))
    } else {
        None
    }
}

/// Distanța Chebyshev (mutări de rege).
pub fn cheb(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}

/// Pătratele traversate de segmentul a→b (fără capete), simetric: linia se
/// calculează mereu de la punctul mai mic lexicografic.
pub fn line_between(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let (p, q) = if a <= b { (a, b) } else { (b, a) };
    let dx = q.0 - p.0;
    let dy = q.1 - p.1;
    let n = dx.abs().max(dy.abs());
    let mut v = Vec::new();
    for i in 1..n {
        // rotunjire pe jumătăți, determinist
        let x = p.0 * 2 * n + 2 * dx * i + n;
        let y = p.1 * 2 * n + 2 * dy * i + n;
        v.push((x.div_euclid(2 * n), y.div_euclid(2 * n)));
    }
    v
}

/// Grilă cu ziduri (blochează mersul și vederea) și acoperiri; vizibilitatea
/// se precalculează o singură dată (se partajează cu `Arc` între clone).
pub struct GridMap {
    pub w: i32,
    pub h: i32,
    pub wall: Vec<bool>,
    pub cover: Vec<bool>,
    los: Vec<bool>,
}

impl GridMap {
    pub fn build(w: i32, h: i32, walls: &[(i32, i32)], covers: &[(i32, i32)]) -> GridMap {
        let n = (w * h) as usize;
        let mut wall = vec![false; n];
        let mut cover = vec![false; n];
        // listele sunt pentru o jumătate; se oglindesc prin rotație de 180°
        for &(x, y) in walls {
            wall[(y * w + x) as usize] = true;
            wall[((h - 1 - y) * w + (w - 1 - x)) as usize] = true;
        }
        for &(x, y) in covers {
            cover[(y * w + x) as usize] = true;
            cover[((h - 1 - y) * w + (w - 1 - x)) as usize] = true;
        }
        for i in 0..n {
            if wall[i] {
                cover[i] = false;
            }
        }
        let mut los = vec![false; n * n];
        for a in 0..n {
            for b in 0..n {
                if wall[a] || wall[b] {
                    continue;
                }
                let ca = (a as i32 % w, a as i32 / w);
                let cb = (b as i32 % w, b as i32 / w);
                los[a * n + b] = line_between(ca, cb)
                    .iter()
                    .all(|&c| !wall[(c.1 * w + c.0) as usize]);
            }
        }
        GridMap {
            w,
            h,
            wall,
            cover,
            los,
        }
    }
    pub fn idx(&self, c: (i32, i32)) -> usize {
        (c.1 * self.w + c.0) as usize
    }
    pub fn inside(&self, c: (i32, i32)) -> bool {
        c.0 >= 0 && c.1 >= 0 && c.0 < self.w && c.1 < self.h
    }
    pub fn see(&self, a: (i32, i32), b: (i32, i32)) -> bool {
        self.los[self.idx(a) * (self.w * self.h) as usize + self.idx(b)]
    }
    pub fn is_wall(&self, c: (i32, i32)) -> bool {
        self.wall[self.idx(c)]
    }
    pub fn is_cover(&self, c: (i32, i32)) -> bool {
        self.cover[self.idx(c)]
    }
}

/// BFS pe 8 vecini pe o grilă; `blocked` interzice un pătrat, `stop` permite
/// intrarea dar nu mai expandează din el. Întoarce (pătrat, distanță) fără start.
pub fn bfs8(
    map: &GridMap,
    start: (i32, i32),
    maxd: i32,
    blocked: &dyn Fn((i32, i32)) -> bool,
    stop: &dyn Fn((i32, i32)) -> bool,
) -> Vec<((i32, i32), i32)> {
    let mut dist = vec![-1i32; (map.w * map.h) as usize];
    let mut q = std::collections::VecDeque::new();
    dist[map.idx(start)] = 0;
    q.push_back(start);
    let mut out = Vec::new();
    while let Some(c) = q.pop_front() {
        let d = dist[map.idx(c)];
        if d >= maxd || (c != start && stop(c)) {
            continue;
        }
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let n = (c.0 + dx, c.1 + dy);
                if !map.inside(n) || map.is_wall(n) || blocked(n) || dist[map.idx(n)] >= 0 {
                    continue;
                }
                dist[map.idx(n)] = d + 1;
                out.push((n, d + 1));
                q.push_back(n);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_roundtrip() {
        assert_eq!(cell_name(3, 3), "d4");
        assert_eq!(parse_cell("d4", 10, 10), Some((3, 3)));
        assert_eq!(parse_cell("k4", 10, 10), None);
        assert_eq!(parse_cell("a0", 10, 10), None);
        assert_eq!(parse_cell("", 10, 10), None);
    }

    #[test]
    fn line_is_symmetric_and_bounded() {
        for ax in 0..6 {
            for ay in 0..6 {
                for bx in 0..6 {
                    for by in 0..6 {
                        let l1 = line_between((ax, ay), (bx, by));
                        let l2 = line_between((bx, by), (ax, ay));
                        assert_eq!(l1, l2);
                        assert_eq!(l1.len() as i32, (cheb((ax, ay), (bx, by)) - 1).max(0));
                        for c in l1 {
                            assert!(c.0 >= 0 && c.0 < 6 && c.1 >= 0 && c.1 < 6);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn clone_scrambles_dice() {
        let a = Dice::new(5);
        let mut b = a.clone();
        let mut c = a.clone();
        let sb: Vec<u8> = (0..20).map(|_| b.d6()).collect();
        let sc: Vec<u8> = (0..20).map(|_| c.d6()).collect();
        assert_ne!(sb, sc);
    }
}
