//! Schița Ascunsă — indicii dintr-un tablou (variantă originală de deducție vizuală, 2 jucători).
//!
//! Zece simboluri (A–J) și șase cărți-tablou (fiecare cu 4 simboluri) la vedere. În fiecare rundă
//! una dintre ele e ținta, cunoscută doar de cel care schițează. Schițatorul adaugă 1–3 simboluri
//! adevărate de pe cartea țintă (`pun S`, apoi `gata`); celălalt ghicește `ghic k` (cartea k,
//! 1–6). Apoi mutarea forțată `verific`. Ghicit corect: ghicitorul +2, schițatorul +1; greșit:
//! schițatorul +2 (a momit bine). 6 runde, rolurile alternează. Câștigă scorul mai mare.
//! Informație ascunsă: ținta e văzută doar de schițator; AI-ul ghicitor nu o citește.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const SYM: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Sketch,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Tablou {
    viewer: Option<Player>,
    cards: Vec<u16>,
    target: usize,
    sketch: u16,
    guess: usize,
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

fn deal(rng: &mut Rng) -> (Vec<u16>, usize) {
    let mut cards: Vec<u16> = Vec::new();
    while cards.len() < 6 {
        let mut idx: Vec<usize> = (0..10).collect();
        shuf(&mut idx, rng);
        let m = idx[..4].iter().fold(0u16, |m, &i| m | (1 << i));
        if !cards.contains(&m) {
            cards.push(m);
        }
    }
    let t = rng.below(6);
    (cards, t)
}

impl Tablou {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let (cards, target) = deal(&mut rng);
        Tablou {
            viewer: None,
            cards,
            target,
            sketch: 0,
            guess: 0,
            ph: Ph::Sketch,
            round: 0,
            pts: [0; 2],
            rng,
        }
    }
    fn sketcher(&self) -> usize {
        (self.round % 2) as usize
    }
    fn consistent(&self) -> usize {
        self.cards
            .iter()
            .filter(|&&c| c & self.sketch == self.sketch)
            .count()
    }
    fn names(m: u16) -> String {
        (0..10)
            .filter(|&i| m & (1 << i) != 0)
            .map(|i| SYM[i].to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Default for Tablou {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Tablou {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let who = if self.ph == Ph::Sketch {
            self.sketcher()
        } else {
            1 - self.sketcher()
        };
        if who == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Sketch => {
                let mut v = Vec::new();
                if self.sketch.count_ones() < 3 {
                    for i in 0..10 {
                        if self.cards[self.target] & (1 << i) != 0 && self.sketch & (1 << i) == 0 {
                            v.push(format!("pun {}", SYM[i]));
                        }
                    }
                }
                if self.sketch != 0 {
                    v.push("gata".into());
                }
                v
            }
            Ph::Guess => (0..6).map(|k| format!("ghic {}", k + 1)).collect(),
            Ph::Resolve => vec!["verific".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Sketch => {
                if m == "gata" {
                    self.ph = Ph::Guess;
                } else {
                    let c = m.as_bytes()[4].to_ascii_uppercase() as char;
                    let i = SYM.iter().position(|&x| x == c).unwrap();
                    self.sketch |= 1 << i;
                }
            }
            Ph::Guess => {
                self.guess = m[5..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                let s = self.sketcher();
                if self.guess == self.target {
                    self.pts[1 - s] += 2;
                    self.pts[s] += 1;
                } else {
                    self.pts[s] += 2;
                }
                self.round += 1;
                self.sketch = 0;
                let (c, t) = deal(&mut self.rng);
                self.cards = c;
                self.target = t;
                self.ph = Ph::Sketch;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 6 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 20 * (self.pts[a] - self.pts[1 - a]);
        let c = self.consistent() as i32;
        match self.ph {
            Ph::Sketch if a == self.sketcher() => base + 10 * c,
            Ph::Guess | Ph::Resolve if a == self.sketcher() => base + (200 - 100 / c.max(1)) / 10,
            Ph::Guess => base,
            Ph::Resolve => {
                // ghicitorul: cartea aleasă trebuie să fie compatibilă cu schița
                if self.cards[self.guess] & self.sketch == self.sketch {
                    base
                } else {
                    base - 100
                }
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Schița Ascunsă — runda {}/6 | schițator: J{} | puncte J1 {}, J2 {}\n",
            (self.round + 1).min(6),
            self.sketcher() + 1,
            self.pts[0],
            self.pts[1]
        );
        for (k, &c) in self.cards.iter().enumerate() {
            let tg = if viewer == self.sketcher() && k == self.target {
                "  <- ținta ta"
            } else {
                ""
            };
            s.push_str(&format!("{}: {}{}\n", k + 1, Self::names(c), tg));
        }
        s.push_str(&format!(
            "Schița: {}\nLa mutare: {} (faza {:?}: pun S | gata | ghic k | verific)\n",
            if self.sketch == 0 {
                "-".into()
            } else {
                Self::names(self.sketch)
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

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Tablou::with_seed, 200);
    }
    #[test]
    fn target_hidden_from_guesser() {
        let mut g = Tablou::with_seed(8);
        g.view(Some(Player::Two));
        assert!(!g.show().contains("ținta ta"));
        g.view(Some(Player::One));
        assert!(g.show().contains("ținta ta"));
    }
    #[test]
    fn guesser_ai_ignores_the_target() {
        let mut a = Tablou::with_seed(8);
        a.ph = Ph::Guess;
        a.sketch = a.cards[0] & a.cards[0].wrapping_neg();
        let mut b = a.clone();
        b.target = (a.target + 1) % 6;
        for seed in 1..6 {
            assert_eq!(
                crate::brand_util::ai_of(&a, seed),
                crate::brand_util::ai_of(&b, seed)
            );
        }
    }
}
