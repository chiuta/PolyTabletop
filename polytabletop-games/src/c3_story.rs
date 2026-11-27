//! Motor cooperativ „scenarii cu resurse” (lista C3, variante originale, 2 jucători în echipă).
//!
//! Un șir de scene deschise (le vezi pe toate cele care urmează). Echipa are 3 feluri de jetoane,
//! timp și moral. Mutări: `fac` (înfrunți scena curentă: plătești costul, primești câștigul,
//! timpul scade), `ocol` (sari peste scenă: -1 moral, -1 timp), `odih t` (+1 jeton de tipul t, -1 timp).
//! După fiecare acțiune urmează faza forțată `intamplare`: cu o șansă fixă se întâmplă ceva
//! (pierzi un jeton, timp sau moral ori primești un ajutor). Câștigi cu `goal` scene încheiate.
//! Pierzi (remiză, nu există adversar) dacă timpul sau moralul se termină sau scenele s-au
//! terminat. Șansa apare doar în mutarea forțată, deci AI-ul nu o poate anticipa.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct SSpec {
    pub title: &'static str,
    pub scene: &'static str,
    pub toks: [&'static str; 3],
    pub start: [i32; 3],
    pub n: usize,
    pub goal: u32,
    pub time: i32,
    pub morale: i32,
    pub twist: usize,
    pub maxcost: u8,
}

pub static SPECS: [SSpec; 9] = [
    SSpec {
        title: "Paleta Evadării",
        scene: "camera",
        toks: ["chei", "indicii", "unelte"],
        start: [2, 2, 1],
        n: 12,
        goal: 7,
        time: 24,
        morale: 6,
        twist: 30,
        maxcost: 2,
    },
    SSpec {
        title: "Ancheta Nocturnă",
        scene: "pista",
        toks: ["martori", "probe", "intuiții"],
        start: [2, 1, 2],
        n: 13,
        goal: 8,
        time: 26,
        morale: 6,
        twist: 35,
        maxcost: 2,
    },
    SSpec {
        title: "Pachetul Aventurierilor",
        scene: "locația",
        toks: ["arme", "vrăji", "bunuri"],
        start: [2, 2, 2],
        n: 12,
        goal: 8,
        time: 25,
        morale: 7,
        twist: 35,
        maxcost: 3,
    },
    SSpec {
        title: "Orașul Asediat",
        scene: "ziua",
        toks: ["hrană", "lemne", "medicamente"],
        start: [3, 2, 1],
        n: 14,
        goal: 9,
        time: 30,
        morale: 8,
        twist: 45,
        maxcost: 2,
    },
    SSpec {
        title: "Fisuri Temporale",
        scene: "epoca",
        toks: ["unități de energie", "ancore", "dovezi"],
        start: [2, 2, 2],
        n: 12,
        goal: 7,
        time: 20,
        morale: 6,
        twist: 30,
        maxcost: 3,
    },
    SSpec {
        title: "Camera cu Ghicitori",
        scene: "ghicitoarea",
        toks: ["litere", "numere", "simboluri"],
        start: [2, 2, 2],
        n: 11,
        goal: 7,
        time: 21,
        morale: 5,
        twist: 25,
        maxcost: 2,
    },
    SSpec {
        title: "Ținutul Cețurilor",
        scene: "poteca",
        toks: ["credință", "oțel", "ierburi"],
        start: [2, 1, 2],
        n: 14,
        goal: 9,
        time: 29,
        morale: 7,
        twist: 40,
        maxcost: 3,
    },
    SSpec {
        title: "Vânătorii de Umbre",
        scene: "misiunea",
        toks: ["amulete", "gloanțe", "informații"],
        start: [2, 2, 1],
        n: 12,
        goal: 8,
        time: 25,
        morale: 6,
        twist: 35,
        maxcost: 3,
    },
    SSpec {
        title: "Calea Cavalerului",
        scene: "ținutul",
        toks: ["mișcare", "influență", "cristale"],
        start: [3, 2, 1],
        n: 14,
        goal: 9,
        time: 30,
        morale: 8,
        twist: 30,
        maxcost: 3,
    },
];

#[derive(Clone)]
struct Scene {
    cost: [u8; 3],
    gain: (usize, u8),
    tc: u8,
}

#[derive(Clone)]
pub struct Story {
    sp: &'static SSpec,
    scenes: Vec<Scene>,
    idx: usize,
    prog: u32,
    time: i32,
    morale: i32,
    tok: [i32; 3],
    pend: bool,
    side: usize,
    rng: Rng,
}

pub fn new(i: usize) -> Story {
    with_seed(i, seed_now())
}

pub fn with_seed(i: usize, seed: u64) -> Story {
    let sp = &SPECS[i];
    let mut rng = Rng::new(seed);
    let mut scenes = Vec::new();
    for _ in 0..sp.n {
        let a = rng.below(3);
        let b = (a + 1 + rng.below(2)) % 3;
        let mut cost = [0u8; 3];
        cost[a] = 1 + rng.below(sp.maxcost as usize) as u8;
        if rng.below(3) > 0 {
            cost[b] = 1 + rng.below(sp.maxcost as usize - 1) as u8;
        }
        scenes.push(Scene {
            cost,
            gain: (rng.below(3), 1 + rng.below(2) as u8),
            tc: 1 + rng.below(2) as u8,
        });
    }
    Story {
        sp,
        scenes,
        idx: 0,
        prog: 0,
        time: sp.time,
        morale: sp.morale,
        tok: sp.start,
        pend: false,
        side: 0,
        rng,
    }
}

impl Story {
    fn can(&self) -> bool {
        match self.scenes.get(self.idx) {
            Some(s) => (0..3).all(|k| self.tok[k] >= s.cost[k] as i32),
            None => false,
        }
    }
}

impl Mini for Story {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend {
            return vec!["intamplare".into()];
        }
        let mut v = Vec::new();
        if self.can() {
            v.push("fac".into());
        }
        v.push("ocol".into());
        for t in 1..=3 {
            v.push(format!("odih {t}"));
        }
        v
    }
    fn step(&mut self, m: &str) {
        if self.pend {
            if self.rng.below(100) < self.sp.twist {
                match self.rng.below(5) {
                    0 | 1 => {
                        let t = self.rng.below(3);
                        if self.tok[t] > 0 {
                            self.tok[t] -= 1;
                        }
                    }
                    2 => self.time -= 1,
                    3 => self.morale -= 1,
                    _ => self.tok[self.rng.below(3)] += 1,
                }
            }
            self.pend = false;
            self.side = 1 - self.side;
            return;
        }
        if m == "fac" {
            let s = self.scenes[self.idx].clone();
            for k in 0..3 {
                self.tok[k] -= s.cost[k] as i32;
            }
            self.tok[s.gain.0] += s.gain.1 as i32;
            self.time -= s.tc as i32;
            self.prog += 1;
            self.idx += 1;
        } else if m == "ocol" {
            self.morale -= 1;
            self.time -= 1;
            self.idx += 1;
        } else {
            let t = m[5..].parse::<usize>().unwrap() - 1;
            self.tok[t] += 1;
            self.time -= 1;
        }
        self.pend = true;
    }
    fn result(&self) -> Option<Outcome> {
        if self.prog >= self.sp.goal {
            Some(Outcome::Win(Player::One))
        } else if self.time <= 0 || self.morale <= 0 || self.idx >= self.sp.n {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn eval(&self, _p: Player) -> i32 {
        let toks: i32 = self.tok.iter().sum();
        let mut e = self.prog as i32 * 100 + toks * 4 + self.time * 3 + self.morale * 6;
        if self.can() {
            e += 5;
        }
        // scena următoare: cât lipsește pentru a o putea înfrunta
        if let Some(s) = self.scenes.get(self.idx) {
            let miss: i32 = (0..3)
                .map(|k| (s.cost[k] as i32 - self.tok[k]).max(0))
                .sum();
            e -= miss * 5;
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — {} {}/{} | timp {} | moral {} | jetoane: {} {}, {} {}, {} {}\n",
            self.sp.title,
            self.sp.scene,
            self.prog,
            self.sp.goal,
            self.time,
            self.morale,
            self.tok[0],
            self.sp.toks[0],
            self.tok[1],
            self.sp.toks[1],
            self.tok[2],
            self.sp.toks[2]
        );
        for (k, sc) in self.scenes.iter().enumerate().skip(self.idx).take(3) {
            let c: Vec<String> = (0..3)
                .filter(|&j| sc.cost[j] > 0)
                .map(|j| format!("{} {}", sc.cost[j], self.sp.toks[j]))
                .collect();
            s.push_str(&format!(
                "{} {}: cost {} → +{} {}, timp {}\n",
                self.sp.scene,
                k + 1,
                c.join(" + "),
                sc.gain.1,
                self.sp.toks[sc.gain.0],
                sc.tc
            ));
        }
        s.push_str(&format!(
            "La mutare: {} — fac | ocol | odih 1..3\n",
            pname(self.side())
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        for i in 0..9 {
            match i {
                0 => check(|s| with_seed(0, s), 3000),
                1 => check(|s| with_seed(1, s), 3000),
                2 => check(|s| with_seed(2, s), 3000),
                3 => check(|s| with_seed(3, s), 3000),
                4 => check(|s| with_seed(4, s), 3000),
                5 => check(|s| with_seed(5, s), 3000),
                6 => check(|s| with_seed(6, s), 3000),
                7 => check(|s| with_seed(7, s), 3000),
                _ => check(|s| with_seed(8, s), 3000),
            }
        }
    }
}
