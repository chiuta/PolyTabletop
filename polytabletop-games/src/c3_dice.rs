//! Motor cooperativ „sarcini cu zaruri” (lista C3, variante originale, 2 jucători în echipă).
//!
//! Trei sarcini deschise; fiecare cere un set de valori minime (ex. 3+4+5 = trei zaruri diferite
//! cu cel puțin 3, 4 și 5). La începutul turei: aruncarea forțată `arunc` (și umplerea sarcinilor
//! lipsă din pachet). Apoi: `inch t` (încheie sarcina t dacă zarurile ajung; zarurile folosite se
//! consumă, restul pot servi altei sarcini), `refa t` (repetă aruncarea zarurilor care nu ajută
//! sarcina t; costă o repetare; rezultatul vine prin mutarea forțată `zar`), `sfarsit` (tura se
//! termină, ceasul/ameninţarea crește cu 1). Echipa câștigă cu `goal` sarcini, pierde la ameninţare
//! maximă (remiză: nu există adversar).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct DSpec {
    pub title: &'static str,
    pub dice: usize,
    pub rerolls: u8,
    pub goal: u32,
    pub doom: u32,
    pub tasks: &'static [&'static [u8]],
    /// sarcinile cu indicele din listă >= acest prag reduc ameninţarea cu 1 când sunt încheiate
    pub bonus_from: usize,
    pub unit: &'static str,
}

pub static SPECS: [DSpec; 2] = [
    DSpec {
        title: "Sigiliul Bătrân",
        dice: 6,
        rerolls: 2,
        goal: 6,
        doom: 14,
        tasks: &[
            &[3, 3],
            &[4, 2],
            &[5, 1],
            &[2, 2, 2],
            &[3, 4, 2],
            &[6, 3],
            &[4, 4, 1],
            &[5, 5],
            &[3, 3, 3, 1],
            &[6, 4, 2],
            &[1, 1, 1, 1],
            &[5, 4, 3],
        ],
        bonus_from: 7,
        unit: "ceasul de miezul nopții",
    },
    DSpec {
        title: "Ceasul Templului",
        dice: 5,
        rerolls: 3,
        goal: 5,
        doom: 11,
        tasks: &[
            &[2, 2],
            &[3, 3, 1],
            &[4, 4],
            &[5, 2],
            &[6],
            &[3, 4, 5],
            &[2, 2, 2, 2],
            &[6, 5],
            &[4, 3, 3],
            &[1, 1, 1],
        ],
        bonus_from: 8,
        unit: "nisipul din clepsidră",
    },
];

#[derive(Clone)]
pub struct Dice {
    sp: &'static DSpec,
    deck: Vec<u8>,
    slots: [Option<u8>; 3],
    dice: Vec<u8>,
    used: Vec<bool>,
    left: u8,
    pending_roll: u8, // 0 nimic, 1 = aruncare completă, 2 = repetare (zarurile neţinute)
    keep: Vec<bool>,
    done: u32,
    doom: u32,
    side: usize,
    rng: Rng,
}

pub fn new(i: usize) -> Dice {
    with_seed(i, seed_now())
}

pub fn with_seed(i: usize, seed: u64) -> Dice {
    let sp = &SPECS[i];
    let mut rng = Rng::new(seed);
    let mut deck: Vec<u8> = (0..sp.tasks.len() as u8).collect();
    shuf(&mut deck, &mut rng);
    Dice {
        sp,
        deck,
        slots: [None; 3],
        dice: vec![1; sp.dice],
        used: vec![false; sp.dice],
        left: sp.rerolls,
        pending_roll: 1,
        keep: vec![false; sp.dice],
        done: 0,
        doom: 0,
        side: 0,
        rng,
    }
}

impl Dice {
    /// Atribuie, cât se poate, zarurile libere la cerințele sarcinii; întoarce indicii folosiți.
    fn assign(&self, t: usize) -> Option<Vec<usize>> {
        let mut need: Vec<u8> = self.sp.tasks[t].to_vec();
        need.sort_unstable_by(|a, b| b.cmp(a));
        let mut free: Vec<usize> = (0..self.dice.len()).filter(|&i| !self.used[i]).collect();
        free.sort_by_key(|&i| self.dice[i]);
        let mut pick = Vec::new();
        for n in need {
            let pos = free.iter().position(|&i| self.dice[i] >= n)?;
            pick.push(free.remove(pos));
        }
        Some(pick)
    }
    fn matched(&self, t: usize) -> Vec<usize> {
        let mut need: Vec<u8> = self.sp.tasks[t].to_vec();
        need.sort_unstable_by(|a, b| b.cmp(a));
        let mut free: Vec<usize> = (0..self.dice.len()).filter(|&i| !self.used[i]).collect();
        free.sort_by_key(|&i| self.dice[i]);
        let mut pick = Vec::new();
        for n in need {
            if let Some(pos) = free.iter().position(|&i| self.dice[i] >= n) {
                pick.push(free.remove(pos));
            }
        }
        pick
    }
}

impl Mini for Dice {
    fn side(&self) -> Player {
        pl2(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.pending_roll {
            1 => return vec!["arunc".into()],
            2 => return vec!["zar".into()],
            _ => {}
        }
        let mut v = Vec::new();
        for (k, s) in self.slots.iter().enumerate() {
            if let Some(t) = s {
                if self.assign(*t as usize).is_some() {
                    v.push(format!("inch {}", k + 1));
                }
                if self.left > 0 {
                    v.push(format!("refa {}", k + 1));
                }
            }
        }
        v.push("sfarsit".into());
        v
    }
    fn step(&mut self, m: &str) {
        match self.pending_roll {
            1 => {
                for k in 0..3 {
                    if self.slots[k].is_none() {
                        self.slots[k] = self.deck.pop();
                        if self.slots[k].is_none() {
                            // pachet epuizat: se amestecă din nou sarcinile
                            self.deck = (0..self.sp.tasks.len() as u8).collect();
                            shuf(&mut self.deck, &mut self.rng);
                            self.slots[k] = self.deck.pop();
                        }
                    }
                }
                for i in 0..self.dice.len() {
                    self.dice[i] = 1 + self.rng.below(6) as u8;
                    self.used[i] = false;
                }
                self.left = self.sp.rerolls;
                self.pending_roll = 0;
                return;
            }
            2 => {
                for i in 0..self.dice.len() {
                    if !self.keep[i] && !self.used[i] {
                        self.dice[i] = 1 + self.rng.below(6) as u8;
                    }
                }
                self.pending_roll = 0;
                return;
            }
            _ => {}
        }
        if let Some(k) = m.strip_prefix("inch ") {
            let k = k.parse::<usize>().unwrap() - 1;
            let t = self.slots[k].unwrap() as usize;
            for i in self.assign(t).unwrap() {
                self.used[i] = true;
            }
            self.slots[k] = None;
            self.done += 1;
            if t >= self.sp.bonus_from {
                self.doom = self.doom.saturating_sub(1);
            }
        } else if let Some(k) = m.strip_prefix("refa ") {
            let k = k.parse::<usize>().unwrap() - 1;
            let t = self.slots[k].unwrap() as usize;
            self.keep = vec![false; self.dice.len()];
            for i in self.matched(t) {
                self.keep[i] = true;
            }
            self.left -= 1;
            self.pending_roll = 2;
        } else {
            self.doom += 1;
            self.side = 1 - self.side;
            self.pending_roll = 1;
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.done >= self.sp.goal {
            Some(Outcome::Win(Player::One))
        } else if self.doom >= self.sp.doom {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn eval(&self, _p: Player) -> i32 {
        let mut e = self.done as i32 * 400 - self.doom as i32 * 30 + self.left as i32 * 5;
        if self.pending_roll == 0 {
            // cât din cea mai apropiată sarcină e acoperit acum (zarurile sunt publice)
            let best = self
                .slots
                .iter()
                .flatten()
                .map(|&t| {
                    self.matched(t as usize).len() as i32 * 6
                        - self.sp.tasks[t as usize].len() as i32 * 6
                })
                .max()
                .unwrap_or(0);
            e += best;
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — sarcini {}/{} | {} {}/{}\n",
            self.sp.title, self.done, self.sp.goal, self.sp.unit, self.doom, self.sp.doom
        );
        for (k, t) in self.slots.iter().enumerate() {
            match t {
                Some(t) => s.push_str(&format!(
                    "Sarcina {}: cere {:?}{}\n",
                    k + 1,
                    self.sp.tasks[*t as usize],
                    if (*t as usize) >= self.sp.bonus_from {
                        " (răsplată: ameninţarea scade)"
                    } else {
                        ""
                    }
                )),
                None => s.push_str(&format!("Sarcina {}: (se umple la aruncare)\n", k + 1)),
            }
        }
        let d: Vec<String> = (0..self.dice.len())
            .map(|i| {
                if self.used[i] {
                    format!("({})", self.dice[i])
                } else {
                    self.dice[i].to_string()
                }
            })
            .collect();
        s.push_str(&format!(
            "Zaruri: {} | repetări rămase: {}\nLa mutare: {} (inch t | refa t | sfarsit)\n",
            d.join(" "),
            self.left,
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

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        check(|s| with_seed(0, s), 3000);
        check(|s| with_seed(1, s), 3000);
    }
}
