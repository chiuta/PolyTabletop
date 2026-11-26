//! Provocări Trăsnite — sarcini cu risc și pariuri (variantă originală, 2 jucători).
//!
//! În fiecare rundă se oferă 3 sarcini (recompensă r, șansă de reușită în procente) din 5 posibile:
//! (1, 90%), (2, 70%), (3, 50%), (4, 35%), (5, 25%). Cel la rând alege `iau k`; adversarul pariază
//! `pe da`, `pe nu` sau `fara` (un punct în joc). Mutarea forțată `incerc` decide sarcina (zar ascuns):
//! reușită → +r pentru cel la rând, ratată → −1; pariul corect aduce +1, cel greșit −1. 8 runde.
//! Informație ascunsă: rezultatul zarului; AI-ul alege doar după șanse publice.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const TASKS: [(i32, i32); 5] = [(1, 90), (2, 70), (3, 50), (4, 35), (5, 25)];
const ROUNDS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Pick,
    Bet(usize),
    Resolve(usize, i32),
}

#[derive(Clone)]
pub struct Trasnite {
    offer: [usize; 3],
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

impl Trasnite {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Trasnite {
            offer: [0; 3],
            ph: Ph::Pick,
            round: 0,
            pts: [0; 2],
            rng: Rng::new(seed),
        };
        g.deal();
        g
    }
    fn deal(&mut self) {
        let mut all: Vec<usize> = (0..5).collect();
        shuf(&mut all, &mut self.rng);
        self.offer = [all[0], all[1], all[2]];
        self.offer.sort_unstable();
    }
    fn mover(&self) -> usize {
        (self.round % 2) as usize
    }
}

impl Default for Trasnite {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Trasnite {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Pick | Ph::Resolve(..) => pl(self.mover()),
            Ph::Bet(_) => pl(1 - self.mover()),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Pick => (1..=3).map(|k| format!("iau {k}")).collect(),
            Ph::Bet(_) => vec!["pe da".into(), "pe nu".into(), "fara".into()],
            Ph::Resolve(..) => vec!["incerc".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Pick => {
                let k: usize = m[4..].parse().unwrap();
                self.ph = Ph::Bet(self.offer[k - 1]);
            }
            Ph::Bet(t) => {
                let b = match m {
                    "pe da" => 1,
                    "pe nu" => -1,
                    _ => 0,
                };
                self.ph = Ph::Resolve(t, b);
            }
            Ph::Resolve(t, b) => {
                let ok = (self.rng.below(100) as i32) < TASKS[t].1;
                let me = self.mover();
                if ok {
                    self.pts[me] += TASKS[t].0;
                } else {
                    self.pts[me] -= 1;
                }
                if b != 0 {
                    self.pts[1 - me] += if (b == 1) == ok { 1 } else { -1 };
                }
                self.round += 1;
                self.ph = Ph::Pick;
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
        let me = self.mover();
        match self.ph {
            Ph::Bet(t) if a == me => {
                let (r, pc) = TASKS[t];
                // câștig așteptat al meu minus pariul optim al adversarului (în sutimi de punct)
                base + 10 * (pc * r - (100 - pc)) - 10 * (2 * pc - 100).abs()
            }
            Ph::Resolve(t, b) if a != me => {
                let pc = TASKS[t].1;
                let ev_ok = 2 * pc - 100; // pariul „da” (în sutimi)
                base + 10 * (b * ev_ok) - 10 * (pc * TASKS[t].0 - (100 - pc)) / 4
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let offer: Vec<String> = self
            .offer
            .iter()
            .enumerate()
            .map(|(i, &t)| format!("{}: +{} la {}%", i + 1, TASKS[t].0, TASKS[t].1))
            .collect();
        format!(
            "Provocări Trăsnite — runda {}/{} | puncte J1 {}, J2 {}\nSarcini: {}\nLa mutare: {} (iau k | pe da | pe nu | fara | incerc)\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.pts[0],
            self.pts[1],
            offer.join(" | "),
            pname(self.side())
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Trasnite::with_seed, 100);
    }
    #[test]
    fn rng_state_does_not_change_the_ai_choice() {
        let a = Trasnite::with_seed(4);
        let mut b = a.clone();
        b.rng = Rng::new(999);
        for seed in 1..6 {
            assert_eq!(ai_of(&a, seed), ai_of(&b, seed));
        }
    }
}
