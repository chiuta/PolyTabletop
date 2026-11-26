//! Definiții Mincinoase — bluf între adevăr și momeală (variantă originală, 2 jucători).
//!
//! Opt „definiții” au fiecare o plauzibilitate publică (1–9); cea adevărată se alege cu șansă proporțională cu
//! plauzibilitatea ei și o cunoaște doar cel care scrie (rolurile alternează). El alege o momeală `momeala b`;
//! cele două variante apar în ordine amestecată, iar celălalt jucător răspunde `cred 1` sau `cred 2`. Mutarea
//! forțată `dezvalui` arată adevărul: ghicit → ghicitorul +2; păcălit → scriitorul +2. 6 runde.
//! Informație ascunsă: definiția adevărată; AI-ul ghicitor judecă doar după plauzibilități publice.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROUNDS: u32 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Pick,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Definitii {
    viewer: Option<Player>,
    plaus: [i32; 8],
    truth: usize,
    opts: [usize; 2],
    guess: usize,
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

impl Definitii {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Definitii {
            viewer: None,
            plaus: [0; 8],
            truth: 0,
            opts: [0; 2],
            guess: 0,
            ph: Ph::Pick,
            round: 0,
            pts: [0; 2],
            rng: Rng::new(seed),
        };
        g.deal();
        g
    }
    fn deal(&mut self) {
        for i in 0..8 {
            self.plaus[i] = 1 + self.rng.below(9) as i32;
        }
        let tot: i32 = self.plaus.iter().sum();
        let mut r = self.rng.below(tot as usize) as i32;
        for i in 0..8 {
            if r < self.plaus[i] {
                self.truth = i;
                break;
            }
            r -= self.plaus[i];
        }
    }
    fn writer(&self) -> usize {
        (self.round % 2) as usize
    }
}

impl Default for Definitii {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Definitii {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Pick => pl(self.writer()),
            _ => pl(1 - self.writer()),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Pick => (0..8)
                .filter(|&b| b != self.truth)
                .map(|b| format!("momeala {}", b + 1))
                .collect(),
            Ph::Guess => vec!["cred 1".into(), "cred 2".into()],
            Ph::Resolve => vec!["dezvalui".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Pick => {
                let b = m[8..].parse::<usize>().unwrap() - 1;
                self.opts = if self.rng.below(2) == 0 {
                    [self.truth, b]
                } else {
                    [b, self.truth]
                };
                self.ph = Ph::Guess;
            }
            Ph::Guess => {
                self.guess = m[5..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                let w = self.writer();
                if self.opts[self.guess] == self.truth {
                    self.pts[1 - w] += 2;
                } else {
                    self.pts[w] += 2;
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
        let (o0, o1) = (self.opts[0], self.opts[1]);
        match self.ph {
            Ph::Guess if a == self.writer() => {
                // ghicitorul alege varianta mai plauzibilă
                let b = if o0 == self.truth { o1 } else { o0 };
                let (pb, pt) = (self.plaus[b], self.plaus[self.truth]);
                base + 2000 * (pb - pt).signum().max(0) + 1000 * (pb == pt) as i32
            }
            Ph::Resolve if a != self.writer() => {
                let (c, o) = (self.opts[self.guess], self.opts[1 - self.guess]);
                base + 100 * (self.plaus[c] - self.plaus[o])
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Definiții Mincinoase — runda {}/{} | scriitor: J{} | puncte J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.writer() + 1,
            self.pts[0],
            self.pts[1]
        );
        let d: Vec<String> = (0..8)
            .map(|i| format!("{}:plauz.{}", i + 1, self.plaus[i]))
            .collect();
        s.push_str(&format!("Definiții: {}\n", d.join(" ")));
        if self.ph != Ph::Pick {
            s.push_str(&format!(
                "Variantele: 1 = definiția {}, 2 = definiția {}\n",
                self.opts[0] + 1,
                self.opts[1] + 1
            ));
        }
        if viewer == self.writer() {
            s.push_str(&format!(
                "Adevărul (doar pentru tine): definiția {}\n",
                self.truth + 1
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (momeala b | cred k | dezvalui)\n",
            pname(self.side())
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
        crate::brand_util::check(Definitii::with_seed, 100);
    }
    #[test]
    fn truth_is_hidden_from_guesser_and_ai_ignores_it() {
        let mut g = Definitii::with_seed(3);
        let m = g.moves()[0].clone();
        g.step(&m);
        let mut v = g.clone();
        v.view(Some(Player::Two));
        assert!(!v.show().contains("Adevărul"));
        v.view(Some(Player::One));
        assert!(v.show().contains("Adevărul"));
        // aceeași pereche de variante, adevăr schimbat între ele: AI-ul ghicitor alege la fel
        let mut h = g.clone();
        h.truth = if g.truth == g.opts[0] {
            g.opts[1]
        } else {
            g.opts[0]
        };
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
    }
}
