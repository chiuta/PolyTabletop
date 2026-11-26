//! Plapuma — cusut o plapumă cu petice, cu timp și nasturi (variantă originală, 2 jucători).
//!
//! Fiecare are o plapumă 7×7. Peticele (16, în cerc) costă nasturi și timp; din cele 3 de după
//! marcaj poți lua unul: `ia i@r,c,o` (i = 1..3, colțul de sus-stânga al peticului la rândul r,
//! coloana c, orientarea o) plătind nasturii și avansând pe pista de timp (40). Sau `avans`: te
//! pui cu un loc înaintea adversarului și primești atâția nasturi cât ai avansat. Mereu mută cine
//! e în urmă (la egalitate, cine a sosit ultimul). Pe câmpurile 8, 16, 24, 32 se primesc nasturii
//! de venit ai peticelor tale. Scor final: nasturi + 2 pe celulă acoperită − 2 pe celulă goală.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: usize = 7;
const TIME: i32 = 40;
// (formă, cost, timp, venit)
const PATCH: [(usize, i32, i32, i32); 16] = [
    (0, 2, 1, 0),
    (1, 2, 2, 0),
    (2, 1, 3, 0),
    (3, 3, 4, 1),
    (4, 3, 2, 0),
    (5, 4, 2, 1),
    (6, 3, 4, 1),
    (7, 3, 2, 1),
    (8, 5, 5, 2),
    (9, 5, 3, 1),
    (10, 4, 4, 1),
    (0, 1, 2, 0),
    (3, 2, 2, 0),
    (6, 5, 3, 1),
    (2, 3, 1, 1),
    (5, 2, 2, 0),
];

#[derive(Clone)]
pub struct Plapuma {
    grid: [[[bool; N]; N]; 2],
    order: Vec<usize>,
    marker: usize,
    pos: [i32; 2],
    arr: [u32; 2],
    clock: u32,
    buttons: [i32; 2],
    income: [i32; 2],
    side: Player,
    shapes: Vec<Vec<Vec<(i8, i8)>>>,
}

impl Plapuma {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..16).collect();
        shuf(&mut order, &mut rng);
        let shapes = (0..11).map(|i| orientations(&poly(i), true)).collect();
        Plapuma {
            grid: [[[false; N]; N]; 2],
            order,
            marker: 0,
            pos: [0; 2],
            arr: [0; 2],
            clock: 0,
            buttons: [5; 2],
            income: [0; 2],
            side: Player::One,
            shapes,
        }
    }
    fn mover(&self) -> usize {
        if self.pos[0] < self.pos[1] || (self.pos[0] == self.pos[1] && self.arr[0] > self.arr[1]) {
            0
        } else {
            1
        }
    }
    fn covered(&self, p: usize) -> i32 {
        self.grid[p].iter().flatten().filter(|&&x| x).count() as i32
    }
    fn final_score(&self, p: usize) -> i32 {
        self.buttons[p] + 2 * self.covered(p) - 2 * ((N * N) as i32 - self.covered(p))
    }
    fn advance_to(&mut self, p: usize, to: i32) {
        let to = to.min(TIME);
        for t in self.pos[p] + 1..=to {
            if t % 8 == 0 {
                self.buttons[p] += self.income[p];
            }
        }
        self.pos[p] = to;
        self.clock += 1;
        self.arr[p] = self.clock;
    }
}

impl Default for Plapuma {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Plapuma {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.mover() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let me = self.mover();
        let mut v = Vec::new();
        for i in 0..3.min(self.order.len()) {
            let pid = self.order[(self.marker + i) % self.order.len()];
            let (sh, cost, _, _) = PATCH[pid];
            if cost > self.buttons[me] {
                continue;
            }
            for (o, cells) in self.shapes[sh].iter().enumerate() {
                for r in 0..N as i8 {
                    for c in 0..N as i8 {
                        if cells.iter().all(|&(dr, dc)| {
                            let (a, b) = (r + dr, c + dc);
                            a < N as i8 && b < N as i8 && !self.grid[me][a as usize][b as usize]
                        }) {
                            v.push(format!("ia {}@{},{},{}", i + 1, r + 1, c + 1, o));
                        }
                    }
                }
            }
        }
        if self.pos[me] < TIME {
            v.push("avans".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.mover();
        if m == "avans" {
            let target = (self.pos[1 - me].max(self.pos[me]) + 1).min(TIME);
            let gain = target - self.pos[me];
            self.buttons[me] += gain;
            self.advance_to(me, target);
        } else {
            let (i, rest) = m[3..].split_once('@').unwrap();
            let i: usize = i.parse::<usize>().unwrap() - 1;
            let v: Vec<i32> = rest.split(',').map(|x| x.parse().unwrap()).collect();
            let idx = (self.marker + i) % self.order.len();
            let pid = self.order[idx];
            let (sh, cost, time, inc) = PATCH[pid];
            for &(dr, dc) in &self.shapes[sh][v[2] as usize] {
                self.grid[me][(v[0] - 1 + dr as i32) as usize][(v[1] - 1 + dc as i32) as usize] =
                    true;
            }
            self.buttons[me] -= cost;
            self.income[me] += inc;
            self.order.remove(idx);
            self.marker = if self.order.is_empty() {
                0
            } else {
                idx % self.order.len()
            };
            let np = self.pos[me] + time;
            self.advance_to(me, np);
        }
        self.side = if self.mover() == 0 {
            Player::One
        } else {
            Player::Two
        };
    }
    fn result(&self) -> Option<Outcome> {
        if self.pos[0] >= TIME && self.pos[1] >= TIME {
            return Some(by_score(self.final_score(0), self.final_score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| 3 * self.buttons[i] + 8 * self.covered(i) + 10 * self.income[i];
        f(a) - f(1 - a) + (self.pos[a] - self.pos[1 - a])
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Plapuma — pista de timp 0..{} | nasturi J1 {} (venit {}), J2 {} (venit {})\n",
            TIME, self.buttons[0], self.income[0], self.buttons[1], self.income[1]
        );
        s.push_str(&format!(
            "Poziții: J1 {} | J2 {}\n",
            self.pos[0], self.pos[1]
        ));
        for p in 0..2 {
            s.push_str(&format!("Plapuma J{}:\n", p + 1));
            for r in 0..N {
                s.push_str("  ");
                for c in 0..N {
                    s.push(if self.grid[p][r][c] { '#' } else { '.' });
                    s.push(' ');
                }
                s.push('\n');
            }
        }
        for i in 0..3.min(self.order.len()) {
            let (sh, cost, time, inc) = PATCH[self.order[(self.marker + i) % self.order.len()]];
            s.push_str(&format!(
                "Petic {}: formă {} ({} celule) cost {} timp {} venit {}\n",
                i + 1,
                sh,
                poly(sh).len(),
                cost,
                time,
                inc
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (ia i@r,c,o | avans)\n",
            pname(self.side())
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Plapuma::with_seed, 300);
    }
}
