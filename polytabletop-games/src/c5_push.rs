//! Motor „zaruri cu risc” (lista C5, 2 jucători), trei familii:
//! * `Pig`: un singur zar; „continu” apoi aruncarea forțată `arunc`; fața-ghinion îți ia punctele
//!   turei, `opresc` le salvează; fața „obligatorie” te forțează să mai arunci o dată.
//! * `Cup`: un păhar cu zaruri de trei feluri; tragi trei, le arunci; fructe (câștig), mușcături
//!   (trei = pierzi tura) și fugi (zarul se aruncă din nou). Primul la `goal` fructe câștigă.
//! * `Chips`: jetoane și zaruri cu fețe „dau”, „potul”, „iau”, „gol”; fără decizii (doar `arunc`).
//!
//! Zarul e re-însămânțat la `clone()`: AI-ul nu vede aruncările viitoare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PMode {
    Pig,
    Cup,
    Chips,
}

pub struct PushSpec {
    pub title: &'static str,
    pub mode: PMode,
    pub goal: i32,
    /// Pig: fața-ghinion. Chips: jetoane de pornire.
    pub a: i32,
    /// Pig: fața care obligă la o nouă aruncare (0 = nu există).
    pub b: i32,
    pub cap: u32,
}

// Cup: fețe 0 = fruct, 1 = mușcătură, 2 = fugă.
const DIE: [[u8; 6]; 3] = [[0, 0, 0, 2, 2, 1], [0, 0, 2, 2, 1, 1], [0, 2, 2, 1, 1, 1]];
const CUP0: [u8; 3] = [6, 4, 3];

#[derive(Clone)]
pub struct Push {
    sp: &'static PushSpec,
    score: [i32; 2],
    tp: i32,
    forced: bool,
    rolling: bool,
    // Cup
    cup: [u8; 3],
    aside: [u8; 3],
    hand: Vec<u8>,
    hits: i32,
    chips: [i32; 2],
    pot: i32,
    last: Vec<u8>,
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Push {
    pub fn new(sp: &'static PushSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static PushSpec, seed: u64) -> Self {
        Push {
            sp,
            score: [0; 2],
            tp: 0,
            forced: false,
            rolling: false,
            cup: CUP0,
            aside: [0; 3],
            hand: Vec::new(),
            hits: 0,
            chips: [sp.a.max(1); 2],
            pot: 0,
            last: Vec::new(),
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn new_turn(&mut self) {
        self.side = 1 - self.side;
        self.tp = 0;
        self.forced = false;
        self.rolling = false;
        self.cup = CUP0;
        self.aside = [0; 3];
        self.hand.clear();
        self.hits = 0;
        self.last.clear();
    }
    fn pdie_k(&self) -> i32 {
        // șansa de a atinge 3 mușcături având `hits` deja (3 zaruri, 1/3 fiecare)
        match self.hits {
            0 => 4,
            1 => 26,
            _ => 70,
        }
    }
}

impl Mini for Push {
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
        if self.sp.mode == PMode::Chips || self.rolling {
            return vec!["arunc".into()];
        }
        if self.forced {
            return vec!["continu".into()];
        }
        let mut v = vec!["continu".to_string()];
        if self.tp > 0 {
            v.push("opresc".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let me = self.side;
        match self.sp.mode {
            PMode::Pig => {
                if m == "continu" {
                    self.rolling = true;
                } else if m == "opresc" {
                    self.score[me] += self.tp;
                    self.new_turn();
                } else {
                    self.rolling = false;
                    let d = self.dice.below(6) as i32 + 1;
                    self.last = vec![d as u8];
                    if d == self.sp.a {
                        self.new_turn();
                    } else {
                        self.tp += d;
                        self.forced = self.sp.b > 0 && d == self.sp.b;
                    }
                }
            }
            PMode::Cup => {
                if m == "continu" {
                    self.rolling = true;
                } else if m == "opresc" {
                    self.score[me] += self.tp;
                    self.new_turn();
                } else {
                    self.rolling = false;
                    // completează mâna la 3 zaruri din pahar (reumple din cele câștigate)
                    while self.hand.len() < 3 {
                        let tot: u8 = self.cup.iter().sum();
                        if tot == 0 {
                            if self.aside.iter().sum::<u8>() == 0 {
                                break;
                            }
                            self.cup = self.aside;
                            self.aside = [0; 3];
                            continue;
                        }
                        let mut r = self.dice.below(tot as usize) as u8;
                        for c in 0..3 {
                            if r < self.cup[c] {
                                self.cup[c] -= 1;
                                self.hand.push(c as u8);
                                break;
                            }
                            r -= self.cup[c];
                        }
                    }
                    let mut keep = Vec::new();
                    let mut res = Vec::new();
                    for &c in &self.hand.clone() {
                        let f = DIE[c as usize][self.dice.below(6)];
                        res.push(f);
                        match f {
                            0 => {
                                self.tp += 1;
                                self.aside[c as usize] += 1;
                            }
                            1 => self.hits += 1,
                            _ => keep.push(c),
                        }
                    }
                    self.last = res;
                    self.hand = keep;
                    self.forced = false;
                    if self.hits >= 3 {
                        self.new_turn();
                    }
                }
            }
            PMode::Chips => {
                let n = self.chips[me].min(3);
                let mut res = Vec::new();
                for _ in 0..n {
                    let f = self.dice.below(6) as u8;
                    res.push(f);
                    match f {
                        0 => {
                            self.chips[me] -= 1;
                            self.chips[1 - me] += 1;
                        }
                        1 => {
                            self.chips[me] -= 1;
                            self.pot += 1;
                        }
                        2 if self.chips[1 - me] > 0 => {
                            self.chips[1 - me] -= 1;
                            self.chips[me] += 1;
                        }
                        _ => {}
                    }
                }
                self.last = res;
                self.side = 1 - me;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        match self.sp.mode {
            PMode::Chips => {
                if self.chips[0] == 0 {
                    return Some(Outcome::Win(Player::Two));
                }
                if self.chips[1] == 0 {
                    return Some(Outcome::Win(Player::One));
                }
                if self.plies >= self.sp.cap {
                    return Some(by_score(self.chips[0], self.chips[1]));
                }
                None
            }
            _ => {
                for s in 0..2 {
                    if self.score[s] >= self.sp.goal {
                        return Some(if s == 0 {
                            Outcome::Win(Player::One)
                        } else {
                            Outcome::Win(Player::Two)
                        });
                    }
                }
                if self.plies >= self.sp.cap {
                    return Some(by_score(self.score[0], self.score[1]));
                }
                None
            }
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let sgn = if self.side == a { 1 } else { -1 };
        match self.sp.mode {
            PMode::Chips => 100 * (self.chips[a] - self.chips[1 - a]),
            PMode::Pig => {
                let base = 600 * (self.score[a] - self.score[1 - a]);
                if self.rolling {
                    // continuare față de salvare, în șesimi de punct: 5 fețe bune din 6
                    let faces: i32 = 21 - self.sp.a;
                    base + sgn * 100 * (5 * self.tp + faces)
                } else {
                    base + sgn * 600 * self.tp
                }
            }
            PMode::Cup => {
                let base = 600 * (self.score[a] - self.score[1 - a]);
                if self.rolling {
                    let pd = self.pdie_k();
                    let cont = (100 - pd) * (self.tp * 100 + 110) / 100;
                    base + sgn * 6 * cont
                } else {
                    base + sgn * 600 * self.tp
                }
            }
        }
    }
    fn show(&self) -> String {
        let mut s = format!("{} — țintă {}\n", self.sp.title, self.sp.goal);
        match self.sp.mode {
            PMode::Chips => {
                s.push_str(&format!(
                    "Jetoane: J1 {}, J2 {}, pot {}\n",
                    self.chips[0], self.chips[1], self.pot
                ));
                if !self.last.is_empty() {
                    let n: Vec<&str> = self
                        .last
                        .iter()
                        .map(|f| ["dau", "pot", "iau", "gol", "gol", "gol"][*f as usize])
                        .collect();
                    s.push_str(&format!("Ultima aruncare: {}\n", n.join(", ")));
                }
            }
            _ => {
                s.push_str(&format!(
                    "Scor: J1 {}, J2 {} | în tură: {}",
                    self.score[0], self.score[1], self.tp
                ));
                if self.sp.mode == PMode::Cup {
                    s.push_str(&format!(
                        " | mușcături {} | în pahar {} | în mână {}",
                        self.hits,
                        self.cup.iter().map(|&x| x as i32).sum::<i32>(),
                        self.hand.len()
                    ));
                }
                s.push('\n');
                if !self.last.is_empty() {
                    let n: Vec<String> = self
                        .last
                        .iter()
                        .map(|f| {
                            if self.sp.mode == PMode::Cup {
                                ["fruct", "mușcătură", "fugă"][*f as usize].to_string()
                            } else {
                                f.to_string()
                            }
                        })
                        .collect();
                    s.push_str(&format!("Ultima aruncare: {}\n", n.join(", ")));
                }
            }
        }
        s.push_str(&format!(
            "La mutare: {} (continu | arunc | opresc)\n",
            pname(self.side())
        ));
        s
    }
}
