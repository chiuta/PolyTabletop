//! Motor cooperativ „focare pe hartă” (lista C2): echipa (J1, doi agenți pe rând) se mișcă pe un
//! graf de noduri, stinge focarele, ridică piese din noduri-sursă și le predă la bază (nodul 0);
//! J2 este Jocul (forțat `eveniment`: focare noi, izbucniri în lanț). Win J1 = piese predate.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct RSpec {
    pub title: &'static str,
    pub heroes: [&'static str; 2],
    pub n: usize,
    pub chord: usize,
    pub src: usize,
    pub goal: i32,
    pub lim: i32,
    pub rate: usize,
    pub ramp: u32,
    pub fly: bool,
    pub node: &'static str,
}

#[derive(Clone)]
pub struct R {
    sp: &'static RSpec,
    tok: Vec<i32>,
    pos: [usize; 2],
    carry: [i32; 2],
    delivered: i32,
    out: i32,
    round: u32,
    hero: usize,
    ap: u8,
    side: Player,
    rng: Rng,
}

impl R {
    pub fn new(sp: &'static RSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static RSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut tok = vec![0; sp.n];
        for _ in 0..3 {
            let i = 1 + rng.below(sp.n - 1);
            tok[i] += 1;
        }
        R {
            sp,
            tok,
            pos: [0, 0],
            carry: [0, 0],
            delivered: 0,
            out: 0,
            round: 1,
            hero: 0,
            ap: 3,
            side: Player::One,
            rng,
        }
    }
    fn adj(&self, i: usize) -> Vec<usize> {
        let n = self.sp.n;
        let mut v = vec![(i + 1) % n, (i + n - 1) % n];
        if self.sp.chord > 0 {
            let j = (i + n / 2) % n;
            if j != i && i.min(j).is_multiple_of(self.sp.chord) {
                v.push(j);
            }
        }
        v.sort();
        v.dedup();
        v
    }
    fn is_src(&self, i: usize) -> bool {
        i != 0 && i.is_multiple_of(self.sp.src)
    }
    fn next_hero(&mut self) {
        self.ap = 3;
        if self.hero == 0 {
            self.hero = 1;
        } else {
            self.hero = 0;
            self.side = Player::Two;
        }
    }
    fn spend(&mut self) {
        self.ap -= 1;
        if self.ap == 0 {
            self.next_hero();
        }
    }
}

impl Mini for R {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.side == Player::Two {
            return vec!["eveniment".into()];
        }
        let h = self.hero;
        let p = self.pos[h];
        let mut v: Vec<String> = self.adj(p).iter().map(|j| format!("mers {}", j)).collect();
        if self.tok[p] > 0 {
            v.push("stinge".into());
        }
        if self.is_src(p) && self.carry[h] < 2 {
            v.push("ia".into());
        }
        if p == 0 && self.carry[h] > 0 {
            v.push("preda".into());
        }
        if self.sp.fly && p == 0 {
            for j in 1..self.sp.n {
                if self.tok[j] > 0 {
                    v.push(format!("zbor {}", j));
                }
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        let h = self.hero;
        match m {
            "eveniment" => {
                let k = self.sp.rate + (self.round / self.sp.ramp.max(1)) as usize;
                let mut burst: Vec<usize> = Vec::new();
                for _ in 0..k {
                    let i = self.rng.below(self.sp.n);
                    if self.tok[i] >= 3 {
                        if !burst.contains(&i) {
                            burst.push(i);
                        }
                    } else {
                        self.tok[i] += 1;
                    }
                }
                for i in burst {
                    self.out += 1;
                    for j in self.adj(i) {
                        self.tok[j] = (self.tok[j] + 1).min(3);
                    }
                }
                self.round += 1;
                self.hero = 0;
                self.ap = 3;
                self.side = Player::One;
            }
            "pas" => self.next_hero(),
            "stinge" => {
                let p = self.pos[h];
                self.tok[p] = if h == 1 { 0 } else { self.tok[p] - 1 };
                self.spend();
            }
            "ia" => {
                self.carry[h] += 1;
                self.spend();
            }
            "preda" => {
                self.delivered += self.carry[h];
                self.carry[h] = 0;
                self.spend();
            }
            _ => {
                let (c, j) = m.split_once(' ').unwrap();
                let j: usize = j.parse().unwrap();
                let _ = c;
                self.pos[h] = j;
                self.spend();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.delivered >= self.sp.goal {
            Some(Outcome::Win(Player::One))
        } else if self.out >= self.sp.lim || self.round > 40 {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let mut v = self.delivered * 20 - self.out * 12;
        for t in &self.tok {
            v -= t * t;
        }
        let n = self.sp.n;
        let rd = |a: usize, b: usize| a.abs_diff(b).min(n - a.abs_diff(b)) as i32;
        for h in 0..2 {
            v += self.carry[h] * 8;
            let pos = self.pos[h];
            if self.carry[h] > 0 {
                v -= rd(pos, 0);
            } else {
                let best = (1..n).filter(|&i| self.is_src(i)).map(|i| rd(pos, i)).min();
                v -= best.unwrap_or(0);
            }
        }
        if p == Player::One {
            v
        } else {
            -v
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {} | predate {}/{} | izbucniri {}/{}\n",
            self.sp.title, self.round, self.delivered, self.sp.goal, self.out, self.sp.lim
        );
        for i in 0..self.sp.n {
            let here: Vec<String> = (0..2)
                .filter(|&h| self.pos[h] == i)
                .map(|h| self.sp.heroes[h].to_string())
                .collect();
            s.push_str(&format!(
                "  {} {:>2}{}{} focare {}{}\n",
                self.sp.node,
                i,
                if i == 0 { " (baza)" } else { "" },
                if self.is_src(i) { " (sursă)" } else { "" },
                self.tok[i],
                if here.is_empty() {
                    String::new()
                } else {
                    format!(" <- {}", here.join(", "))
                }
            ));
        }
        s.push_str(&format!(
            "{}: poartă {} | {}: poartă {}\nLa rând: {} ({} acțiuni) — mers N | stinge | ia | preda | zbor N | pas\n",
            self.sp.heroes[0], self.carry[0], self.sp.heroes[1], self.carry[1],
            self.sp.heroes[self.hero], self.ap
        ));
        s
    }
}
