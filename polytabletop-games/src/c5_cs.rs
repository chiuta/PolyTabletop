//! Motor „cățărare pe coloane” (lista C5, 2 jucători): coloanele 2..12 au lungimi proprii;
//! aruncarea forțată `arunc` dă `nd` zaruri, tu alegi o împerechere (`cuplu k`) și avansezi
//! alpiniștii temporari (cel mult `runners` coloane pe tură). `continu` aruncă din nou, `opresc` salvează
//! progresul; dacă nicio împerechere nu e jucabilă, `bust` pierde tot progresul turei.
//! Câștigă cine închide primul `win` coloane. Zarul e re-însămânțat la `clone()`.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

pub struct CsSpec {
    pub title: &'static str,
    pub lens: [i32; 11],
    pub runners: usize,
    pub win: usize,
    pub cap: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Roll,
    Pair,
    Decide,
}

#[derive(Clone)]
pub struct Cs {
    sp: &'static CsSpec,
    perm: [[i32; 11]; 2],
    temp: [i32; 11],
    run: Vec<usize>,
    claimed: [Option<usize>; 11],
    d: [u8; 4],
    ph: Ph,
    side: usize,
    dice: Dice,
    plies: u32,
}

fn pairings(d: &[u8; 4]) -> [(usize, usize); 3] {
    [
        ((d[0] + d[1]) as usize, (d[2] + d[3]) as usize),
        ((d[0] + d[2]) as usize, (d[1] + d[3]) as usize),
        ((d[0] + d[3]) as usize, (d[1] + d[2]) as usize),
    ]
}

impl Cs {
    pub fn new(sp: &'static CsSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static CsSpec, seed: u64) -> Self {
        Cs {
            sp,
            perm: [[0; 11]; 2],
            temp: [0; 11],
            run: Vec::new(),
            claimed: [None; 11],
            d: [1; 4],
            ph: Ph::Roll,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn valid(&self, s: usize, run: &[usize], temp: &[i32; 11]) -> bool {
        if !(2..=12).contains(&s) {
            return false;
        }
        let c = s - 2;
        if self.claimed[c].is_some() || self.perm[self.side][c] + temp[c] >= self.sp.lens[c] {
            return false;
        }
        run.contains(&c) || run.len() < self.sp.runners
    }
    fn apply(&self, s: usize, run: &mut Vec<usize>, temp: &mut [i32; 11]) {
        let c = s - 2;
        if !run.contains(&c) {
            run.push(c);
        }
        temp[c] += 1;
    }
    /// Variantele de mutare pentru zaruri date: (nume, sume aplicate).
    fn options(&self, d: &[u8; 4]) -> Vec<(String, Vec<usize>)> {
        let mut out: Vec<(String, Vec<usize>)> = Vec::new();
        for (k, &(x, y)) in pairings(d).iter().enumerate() {
            let (vx, vy) = (
                self.valid(x, &self.run, &self.temp),
                self.valid(y, &self.run, &self.temp),
            );
            let mut cand: Vec<(String, Vec<usize>)> = Vec::new();
            if vx && vy {
                let mut run = self.run.clone();
                let mut temp = self.temp;
                self.apply(x, &mut run, &mut temp);
                if self.valid(y, &run, &temp) {
                    cand.push((format!("cuplu {}", k + 1), vec![x, y]));
                } else {
                    cand.push((format!("cuplu {} {}", k + 1, x), vec![x]));
                    cand.push((format!("cuplu {} {}", k + 1, y), vec![y]));
                }
            } else if vx {
                cand.push((format!("cuplu {} {}", k + 1, x), vec![x]));
            } else if vy {
                cand.push((format!("cuplu {} {}", k + 1, y), vec![y]));
            }
            for c in cand {
                if !out.iter().any(|o| o.0 == c.0) {
                    out.push(c);
                }
            }
        }
        out
    }
    fn pbust(&self) -> i32 {
        let mut bust = 0;
        for i in 0..1296 {
            let d = [
                (i % 6) as u8 + 1,
                (i / 6 % 6) as u8 + 1,
                (i / 36 % 6) as u8 + 1,
                (i / 216) as u8 + 1,
            ];
            let ok = pairings(&d).iter().any(|&(x, y)| {
                self.valid(x, &self.run, &self.temp) || self.valid(y, &self.run, &self.temp)
            });
            if !ok {
                bust += 1;
            }
        }
        bust * 100 / 1296
    }
    fn perm_val(&self, s: usize) -> i32 {
        let mut v = 0;
        for c in 0..11 {
            if self.claimed[c] == Some(s) {
                v += 300;
            } else if self.claimed[c].is_none() {
                v += 100 * self.perm[s][c] / self.sp.lens[c];
            }
        }
        v
    }
    fn temp_val(&self) -> i32 {
        (0..11).map(|c| 100 * self.temp[c] / self.sp.lens[c]).sum()
    }
    fn nclaim(&self, s: usize) -> usize {
        self.claimed.iter().filter(|c| **c == Some(s)).count()
    }
    fn end_turn(&mut self, keep: bool) {
        if keep {
            for c in 0..11 {
                if self.temp[c] > 0 {
                    self.perm[self.side][c] += self.temp[c];
                    if self.perm[self.side][c] >= self.sp.lens[c] && self.claimed[c].is_none() {
                        self.claimed[c] = Some(self.side);
                    }
                }
            }
        }
        self.temp = [0; 11];
        self.run.clear();
        self.side = 1 - self.side;
        self.ph = Ph::Roll;
    }
}

impl Mini for Cs {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["arunc".into()],
            Ph::Pair => {
                let o = self.options(&self.d);
                if o.is_empty() {
                    vec!["bust".into()]
                } else {
                    o.into_iter().map(|x| x.0).collect()
                }
            }
            Ph::Decide => vec!["continu".into(), "opresc".into()],
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        match self.ph {
            Ph::Roll => {
                for i in 0..4 {
                    self.d[i] = self.dice.below(6) as u8 + 1;
                }
                self.ph = Ph::Pair;
            }
            Ph::Pair => {
                if m == "bust" {
                    self.end_turn(false);
                    return;
                }
                let o = self.options(&self.d);
                let sums = o.into_iter().find(|x| x.0 == m).unwrap().1;
                let (mut run, mut temp) = (self.run.clone(), self.temp);
                for s in sums {
                    self.apply(s, &mut run, &mut temp);
                }
                self.run = run;
                self.temp = temp;
                self.ph = Ph::Decide;
            }
            Ph::Decide => {
                if m == "continu" {
                    self.ph = Ph::Roll;
                } else {
                    self.end_turn(true);
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        for s in 0..2 {
            if self.nclaim(s) >= self.sp.win {
                return Some(if s == 0 {
                    Outcome::Win(Player::One)
                } else {
                    Outcome::Win(Player::Two)
                });
            }
        }
        if self.plies >= self.sp.cap {
            return Some(by_score(self.perm_val(0), self.perm_val(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 10 * (self.perm_val(a) - self.perm_val(1 - a));
        let sgn = if self.side == a { 1 } else { -1 };
        match self.ph {
            // după „continu”: aruncarea următoare poate da bust (se pierde tot ce e temporar)
            Ph::Roll if !self.run.is_empty() => {
                let pb = self.pbust();
                base + sgn * 10 * ((100 - pb) * (self.temp_val() + 40) / 100)
            }
            Ph::Decide | Ph::Pair => base + sgn * 10 * self.temp_val(),
            _ => base,
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — câștigă cine închide {} coloane\n",
            self.sp.title, self.sp.win
        );
        for c in 0..11 {
            let own = |p: usize| {
                if self.claimed[c] == Some(p) {
                    "închisă".to_string()
                } else {
                    format!("{}", self.perm[p][c])
                }
            };
            s.push_str(&format!(
                "coloana {:>2} (lungime {}): J1 {} | J2 {}{}\n",
                c + 2,
                self.sp.lens[c],
                own(0),
                own(1),
                if self.temp[c] > 0 {
                    format!(" | temporar +{}", self.temp[c])
                } else {
                    String::new()
                }
            ));
        }
        if self.ph != Ph::Roll {
            s.push_str(&format!("Zaruri: {:?}\n", self.d));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | cuplu k [suma] | continu | opresc | bust)\n",
            pname(self.side())
        ));
        s
    }
}
