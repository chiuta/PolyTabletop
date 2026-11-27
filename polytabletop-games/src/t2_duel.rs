//! Motor „duel de aventurieri” (familia T2, variante originale pentru doi jucători).
//!
//! Fiecare jucător alege o clasă (`alege k`, public), apoi alternați: `lovesc` (aruncă zarurile
//! clasei), `lovesc+` (cheltuiești 1 resursă pentru un zar în plus, dacă regula „noroc” e activă),
//! `parez` (gardă: armură mai mare până la tura ta, +1 resursă), `special` (puterea clasei, costă
//! resursă), `potiune`. Zarurile se aruncă doar în mutarea forțată `tira`, deci AI-ul (care nu
//! trece prin ea) nu poate vedea aruncările viitoare. Informație complet publică.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sp {
    Heavy,
    Heal,
    Pierce,
    Stun,
    Drain,
    Poison,
}

pub struct Cls {
    pub name: &'static str,
    pub hp: i8,
    pub dice: i8,
    pub tn: i8,
    pub dmg: i8,
    pub armor: i8,
    pub sp: Sp,
    pub cost: i8,
    pub val: i8,
    pub spname: &'static str,
}

pub const BLEED: u8 = 1;
pub const RAGE: u8 = 2;
pub const COUNTER: u8 = 4;
pub const CRIT: u8 = 8;
pub const FUMBLE: u8 = 16;
pub const LUCK: u8 = 32;

pub struct Cfg {
    pub title: &'static str,
    pub res: &'static str,
    pub sides: i8,
    pub flags: u8,
    pub turns: u32,
    pub potions: i8,
    pub classes: &'static [Cls],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Pick,
    Act,
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Act {
    Hit(bool),
    Spec,
}

#[derive(Clone)]
pub struct Duel {
    cfg: &'static Cfg,
    ph: Ph,
    picked: [bool; 2],
    cls: [usize; 2],
    hp: [i32; 2],
    res: [i32; 2],
    guard: [bool; 2],
    stun: [bool; 2],
    bleed: [i32; 2],
    pois: [i32; 2],
    pot: [i32; 2],
    n: [u32; 2],
    turn: usize,
    pend: Option<Act>,
    rng: Rng,
}

fn p_hit(sides: i32, tn: i32) -> i32 {
    ((sides - tn + 1) * 100 / sides).clamp(0, 100)
}

impl Duel {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        Duel {
            cfg,
            ph: Ph::Pick,
            picked: [false; 2],
            cls: [0; 2],
            hp: [0; 2],
            res: [0; 2],
            guard: [false; 2],
            stun: [false; 2],
            bleed: [0; 2],
            pois: [0; 2],
            pot: [0; 2],
            n: [0; 2],
            turn: 0,
            pend: None,
            rng: Rng::new(seed),
        }
    }
    fn c(&self, p: usize) -> &'static Cls {
        &self.cfg.classes[self.cls[p]]
    }
    fn has(&self, f: u8) -> bool {
        self.cfg.flags & f != 0
    }
    fn begin_turn(&mut self) {
        let t = self.turn;
        self.guard[t] = false;
        self.res[t] = (self.res[t] + 1).min(5);
        if self.bleed[t] > 0 {
            self.hp[t] -= 1;
            self.bleed[t] -= 1;
        }
        if self.pois[t] > 0 {
            self.hp[t] -= 1;
            self.pois[t] -= 1;
        }
    }
    fn end_turn(&mut self) {
        self.n[self.turn] += 1;
        self.turn = 1 - self.turn;
        self.ph = Ph::Act;
        self.begin_turn();
    }
    fn power(&self, k: usize) -> i32 {
        let c = &self.cfg.classes[k];
        let ph = p_hit(self.cfg.sides as i32, c.tn as i32);
        c.hp as i32 + c.dice as i32 * ph * c.dmg as i32 / 25 + 3 * c.armor as i32 + c.val as i32
    }
    /// Lovitură așteptată (în sutimi de punct de viață) a atacatorului curent.
    fn expect(&self, a: usize, act: Act) -> i32 {
        let d = 1 - a;
        let c = self.c(a);
        let dc = self.c(d);
        let sides = self.cfg.sides as i32;
        let mut dice = c.dice as i32;
        let mut dmg = c.dmg as i32;
        let mut arm = dc.armor as i32 + if self.guard[d] { 2 } else { 0 };
        match act {
            Act::Hit(plus) => {
                if plus {
                    dice += 1;
                }
            }
            Act::Spec => match c.sp {
                Sp::Heavy => dmg += c.val as i32,
                Sp::Pierce => arm = 0,
                _ => {}
            },
        }
        let p = p_hit(sides, c.tn as i32);
        let none = {
            let mut x = 100;
            for _ in 0..dice {
                x = x * (100 - p) / 100;
            }
            x
        };
        let any = 100 - none;
        let raw = dice * p * dmg;
        let mut v = (raw - arm * any).max(any);
        match (act, c.sp) {
            (Act::Spec, Sp::Stun) => v += 150 * any / 100,
            (Act::Spec, Sp::Poison) => v += 100 * c.val as i32 * any / 100,
            (Act::Spec, Sp::Drain) => v += v / 2,
            _ => {}
        }
        v
    }
    fn resolve(&mut self) {
        let a = self.turn;
        let d = 1 - a;
        let act = self.pend.take().unwrap();
        let c = self.c(a);
        let dc = self.c(d);
        let sides = self.cfg.sides as usize;
        let mut dice = c.dice as i32;
        let mut dmg = c.dmg as i32;
        let mut pierce = false;
        match act {
            Act::Hit(plus) => {
                if plus {
                    dice += 1;
                }
            }
            Act::Spec => match c.sp {
                Sp::Heavy => dmg += c.val as i32,
                Sp::Pierce => pierce = true,
                _ => {}
            },
        }
        let mut succ = 0;
        let mut ones = 0;
        for _ in 0..dice {
            let f = 1 + self.rng.below(sides) as i32;
            if f >= c.tn as i32 {
                succ += 1;
            }
            if f == 1 {
                ones += 1;
            }
        }
        if self.has(FUMBLE) && ones == dice && dice >= 2 {
            self.hp[a] -= 1;
            return;
        }
        let mut hits = succ;
        if self.has(CRIT) && dice >= 2 && succ == dice {
            hits *= 2;
        }
        if hits == 0 {
            return;
        }
        let mut raw = hits * dmg;
        if self.has(RAGE) && self.hp[a] * 2 <= c.hp as i32 {
            raw += 1;
        }
        let arm = if pierce {
            0
        } else {
            dc.armor as i32 + if self.guard[d] { 2 } else { 0 }
        };
        let dealt = (raw - arm).max(1);
        self.hp[d] -= dealt;
        if self.has(COUNTER) && self.guard[d] && !pierce {
            self.hp[a] -= 1;
        }
        if self.has(BLEED) && dealt >= 3 {
            self.bleed[d] = (self.bleed[d] + 1).min(3);
        }
        if let Act::Spec = act {
            match c.sp {
                Sp::Stun => self.stun[d] = true,
                Sp::Poison => self.pois[d] += c.val as i32,
                Sp::Drain => {
                    self.hp[a] = (self.hp[a] + (dealt / 2).max(1)).min(c.hp as i32);
                }
                _ => {}
            }
        }
    }
    fn pct(&self, p: usize) -> i32 {
        self.hp[p].max(0) * 100 / self.c(p).hp as i32
    }
}

impl Mini for Duel {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.turn == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let t = self.turn;
        match self.ph {
            Ph::Pick => (0..self.cfg.classes.len())
                .map(|k| format!("alege {k}"))
                .collect(),
            Ph::Roll => vec!["tira".into()],
            Ph::Act => {
                if self.stun[t] {
                    return vec!["trec".into()];
                }
                let c = self.c(t);
                let mut v = vec!["lovesc".to_string()];
                if self.has(LUCK) && self.res[t] >= 1 {
                    v.push("lovesc+".into());
                }
                v.push("parez".into());
                if self.res[t] >= c.cost as i32 && (c.sp != Sp::Heal || self.hp[t] < c.hp as i32) {
                    v.push("special".into());
                }
                if self.pot[t] > 0 && self.hp[t] < c.hp as i32 {
                    v.push("potiune".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let t = self.turn;
        let mut it = m.split(' ');
        match it.next().unwrap() {
            "alege" => {
                let k: usize = it.next().unwrap().parse().unwrap();
                self.cls[t] = k;
                self.picked[t] = true;
                self.hp[t] = self.cfg.classes[k].hp as i32;
                self.res[t] = 1;
                self.pot[t] = self.cfg.potions as i32;
                if t == 0 {
                    self.turn = 1;
                } else {
                    self.turn = 0;
                    self.ph = Ph::Act;
                    self.begin_turn();
                }
            }
            "trec" => {
                self.stun[t] = false;
                self.end_turn();
            }
            "lovesc" => {
                self.pend = Some(Act::Hit(false));
                self.ph = Ph::Roll;
            }
            "lovesc+" => {
                self.res[t] -= 1;
                self.pend = Some(Act::Hit(true));
                self.ph = Ph::Roll;
            }
            "parez" => {
                self.guard[t] = true;
                self.res[t] = (self.res[t] + 1).min(5);
                self.end_turn();
            }
            "special" => {
                let c = self.c(t);
                self.res[t] -= c.cost as i32;
                if c.sp == Sp::Heal {
                    self.hp[t] = (self.hp[t] + c.val as i32).min(c.hp as i32);
                    self.end_turn();
                } else {
                    self.pend = Some(Act::Spec);
                    self.ph = Ph::Roll;
                }
            }
            "potiune" => {
                self.pot[t] -= 1;
                self.hp[t] = (self.hp[t] + 4).min(self.c(t).hp as i32);
                self.end_turn();
            }
            _ => {
                self.resolve();
                self.end_turn();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph == Ph::Pick {
            return None;
        }
        let d0 = self.hp[0] <= 0;
        let d1 = self.hp[1] <= 0;
        if d0 && d1 {
            return Some(Outcome::Draw);
        }
        if d0 {
            return Some(Outcome::Win(Player::Two));
        }
        if d1 {
            return Some(Outcome::Win(Player::One));
        }
        if self.ph == Ph::Act && self.n[0] >= self.cfg.turns && self.n[1] >= self.cfg.turns {
            return Some(by_score(self.pct(0), self.pct(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        if self.ph == Ph::Pick || !(self.picked[0] && self.picked[1]) {
            let f = |x: usize| {
                if self.picked[x] {
                    self.power(self.cls[x])
                } else {
                    0
                }
            };
            return 10 * (f(i) - f(o));
        }
        let mut s = 10 * (self.hp[i] - self.hp[o])
            + 3 * (self.res[i] - self.res[o])
            + 4 * (self.pot[i] - self.pot[o])
            - 5 * (self.bleed[i] + self.pois[i] - self.bleed[o] - self.pois[o])
            + 2 * (self.power(self.cls[i]) - self.power(self.cls[o]));
        if self.guard[i] {
            s += 4;
        }
        if self.guard[o] {
            s -= 4;
        }
        if self.stun[i] {
            s -= 12;
        }
        if self.stun[o] {
            s += 12;
        }
        if let Some(act) = self.pend {
            let e = self.expect(self.turn, act) / 10;
            if self.turn == i {
                s += e;
            } else {
                s -= e;
            }
        }
        s
    }
    fn show(&self) -> String {
        let cfg = self.cfg;
        let mut s = format!(
            "{} — duel de aventurieri, tura {}/{}\n",
            cfg.title,
            self.n[self.turn.min(1)] + 1,
            cfg.turns
        );
        for (k, c) in cfg.classes.iter().enumerate() {
            s.push_str(&format!(
                "  clasa {k}: {} (PV {}, {} zaruri d{} ≥{}, daune {}, armură {}, special «{}» cost {})\n",
                c.name, c.hp, c.dice, cfg.sides, c.tn, c.dmg, c.armor, c.spname, c.cost
            ));
        }
        for p in 0..2 {
            if !self.picked[p] {
                s.push_str(&format!("J{}: încă nu a ales clasa\n", p + 1));
                continue;
            }
            let c = self.c(p);
            s.push_str(&format!(
                "J{}: {} PV {}/{} | {} {} | poțiuni {}{}{}{}{}{}\n",
                p + 1,
                c.name,
                self.hp[p].max(0),
                c.hp,
                cfg.res,
                self.res[p],
                self.pot[p],
                if self.guard[p] { " | gardă" } else { "" },
                if self.stun[p] { " | amețit" } else { "" },
                if self.bleed[p] > 0 {
                    format!(" | sângerare {}", self.bleed[p])
                } else {
                    String::new()
                },
                if self.pois[p] > 0 {
                    format!(" | otravă {}", self.pois[p])
                } else {
                    String::new()
                },
                if self.turn == p { " ◄" } else { "" }
            ));
        }
        match self.ph {
            Ph::Pick => s.push_str(&format!("J{} alege clasa: alege k\n", self.turn + 1)),
            Ph::Roll => s.push_str("Aruncarea așteaptă: tira\n"),
            Ph::Act => {}
        }
        s
    }
}

#[cfg(test)]
impl Duel {
    fn reseed(&mut self, s: u64) {
        self.rng = Rng::new(s);
    }
}

#[cfg(test)]
pub fn check_cfg(cfg: &'static Cfg) {
    crate::t2_util::check_fn(&|s| Duel::with_seed(cfg, s), 400, Duel::reseed);
}
