//! Motor „duel cu zaruri” (lista C5, 2 jucători), trei familii cu aceeași mecanică de bază
//! (arunci toate zarurile, de până la 3 ori, păstrând ce vrei: `reia m` = zarurile cu bit 1 se
//! aruncă din nou; `gata` încheie aruncările):
//! * `City`: număr/inimă/gheară/energie, centrul orașului, cedare, cheltuirea energiei;
//! * `Prairie`: săgeți (luate din teanc, blocate), dinamită (3 = pierzi o viață), împușcături,
//!   bere și mitralieră, distanță alternantă;
//! * `Heroes`: fiecare erou are o foaie de abilități (n zaruri cu același simbol, șiruri) și puncte de luptă.
//!
//! Aruncările se fac la mutări forțate (`arunc`); AI-ul estimează doar valori așteptate.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DMode {
    City,
    Prairie,
    Heroes,
}

#[derive(Clone, Copy)]
pub enum Req {
    /// cel puțin n zaruri cu simbolul s
    Of(u8, u8),
    /// n simboluri diferite consecutive (în ordinea 0..5)
    Run(u8),
    /// două perechi (simboluri diferite)
    TwoPairs,
}

pub struct Ab {
    pub name: &'static str,
    pub req: Req,
    pub dmg: i32,
    pub heal: i32,
    pub cp: i32,
}

pub struct Hero {
    pub name: &'static str,
    pub syms: [&'static str; 6],
    pub abil: &'static [Ab],
}

pub struct DuelSpec {
    pub title: &'static str,
    pub mode: DMode,
    pub dice: usize,
    pub hp: i32,
    pub goal: i32,
    pub heroes: [&'static Hero; 2],
    pub cap: u32,
}

pub static NONE: Hero = Hero {
    name: "-",
    syms: [""; 6],
    abil: &[],
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Roll,
    Keep,
    Yield,
    Buy,
    Abil,
}

#[derive(Clone)]
pub struct Duel {
    sp: &'static DuelSpec,
    hp: [i32; 2],
    vp: [i32; 2],
    en: [i32; 2],
    claw: [i32; 2],
    center: Option<usize>,
    arrows: [i32; 2],
    pile: i32,
    d: Vec<u8>,
    lock: Vec<bool>,
    re: Vec<bool>,
    left: u8,
    pend: i32,
    ph: Ph,
    turn: u32,
    side: usize,
    dice: Dice,
    plies: u32,
}

fn pl2(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

impl Duel {
    pub fn new(sp: &'static DuelSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static DuelSpec, seed: u64) -> Self {
        Duel {
            sp,
            hp: [sp.hp; 2],
            vp: [0; 2],
            en: [0; 2],
            claw: [0; 2],
            center: None,
            arrows: [0; 2],
            pile: 9,
            d: vec![0; sp.dice],
            lock: vec![false; sp.dice],
            re: vec![false; sp.dice],
            left: 3,
            pend: 0,
            ph: Ph::Roll,
            turn: 0,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn counts(&self) -> [i32; 6] {
        let mut c = [0; 6];
        for &x in &self.d {
            c[x as usize] += 1;
        }
        c
    }
    fn hero(&self, p: usize) -> &'static Hero {
        self.sp.heroes[p]
    }
    fn req_ok(r: Req, c: &[i32; 6]) -> bool {
        match r {
            Req::Of(s, n) => c[s as usize] >= n as i32,
            Req::Run(n) => {
                let mut best = 0;
                let mut cur = 0;
                for i in 0..6 {
                    if c[i] > 0 {
                        cur += 1;
                        best = best.max(cur);
                    } else {
                        cur = 0;
                    }
                }
                best >= n as i32
            }
            Req::TwoPairs => c.iter().filter(|&&x| x >= 2).count() >= 2,
        }
    }
    fn abil_list(&self) -> Vec<usize> {
        let c = self.counts();
        let h = self.hero(self.side);
        (0..h.abil.len())
            .filter(|&k| Self::req_ok(h.abil[k].req, &c) && h.abil[k].cp <= self.en[self.side])
            .collect()
    }
    fn end_turn(&mut self) {
        self.side = 1 - self.side;
        self.turn += 1;
        self.left = 3;
        self.ph = Ph::Roll;
        self.lock = vec![false; self.sp.dice];
        self.pend = 0;
        if self.sp.mode == DMode::City && self.center == Some(self.side) {
            self.vp[self.side] += 1;
        }
        if self.sp.mode == DMode::Heroes {
            self.en[self.side] += 1;
        }
    }
    /// Efectul zarurilor din modul prerie într-o aruncare: săgeți luate, dinamită blocată.
    fn arrow_lock(&mut self) {
        let me = self.side;
        for i in 0..self.d.len() {
            if self.lock[i] {
                continue;
            }
            if self.d[i] == 0 {
                self.lock[i] = true;
                self.arrows[me] += 1;
                self.pile -= 1;
                if self.pile == 0 {
                    for p in 0..2 {
                        self.hp[p] -= self.arrows[p];
                        self.arrows[p] = 0;
                    }
                    self.pile = 9;
                }
            } else if self.d[i] == 1 {
                self.lock[i] = true;
            }
        }
    }
    fn dyn_count(&self) -> i32 {
        self.d.iter().filter(|&&x| x == 1).count() as i32
    }
    fn resolve(&mut self) {
        let me = self.side;
        let c = self.counts();
        match self.sp.mode {
            DMode::City => {
                for n in 0..3 {
                    if c[n] >= 3 {
                        self.vp[me] += n as i32 + 1 + (c[n] - 3);
                    }
                }
                if self.center != Some(me) {
                    self.hp[me] = (self.hp[me] + c[3]).min(self.sp.hp);
                }
                self.en[me] += c[5];
                let dmg = if c[4] > 0 { c[4] + self.claw[me] } else { 0 };
                if dmg > 0 {
                    match self.center {
                        None => {
                            self.center = Some(me);
                            self.vp[me] += 1;
                        }
                        Some(x) if x == me => self.hp[1 - me] -= dmg,
                        Some(x) => {
                            self.hp[x] -= dmg;
                            if self.hp[x] > 0 {
                                self.pend = 1;
                                self.ph = Ph::Yield;
                                return;
                            }
                        }
                    }
                }
                self.ph = Ph::Buy;
            }
            DMode::Prairie => {
                if self.dyn_count() >= 3 {
                    self.hp[me] -= 1;
                } else {
                    let near = self.turn.is_multiple_of(2);
                    let shots = if near { c[2] } else { c[3] };
                    let mut dmg = shots.min(2);
                    if c[2] + c[3] >= 4 {
                        dmg += 1;
                    }
                    if c[5] >= 3 {
                        dmg += 1;
                        self.pile = (self.pile + self.arrows[me]).min(9);
                        self.arrows[me] = 0;
                    }
                    self.hp[1 - me] -= dmg;
                    self.hp[me] = (self.hp[me] + c[4]).min(self.sp.hp);
                }
                self.end_turn();
            }
            DMode::Heroes => {
                self.en[me] += c[5];
                self.ph = Ph::Abil;
            }
        }
    }
    /// Valoarea (în sutimi) a unor zaruri „așteptate” pentru jucătorul curent.
    fn exp_value(&self) -> i32 {
        let me = self.side;
        let mut c = [0f32; 6];
        let mut nre = 0f32;
        for i in 0..self.d.len() {
            if self.re[i] && !self.lock[i] {
                nre += 1.0;
            } else {
                c[self.d[i] as usize] += 1.0;
            }
        }
        for f in 0..6 {
            c[f] += nre / 6.0;
        }
        let v: f32 = match self.sp.mode {
            DMode::City => {
                let mut v = 0.0;
                for n in 0..3 {
                    v += if c[n] >= 3.0 {
                        (n as f32 + 1.0) + (c[n] - 3.0)
                    } else {
                        c[n] * (n as f32 + 1.0) / 6.0
                    };
                }
                let in_c = self.center == Some(me);
                v += if in_c { 0.0 } else { 0.7 * c[3] };
                v += 1.1
                    * (c[4]
                        + if c[4] > 0.5 {
                            self.claw[me] as f32
                        } else {
                            0.0
                        });
                v + 0.25 * c[5]
            }
            DMode::Prairie => {
                let near = self.turn.is_multiple_of(2);
                let shots = if near { c[2] } else { c[3] };
                1.2 * shots.min(2.0) + 0.5 * c[4] - 0.8 * c[0] - 1.0 * (c[1] - 1.0).max(0.0)
                    + 0.3 * c[5]
            }
            DMode::Heroes => {
                let h = self.hero(me);
                let mut best = 0.0f32;
                for a in h.abil {
                    let f = match a.req {
                        Req::Of(s, n) => (c[s as usize] / n as f32).min(1.0),
                        Req::Run(n) => {
                            (c.iter().filter(|&&x| x >= 0.9).count() as f32 / n as f32).min(1.0)
                        }
                        Req::TwoPairs => {
                            (c.iter().filter(|&&x| x >= 1.9).count() as f32 / 2.0).min(1.0)
                        }
                    };
                    let usable = if a.cp <= self.en[me] { 1.0 } else { 0.0 };
                    best = best.max(f * f * (a.dmg as f32 + 0.5 * a.heal as f32) * usable);
                }
                best
            }
        };
        (100.0 * v) as i32
    }
}

impl Mini for Duel {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.ph == Ph::Yield {
            pl2(1 - self.side)
        } else {
            pl2(self.side)
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["arunc".into()],
            Ph::Keep => {
                let mut v = vec!["gata".to_string()];
                if self.left > 0 {
                    let free: Vec<usize> = (0..self.d.len()).filter(|&i| !self.lock[i]).collect();
                    for m in 1..(1u32 << free.len()) {
                        let mut s = String::from("reia ");
                        for (j, _) in free.iter().enumerate() {
                            s.push(if m >> j & 1 == 1 { '1' } else { '0' });
                        }
                        v.push(s);
                    }
                }
                v
            }
            Ph::Yield => vec!["cedez".into(), "raman".into()],
            Ph::Buy => {
                let me = self.side;
                let mut v = vec!["fin".to_string()];
                if self.en[me] >= 3 {
                    v.push("e vp".into());
                }
                if self.en[me] >= 6 {
                    v.push("e gheare".into());
                }
                v
            }
            Ph::Abil => {
                let mut v: Vec<String> = self
                    .abil_list()
                    .iter()
                    .map(|k| format!("abil {}", k + 1))
                    .collect();
                v.push("nimic".into());
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let me = self.side;
        match self.ph {
            Ph::Roll => {
                if self.left == 3 {
                    self.re = vec![true; self.d.len()];
                }
                for i in 0..self.d.len() {
                    if self.re[i] && !self.lock[i] {
                        self.d[i] = self.dice.below(6) as u8;
                    }
                }
                self.left -= 1;
                self.re = vec![false; self.d.len()];
                if self.sp.mode == DMode::Prairie {
                    self.arrow_lock();
                    if self.hp[0] <= 0 || self.hp[1] <= 0 {
                        return;
                    }
                    if self.dyn_count() >= 3 {
                        self.resolve();
                        return;
                    }
                }
                if self.left == 0 {
                    self.resolve();
                } else {
                    self.ph = Ph::Keep;
                }
            }
            Ph::Keep => {
                if m == "gata" {
                    self.resolve();
                } else {
                    let bits: Vec<char> = m[5..].chars().collect();
                    let mut j = 0;
                    for i in 0..self.d.len() {
                        if !self.lock[i] {
                            self.re[i] = bits[j] == '1';
                            j += 1;
                        }
                    }
                    self.ph = Ph::Roll;
                }
            }
            Ph::Yield => {
                if m == "cedez" {
                    self.center = Some(me);
                    self.vp[me] += 1;
                }
                self.pend = 0;
                self.ph = Ph::Buy;
            }
            Ph::Buy => match m {
                "e vp" => {
                    self.en[me] -= 3;
                    self.vp[me] += 2;
                }
                "e gheare" => {
                    self.en[me] -= 6;
                    self.claw[me] += 1;
                }
                _ => self.end_turn(),
            },
            Ph::Abil => {
                if m != "nimic" {
                    let k = m[5..].parse::<usize>().unwrap() - 1;
                    let a = &self.hero(me).abil[k];
                    self.hp[1 - me] -= a.dmg;
                    self.hp[me] = (self.hp[me] + a.heal).min(self.sp.hp);
                    self.en[me] -= a.cp;
                }
                self.end_turn();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        let dead = |p: usize| self.hp[p] <= 0;
        match (dead(0), dead(1)) {
            (true, false) => return Some(Outcome::Win(Player::Two)),
            (false, true) => return Some(Outcome::Win(Player::One)),
            (true, true) => return Some(Outcome::Draw),
            _ => {}
        }
        if self.sp.mode == DMode::City {
            let (a, b) = (self.vp[0] >= self.sp.goal, self.vp[1] >= self.sp.goal);
            if a || b {
                return Some(match (a, b) {
                    (true, false) => Outcome::Win(Player::One),
                    (false, true) => Outcome::Win(Player::Two),
                    _ => by_score(self.vp[0], self.vp[1]),
                });
            }
        }
        if self.plies >= self.sp.cap {
            return Some(by_score(self.hp[0] + self.vp[0], self.hp[1] + self.vp[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let mut v = 100 * (self.hp[a] - self.hp[o]) + 10 * (self.en[a] - self.en[o]);
        match self.sp.mode {
            DMode::City => {
                v += 90 * (self.vp[a] - self.vp[o]) + 40 * (self.claw[a] - self.claw[o]);
                if self.center == Some(a) {
                    v += 80;
                } else if self.center == Some(o) {
                    v -= 80;
                }
            }
            DMode::Prairie => v -= 40 * (self.arrows[a] - self.arrows[o]),
            DMode::Heroes => {}
        }
        if self.ph == Ph::Roll && self.left < 3 {
            let w = self.exp_value();
            v += if self.side == a { w } else { -w };
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{} — tura {}\n", self.sp.title, self.turn + 1);
        for p in 0..2 {
            s.push_str(&format!(
                "{}{}: viață {} | energie {}{}{}{}\n",
                pname(pl2(p)),
                if self.sp.mode == DMode::Heroes {
                    format!(" ({})", self.hero(p).name)
                } else {
                    String::new()
                },
                self.hp[p],
                self.en[p],
                if self.sp.mode == DMode::City {
                    format!(" | puncte {} | gheare+{}", self.vp[p], self.claw[p])
                } else {
                    String::new()
                },
                if self.sp.mode == DMode::Prairie {
                    format!(" | săgeți {}", self.arrows[p])
                } else {
                    String::new()
                },
                if self.center == Some(p) {
                    " | în centru"
                } else {
                    ""
                }
            ));
        }
        if self.sp.mode == DMode::Prairie {
            s.push_str(&format!("Teanc de săgeți: {}\n", self.pile));
        }
        let names: Vec<String> = match self.sp.mode {
            DMode::City => ["1", "2", "3", "inimă", "gheară", "energie"]
                .map(String::from)
                .to_vec(),
            DMode::Prairie => [
                "săgeată",
                "dinamită",
                "foc-aproape",
                "foc-departe",
                "bere",
                "mitralieră",
            ]
            .map(String::from)
            .to_vec(),
            DMode::Heroes => self.hero(self.side).syms.map(String::from).to_vec(),
        };
        if self.ph != Ph::Roll {
            let t: Vec<String> = self
                .d
                .iter()
                .enumerate()
                .map(|(i, &x)| {
                    format!(
                        "{}{}",
                        names[x as usize],
                        if self.lock[i] { "*" } else { "" }
                    )
                })
                .collect();
            s.push_str(&format!(
                "Zaruri (aruncări rămase {}): {}\n",
                self.left,
                t.join(" | ")
            ));
        }
        if self.sp.mode == DMode::Heroes && self.ph == Ph::Abil {
            for (k, a) in self.hero(self.side).abil.iter().enumerate() {
                s.push_str(&format!(
                    "  abil {}: {} (daună {}, vindecare {}, cost {})\n",
                    k + 1,
                    a.name,
                    a.dmg,
                    a.heal,
                    a.cp
                ));
            }
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | reia m | gata | cedez | raman | e vp | e gheare | fin | abil k | nimic)\n",
            pname(self.side())
        ));
        s
    }
}
