//! Howzat! (dice cricket), joc de zaruri inspirat de cricket, 2 jucători (variantă proprie de reguli publice simple).
//!
//! * Două reprize de câte 30 de mingi (5 over-uri) și 5 wicket-uri; J1 bate primul, J2 încearcă să depășească.
//! * La fiecare minge, bătătorul alege `lovesc` (atac) sau `apar` (apărare), apoi aruncătorul (`arunc`) dă zarul:
//!   atac: 1–2 = **Howzat! (out)**, 3 = 1 run, 4 = 2, 5 = 4, 6 = 6;
//!   apărare: 1 = **out**, 2–3 = 0 runs, 4–5 = 1, 6 = 2.
//! * Repriza se termină la 30 de mingi, la 5 outuri sau (reprize 2) când ținta e depășită.
//!   Câștigă cine are mai multe runs (egal = remiză).

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const BALLS: i32 = 30;
const WICKETS: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Attack,
    Defend,
    Bowl,
}

#[derive(Clone)]
pub struct Howzat {
    rng: Rng,
    runs: [i32; 2],
    innings: u8,
    balls_left: i32,
    outs: i32,
    pending: Option<Move>,
    over: bool,
    log: String,
}

fn table(attack: bool, face: u8) -> Option<i32> {
    if attack {
        match face {
            1 | 2 => None,
            3 => Some(1),
            4 => Some(2),
            5 => Some(4),
            _ => Some(6),
        }
    } else {
        match face {
            1 => None,
            2 | 3 => Some(0),
            4 | 5 => Some(1),
            _ => Some(2),
        }
    }
}

impl Default for Howzat {
    fn default() -> Self {
        Self::new()
    }
}

impl Howzat {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Howzat {
            rng: Rng::new(seed),
            runs: [0; 2],
            innings: 1,
            balls_left: BALLS,
            outs: 0,
            pending: None,
            over: false,
            log: String::new(),
        }
    }
    pub fn runs(&self) -> (i32, i32) {
        (self.runs[0], self.runs[1])
    }
    fn batter(&self) -> Player {
        if self.innings == 1 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn end_innings_check(&mut self) {
        let chased = self.innings == 2 && self.runs[1] > self.runs[0];
        if self.balls_left == 0 || self.outs >= WICKETS || chased {
            if self.innings == 1 {
                self.innings = 2;
                self.balls_left = BALLS;
                self.outs = 0;
            } else {
                self.over = true;
            }
        }
    }
}

impl Game for Howzat {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            self.batter().other()
        } else {
            self.batter()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.pending.is_some() {
            vec![Move::Bowl]
        } else {
            vec![Move::Attack, Move::Defend]
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Attack | Move::Defend => {
                self.pending = Some(*m);
                self.log.clear();
            }
            Move::Bowl => {
                let attack = self.pending.take() == Some(Move::Attack);
                let face = self.rng.d6();
                let b = self.batter().idx();
                self.balls_left -= 1;
                match table(attack, face) {
                    None => {
                        self.outs += 1;
                        self.log = format!("Zar {face}: HOWZAT! Out ({}/{WICKETS}).", self.outs);
                    }
                    Some(r) => {
                        self.runs[b] += r;
                        self.log = format!("Zar {face}: {r} run(s).");
                    }
                }
                self.end_innings_check();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over {
            Some(by_score(self.runs[0], self.runs[1]))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let bat = self.batter();
        let wl = (WICKETS - self.outs).max(1) as f64;
        let bl = self.balls_left as f64;
        let mut ev = 0.0;
        if let Some(c) = self.pending {
            let attack = c == Move::Attack;
            let (e, po) = if attack {
                (13.0 / 6.0, 2.0 / 6.0)
            } else {
                (4.0 / 6.0, 1.0 / 6.0)
            };
            let need = if self.innings == 2 {
                (self.runs[0] - self.runs[1] + 1) as f64
            } else {
                0.0
            };
            let rr = if self.innings == 2 {
                need / bl.max(1.0)
            } else {
                0.0
            };
            let mut cost = 4.0 + 2.0 * bl / wl;
            if rr > 2.0 {
                cost *= 0.5;
            }
            ev = e - po * cost;
        }
        let diff = (self.runs[p.idx()] - self.runs[1 - p.idx()]) as f64 * 10.0;
        let bonus = if bat == p { ev * 10.0 } else { -ev * 10.0 };
        (diff + bonus) as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Howzat! — repriza {}: J{} bate. Runs: J1 {}, J2 {}. Mingi rămase {}, outuri {}/{WICKETS}\n",
            self.innings,
            self.batter().idx() + 1,
            self.runs[0],
            self.runs[1],
            self.balls_left,
            self.outs
        );
        if self.innings == 2 && !self.over {
            s.push_str(&format!("Ținta: {} runs.\n", self.runs[0] + 1));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&end_line(self.outcome(), self.to_move()));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Attack => "lovesc",
            Move::Defend => "apar",
            Move::Bowl => "arunc",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_and_innings() {
        assert_eq!(table(true, 2), None);
        assert_eq!(table(true, 6), Some(6));
        assert_eq!(table(false, 1), None);
        assert_eq!(table(false, 6), Some(2));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = Howzat::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 400);
            }
        }
    }

    #[test]
    fn ai_never_sees_the_die() {
        let a = Howzat::with_seed(1);
        let mut b = a.clone();
        b.rng = Rng::new(31);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 8),
            polytabletop_core::GameObj::ai_move(&b, 8)
        );
    }
}
