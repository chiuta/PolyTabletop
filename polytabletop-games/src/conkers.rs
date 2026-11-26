#![allow(clippy::doc_lazy_continuation)]
//! Conkers (Marea Britanie), jocul cu castane de cal legate pe sfoară, pentru doi jucători.
//!
//! * Fiecare are o castană cu 6 puncte de rezistență. Jucătorii lovesc pe rând (J1 primul): cel
//!   care lovește alege `usor` (70% reușită, 1 pagubă castanei adverse) sau `tare` (45% reușită,
//!   2 pagube; la o ratare, 25% șanse ca sfoara să se încurce și castana proprie să ia 1 pagubă).
//! * Prima castană ale cărei puncte ajung la 0 se sparge: acel jucător pierde. După 60 de lovituri
//!   câștigă cine are castana mai întreagă (egal = remiză).
//! * Rezultatul se stabilește la pasul forțat `lovitura` (arbitru J2); AI-ul alege după
//!   valoarea așteptată, fără să vadă zarurile.
//! Mutări: `usor`, `tare`, `lovitura`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const HP: i32 = 6;
const MAX_STRIKES: u16 = 60;

fn params(hard: bool) -> (f64, i32) {
    if hard {
        (0.45, 2)
    } else {
        (0.70, 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Light,
    Hard,
    Strike,
}

#[derive(Clone)]
pub struct Conkers {
    rng: Rng,
    hp: [i32; 2],
    side: Player,
    pending: Option<bool>,
    strikes: u16,
    last: String,
}

impl Default for Conkers {
    fn default() -> Self {
        Self::new()
    }
}

impl Conkers {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Conkers {
            rng: Rng::new(seed),
            hp: [HP; 2],
            side: Player::One,
            pending: None,
            strikes: 0,
            last: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.hp.iter().any(|&h| h <= 0) || self.strikes >= MAX_STRIKES
    }
}

impl Game for Conkers {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            Player::Two
        } else {
            self.side
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Strike];
        }
        vec![Move::Light, Move::Hard]
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Light => self.pending = Some(false),
            Move::Hard => self.pending = Some(true),
            Move::Strike => {
                let hard = self.pending.take().unwrap();
                let (p, dmg) = params(hard);
                let me = self.side.idx();
                let you = 1 - me;
                if (self.rng.below(1000) as f64) < p * 1000.0 {
                    self.hp[you] -= dmg;
                    self.last = format!(
                        "J{} lovește {} și reușește: castana lui J{} pierde {dmg}",
                        me + 1,
                        if hard { "tare" } else { "ușor" },
                        you + 1
                    );
                } else if hard && self.rng.below(4) == 0 {
                    self.hp[me] -= 1;
                    self.last = format!(
                        "J{} ratează, sfoara se încurcă: castana lui pierde 1",
                        me + 1
                    );
                } else {
                    self.last = format!("J{} ratează", me + 1);
                }
                self.strikes += 1;
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.hp[0].cmp(&self.hp[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.hp[p.idx()] - self.hp[p.other().idx()]) * 100;
        if let Some(hard) = self.pending {
            let (pr, dmg) = params(hard);
            let mut ev = pr * dmg as f64;
            if hard {
                ev -= (1.0 - pr) * 0.25;
            }
            v += if self.side == p {
                (ev * 100.0) as i32
            } else {
                -(ev * 100.0) as i32
            };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Conkers — rezistența castanelor: J1 {}, J2 {} (lovitura {}/{MAX_STRIKES}).\n",
            self.hp[0].max(0),
            self.hp[1].max(0),
            self.strikes + 1
        );
        if !self.over() {
            s.push_str(&format!(
                "Lovește J{}: `usor` sau `tare`.\n",
                self.side.idx() + 1
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Light => "usor",
            Move::Hard => "tare",
            Move::Strike => "lovitura",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_end_and_ai_is_blind() {
        for seed in 1..=30u64 {
            let mut g = Conkers::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
        let (a, b) = (Conkers::with_seed(1), Conkers::with_seed(2));
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 5)
        );
    }

    #[test]
    fn broken_conker_loses() {
        let mut g = Conkers::with_seed(3);
        g.hp = [1, 6];
        assert!(g.outcome().is_none());
        g.hp = [0, 6];
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }
}
