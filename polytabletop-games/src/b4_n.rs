#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::unnecessary_map_or,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
//! Motor „rețea de rute” (lista B4): muchii cu cost, orașe cu cerere. Moduri:
//! `a` = rute comune (poți livra și pe muchiile rivalului contra unei taxe de 1),
//! `b` = doar rutele proprii, fiecare livrare crește venitul permanent,
//! `n` = doar rutele proprii, prima livrare într-un oraș dă 2 monede,
//! `h` = cuburi pe muchii (majoritate pe muchia completă primește punctele muchiei).
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

pub struct NSpec {
    pub title: &'static str,
    /// "a-b:cost,..."
    pub edges: &'static str,
    pub vals: &'static [i32],
    pub hubs: [usize; 2],
    pub mode: char,
    pub turns: u32,
    pub depth: u32,
}

#[derive(Clone)]
pub struct N {
    sp: &'static NSpec,
    e: Vec<(usize, usize, i32)>,
    own: Vec<i8>,
    cubes: Vec<[u8; 2]>,
    dem: Vec<i32>,
    first: Vec<i8>,
    coins: [i32; 2],
    inc: [i32; 2],
    vp: [i32; 2],
    turns: [u32; 2],
    side: Player,
}

impl N {
    pub fn new(sp: &'static NSpec) -> Self {
        let e: Vec<(usize, usize, i32)> = sp
            .edges
            .split(',')
            .map(|s| {
                let (ab, c) = s.split_once(':').unwrap();
                let (a, b) = ab.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())
            })
            .collect();
        N {
            sp,
            own: vec![-1; e.len()],
            cubes: vec![[0; 2]; e.len()],
            e,
            dem: sp.vals.to_vec(),
            first: vec![-1; sp.vals.len()],
            coins: [6, 7],
            inc: [3, 3],
            vp: [0; 2],
            turns: [0; 2],
            side: Player::One,
        }
    }
    pub fn with_seed(sp: &'static NSpec, _seed: u64) -> Self {
        Self::new(sp)
    }
    fn usable(&self, p: usize, i: usize) -> bool {
        self.own[i] == p as i8 || (self.sp.mode == 'a' && self.own[i] >= 0)
    }
    /// cel mai ieftin cost de taxe până la `t` (BFS pe număr de muchii străine, Dijkstra mic)
    fn fee_to(&self, p: usize, t: usize) -> Option<i32> {
        let n = self.dem.len();
        let mut d = vec![i32::MAX; n];
        d[self.sp.hubs[p]] = 0;
        let mut done = vec![false; n];
        loop {
            let mut x = None;
            for i in 0..n {
                if !done[i] && d[i] < i32::MAX && x.map_or(true, |y: usize| d[i] < d[y]) {
                    x = Some(i);
                }
            }
            let Some(x) = x else { break };
            done[x] = true;
            for (k, &(a, b, _)) in self.e.iter().enumerate() {
                if !self.usable(p, k) {
                    continue;
                }
                let y = if a == x {
                    b
                } else if b == x {
                    a
                } else {
                    continue;
                };
                let w = if self.own[k] == p as i8 { 0 } else { 1 };
                if d[x] + w < d[y] {
                    d[y] = d[x] + w;
                }
            }
        }
        (d[t] < i32::MAX).then_some(d[t])
    }
    fn score(&self, p: usize) -> i32 {
        self.vp[p] + self.coins[p] / 4
    }
    fn complete(&self, k: usize) -> bool {
        self.cubes[k][0] + self.cubes[k][1] >= 3
    }
}

impl Mini for N {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = vec!["inc".to_string()];
        if self.sp.mode == 'h' {
            for k in 0..self.e.len() {
                if !self.complete(k) && self.coins[me] >= 1 {
                    v.push(format!("cub {k}"));
                }
            }
            return v;
        }
        for k in 0..self.e.len() {
            if self.own[k] < 0 && self.coins[me] >= self.e[k].2 {
                v.push(format!("tra {k}"));
            }
        }
        for t in 0..self.dem.len() {
            if t != self.sp.hubs[me] && self.dem[t] > 0 {
                if let Some(f) = self.fee_to(me, t) {
                    if f <= self.coins[me] {
                        v.push(format!("ent {t}"));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if m == "inc" {
            self.coins[me] += self.inc[me];
        } else if let Some(k) = m.strip_prefix("cub ") {
            let k: usize = k.parse().unwrap();
            self.coins[me] -= 1;
            self.cubes[k][me] += 1;
            if self.complete(k) {
                let (a, b) = (self.cubes[k][0], self.cubes[k][1]);
                if a != b {
                    let w = if a > b { 0 } else { 1 };
                    self.vp[w] += self.e[k].2 * 2;
                    self.own[k] = w as i8;
                }
            }
        } else if let Some(k) = m.strip_prefix("tra ") {
            let k: usize = k.parse().unwrap();
            self.coins[me] -= self.e[k].2;
            self.own[k] = me as i8;
            self.vp[me] += 1;
        } else if let Some(t) = m.strip_prefix("ent ") {
            let t: usize = t.parse().unwrap();
            let f = self.fee_to(me, t).unwrap();
            self.coins[me] -= f;
            if self.sp.mode == 'a' {
                // taxele merg proprietarilor (aproximare: la celălalt jucător)
                self.coins[1 - me] += f;
            }
            self.vp[me] += self.dem[t];
            self.dem[t] -= 1;
            if self.sp.mode == 'b' {
                self.inc[me] += 1;
            }
            if self.sp.mode == 'n' && self.first[t] < 0 {
                self.first[t] = me as i8;
                self.coins[me] += 2;
            }
        }
        self.turns[me] += 1;
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.turns[0] >= self.sp.turns && self.turns[1] >= self.sp.turns {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |q: usize| {
            let edges = self.own.iter().filter(|&&o| o == q as i8).count() as i32;
            let cub: i32 = self.cubes.iter().map(|c| c[q] as i32).sum();
            6 * self.vp[q] + self.coins[q] + 2 * self.inc[q] + edges + cub
        };
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{}\n",
            self.sp.title,
            self.turns[0].min(self.turns[1]) + 1,
            self.sp.turns
        );
        for (k, &(a, b, c)) in self.e.iter().enumerate() {
            s.push_str(&format!(
                "  muchia {k:>2}: {a}-{b} cost {c} proprietar {} cuburi {:?}\n",
                match self.own[k] {
                    0 => "J1",
                    1 => "J2",
                    _ => "—",
                },
                self.cubes[k]
            ));
        }
        s.push_str(&format!(
            "  orașe cerere {:?} | hub-uri {:?}\n",
            self.dem, self.sp.hubs
        ));
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: monede {} venit {} puncte {} scor {}\n",
                p + 1,
                self.coins[p],
                self.inc[p],
                self.vp[p],
                self.score(p)
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (inc | {})\n",
            pname(self.side),
            if self.sp.mode == 'h' {
                "cub k"
            } else {
                "tra k | ent n"
            }
        ));
        s
    }
}
