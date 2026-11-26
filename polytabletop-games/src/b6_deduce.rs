//! Motor comun „descriere → ghicire” pentru jocurile de petrecere din lista B6 (variante originale).
//!
//! O țintă ascunsă dintr-un set de elemente cu atribute (A, B, C…, fiecare cu câteva valori).
//! Cel care descrie (rolurile alternează) dă indicii adevărate `ind A=2` (cel mult limita rundei),
//! apoi `gata`; celălalt ghicește `ghic k`; urmează mutarea forțată `verific`.
//! Ghicit corect: cel care descrie primește (limită + 1 − indicii folosite), ghicitorul 1.
//! Informație ascunsă: ținta e văzută doar de cel care descrie; AI-ul ghicitor nu o citește
//! (rezultatul unei ghiciri apare abia la mutarea forțată `verific`).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct DSpec {
    pub title: &'static str,
    /// Numărul de valori ale fiecărui atribut.
    pub vals: &'static [usize],
    /// Limita de indicii pe rundă (ciclic).
    pub caps: &'static [u32],
    /// Atributele permise ca indicii (bit per atribut).
    pub mask: u32,
    pub rounds: u32,
    pub attr_names: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Hint,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Deduce {
    spec: &'static DSpec,
    viewer: Option<Player>,
    target: usize,
    hints: Vec<(usize, usize)>,
    guess: usize,
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

fn n_items(s: &DSpec) -> usize {
    s.vals.iter().product()
}

pub fn attr_val(s: &DSpec, item: usize, a: usize) -> usize {
    let stride: usize = s.vals[..a].iter().product();
    (item / stride) % s.vals[a]
}

impl Deduce {
    pub fn new(spec: &'static DSpec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static DSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let target = rng.below(n_items(spec));
        Deduce {
            spec,
            viewer: None,
            target,
            hints: Vec::new(),
            guess: 0,
            ph: Ph::Hint,
            round: 0,
            pts: [0; 2],
            rng,
        }
    }
    fn giver(&self) -> usize {
        (self.round % 2) as usize
    }
    fn cap(&self) -> u32 {
        let c = self.spec.caps[self.round as usize % self.spec.caps.len()];
        c.min(self.spec.mask.count_ones())
    }
    fn consistent(&self, k: usize) -> bool {
        self.hints
            .iter()
            .all(|&(a, v)| attr_val(self.spec, k, a) == v)
    }
    fn n_cons(&self) -> i32 {
        (0..n_items(self.spec))
            .filter(|&k| self.consistent(k))
            .count() as i32
    }
    fn reward(&self) -> i32 {
        self.cap() as i32 + 1 - self.hints.len() as i32
    }
    fn item_str(&self, k: usize) -> String {
        let mut s = format!("{:>2}:", k + 1);
        for a in 0..self.spec.vals.len() {
            s.push_str(&format!(
                " {}{}",
                (b'A' + a as u8) as char,
                attr_val(self.spec, k, a) + 1
            ));
        }
        s
    }
}

impl Mini for Deduce {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let who = if self.ph == Ph::Hint {
            self.giver()
        } else {
            1 - self.giver()
        };
        if who == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Hint => {
                let mut v = Vec::new();
                if (self.hints.len() as u32) < self.cap() {
                    for a in 0..self.spec.vals.len() {
                        if self.spec.mask & (1 << a) != 0 && !self.hints.iter().any(|h| h.0 == a) {
                            v.push(format!(
                                "ind {}={}",
                                (b'A' + a as u8) as char,
                                attr_val(self.spec, self.target, a) + 1
                            ));
                        }
                    }
                }
                if !self.hints.is_empty() {
                    v.push("gata".into());
                }
                v
            }
            Ph::Guess => (0..n_items(self.spec))
                .map(|k| format!("ghic {}", k + 1))
                .collect(),
            Ph::Resolve => vec!["verific".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Hint => {
                if m == "gata" {
                    self.ph = Ph::Guess;
                } else {
                    let a = (m.as_bytes()[4].to_ascii_uppercase() - b'A') as usize;
                    let v = m[6..].parse::<usize>().unwrap() - 1;
                    self.hints.push((a, v));
                }
            }
            Ph::Guess => {
                self.guess = m[5..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                let g = self.giver();
                if self.guess == self.target {
                    self.pts[g] += self.reward();
                    self.pts[1 - g] += 1;
                }
                self.round += 1;
                self.hints.clear();
                self.target = self.rng.below(n_items(self.spec));
                self.ph = Ph::Hint;
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
        let base = 20 * (self.pts[a] - self.pts[1 - a]);
        match self.ph {
            Ph::Hint | Ph::Guess if a == self.giver() => {
                base + 1000 * self.reward().max(1) / self.n_cons().max(1)
            }
            Ph::Hint => base,
            Ph::Guess => base,
            Ph::Resolve => {
                if a == self.giver() {
                    base
                } else if self.consistent(self.guess) {
                    base + 500
                } else {
                    base - 500
                }
            }
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "{} — runda {}/{} | descrie: J{} | puncte J1 {}, J2 {}\n",
            self.spec.title,
            (self.round + 1).min(self.spec.rounds),
            self.spec.rounds,
            self.giver() + 1,
            self.pts[0],
            self.pts[1]
        );
        let names: Vec<String> = self
            .spec
            .attr_names
            .iter()
            .enumerate()
            .map(|(i, n)| format!("{}={}", (b'A' + i as u8) as char, n))
            .collect();
        s.push_str(&format!("Atribute: {}\n", names.join(", ")));
        for k in 0..n_items(self.spec) {
            let mark = if viewer == self.giver() && k == self.target {
                "  <- ținta ta"
            } else {
                ""
            };
            s.push_str(&format!("{}{}\n", self.item_str(k), mark));
        }
        let h: Vec<String> = self
            .hints
            .iter()
            .map(|&(a, v)| format!("{}={}", (b'A' + a as u8) as char, v + 1))
            .collect();
        s.push_str(&format!(
            "Indicii ({}/{}): {}\nLa mutare: {} (faza {:?}: ind X=v | gata | ghic k | verific)\n",
            self.hints.len(),
            self.cap(),
            if h.is_empty() {
                "-".into()
            } else {
                h.join(" ")
            },
            pname(self.side()),
            self.ph
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

/// Teste comune: ținta ascunsă ghicitorului, AI-ul ghicitor nu citește ținta.
#[cfg(test)]
pub fn audit(spec: &'static DSpec) {
    let mut a = Deduce::with_seed(spec, 8);
    a.ph = Ph::Guess;
    a.hints.push((0, attr_val(spec, a.target, 0)));
    let mut v = a.clone();
    v.view(Some(crate::b6_util::pl(1 - a.giver())));
    assert!(!v.show().contains("ținta ta"));
    let mut v = a.clone();
    v.view(Some(crate::b6_util::pl(a.giver())));
    assert!(v.show().contains("ținta ta"));
    let mut b = a.clone();
    // altă țintă cu aceleași indicii (deci aceeași informație publică)
    b.target = (0..n_items(spec))
        .find(|&k| k != a.target && b.consistent(k))
        .unwrap_or(a.target);
    for seed in 1..6 {
        assert_eq!(
            crate::brand_util::ai_of(&a, seed),
            crate::brand_util::ai_of(&b, seed)
        );
    }
}
