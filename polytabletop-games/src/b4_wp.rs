#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::unnecessary_map_or,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
//! Motor „plasare de lucrători” (lista B4): spații cu cost/câștig, doi jucători, runde fixe.
//! Informație perfectă, fără hazard după pregătire: AI alpha-beta cu adâncime fixă.
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

pub struct Sp {
    pub code: &'static str,
    pub cost: &'static str,
    pub gain: &'static str,
    pub vp: i32,
    /// x = exclusiv, a = se acumulează, g = +1 lucrător permanent, r = puncte crescătoare cu folosirile tale
    pub fl: &'static str,
}

pub struct WpSpec {
    pub title: &'static str,
    pub res: &'static str,
    pub spaces: &'static [Sp],
    pub workers: u8,
    pub rounds: u32,
    pub income: &'static str,
    pub start: &'static str,
    pub setb: i32,
    pub conv: [i32; 5],
    pub depth: u32,
}

fn parse(s: &str, res: &str) -> [i32; 5] {
    let mut o = [0; 5];
    let (mut n, mut have) = (0, false);
    for ch in s.chars() {
        if let Some(d) = ch.to_digit(10) {
            n = n * 10 + d as i32;
            have = true;
        } else if let Some(k) = res.find(ch) {
            o[k] += if have { n } else { 1 };
            n = 0;
            have = false;
        }
    }
    o
}

#[derive(Clone)]
pub struct Wp {
    sp: &'static WpSpec,
    res: [[i32; 5]; 2],
    vp: [i32; 2],
    occ: Vec<[u8; 2]>,
    acc: Vec<i32>,
    uses: [Vec<u8>; 2],
    wk: [u8; 2],
    left: [u8; 2],
    round: u32,
    first: usize,
    side: Player,
}

impl Wp {
    pub fn new(sp: &'static WpSpec) -> Self {
        let n = sp.spaces.len();
        let st = parse(sp.start, sp.res);
        let mut w = Wp {
            sp,
            res: [st; 2],
            vp: [0; 2],
            occ: vec![[0; 2]; n],
            acc: vec![0; n],
            uses: [vec![0; n], vec![0; n]],
            wk: [sp.workers; 2],
            left: [sp.workers; 2],
            round: 0,
            first: 0,
            side: Player::One,
        };
        // al doilea jucător primește o monedă de compensare (prima resursă)
        w.res[1][0] += 1;
        w
    }
    pub fn with_seed(sp: &'static WpSpec, _seed: u64) -> Self {
        Self::new(sp)
    }
    fn score(&self, p: usize) -> i32 {
        let r = &self.res[p];
        let mut s = self.vp[p];
        for k in 0..5 {
            s += self.sp.conv[k] * r[k];
        }
        s + self.sp.setb * r[..4].iter().copied().min().unwrap()
    }
    fn gain_of(&self, i: usize) -> [i32; 5] {
        let mut g = parse(self.sp.spaces[i].gain, self.sp.res);
        if self.sp.spaces[i].fl.contains('a') {
            let k = self
                .sp
                .res
                .find(
                    self.sp.spaces[i]
                        .gain
                        .chars()
                        .find(|c| c.is_alphabetic())
                        .unwrap_or('?'),
                )
                .unwrap_or(0);
            g[k] += self.acc[i];
        }
        g
    }
    fn can(&self, p: usize, i: usize) -> bool {
        let s = &self.sp.spaces[i];
        if s.fl.contains('x') && self.occ[i][0] + self.occ[i][1] > 0 {
            return false;
        }
        let c = parse(s.cost, self.sp.res);
        (0..5).all(|k| self.res[p][k] >= c[k])
    }
}

impl Mini for Wp {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for i in 0..self.sp.spaces.len() {
            if self.can(me, i) {
                v.push(format!("pun {}", self.sp.spaces[i].code));
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if m == "pas" {
            self.left[me] = 0;
        } else {
            let code = &m[4..];
            let i = self.sp.spaces.iter().position(|s| s.code == code).unwrap();
            let s = &self.sp.spaces[i];
            let c = parse(s.cost, self.sp.res);
            let g = self.gain_of(i);
            for k in 0..5 {
                self.res[me][k] += g[k] - c[k];
            }
            let mult = if s.fl.contains('r') {
                1 + self.uses[me][i] as i32
            } else {
                1
            };
            self.vp[me] += s.vp * mult;
            self.uses[me][i] = self.uses[me][i].saturating_add(1);
            self.occ[i][me] += 1;
            self.acc[i] = 0;
            if s.fl.contains('g') {
                self.wk[me] = (self.wk[me] + 1).min(5);
            }
            self.left[me] -= 1;
        }
        let o = 1 - me;
        if self.left[o] > 0 {
            self.side = self.side.other();
        } else if self.left[me] > 0 {
            // rămâne același jucător
        } else {
            // sfârșit de rundă
            for p in 0..2 {
                let inc = parse(self.sp.income, self.sp.res);
                for k in 0..5 {
                    self.res[p][k] += inc[k];
                }
            }
            for i in 0..self.sp.spaces.len() {
                if self.sp.spaces[i].fl.contains('a') && self.occ[i][0] + self.occ[i][1] == 0 {
                    self.acc[i] = (self.acc[i] + 1).min(6);
                }
                self.occ[i] = [0; 2];
            }
            self.round += 1;
            self.first = 1 - self.first;
            self.left = self.wk;
            self.side = if self.first == 0 {
                Player::One
            } else {
                Player::Two
            };
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.sp.rounds {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        4 * (self.score(a) - self.score(1 - a)) + 3 * (self.wk[a] as i32 - self.wk[1 - a] as i32)
    }
    fn show(&self) -> String {
        let sp = self.sp;
        let mut s = format!(
            "{} — runda {}/{} (resurse: {})\n",
            sp.title,
            (self.round + 1).min(sp.rounds),
            sp.rounds,
            sp.res
        );
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: puncte {} | resurse {:?} | lucrători {}/{} | scor {}\n",
                p + 1,
                self.vp[p],
                self.res[p],
                self.left[p],
                self.wk[p],
                self.score(p)
            ));
        }
        for (i, x) in sp.spaces.iter().enumerate() {
            s.push_str(&format!(
                "  {:<10} cost {:<6} câștig {:<6} p{} {} [{}/{}]{}\n",
                x.code,
                x.cost,
                x.gain,
                x.vp,
                x.fl,
                self.occ[i][0],
                self.occ[i][1],
                if self.acc[i] > 0 {
                    format!(" +{}", self.acc[i])
                } else {
                    String::new()
                }
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (pun <cod> | pas)\n",
            pname(self.side)
        ));
        s
    }
}
