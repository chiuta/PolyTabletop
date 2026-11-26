//! Motor comun „ghicește alegerea celuilalt” (variante originale B6, 2 jucători).
//!
//! La fiecare rundă (rolurile alternează) cel care alege fixează în secret o variantă `alege f`
//! dintre `n`; ghicitorul răspunde `ghic g` (sau `ghic g s` cu miza `s` = 1–3 jetoane); mutarea forțată
//! `dezvalui` arată alegerea. Modul cu miză: ghicit corect → ghicitorul +s, greșit → cel care alege +1.
//! Modul cu apropiere: ghicitorul primește max(0, 3 − |f − g|), cel care alege primește min(3, |f − g|).
//! Istoricul alegerilor dezvăluite e public; AI-ul îl folosește ca model al adversarului, nu citește alegerea ascunsă.
#![allow(clippy::needless_range_loop)]
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

pub struct PSpec {
    pub title: &'static str,
    pub n: usize,
    pub stake: bool,
    pub rounds: u32,
    pub label: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Choose,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Pick {
    spec: &'static PSpec,
    viewer: Option<Player>,
    choice: usize,
    guess: (usize, i32),
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    /// De câte ori a ales fiecare jucător fiecare variantă (public după dezvăluire).
    hist: [Vec<i32>; 2],
}

impl Pick {
    pub fn new(spec: &'static PSpec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static PSpec, _seed: u64) -> Self {
        Pick {
            spec,
            viewer: None,
            choice: 0,
            guess: (0, 1),
            ph: Ph::Choose,
            round: 0,
            pts: [0; 2],
            hist: [vec![0; spec.n], vec![0; spec.n]],
        }
    }
    fn chooser(&self) -> usize {
        (self.round % 2) as usize
    }
    /// (câștigul ghicitorului, câștigul celui care alege)
    fn pay(&self, f: usize, g: usize, s: i32) -> (i32, i32) {
        if self.spec.stake {
            if f == g {
                (s, 0)
            } else {
                (0, 1)
            }
        } else {
            let d = (f as i32 - g as i32).abs();
            ((3 - d).max(0), d.min(3))
        }
    }
    fn weights(&self) -> Vec<i32> {
        let c = self.chooser();
        (0..self.spec.n).map(|f| 1 + self.hist[c][f]).collect()
    }
}

impl Mini for Pick {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Choose => pl(self.chooser()),
            _ => pl(1 - self.chooser()),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Choose => (0..self.spec.n)
                .map(|f| format!("alege {}", f + 1))
                .collect(),
            Ph::Guess => {
                let mut v = Vec::new();
                for g in 0..self.spec.n {
                    if self.spec.stake {
                        for s in 1..=3 {
                            v.push(format!("ghic {} {}", g + 1, s));
                        }
                    } else {
                        v.push(format!("ghic {}", g + 1));
                    }
                }
                v
            }
            Ph::Resolve => vec!["dezvalui".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Choose => {
                self.choice = m[6..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Guess;
            }
            Ph::Guess => {
                let mut it = m[5..].split(' ');
                let g = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let s = it.next().map(|x| x.parse::<i32>().unwrap()).unwrap_or(1);
                self.guess = (g, s);
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                let c = self.chooser();
                let (gp, cp) = self.pay(self.choice, self.guess.0, self.guess.1);
                self.pts[1 - c] += gp;
                self.pts[c] += cp;
                self.hist[c][self.choice] += 1;
                self.round += 1;
                self.ph = Ph::Choose;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.spec.rounds {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 10000 * (self.pts[a] - self.pts[1 - a]);
        let w = self.weights();
        let tot: i32 = w.iter().sum();
        match self.ph {
            Ph::Guess if a == self.chooser() => {
                // alegerea mea f: ghicitorul presupune istoricul meu
                let mut e = 0;
                for g in 0..self.spec.n {
                    let (gp, cp) = self.pay(self.choice, g, 1);
                    e += w[g] * (cp - gp);
                }
                base + 10000 * e / tot
            }
            Ph::Resolve if a != self.chooser() => {
                let (g, s) = self.guess;
                let mut e = 0;
                for f in 0..self.spec.n {
                    let (gp, cp) = self.pay(f, g, s);
                    e += w[f] * (gp - cp);
                }
                base + 10000 * e / tot
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "{} — runda {}/{} | alege: J{} | puncte J1 {}, J2 {}\n",
            self.spec.title,
            (self.round + 1).min(self.spec.rounds),
            self.spec.rounds,
            self.chooser() + 1,
            self.pts[0],
            self.pts[1]
        );
        s.push_str(&format!("{} (1–{})\n", self.spec.label, self.spec.n));
        for p in 0..2 {
            s.push_str(&format!(
                "Alegerile dezvăluite de J{}: {:?}\n",
                p + 1,
                self.hist[p]
            ));
        }
        if viewer == self.chooser() && self.ph != Ph::Choose {
            s.push_str(&format!("Alegerea ta: {}\n", self.choice + 1));
        }
        s.push_str(&format!(
            "La mutare: {} (faza {:?}: alege f | ghic g [s] | dezvalui)\n",
            pname(self.side()),
            self.ph
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

/// Teste: ghicitorul AI nu citește alegerea ascunsă; randarea o ascunde.
#[cfg(test)]
pub fn audit(spec: &'static PSpec) {
    let mut g = Pick::with_seed(spec, 1);
    g.step("alege 2");
    let mut h = g.clone();
    h.choice = 0;
    for seed in 1..6 {
        assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
    }
    let mut v = g.clone();
    v.view(Some(Player::Two));
    assert!(!v.show().contains("Alegerea ta"));
    v.view(Some(Player::One));
    assert!(v.show().contains("Alegerea ta: 2"));
}
