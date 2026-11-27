//! Motor „cursa țestoaselor” (lista C5, 2 jucători): cinci țestoase, fiecare jucător își alege
//! public două (draft), apoi la fiecare tură încurajezi o culoare (`incurajez t`) și o mutare
//! forțată (`trag`) trage o culoare la întâmplare: aceea avansează 1, iar dacă a fost cea încurajată,
//! avansează cu `bonus`. Cine are țestoasa care trece prima linia de sosire câștigă; dacă trece
//! cea neluată de nimeni, partida e remiză. Informație completă.
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

pub struct BSpec {
    pub title: &'static str,
    pub len: i32,
    pub bonus: i32,
    pub cap: u32,
    pub names: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
enum Ph {
    Draft,
    Cheer,
    Draw(usize),
}

#[derive(Clone)]
pub struct Bet {
    sp: &'static BSpec,
    pos: Vec<i32>,
    owner: Vec<Option<usize>>,
    ph: Ph,
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Bet {
    pub fn new(sp: &'static BSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static BSpec, seed: u64) -> Self {
        let n = sp.names.len();
        Bet {
            sp,
            pos: vec![0; n],
            owner: vec![None; n],
            ph: Ph::Draft,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn mine(&self, s: usize) -> usize {
        self.owner.iter().filter(|o| **o == Some(s)).count()
    }
    fn total(&self, s: usize) -> i32 {
        (0..self.pos.len())
            .filter(|&t| self.owner[t] == Some(s))
            .map(|t| self.pos[t])
            .sum()
    }
}

impl Mini for Bet {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl2(self.side)
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Draft => (0..self.pos.len())
                .filter(|&t| self.owner[t].is_none())
                .map(|t| format!("alege {}", t + 1))
                .collect(),
            Ph::Cheer => (0..self.pos.len())
                .map(|t| format!("incurajez {}", t + 1))
                .collect(),
            Ph::Draw(_) => vec!["trag".into()],
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let me = self.side;
        match self.ph {
            Ph::Draft => {
                let t = m[6..].parse::<usize>().unwrap() - 1;
                self.owner[t] = Some(me);
                self.side = 1 - me;
                if self.mine(0) == 2 && self.mine(1) == 2 {
                    self.ph = Ph::Cheer;
                    self.side = 0;
                }
            }
            Ph::Cheer => {
                let t = m[10..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Draw(t);
            }
            Ph::Draw(c) => {
                let t = self.dice.below(self.pos.len());
                self.pos[t] += 1 + if t == c { self.sp.bonus } else { 0 };
                self.ph = Ph::Cheer;
                self.side = 1 - me;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        let mut best = (0, 0usize);
        for t in 0..self.pos.len() {
            if self.pos[t] > best.0 {
                best = (self.pos[t], t);
            }
        }
        if best.0 >= self.sp.len {
            return Some(match self.owner[best.1] {
                Some(0) => Outcome::Win(Player::One),
                Some(_) => Outcome::Win(Player::Two),
                None => Outcome::Draw,
            });
        }
        if self.plies >= self.sp.cap {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 10 * (self.total(a) - self.total(1 - a));
        if let Ph::Draw(c) = self.ph {
            if let Some(o) = self.owner[c] {
                let w = 2 * self.sp.bonus * 10 / self.pos.len() as i32;
                v += if o == a { w } else { -w };
            }
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{} — sosire la {}\n", self.sp.title, self.sp.len);
        for t in 0..self.pos.len() {
            s.push_str(&format!(
                "{}. {} ({}): {}\n",
                t + 1,
                self.sp.names[t],
                match self.owner[t] {
                    Some(0) => "J1",
                    Some(_) => "J2",
                    None => "liberă",
                },
                self.pos[t]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (alege t | incurajez t | trag)\n",
            pname(self.side())
        ));
        s
    }
}

fn pl2(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}
