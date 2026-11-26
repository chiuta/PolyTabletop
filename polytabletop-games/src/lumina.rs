//! Lumina Pădurii — copaci care își fac umbră (variantă originală pe hexagoane, raza 3).
//!
//! Soarele se rotește: în fiecare dintre cele 12 zile lumina vine dintr-o altă direcție. Copacii
//! au mărimea 0 (sămânță), 1, 2, 3. Umbra unui copac de mărime t cade pe t celule în direcția
//! opusă soarelui și acoperă copacii de mărime ≤ t. La finalul zilei fiecare copac neacoperit
//! îți dă lumină = mărimea lui. Pe parcursul zilei cheltui lumină: `planta q,r` (1; o sămânță la
//! cel mult mărime celule de un copac al tău), `creste q,r` (mărimea nouă; 0→1 costă 1),
//! `culeg q,r` (4; copacul de mărime 3 dispare și dă 4/3/2/1 puncte după inelul celulei).
//! Începutul: fiecare pune 2 copaci de mărime 1 pe inelul exterior (`pune q,r`). `pas` = ai
//! terminat ziua; când amândoi ați pasat, se strânge lumina. Maxim 8 semințe/copaci pe jucător.
//! Câștigă cine are mai multe puncte (lumina rămasă: 1 punct la 3).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

const R: i32 = 3;
const DAYS: u32 = 12;

#[derive(Clone)]
pub struct Lumina {
    tree: [[i8; 7]; 7],  // -1 gol, 0..3
    owner: [[u8; 7]; 7], // 0/1
    light: [i32; 2],
    pts: [i32; 2],
    placed: [u32; 2],
    init: [u8; 2],
    passed: [bool; 2],
    day: u32,
    side: Player,
}

fn inb(q: i32, r: i32) -> bool {
    q.abs() <= R && r.abs() <= R && (q + r).abs() <= R
}
fn ring(q: i32, r: i32) -> i32 {
    q.abs().max(r.abs()).max((q + r).abs())
}

impl Lumina {
    pub fn new() -> Self {
        Self::with_seed(0)
    }
    pub fn with_seed(_seed: u64) -> Self {
        Lumina {
            tree: [[-1; 7]; 7],
            owner: [[0; 7]; 7],
            light: [2, 2],
            pts: [0; 2],
            placed: [0; 2],
            init: [0; 2],
            passed: [false; 2],
            day: 0,
            side: Player::One,
        }
    }
    fn t(&self, q: i32, r: i32) -> i8 {
        if inb(q, r) {
            self.tree[(q + R) as usize][(r + R) as usize]
        } else {
            -2
        }
    }
    fn cells() -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for q in -R..=R {
            for r in -R..=R {
                if inb(q, r) {
                    v.push((q, r));
                }
            }
        }
        v
    }
    fn shadowed(&self, q: i32, r: i32) -> bool {
        let sun = (self.day % 6) as usize;
        let me = self.t(q, r) as i32;
        // umbra vine din direcția soarelui: căutăm în direcția soarelui copaci înalți
        for dist in 1..=3 {
            let (cq, cr) = (q + HEX[sun].0 * dist, r + HEX[sun].1 * dist);
            let o = self.t(cq, cr) as i32;
            if o >= dist && o >= me {
                return true;
            }
        }
        false
    }
    fn income(&self, p: usize) -> i32 {
        let mut s = 0;
        for (q, r) in Self::cells() {
            if self.t(q, r) > 0
                && self.owner[(q + R) as usize][(r + R) as usize] as usize == p
                && !self.shadowed(q, r)
            {
                s += self.t(q, r) as i32;
            }
        }
        s
    }
    fn trees(&self, p: usize) -> i32 {
        Self::cells()
            .iter()
            .filter(|&&(q, r)| {
                self.t(q, r) >= 0 && self.owner[(q + R) as usize][(r + R) as usize] as usize == p
            })
            .count() as i32
    }
    fn total(&self, p: usize) -> i32 {
        self.pts[p] + self.light[p] / 3
    }
    fn setup(&self) -> bool {
        self.init[0] < 2 || self.init[1] < 2
    }
}

impl Default for Lumina {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Lumina {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        if self.init[me] < 2 {
            for (q, r) in Self::cells() {
                if ring(q, r) == R && self.t(q, r) == -1 {
                    v.push(format!("pune {},{}", q, r));
                }
            }
            return v;
        }
        let l = self.light[me];
        for (q, r) in Self::cells() {
            let t = self.t(q, r);
            let mine = t >= 0 && self.owner[(q + R) as usize][(r + R) as usize] as usize == me;
            if t == -1 && l >= 1 && self.placed[me] < 8 {
                let near = Self::cells().iter().any(|&(a, b)| {
                    let tt = self.t(a, b);
                    tt >= 1
                        && self.owner[(a + R) as usize][(b + R) as usize] as usize == me
                        && ring(a - q, b - r).max(((a - q) + (b - r)).abs()) <= tt as i32
                });
                if near {
                    v.push(format!("planta {},{}", q, r));
                }
            }
            if mine && t < 3 && l > t as i32 {
                v.push(format!("creste {},{}", q, r));
            }
            if mine && t == 3 && l >= 4 {
                v.push(format!("culeg {},{}", q, r));
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if m == "pas" {
            self.passed[me] = true;
        } else {
            let (cmd, xy) = m.split_once(' ').unwrap();
            let (q, r) = xy.split_once(',').unwrap();
            let (q, r): (i32, i32) = (q.parse().unwrap(), r.parse().unwrap());
            let (i, j) = ((q + R) as usize, (r + R) as usize);
            match cmd {
                "pune" => {
                    self.tree[i][j] = 1;
                    self.owner[i][j] = me as u8;
                    self.init[me] += 1;
                    self.placed[me] += 1;
                }
                "planta" => {
                    self.tree[i][j] = 0;
                    self.owner[i][j] = me as u8;
                    self.light[me] -= 1;
                    self.placed[me] += 1;
                }
                "creste" => {
                    self.light[me] -= self.tree[i][j] as i32 + 1;
                    self.tree[i][j] += 1;
                }
                _ => {
                    self.light[me] -= 4;
                    self.pts[me] += 4 - ring(q, r);
                    self.tree[i][j] = -1;
                }
            }
        }
        if !self.setup() && self.passed[0] && self.passed[1] {
            for p in 0..2 {
                self.light[p] += self.income(p);
            }
            self.passed = [false; 2];
            self.day += 1;
        }
        self.side = self.side.other();
        if !self.setup() && self.passed[pi(self.side)] && !self.passed[1 - pi(self.side)] {
            self.side = self.side.other();
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.day >= DAYS {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f =
            |i: usize| 12 * self.pts[i] + 3 * self.light[i] + 2 * self.income(i) + self.trees(i);
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let sun = (self.day % 6) as usize;
        let mut s = format!(
            "Lumina Pădurii — ziua {}/{} | soarele din direcția {} (HEX {:?})\n",
            (self.day + 1).min(DAYS),
            DAYS,
            sun,
            HEX[sun]
        );
        for r in -R..=R {
            let qmin = (-R).max(-R - r);
            let qmax = R.min(R - r);
            s.push_str(&format!("r={:>2} q={:>2}.. ", r, qmin));
            s.push_str(&" ".repeat(r.unsigned_abs() as usize));
            for q in qmin..=qmax {
                let t = self.t(q, r);
                if t == -1 {
                    s.push_str(". ");
                } else {
                    let o = if self.owner[(q + R) as usize][(r + R) as usize] == 0 {
                        'a'
                    } else {
                        'b'
                    };
                    s.push(o);
                    s.push((b'0' + t as u8) as char);
                }
            }
            s.push('\n');
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{} ({}): lumină {} | puncte {} | venit zi {}\n",
                p + 1,
                if p == 0 { 'a' } else { 'b' },
                self.light[p],
                self.pts[p],
                self.income(p)
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (pune/planta/creste/culeg q,r | pas)\n",
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
        crate::brand_util::check(Lumina::with_seed, 1500);
    }
    #[test]
    fn shadow_blocks_smaller() {
        let mut g = Lumina::new();
        g.init = [2, 2];
        // soarele din direcția 0 (est): copacul de la (1,0) de mărime 2 umbrește (0,0) de mărime 1
        g.tree[(1 + R) as usize][R as usize] = 2;
        g.tree[R as usize][R as usize] = 1;
        assert!(g.shadowed(0, 0));
        assert!(!g.shadowed(1, 0));
    }
}
