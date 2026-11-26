//! Cadranul Ascuns — ghicit pe o scală, cu percepții diferite (variantă originală, 2 jucători).
//!
//! Scala merge de la 0 („rece”) la 20 („cald”). Zece cuvinte-indiciu au o poziție de bază publică,
//! dar fiecare jucător are o înclinație personală ascunsă (−3..+3) care îi mută percepția: cuvântul w
//! „înseamnă” pentru el q(w)+înclinația. 8 runde, dătătorul alternează. Mutarea forțată `trag` alege
//! ținta secretă (o vede doar dătătorul); dătătorul alege un cuvânt: `indiciu w`; ghicitorul pune
//! cadranul pe `ghic p` (0..20); mutarea forțată `verifica` dezvăluie ținta: diferență ≤1: 4 puncte,
//! ≤2: 3, ≤4: 2, ≤6: 1 (ghicitorul); dătătorul ia 1 punct dacă ghicitorul a luat ≥3. Istoricul
//! rundelor e public: poți afla cum „vede” scala celălalt.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const WORDS: [&str; 10] = [
    "înghețat",
    "polar",
    "rece",
    "răcoros",
    "toamnă",
    "neutru",
    "călduț",
    "cald",
    "tropical",
    "fierbinte",
];
const Q: [i32; 10] = [1, 2, 4, 7, 8, 10, 12, 14, 17, 19];
const ROUNDS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Clue,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Cadran {
    viewer: Option<Player>,
    bias: [i32; 2],
    target: i32,
    clue: usize,
    guess: i32,
    /// (dătător, cuvânt, țintă, ghicit)
    hist: Vec<(usize, usize, i32, i32)>,
    score: [i32; 2],
    round: u32,
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Cadran {
    Cadran::with_seed(seed_now())
}

fn pts(d: i32) -> i32 {
    match d.abs() {
        0..=1 => 4,
        2 => 3,
        3..=4 => 2,
        5..=6 => 1,
        _ => 0,
    }
}

impl Cadran {
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let bias = [rng.below(7) as i32 - 3, rng.below(7) as i32 - 3];
        Cadran {
            viewer: None,
            bias,
            target: 10,
            clue: 0,
            guess: 10,
            hist: Vec::new(),
            score: [0; 2],
            round: 0,
            ph: Ph::Deal,
            rng,
        }
    }
    fn giver(&self) -> usize {
        (self.round % 2) as usize
    }
    /// Deplasarea medie a dătătorului g, deduse din istoricul public.
    fn est_off(&self, g: usize) -> i32 {
        let v: Vec<i32> = self
            .hist
            .iter()
            .filter(|h| h.0 == g)
            .map(|h| h.2 - Q[h.1])
            .collect();
        if v.is_empty() {
            0
        } else {
            (v.iter().sum::<i32>() as f64 / v.len() as f64).round() as i32
        }
    }
}

impl Mini for Cadran {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Deal | Ph::Resolve => 0,
            Ph::Clue => self.giver(),
            Ph::Guess => 1 - self.giver(),
        };
        if t == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Resolve => vec!["verifica".into()],
            Ph::Clue => (0..10).map(|w| format!("indiciu {w}")).collect(),
            Ph::Guess => (0..=20).map(|p| format!("ghic {p}")).collect(),
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                self.target = 2 + self.rng.below(17) as i32;
                self.ph = Ph::Clue;
            }
            "verifica" => {
                let g = self.giver();
                let p = pts(self.guess - self.target);
                self.score[1 - g] += p;
                if p >= 3 {
                    self.score[g] += 1;
                }
                self.hist.push((g, self.clue, self.target, self.guess));
                self.round += 1;
                self.ph = Ph::Deal;
            }
            _ if m.starts_with("indiciu") => {
                self.clue = m[8..].parse().unwrap();
                self.ph = Ph::Guess;
            }
            _ => {
                self.guess = m[5..].parse().unwrap();
                self.ph = Ph::Resolve;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            Some(by_score(self.score[0], self.score[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut e = 20 * (self.score[i] - self.score[o]);
        match self.ph {
            Ph::Guess if i == self.giver() => {
                // dătătorul își cunoaște ținta și înclinația
                let seen = Q[self.clue] + self.bias[i];
                e += 4 * pts(seen - self.target) - (seen - self.target).abs();
            }
            Ph::Resolve if i != self.giver() => {
                // ghicitorul: distanța față de ce crede el că înseamnă indiciul (fără a citi ținta)
                let believed = Q[self.clue] + self.est_off(self.giver());
                let d = (self.guess - believed).abs();
                e += 4 * pts(d) - d;
            }
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Cadranul Ascuns — runda {}/{} | J1 {} / J2 {}\nCuvinte: ",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.score[0],
            self.score[1]
        );
        let w: Vec<String> = (0..10)
            .map(|i| format!("{i}:{}(~{})", WORDS[i], Q[i]))
            .collect();
        s.push_str(&w.join(", "));
        s.push('\n');
        s.push_str(&format!(
            "Înclinația ta (J{}): {:+}\n",
            me + 1,
            self.bias[me]
        ));
        if self.ph != Ph::Deal {
            s.push_str(&format!("Dătător J{}\n", self.giver() + 1));
            if me == self.giver() && self.ph != Ph::Resolve {
                s.push_str(&format!("Ținta ta secretă: {}\n", self.target));
            }
            if self.ph != Ph::Clue {
                s.push_str(&format!("Indiciu: {}\n", WORDS[self.clue]));
            }
        }
        for h in self.hist.iter().rev().take(4) {
            s.push_str(&format!(
                "  istoric: J{} a spus „{}”, ținta {}, ghicit {}\n",
                h.0 + 1,
                WORDS[h.1],
                h.2,
                h.3
            ));
        }
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cadran_ok() {
        check(Cadran::with_seed, 200);
    }

    #[test]
    fn ghicitorul_nu_citeste_tinta() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let mut g = Cadran::with_seed(4);
        for _ in 0..3 {
            let m = g.moves()[0].clone();
            g.step(&m);
            let m = g.moves()[3].clone();
            g.step(&m);
            let m = g.moves()[5].clone();
            g.step(&m);
            g.step("verifica");
        }
        g.step("trag");
        g.step("indiciu 4");
        assert!(g.ph == Ph::Guess);
        let a = ai_of(&g, 7);
        let mut h = g.clone();
        h.target = (g.target + 7) % 19;
        assert_eq!(a, ai_of(&h, 7));
    }
}
