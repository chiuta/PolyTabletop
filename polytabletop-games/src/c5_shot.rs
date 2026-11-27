//! Motor „încercări cu risc” (lista C5, 2 jucători): jocurile de îndemânare pentru copii, traduse în
//! șansă controlată. Alegi o încercare (`tir a`: fiecare are o șansă publică de reușită și un câștig),
//! apoi mutarea forțată `lansez` o rezolvă. Serie de reușite = bonus de precizie; în modul `risk` o
//! ratare îți ia punctele nesalvate ale turei (`stau` le salvează); în modul `collect` câștigul e un
//! obiect dintr-un fel și câștigă cine strânge primul `goal` din fiecare fel.
//! Zarul e re-însămânțat la `clone()` (AI-ul nu vede rezultatul viitor).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

pub struct Act {
    pub label: &'static str,
    /// Șansa de reușită, în procente.
    pub p: i32,
    pub gain: i32,
    /// Penalizare la ratare (se adaugă la scor, ≤ 0).
    pub fail: i32,
    /// Modul `collect`: felul obținut (−1 = la întâmplare).
    pub item: i8,
}

pub struct ShotSpec {
    pub title: &'static str,
    pub acts: &'static [Act],
    pub shots: u32,
    pub goal: i32,
    /// 0 = puncte; altfel numărul de feluri de obiecte de strâns (câte `goal` din fiecare).
    pub collect: usize,
    pub risk: bool,
    /// Bonus de precizie (procente) pe reușită consecutivă în aceeași tură.
    pub streak: i32,
    pub fail_ends: bool,
    pub cap: u32,
    pub item_names: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
enum Ph {
    Choose,
    Fire(usize),
}

#[derive(Clone)]
pub struct Shot {
    sp: &'static ShotSpec,
    score: [i32; 2],
    items: [Vec<i32>; 2],
    tp: i32,
    streak: i32,
    left: u32,
    ph: Ph,
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Shot {
    pub fn new(sp: &'static ShotSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static ShotSpec, seed: u64) -> Self {
        Shot {
            sp,
            score: [0; 2],
            items: [vec![0; sp.collect], vec![0; sp.collect]],
            tp: 0,
            streak: 0,
            left: sp.shots,
            ph: Ph::Choose,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn p_eff(&self, a: usize) -> i32 {
        (self.sp.acts[a].p + self.streak * self.sp.streak).clamp(5, 95)
    }
    fn end_turn(&mut self) {
        self.side = 1 - self.side;
        self.left = self.sp.shots;
        self.tp = 0;
        self.streak = 0;
        self.ph = Ph::Choose;
    }
    fn total(&self, s: usize) -> i32 {
        if self.sp.collect == 0 {
            self.score[s]
        } else {
            self.items[s].iter().map(|&x| x.min(self.sp.goal)).sum()
        }
    }
    /// Valoarea (în sutimi) a unui obiect de felul `k` pentru jucătorul `s`.
    fn item_val(&self, s: usize, k: i8) -> i32 {
        if k >= 0 {
            return if self.items[s][k as usize] < self.sp.goal {
                100
            } else {
                10
            };
        }
        let n = self.sp.collect as i32;
        (0..self.sp.collect)
            .map(|j| {
                if self.items[s][j] < self.sp.goal {
                    100
                } else {
                    10
                }
            })
            .sum::<i32>()
            / n.max(1)
    }
}

impl Mini for Shot {
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
            Ph::Fire(_) => vec!["lansez".into()],
            Ph::Choose => {
                let mut v: Vec<String> = (0..self.sp.acts.len())
                    .map(|a| format!("tir {}", a + 1))
                    .collect();
                if self.left < self.sp.shots {
                    v.push("stau".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let me = self.side;
        match self.ph {
            Ph::Choose => {
                if m == "stau" {
                    self.score[me] += self.tp;
                    self.end_turn();
                } else {
                    self.ph = Ph::Fire(m[4..].parse::<usize>().unwrap() - 1);
                }
            }
            Ph::Fire(a) => {
                let ok = (self.dice.below(100) as i32) < self.p_eff(a);
                let act = &self.sp.acts[a];
                self.left -= 1;
                self.ph = Ph::Choose;
                if ok {
                    self.streak += 1;
                    if self.sp.collect > 0 {
                        let k = if act.item >= 0 {
                            act.item as usize
                        } else {
                            self.dice.below(self.sp.collect)
                        };
                        self.items[me][k] += act.gain.max(1);
                    } else if self.sp.risk {
                        self.tp += act.gain;
                    } else {
                        self.score[me] += act.gain;
                    }
                } else {
                    self.streak = 0;
                    if self.sp.risk {
                        self.tp = 0;
                    }
                    self.score[me] = (self.score[me] + act.fail).max(0);
                    if self.sp.fail_ends || self.sp.risk {
                        self.end_turn();
                        return;
                    }
                }
                if self.left == 0 {
                    self.score[me] += self.tp;
                    self.end_turn();
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph != Ph::Choose {
            return None;
        }
        let done = |s: usize| {
            if self.sp.collect == 0 {
                self.score[s] >= self.sp.goal
            } else {
                self.items[s].iter().all(|&x| x >= self.sp.goal)
            }
        };
        match (done(0), done(1)) {
            (true, false) => return Some(Outcome::Win(Player::One)),
            (false, true) => return Some(Outcome::Win(Player::Two)),
            (true, true) => return Some(by_score(self.total(0), self.total(1))),
            _ => {}
        }
        if self.plies >= self.sp.cap {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 100 * (self.total(a) - self.total(1 - a));
        let sgn = if self.side == a { 1 } else { -1 };
        match self.ph {
            Ph::Fire(x) => {
                let act = &self.sp.acts[x];
                let pe = self.p_eff(x);
                let gain = if self.sp.collect > 0 {
                    self.item_val(self.side, act.item) * act.gain.max(1)
                } else {
                    100 * act.gain
                };
                let loss = if self.sp.risk {
                    100 * self.tp - 100 * act.fail
                } else {
                    -100 * act.fail
                };
                base + sgn * ((pe * gain) / 100 - ((100 - pe) * loss) / 100)
            }
            Ph::Choose => base + sgn * 100 * if self.sp.collect == 0 { self.tp } else { 0 },
        }
    }
    fn show(&self) -> String {
        let mut s = format!("{} — țintă {}\n", self.sp.title, self.sp.goal);
        for p in 0..2 {
            let it: Vec<String> = self.items[p]
                .iter()
                .enumerate()
                .map(|(k, x)| format!("{} {}", self.sp.item_names.get(k).unwrap_or(&"?"), x))
                .collect();
            s.push_str(&format!(
                "{}: scor {}{}\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                self.score[p],
                if it.is_empty() {
                    String::new()
                } else {
                    format!(" | {}", it.join(", "))
                }
            ));
        }
        for (i, a) in self.sp.acts.iter().enumerate() {
            s.push_str(&format!(
                "  {}. {} — șansă {}%, câștig {}, ratare {}\n",
                i + 1,
                a.label,
                self.p_eff(i),
                a.gain,
                a.fail
            ));
        }
        s.push_str(&format!(
            "Încercări rămase: {} | nesalvat: {} | serie: {}\nLa mutare: {} (tir a | lansez | stau)\n",
            self.left,
            self.tp,
            self.streak,
            pname(self.side())
        ));
        s
    }
}
