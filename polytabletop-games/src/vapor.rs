//! Vaporul de pe Râu — cursă de vapoare cu viteză și cărbune (variantă originală).
//!
//! Râul are 60 de câmpuri și 3 benzi (0–2). Fiecare vapor pornește cu viteza 1 și 6 unități de
//! cărbune. Pe tură alegi schimbarea vitezei (−2..+2; ±1 e gratuită, ±2 costă 1 cărbune; viteza
//! rămâne între 1 și 4) și banda (cel mult o bandă mai la stânga/dreapta): `v+1,b2`. Apoi vaporul
//! înaintează atâtea câmpuri cât viteza. Banc de nisip pe drum: te oprești înaintea lui, pierzi 1
//! cărbune și viteza devine 1. Nu poți trece prin celălalt vapor (te oprești în spatele lui).
//! Pasagerii de pe câmpuri se urcă când treci sau aterizezi (+1 punct). Cine trece primul de
//! câmpul 59 primește +4, al doilea +2. Se termină când ambii au terminat sau după 80 de mutări.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const LEN: usize = 60;

#[derive(Clone)]
pub struct Vapor {
    bar: [[bool; 3]; LEN],
    pas: [[bool; 3]; LEN],
    pos: [usize; 2],
    lane: [usize; 2],
    speed: [i32; 2],
    coal: [i32; 2],
    pts: [i32; 2],
    fin: [bool; 2],
    finished: u32,
    side: Player,
    ply: u32,
}

impl Vapor {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bar = [[false; 3]; LEN];
        let mut pas = [[false; 3]; LEN];
        for i in 3..LEN - 1 {
            if rng.below(100) < 18 {
                bar[i][rng.below(3)] = true;
            }
            if rng.below(100) < 14 {
                let l = rng.below(3);
                if !bar[i][l] {
                    pas[i][l] = true;
                }
            }
        }
        Vapor {
            bar,
            pas,
            pos: [0, 0],
            lane: [0, 2],
            speed: [1, 1],
            coal: [6, 6],
            pts: [0; 2],
            fin: [false; 2],
            finished: 0,
            side: Player::One,
            ply: 0,
        }
    }
    fn who(&self) -> usize {
        pi(self.side)
    }
    fn sim(&self, me: usize, d: i32, lane: usize) -> (usize, i32, i32, i32) {
        // întoarce (poziție, viteză, cărbune, pasageri)
        let sp = (self.speed[me] + d).clamp(1, 4);
        let mut coal = self.coal[me] - if d.abs() == 2 { 1 } else { 0 };
        let mut pos = self.pos[me];
        let mut got = 0;
        let mut speed = sp;
        for _ in 0..sp {
            let np = pos + 1;
            if np >= LEN {
                pos = LEN - 1 + 1;
                break;
            }
            if self.bar[np][lane] {
                coal -= 1;
                speed = 1;
                break;
            }
            if !self.fin[1 - me] && self.pos[1 - me] == np && self.lane[1 - me] == lane {
                break;
            }
            pos = np;
            if self.pas[np][lane] {
                got += 1;
            }
        }
        (pos, speed, coal, got)
    }
}

impl Default for Vapor {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Vapor {
    fn depth(&self) -> u32 {
        3
    }
    fn side(&self) -> Player {
        if self.fin[self.who()] && !self.fin[1 - self.who()] {
            self.side.other()
        } else {
            self.side
        }
    }
    fn moves(&self) -> Vec<String> {
        let me = if self.fin[self.who()] {
            1 - self.who()
        } else {
            self.who()
        };
        let mut v = Vec::new();
        for d in -2i32..=2 {
            let ns = self.speed[me] + d;
            if !(1..=4).contains(&ns) || (d.abs() == 2 && self.coal[me] < 1) {
                continue;
            }
            for l in self.lane[me].saturating_sub(1)..=(self.lane[me] + 1).min(2) {
                v.push(format!("v{:+},b{}", d, l));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = if self.fin[self.who()] {
            1 - self.who()
        } else {
            self.who()
        };
        let (a, b) = m[1..].split_once(",b").unwrap();
        let d: i32 = a.parse().unwrap();
        let lane: usize = b.parse().unwrap();
        let (pos, speed, coal, got) = self.sim(me, d, lane);
        self.lane[me] = lane;
        self.speed[me] = speed;
        self.coal[me] = coal;
        self.pts[me] += got;
        if pos >= LEN {
            self.pos[me] = LEN - 1;
            self.fin[me] = true;
            self.pts[me] += if self.finished == 0 { 4 } else { 2 };
            self.finished += 1;
        } else {
            self.pos[me] = pos;
        }
        for i in self.pos[me].saturating_sub(5)..=self.pos[me] {
            let _ = i;
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if (self.fin[0] && self.fin[1]) || self.ply >= 80 {
            let a = self.pts[0] + self.pos[0] as i32 / 10;
            let b = self.pts[1] + self.pos[1] as i32 / 10;
            return Some(by_score(a, b));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f =
            |i: usize| 8 * self.pts[i] + 2 * self.pos[i] as i32 + 3 * self.coal[i] + self.speed[i];
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from("Vaporul de pe Râu — # banc de nisip, p pasager, 1/2 vapoare (câmp 0..59, benzi 0..2)\n");
        for l in 0..3 {
            s.push_str(&format!("b{} ", l));
            for i in 0..LEN {
                let mut ch = if self.bar[i][l] {
                    '#'
                } else if self.pas[i][l] {
                    'p'
                } else {
                    '.'
                };
                for p in 0..2 {
                    if self.pos[p] == i && self.lane[p] == l {
                        ch = if p == 0 { '1' } else { '2' };
                    }
                }
                s.push(ch);
            }
            s.push('\n');
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: câmp {} viteza {} cărbune {} puncte {}{}\n",
                p + 1,
                self.pos[p],
                self.speed[p],
                self.coal[p],
                self.pts[p],
                if self.fin[p] { " (a terminat)" } else { "" }
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (v±d,bL: schimbarea vitezei și banda)\n",
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
        crate::brand_util::check(Vapor::with_seed, 300);
    }
}
