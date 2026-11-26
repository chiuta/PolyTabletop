//! Pariu pe Cifre — estimări și pariuri (variantă originală, 2 jucători).
//!
//! În fiecare rundă o cifră ascunsă T (0–99) are un interval public de 40 de valori care o conține.
//! Fiecare jucător estimează în secret `est g` (g din interval, din 5 în 5); apoi, tot în secret, pariază
//! `pariez j s` (pe estimarea jucătorului j, cu s = 1–3 jetoane); mutarea forțată `verific` arată T:
//! cea mai apropiată estimare (la egalitate, nimeni) aduce autorului +2, iar fiecare pariu pe ea aduce +s.
//! Informație ascunsă: T, estimarea și pariul primului jucător până la verificare; AI-ul nu le citește.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROUNDS: u32 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Est1,
    Est2,
    Bet1,
    Bet2,
    Resolve,
}

#[derive(Clone)]
pub struct Pariu {
    viewer: Option<Player>,
    target: i32,
    lo: i32,
    est: [i32; 2],
    bet: [(usize, i32); 2],
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

impl Pariu {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pariu {
            viewer: None,
            target: 0,
            lo: 0,
            est: [0; 2],
            bet: [(0, 0); 2],
            ph: Ph::Est1,
            round: 0,
            pts: [0; 2],
            rng: Rng::new(seed),
        };
        g.deal();
        g
    }
    fn deal(&mut self) {
        self.lo = self.rng.below(61) as i32;
        self.target = self.lo + self.rng.below(40) as i32;
    }
    /// Câte valori din interval (din 40) sunt mai aproape de estimarea `x` decât de `y`.
    fn closer(&self, x: i32, y: i32) -> i32 {
        (self.lo..self.lo + 40)
            .filter(|&t| (t - x).abs() < (t - y).abs())
            .count() as i32
    }
}

impl Default for Pariu {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Pariu {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Est1 | Ph::Bet1 | Ph::Resolve => Player::One,
            Ph::Est2 | Ph::Bet2 => Player::Two,
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Est1 | Ph::Est2 => (0..8)
                .map(|i| format!("est {}", self.lo + 5 * i + 2))
                .collect(),
            Ph::Bet1 | Ph::Bet2 => {
                let mut v = Vec::new();
                for j in 1..=2 {
                    for s in 1..=3 {
                        v.push(format!("pariez {j} {s}"));
                    }
                }
                v
            }
            Ph::Resolve => vec!["verific".into()],
        }
    }
    fn step(&mut self, m: &str) {
        let p = pi(self.side());
        match self.ph {
            Ph::Est1 | Ph::Est2 => {
                self.est[p] = m[4..].parse().unwrap();
                self.ph = if self.ph == Ph::Est1 {
                    Ph::Est2
                } else {
                    Ph::Bet1
                };
            }
            Ph::Bet1 | Ph::Bet2 => {
                let mut it = m[7..].split(' ');
                let j: usize = it.next().unwrap().parse().unwrap();
                let s: i32 = it.next().unwrap().parse().unwrap();
                self.bet[p] = (j - 1, s);
                self.ph = if self.ph == Ph::Bet1 {
                    Ph::Bet2
                } else {
                    Ph::Resolve
                };
            }
            Ph::Resolve => {
                let d = [
                    (self.est[0] - self.target).abs(),
                    (self.est[1] - self.target).abs(),
                ];
                if d[0] != d[1] {
                    let w = if d[0] < d[1] { 0 } else { 1 };
                    self.pts[w] += 2;
                    for p in 0..2 {
                        if self.bet[p].0 == w {
                            self.pts[p] += self.bet[p].1;
                        }
                    }
                }
                self.round += 1;
                self.ph = Ph::Est1;
                self.deal();
            }
        }
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
        let mid = self.lo * 2 + 39; // mijlocul intervalului ×2
        match self.ph {
            Ph::Est2 | Ph::Bet1 if a == 0 => base - 5 * (2 * self.est[0] - mid).abs(),
            Ph::Bet1 | Ph::Bet2 | Ph::Resolve if self.bet[a].1 > 0 => {
                // câștigă estimarea j cu probabilitatea (valori mai apropiate)/40
                let (j, s) = self.bet[a];
                let (x, y) = (self.est[j], self.est[1 - j]);
                let pw = self.closer(x, y);
                let gain = 25 * pw * (s + if j == a { 2 } else { 0 }) / 10;
                base + gain
            }
            Ph::Est2 | Ph::Est1 if a == 1 => base - 5 * (2 * self.est[1] - mid).abs(),
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Pariu pe Cifre — runda {}/{} | puncte J1 {}, J2 {}\nT e în intervalul {}–{}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.pts[0],
            self.pts[1],
            self.lo,
            self.lo + 39
        );
        for p in 0..2 {
            let committed = match self.ph {
                Ph::Est1 => false,
                Ph::Est2 => p == 0,
                _ => true,
            };
            if committed && (p == viewer || matches!(self.ph, Ph::Bet1 | Ph::Bet2 | Ph::Resolve)) {
                s.push_str(&format!("Estimarea lui J{}: {}\n", p + 1, self.est[p]));
            }
        }
        if self.bet[viewer].1 > 0 && matches!(self.ph, Ph::Bet2 | Ph::Resolve) {
            s.push_str(&format!(
                "Pariul tău: pe estimarea lui J{} cu {}\n",
                self.bet[viewer].0 + 1,
                self.bet[viewer].1
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (faza {:?}: est g | pariez j s | verific)\n",
            pname(self.side()),
            self.ph
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Pariu::with_seed, 300);
    }
    #[test]
    fn hidden_values_are_not_leaked() {
        let mut g = Pariu::with_seed(2);
        g.step("est 10");
        let mut a = g.clone();
        a.est[0] = 77;
        a.target = (g.target + 7) % 100;
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&a, seed));
        }
        g.view(Some(Player::Two));
        assert!(!g.show().contains("Estimarea lui J1"));
    }
}
