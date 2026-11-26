//! Peisaj Liniștit — așezat plăcuțe hexagonale cu margini de teren (variantă originală, 2 jucători).
//!
//! Plăcuțele hexagonale au 6 margini de câte un teren (pădure, câmp, apă, sat). Pe rând așezi
//! plăcuța curentă (se vede și următoarea) lângă cele deja puse, rotită 0–5: `pune q,r,rot`.
//! Fiecare margine care atinge o plăcuță vecină: +1 dacă terenul se potrivește, −1 dacă nu.
//! Plăcuța de la început e pusă în (0,0). 36 de plăcuțe; câștigă cine are mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const R: i32 = 8;
const N: usize = 17;
const NAME: [char; 4] = ['p', 'c', 'a', 's'];

type Tile = [u8; 6];

fn rot(t: &Tile, k: usize) -> Tile {
    let mut o = [0u8; 6];
    for i in 0..6 {
        o[(i + k) % 6] = t[i];
    }
    o
}

#[derive(Clone)]
pub struct Peisaj {
    board: [[Option<Tile>; N]; N],
    cur: Tile,
    next: Option<Tile>,
    deck: Vec<Tile>,
    pts: [i32; 2],
    side: Player,
    placed: u32,
}

fn gen(rng: &mut Rng) -> Tile {
    let a = rng.below(4) as u8;
    let mut b = rng.below(4) as u8;
    if b == a {
        b = (a + 1) % 4;
    }
    let c = rng.below(4) as u8;
    let base: Tile = if rng.below(2) == 0 {
        [a, a, a, b, b, b]
    } else {
        [a, a, b, b, c, c]
    };
    rot(&base, rng.below(6))
}

impl Peisaj {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut board = [[None; N]; N];
        board[R as usize][R as usize] = Some(gen(&mut rng));
        let mut deck: Vec<Tile> = (0..36).map(|_| gen(&mut rng)).collect();
        let cur = deck.pop().unwrap();
        let next = deck.pop();
        Peisaj {
            board,
            cur,
            next,
            deck,
            pts: [0; 2],
            side: Player::One,
            placed: 0,
        }
    }
    fn at(&self, q: i32, r: i32) -> Option<Tile> {
        if q.abs() > R || r.abs() > R {
            None
        } else {
            self.board[(q + R) as usize][(r + R) as usize]
        }
    }
    fn gain(&self, q: i32, r: i32, t: &Tile) -> i32 {
        let mut g = 0;
        for d in 0..6 {
            let (nq, nr) = (q + HEX[d].0, r + HEX[d].1);
            if let Some(o) = self.at(nq, nr) {
                g += if o[(d + 3) % 6] == t[d] { 1 } else { -1 };
            }
        }
        g
    }
    fn frontier(&self) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for q in -R + 1..R {
            for r in -R + 1..R {
                if self.at(q, r).is_none()
                    && (0..6).any(|d| self.at(q + HEX[d].0, r + HEX[d].1).is_some())
                {
                    v.push((q, r));
                }
            }
        }
        v
    }
}

impl Default for Peisaj {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Peisaj {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let mut v = Vec::new();
        for (q, r) in self.frontier() {
            for k in 0..6 {
                v.push(format!("pune {},{},{}", q, r, k));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let x: Vec<i32> = m[5..].split(',').map(|s| s.parse().unwrap()).collect();
        let t = rot(&self.cur, x[2] as usize);
        self.pts[me] += self.gain(x[0], x[1], &t);
        self.board[(x[0] + R) as usize][(x[1] + R) as usize] = Some(t);
        self.placed += 1;
        if let Some(n) = self.next {
            self.cur = n;
            self.next = self.deck.pop();
        } else {
            self.placed = 99;
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.placed >= 36 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.pts[a] - self.pts[1 - a]
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Peisaj Liniștit — plăcuțe {}/36 | puncte J1 {}, J2 {}\n",
            self.placed, self.pts[0], self.pts[1]
        );
        let tile = |t: &Tile| t.iter().map(|&x| NAME[x as usize]).collect::<String>();
        for r in -R..=R {
            let qmin = (-R).max(-R - r);
            let qmax = R.min(R - r);
            let row: Vec<String> = (qmin..=qmax)
                .filter_map(|q| self.at(q, r).map(|t| format!("({},{}){}", q, r, tile(&t))))
                .collect();
            if !row.is_empty() {
                s.push_str(&format!("r={:>2}: {}\n", r, row.join(" ")));
            }
        }
        s.push_str(&format!(
            "Plăcuța curentă (margini 0..5: E NE NV V SV SE): {}",
            tile(&self.cur)
        ));
        if let Some(n) = self.next {
            s.push_str(&format!(" | următoarea: {}", tile(&n)));
        }
        s.push_str(&format!(
            "\nLa mutare: {} (pune q,r,rot) — p pădure, c câmp, a apă, s sat\n",
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
        crate::brand_util::check(Peisaj::with_seed, 100);
    }
    #[test]
    fn matching_edges_score() {
        let mut g = Peisaj::with_seed(2);
        g.board = [[None; N]; N];
        g.board[R as usize][R as usize] = Some([1; 6]);
        let all1 = [1u8; 6];
        assert_eq!(g.gain(1, 0, &all1), 1);
        let all2 = [2u8; 6];
        assert_eq!(g.gain(1, 0, &all2), -1);
    }
}
