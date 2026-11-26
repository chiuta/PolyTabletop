//! Fitilul Cuvintelor — „cartofi fierbinți” cu fitil ascuns (variantă originală, 2 jucători).
//!
//! În fiecare rundă un fitil ascuns durează `f` cuvinte (între 6 și 14, cunoscut doar ca interval).
//! Pe rând spui `spun k` (1–3 cuvinte descrise): contorul crește cu k, iar tu notezi k cuvinte în runda asta.
//! Mutarea forțată `suna` verifică fitilul: dacă s-a ars (contor ≥ f), cel care a spus ultimul pierde:
//! celălalt primește cuvintele lui notate în rundă, iar el nimic. 5 runde, rolurile de început alternează.
//! Informație ascunsă: lungimea fitilului (nimeni n-o vede); AI-ul decide doar după interval.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const LO: i32 = 6;
const HI: i32 = 14;
const ROUNDS: u32 = 5;

#[derive(Clone)]
pub struct Fitil {
    fuse: i32,
    count: i32,
    tally: [i32; 2],
    pend: Option<usize>,
    mover: usize,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

impl Fitil {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let fuse = LO + rng.below((HI - LO + 1) as usize) as i32;
        Fitil {
            fuse,
            count: 0,
            tally: [0; 2],
            pend: None,
            mover: 0,
            round: 0,
            pts: [0; 2],
            rng,
        }
    }
    /// Probabilitatea (în sutimi) ca fitilul să se fi ars până la `c`, știind că nu s-a ars până la `prev`.
    fn p_burst(prev: i32, c: i32) -> i32 {
        let lo = prev.max(LO - 1);
        let total = HI - lo;
        if total <= 0 {
            return 100;
        }
        100 * (c.min(HI) - lo).max(0) / total
    }
}

impl Default for Fitil {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Fitil {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.mover)
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["suna".into()];
        }
        (1..=3).map(|k| format!("spun {k}")).collect()
    }
    fn step(&mut self, m: &str) {
        if let Some(prev) = self.pend {
            let _ = prev;
            let me = self.mover;
            if self.count >= self.fuse {
                self.pts[1 - me] += self.tally[1 - me];
                self.round += 1;
                self.count = 0;
                self.tally = [0; 2];
                self.fuse = LO + self.rng.below((HI - LO + 1) as usize) as i32;
                self.mover = (self.round % 2) as usize;
            } else {
                self.mover = 1 - me;
            }
            self.pend = None;
            return;
        }
        let k: i32 = m[5..].parse().unwrap();
        self.pend = Some(self.count as usize);
        self.count += k;
        self.tally[self.mover] += k;
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 1000 * (self.pts[a] - self.pts[1 - a]);
        if let (Some(prev), true) = (self.pend, a == self.mover) {
            let pb = Self::p_burst(prev as i32, self.count);
            let k = self.tally[a];
            // dacă ard fitilul: pierd; altfel cuvintele mele cresc valoarea rundei
            base + (100 - pb) * 6 * k / 10 - pb * 10 * self.tally[1 - a]
        } else {
            base
        }
    }
    fn show(&self) -> String {
        format!(
            "Fitilul Cuvintelor — runda {}/{} | puncte J1 {}, J2 {}\nContor: {} cuvinte (fitilul are între {} și {}) | notate: J1 {}, J2 {}\nLa mutare: {} (spun k | suna)\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.pts[0],
            self.pts[1],
            self.count,
            LO,
            HI,
            self.tally[0],
            self.tally[1],
            pname(self.side())
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Fitil::with_seed, 200);
    }
    #[test]
    fn ai_ignores_fuse() {
        let mut a = Fitil::with_seed(3);
        a.count = 7;
        let mut b = a.clone();
        b.fuse = 14;
        a.fuse = 9;
        for seed in 1..6 {
            assert_eq!(ai_of(&a, seed), ai_of(&b, seed));
        }
    }
    #[test]
    fn burst_probability_is_monotone() {
        assert!(Fitil::p_burst(0, 8) < Fitil::p_burst(0, 12));
        assert_eq!(Fitil::p_burst(13, 14), 100);
    }
}
