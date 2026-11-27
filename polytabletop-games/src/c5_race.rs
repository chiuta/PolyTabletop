//! Motor „cursă pe traseu” (lista C5, 2 jucători): jetoane pe un traseu liniar, zar forțat
//! (`arunc`), alegi ce jeton muți (`muta k`), pătrate speciale, capturi și intrare la valoare fixă.
//! Zarul e re-însămânțat la `clone()`, deci AI-ul nu poate prevedea aruncarea (o singură mutare
//! înainte, evaluare onestă). Informație completă.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy)]
pub enum Sq {
    Fwd(i32),
    Back(i32),
    Extra,
    Skip,
}

pub struct RSpec {
    pub title: &'static str,
    pub len: i32,
    pub toks: usize,
    pub need: usize,
    pub sides: u8,
    pub capture: bool,
    pub exact: bool,
    /// 0 = fără; altfel valoarea zarului care scoate un jeton din start (și dă o tură în plus).
    pub enter: u8,
    pub cap: u32,
    pub sq: &'static [(i32, Sq)],
}

#[derive(Clone)]
pub struct Race {
    sp: &'static RSpec,
    pos: [Vec<i32>; 2],
    side: usize,
    die: Option<u8>,
    skip: [u8; 2],
    dice: Dice,
    plies: u32,
}

impl Race {
    pub fn new(sp: &'static RSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static RSpec, seed: u64) -> Self {
        Race {
            sp,
            pos: [vec![0; sp.toks], vec![0; sp.toks]],
            side: 0,
            die: None,
            skip: [0; 2],
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn target(&self, k: usize) -> Option<i32> {
        let d = self.die? as i32;
        let p = self.pos[self.side][k];
        if p >= self.sp.len {
            return None;
        }
        let t = if p == 0 {
            if self.sp.enter > 0 {
                if self.die? == self.sp.enter {
                    1
                } else {
                    return None;
                }
            } else {
                d
            }
        } else {
            p + d
        };
        if t > self.sp.len {
            if self.sp.exact {
                None
            } else {
                Some(self.sp.len)
            }
        } else {
            Some(t)
        }
    }
    fn fin(&self, s: usize) -> usize {
        self.pos[s].iter().filter(|&&p| p >= self.sp.len).count()
    }
    fn prog(&self, s: usize) -> i32 {
        self.pos[s].iter().sum()
    }
}

impl Mini for Race {
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
        if self.die.is_none() {
            return vec!["arunc".into()];
        }
        let mut v = Vec::new();
        for k in 0..self.sp.toks {
            if self.target(k).is_some() {
                v.push(format!("muta {}", k + 1));
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.side;
        self.plies += 1;
        if m == "arunc" {
            if self.skip[me] > 0 {
                self.skip[me] -= 1;
                self.side = 1 - me;
                return;
            }
            self.die = Some(self.dice.below(self.sp.sides as usize) as u8 + 1);
            return;
        }
        let mut extra = false;
        if m == "pas" {
            self.die = None;
            self.side = 1 - me;
            return;
        }
        let k = m[5..].parse::<usize>().unwrap() - 1;
        let mut t = self.target(k).unwrap();
        if self.sp.enter > 0 && self.die == Some(self.sp.enter) {
            extra = true;
        }
        if let Some(&(_, sq)) = self.sp.sq.iter().find(|(p, _)| *p == t) {
            match sq {
                Sq::Fwd(n) => t = (t + n).min(self.sp.len),
                Sq::Back(n) => t = (t - n).max(1),
                Sq::Extra => extra = true,
                Sq::Skip => self.skip[me] = 1,
            }
        }
        if self.sp.capture && t > 0 && t < self.sp.len {
            for q in self.pos[1 - me].iter_mut() {
                if *q == t {
                    *q = 0;
                }
            }
        }
        self.pos[me][k] = t;
        self.die = None;
        if !extra || self.fin(me) >= self.sp.need {
            self.side = 1 - me;
        }
    }
    fn result(&self) -> Option<Outcome> {
        for s in 0..2 {
            if self.fin(s) >= self.sp.need {
                return Some(if s == 0 {
                    Outcome::Win(Player::One)
                } else {
                    Outcome::Win(Player::Two)
                });
            }
        }
        if self.plies >= self.sp.cap {
            return Some(by_score(self.prog(0), self.prog(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.fin(a) as i32 - self.fin(1 - a) as i32) + 10 * (self.prog(a) - self.prog(1 - a))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — traseu 0..{} (0 = start, {} = sosire)\n",
            self.sp.title, self.sp.len, self.sp.len
        );
        for p in 0..2 {
            let t: Vec<String> = self.pos[p]
                .iter()
                .enumerate()
                .map(|(k, x)| format!("{}:{}", k + 1, x))
                .collect();
            s.push_str(&format!(
                "{}: {} (ajunse {}/{})\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                t.join(" "),
                self.fin(p),
                self.sp.need
            ));
        }
        let sp: Vec<String> = self
            .sp
            .sq
            .iter()
            .map(|(p, q)| {
                let n = match q {
                    Sq::Fwd(n) => format!("+{n}"),
                    Sq::Back(n) => format!("-{n}"),
                    Sq::Extra => "încă o tură".to_string(),
                    Sq::Skip => "pierzi tura".to_string(),
                };
                format!("{p}:{n}")
            })
            .collect();
        s.push_str(&format!("Pătrate speciale: {}\n", sp.join(", ")));
        if let Some(d) = self.die {
            s.push_str(&format!("Zar: {d}\n"));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | muta k | pas)\n",
            pname(self.side())
        ));
        s
    }
}
