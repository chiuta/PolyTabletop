//! Motor „făurari de zaruri” (lista C5, 2 jucători): fiecare are două zaruri cu fețe de recompensă
//! (aur, soare, lună, glorie). La începutul rundei mutarea forțată `binecuvantare` aruncă toate
//! zarurile; apoi fiecare, pe rând, are până la `acts` acțiuni: `forja i d f` (cumperi fața i din
//! sanctuar contra aur și o pui peste fața f a zarului d), `cartea c` (plătești soare/lună pentru
//! glorie) sau `pas`. Câștigă cine are mai multă glorie după `rounds` runde.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

pub struct ForgeSpec {
    pub title: &'static str,
    pub rounds: u32,
    pub acts: u32,
    pub gold0: i32,
    /// (cost aur, fel 0=aur 1=soare 2=lună 3=glorie, valoare, stoc)
    pub items: &'static [(i32, u8, i32, i32)],
    /// (soare, lună, glorie)
    pub cards: &'static [(i32, i32, i32)],
    pub dice0: [[(u8, i32); 6]; 2],
}

const KN: [&str; 4] = ["aur", "soare", "lună", "glorie"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Bless,
    Act,
}

#[derive(Clone)]
pub struct Forge {
    sp: &'static ForgeSpec,
    faces: [[[(u8, i32); 6]; 2]; 2],
    res: [[i32; 4]; 2],
    stock: Vec<i32>,
    card_left: Vec<bool>,
    ph: Ph,
    acts_left: u32,
    first: usize,
    side: usize,
    done_first: bool,
    round: u32,
    dice: Dice,
}

impl Forge {
    pub fn new(sp: &'static ForgeSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static ForgeSpec, seed: u64) -> Self {
        Forge {
            sp,
            faces: [sp.dice0, sp.dice0],
            res: [[sp.gold0, 0, 0, 0]; 2],
            stock: sp.items.iter().map(|x| x.3).collect(),
            card_left: vec![true; sp.cards.len()],
            ph: Ph::Bless,
            acts_left: sp.acts,
            first: 0,
            side: 0,
            done_first: false,
            round: 0,
            dice: Dice::new(seed),
        }
    }
    fn gain(&mut self, p: usize, k: u8, n: i32) {
        let cap = [14, 6, 6, 999][k as usize];
        let r = &mut self.res[p][k as usize];
        *r = (*r + n).min(cap);
    }
    fn fval(f: (u8, i32)) -> i32 {
        f.1 * [60, 120, 120, 100][f.0 as usize]
    }
    fn dice_val(&self, p: usize) -> i32 {
        (0..2)
            .map(|d| self.faces[p][d].iter().map(|&f| Self::fval(f)).sum::<i32>() / 6)
            .sum()
    }
}

impl Mini for Forge {
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
        if self.ph == Ph::Bless {
            return vec!["binecuvantare".into()];
        }
        let me = self.side;
        let mut v = vec!["pas".to_string()];
        for (i, it) in self.sp.items.iter().enumerate() {
            if self.stock[i] > 0 && self.res[me][0] >= it.0 {
                for d in 0..2 {
                    for f in 0..6 {
                        if self.faces[me][d][f] != (it.1, it.2) {
                            v.push(format!("forja {} {} {}", i + 1, d + 1, f + 1));
                        }
                    }
                }
            }
        }
        for (c, cd) in self.sp.cards.iter().enumerate() {
            if self.card_left[c] && self.res[me][1] >= cd.0 && self.res[me][2] >= cd.1 {
                v.push(format!("cartea {}", c + 1));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Bless => {
                for p in 0..2 {
                    for d in 0..2 {
                        let f = self.faces[p][d][self.dice.below(6)];
                        self.gain(p, f.0, f.1);
                    }
                }
                self.ph = Ph::Act;
                self.side = self.first;
                self.acts_left = self.sp.acts;
                self.done_first = false;
            }
            Ph::Act => {
                let me = self.side;
                let mut pass = m == "pas";
                if let Some(rest) = m.strip_prefix("forja ") {
                    let mut it = rest.split(' ').map(|x| x.parse::<usize>().unwrap());
                    let (i, d, f) = (
                        it.next().unwrap() - 1,
                        it.next().unwrap() - 1,
                        it.next().unwrap() - 1,
                    );
                    let item = self.sp.items[i];
                    self.res[me][0] -= item.0;
                    self.stock[i] -= 1;
                    self.faces[me][d][f] = (item.1, item.2);
                } else if let Some(rest) = m.strip_prefix("cartea ") {
                    let c = rest.parse::<usize>().unwrap() - 1;
                    let cd = self.sp.cards[c];
                    self.res[me][1] -= cd.0;
                    self.res[me][2] -= cd.1;
                    self.res[me][3] += cd.2;
                    self.card_left[c] = false;
                } else {
                    pass = true;
                }
                if !pass {
                    self.acts_left -= 1;
                }
                if pass || self.acts_left == 0 {
                    if self.done_first {
                        self.round += 1;
                        self.first = 1 - self.first;
                        self.ph = Ph::Bless;
                        self.side = self.first;
                    } else {
                        self.done_first = true;
                        self.side = 1 - self.first;
                        self.acts_left = self.sp.acts;
                    }
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.sp.rounds {
            return Some(by_score(self.res[0][3], self.res[1][3]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let left = (self.sp.rounds - self.round) as i32;
        100 * (self.res[a][3] - self.res[o][3])
            + (5 * left).min(40) * (self.res[a][0] - self.res[o][0])
            + 90 * (self.res[a][1] + self.res[a][2] - self.res[o][1] - self.res[o][2])
            + left * (self.dice_val(a) - self.dice_val(o))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds
        );
        for p in 0..2 {
            let r = &self.res[p];
            s.push_str(&format!(
                "{}: {} {}, {} {}, {} {}, {} {}\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                KN[0],
                r[0],
                KN[1],
                r[1],
                KN[2],
                r[2],
                KN[3],
                r[3]
            ));
            for d in 0..2 {
                let t: Vec<String> = self.faces[p][d]
                    .iter()
                    .map(|f| format!("{}{}", &KN[f.0 as usize][..1], f.1))
                    .collect();
                s.push_str(&format!("   zar {}: {}\n", d + 1, t.join(" ")));
            }
        }
        s.push_str("Sanctuar:");
        for (i, it) in self.sp.items.iter().enumerate() {
            s.push_str(&format!(
                " {}) {}×{} cost {} stoc {};",
                i + 1,
                KN[it.1 as usize],
                it.2,
                it.0,
                self.stock[i]
            ));
        }
        s.push_str("\nCărți:");
        for (c, cd) in self.sp.cards.iter().enumerate() {
            if self.card_left[c] {
                s.push_str(&format!(
                    " {}) {}s+{}l → {} glorie;",
                    c + 1,
                    cd.0,
                    cd.1,
                    cd.2
                ));
            }
        }
        s.push_str(&format!(
            "\nLa mutare: {} (binecuvantare | forja i d f | cartea c | pas)\n",
            pname(self.side())
        ));
        s
    }
}
