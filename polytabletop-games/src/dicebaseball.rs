#![allow(clippy::doc_lazy_continuation)]
//! Dice Baseball (SUA): baseball cu două zaruri, pentru doi jucători, fără decizii.
//!
//! * 9 reprize; în fiecare, J1 bate (vizitator) apoi J2 (gazdă) până la trei outuri. Dacă după
//!   9 reprize e egalitate se joacă reprize suplimentare (cel mult 12, apoi remiză). Gazda nu mai
//!   bate în repriza a 9-a dacă deja conduce.
//! * Bătătorul aruncă două zaruri (total): 2 aruncare ratată (toți alergătorii +1; cu baze goale se
//!   reia), 3 triplu, 4 dublu, 5 walk (alergătorii înaintează doar dacă sunt forțați), 6 fly out,
//!   7 ground out, 8 simplu, 9 strike out, 10 minge afară (se reia), 11 dublă eliminare (două outuri
//!   dacă există alergător, altfel se reia), 12 home run. La lovituri alergătorii înaintează tot
//!   atâtea baze ca bătătorul; cine trece de baza a treia înscrie un punct.
//! Mutare: `arunc`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const INNINGS: u8 = 9;
const MAX_INNINGS: u8 = 12;

#[derive(Clone)]
pub struct DiceBaseball {
    rng: Rng,
    runs: [u32; 2],
    inning: u8,
    batting: Player,
    outs: u8,
    bases: [bool; 3],
    last: String,
    done: bool,
}

impl Default for DiceBaseball {
    fn default() -> Self {
        Self::new()
    }
}

impl DiceBaseball {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        DiceBaseball {
            rng: Rng::new(seed),
            runs: [0; 2],
            inning: 1,
            batting: Player::One,
            outs: 0,
            bases: [false; 3],
            last: String::new(),
            done: false,
        }
    }

    fn advance(&mut self, n: usize, batter_on: bool) -> u32 {
        let mut scored = 0;
        let mut nb = [false; 3];
        for b in (0..3).rev() {
            if self.bases[b] {
                if b + n >= 3 {
                    scored += 1;
                } else {
                    nb[b + n] = true;
                }
            }
        }
        if batter_on {
            if n >= 4 {
                scored += 1;
            } else if n >= 1 {
                nb[n - 1] = true;
            }
        }
        self.bases = nb;
        scored
    }

    fn walk(&mut self) -> u32 {
        // alergătorii avansează doar dacă sunt forțați
        let mut scored = 0;
        if self.bases[0] {
            if self.bases[1] {
                if self.bases[2] {
                    scored = 1;
                }
                self.bases[2] = true;
            }
            self.bases[1] = true;
        }
        self.bases[0] = true;
        scored
    }

    fn end_half(&mut self) {
        self.outs = 0;
        self.bases = [false; 3];
        if self.batting == Player::One {
            self.batting = Player::Two;
            // gazda nu bate dacă deja conduce în repriza finală sau suplimentară
            if self.inning >= INNINGS && self.runs[1] > self.runs[0] {
                self.finish();
            }
        } else {
            self.batting = Player::One;
            if (self.inning >= INNINGS && self.runs[0] != self.runs[1])
                || self.inning >= MAX_INNINGS
            {
                self.finish();
            } else {
                self.inning += 1;
            }
        }
    }

    fn finish(&mut self) {
        self.done = true;
    }
}

impl Game for DiceBaseball {
    type Move = ();

    fn to_move(&self) -> Player {
        self.batting
    }

    fn legal_moves(&self) -> Vec<()> {
        if self.done {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn apply(&mut self, _: &()) {
        let t = self.rng.d6() + self.rng.d6();
        let b = self.batting.idx();
        let any = self.bases.iter().any(|&x| x);
        let (txt, runs): (&str, u32) = match t {
            2 if any => ("aruncare ratată", self.advance(1, false)),
            2 | 10 => ("se reia", 0),
            3 => ("triplu", self.advance(3, true)),
            4 => ("dublu", self.advance(2, true)),
            5 => ("walk", self.walk()),
            8 => ("simplu", self.advance(1, true)),
            12 => ("home run", self.advance(4, true)),
            11 if !any => ("se reia", 0),
            11 => {
                self.outs += 2;
                ("dublă eliminare", 0)
            }
            6 => {
                self.outs += 1;
                ("fly out", 0)
            }
            7 => {
                self.outs += 1;
                ("ground out", 0)
            }
            _ => {
                self.outs += 1;
                ("strike out", 0)
            }
        };
        self.runs[b] += runs;
        self.last = format!(
            "J{}: zaruri {t} — {txt}{}",
            b + 1,
            if runs > 0 {
                format!(", +{runs} puncte")
            } else {
                String::new()
            }
        );
        if self.outs >= 3 {
            self.end_half();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done {
            return None;
        }
        Some(match self.runs[0].cmp(&self.runs[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn render(&self) -> String {
        format!(
            "Dice Baseball — repriza {} ({}). Scor: J1 {} – J2 {}.\nBate J{} — outuri {}; baze: 1[{}] 2[{}] 3[{}]\n{}\n",
            self.inning,
            if self.batting == Player::One { "sus" } else { "jos" },
            self.runs[0],
            self.runs[1],
            self.batting.idx() + 1,
            self.outs,
            if self.bases[0] { "X" } else { " " },
            if self.bases[1] { "X" } else { " " },
            if self.bases[2] { "X" } else { " " },
            self.last
        )
    }

    fn format_move(&self, _: &()) -> String {
        "arunc".into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_terminate() {
        for seed in 1..=40u64 {
            let mut g = DiceBaseball::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                g.apply(&());
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn home_run_clears_the_bases() {
        let mut g = DiceBaseball::with_seed(1);
        g.bases = [true, true, true];
        assert_eq!(g.advance(4, true), 4);
        assert_eq!(g.bases, [false; 3]);
        g.bases = [true, false, true];
        assert_eq!(g.walk(), 0);
        assert_eq!(g.bases, [true, true, true]);
        assert_eq!(g.walk(), 1);
    }
}
