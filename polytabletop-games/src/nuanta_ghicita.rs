//! Nuanța Ghicită — o țintă pe o grilă 8×8 de nuanțe, indicată prin benzi (variantă originală, 2 jucători).
//!
//! Ținta e o căsuță (rând, coloană) cunoscută doar de cel care dă indiciile (rolurile alternează). El dă
//! unul sau două indicii despre bandă: `rand b` (rândurile 2b−1 și 2b, b = 1–4) și/sau `col b`
//! (coloanele 2b−1 și 2b), apoi `gata`. Celălalt pune un marcaj `ghic r c`; mutarea forțată `verific` dezvăluie ținta.
//! Puncte pentru ghicitor: exact 3, vecin (inclusiv diagonal) 2, la distanță 2 → 1; indicatorul primește 1 dacă
//! ghicitorul a luat cel puțin 2 (+1 dacă a dat un singur indiciu). 6 runde.
//! Informație ascunsă: ținta; AI-ul ghicitor nu o citește.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROUNDS: u32 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Clue,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Nuanta {
    viewer: Option<Player>,
    target: (i32, i32),
    rowb: Option<i32>,
    colb: Option<i32>,
    guess: (i32, i32),
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}

fn points(d: i32) -> i32 {
    match d {
        0 => 3,
        1 => 2,
        2 => 1,
        _ => 0,
    }
}

impl Nuanta {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let target = (rng.below(8) as i32, rng.below(8) as i32);
        Nuanta {
            viewer: None,
            target,
            rowb: None,
            colb: None,
            guess: (0, 0),
            ph: Ph::Clue,
            round: 0,
            pts: [0; 2],
            rng,
        }
    }
    fn giver(&self) -> usize {
        (self.round % 2) as usize
    }
    /// Căsuțele compatibile cu indiciile publice.
    fn region(&self) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for r in 0..8 {
            for c in 0..8 {
                if self.rowb.is_none_or(|b| r / 2 == b) && self.colb.is_none_or(|b| c / 2 == b) {
                    v.push((r, c));
                }
            }
        }
        v
    }
    /// Cel mai bun scor așteptat (×1000) al unui marcaj, pe regiunea publică.
    fn best_expect(&self) -> i32 {
        let reg = self.region();
        let mut best = 0;
        for r in 0..8 {
            for c in 0..8 {
                let s: i32 = reg.iter().map(|&t| points(dist((r, c), t))).sum();
                best = best.max(1000 * s / reg.len() as i32);
            }
        }
        best
    }
    fn nclues(&self) -> i32 {
        self.rowb.is_some() as i32 + self.colb.is_some() as i32
    }
}

impl Default for Nuanta {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Nuanta {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Clue => pl(self.giver()),
            _ => pl(1 - self.giver()),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Clue => {
                let mut v = Vec::new();
                if self.rowb.is_none() {
                    v.push(format!("rand {}", self.target.0 / 2 + 1));
                }
                if self.colb.is_none() {
                    v.push(format!("col {}", self.target.1 / 2 + 1));
                }
                if self.nclues() > 0 {
                    v.push("gata".into());
                }
                v
            }
            Ph::Guess => (1..=8)
                .flat_map(|r| (1..=8).map(move |c| format!("ghic {r} {c}")))
                .collect(),
            Ph::Resolve => vec!["verific".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Clue => {
                if m == "gata" {
                    self.ph = Ph::Guess;
                } else if let Some(b) = m.strip_prefix("rand ") {
                    self.rowb = Some(b.parse::<i32>().unwrap() - 1);
                } else {
                    self.colb = Some(m[4..].parse::<i32>().unwrap() - 1);
                }
                if self.nclues() == 2 && self.ph == Ph::Clue {
                    self.ph = Ph::Guess;
                }
            }
            Ph::Guess => {
                let mut it = m[5..].split(' ');
                let r: i32 = it.next().unwrap().parse().unwrap();
                let c: i32 = it.next().unwrap().parse().unwrap();
                self.guess = (r - 1, c - 1);
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                let g = self.giver();
                let pt = points(dist(self.guess, self.target));
                self.pts[1 - g] += pt;
                if pt >= 2 {
                    self.pts[g] += 1 + (self.nclues() == 1) as i32;
                }
                self.round += 1;
                self.rowb = None;
                self.colb = None;
                self.target = (self.rng.below(8) as i32, self.rng.below(8) as i32);
                self.ph = Ph::Clue;
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
        let base = 3000 * (self.pts[a] - self.pts[1 - a]);
        match self.ph {
            Ph::Clue | Ph::Guess if a == self.giver() => {
                // cu cât regiunea publică e mai mică, cu atât ghicitul e mai probabil; al doilea indiciu costă 1 punct
                base + self.best_expect() - 600 * (self.nclues() - 1).max(0)
            }
            Ph::Resolve if a != self.giver() => {
                let reg = self.region();
                let s: i32 = reg.iter().map(|&t| points(dist(self.guess, t))).sum();
                base + 1000 * s / reg.len() as i32
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Nuanța Ghicită — runda {}/{} | indicator: J{} | puncte J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.giver() + 1,
            self.pts[0],
            self.pts[1]
        );
        for r in 0..8 {
            let mut line = String::new();
            for c in 0..8 {
                let inreg =
                    self.rowb.is_none_or(|b| r / 2 == b) && self.colb.is_none_or(|b| c / 2 == b);
                let ch = if viewer == self.giver() && (r, c) == self.target {
                    'X'
                } else if self.ph == Ph::Resolve && (r, c) == self.guess {
                    'o'
                } else if inreg && (self.rowb.is_some() || self.colb.is_some()) {
                    '+'
                } else {
                    '.'
                };
                line.push(ch);
                line.push(' ');
            }
            s.push_str(&format!("{} | {}\n", r + 1, line));
        }
        s.push_str(&format!(
            "Indicii: rând {} | coloană {}\nLa mutare: {} (rand b | col b | gata | ghic r c | verific)\n",
            self.rowb.map(|b| (b + 1).to_string()).unwrap_or("-".into()),
            self.colb.map(|b| (b + 1).to_string()).unwrap_or("-".into()),
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
        crate::brand_util::check(Nuanta::with_seed, 200);
    }
    #[test]
    fn target_is_private_and_guesser_ai_ignores_it() {
        let mut g = Nuanta::with_seed(7);
        g.step(&format!("rand {}", g.target.0 / 2 + 1));
        g.step("gata");
        let mut h = g.clone();
        h.target = (g.target.0 ^ 1, g.target.1);
        let mut v = g.clone();
        v.view(Some(Player::Two));
        assert!(!v.show().contains('X'));
        v.view(Some(Player::One));
        assert!(v.show().contains('X'));
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_tinta_secreta_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "nuanta_ghicita",
            |s| B(Nuanta::with_seed(s)),
            |g, me, seed| {
                let n = &mut g.0;
                // ținta e secretă pentru ghicitor și cele viitoare nu le știe nimeni
                n.rng = Rng::new(seed ^ 0x7a);
                if me.idx() != n.giver() {
                    let reg = n.region();
                    n.target = reg[seed as usize % reg.len()];
                }
            },
            12,
        );
    }
}
