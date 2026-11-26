#![allow(clippy::doc_lazy_continuation)]
//! Craps (SUA), dueluri la masa de zaruri pentru doi jucători.
//!
//! * „Shooterul” (J1 la prima rundă, apoi alternativ) alege miza (5, 10 sau 20 de jetoane) și
//!   pariul: `pass` (pass line) sau `dont` (don't pass); celălalt jucător acoperă pariul opus.
//! * Come-out: 7 sau 11 = pass câștigă; 2, 3 sau 12 = „craps”: pass pierde (don't pass câștigă
//!   la 2 și 3, la 12 miza se anulează); orice alt total devine „punct”. Se aruncă (două zaruri)
//!   până apare punctul (pass câștigă) sau 7 (pass pierde).
//! * Fiecare jucător începe cu 100 de jetoane; 16 runde; cine rămâne fără jetoane pierde, altfel
//!   câștigă cine are mai multe la final.
//! Mutări: `pass 10`, `dont 5`…, apoi pasul forțat `arunc` (arbitru = J2). AI-ul alege miza după
//! valoarea așteptată (don't pass are o ușoară avansă) și după scor; nu vede zarurile viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 100;
const ROUNDS: u8 = 16;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bet { pass: bool, stake: i32 },
    Roll,
}

#[derive(Clone)]
pub struct Craps {
    rng: Rng,
    chips: [i32; 2],
    shooter: Player,
    bet: Option<(bool, i32)>,
    point: u8,
    done: u8,
    last: String,
}

impl Default for Craps {
    fn default() -> Self {
        Self::new()
    }
}

impl Craps {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Craps {
            rng: Rng::new(seed),
            chips: [START; 2],
            shooter: Player::One,
            bet: None,
            point: 0,
            done: 0,
            last: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.done >= ROUNDS || self.chips.iter().any(|&c| c <= 0)
    }

    /// `pass_wins`: Some(true) = pass câștigă, Some(false) = don't pass, None = anulat.
    fn settle(&mut self, pass_wins: Option<bool>) {
        let (pass, stake) = self.bet.take().unwrap();
        let s = self.shooter.idx();
        let o = 1 - s;
        if let Some(pw) = pass_wins {
            let shooter_wins = pw == pass;
            let (w, l) = if shooter_wins { (s, o) } else { (o, s) };
            self.chips[w] += stake;
            self.chips[l] -= stake;
        }
        self.done += 1;
        self.point = 0;
        self.shooter = self.shooter.other();
    }
}

impl Game for Craps {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.bet.is_some() {
            Player::Two
        } else {
            self.shooter
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.bet.is_some() {
            return vec![Move::Roll];
        }
        let cap = self.chips[0].min(self.chips[1]);
        let mut v = Vec::new();
        for pass in [true, false] {
            for &stake in STAKES.iter().filter(|&&s| s <= cap) {
                v.push(Move::Bet { pass, stake });
            }
            if cap < STAKES[0] {
                v.push(Move::Bet { pass, stake: cap });
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Bet { pass, stake } => {
                self.bet = Some((pass, stake));
                self.point = 0;
            }
            Move::Roll => {
                let t = self.rng.d6() + self.rng.d6();
                if self.point == 0 {
                    match t {
                        7 | 11 => {
                            self.last = format!("come-out {t}: pass câștigă");
                            self.settle(Some(true));
                        }
                        2 | 3 => {
                            self.last = format!("come-out {t} (craps): don't pass câștigă");
                            self.settle(Some(false));
                        }
                        12 => {
                            self.last = "come-out 12: miza se anulează".into();
                            self.settle(None);
                        }
                        _ => {
                            self.point = t;
                            self.last = format!("come-out {t}: punctul este {t}");
                        }
                    }
                } else if t == self.point {
                    self.last = format!("{t}: punctul e făcut, pass câștigă");
                    self.settle(Some(true));
                } else if t == 7 {
                    self.last = "7 înainte de punct: don't pass câștigă".into();
                    self.settle(Some(false));
                } else {
                    self.last = format!("{t}: se aruncă din nou (punct {})", self.point);
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar jetoanele, miza aleasă și valoarea așteptată (nu zarurile viitoare).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let diff = self.chips[me] - self.chips[1 - me];
        let mut v = diff * 100;
        if let Some((pass, stake)) = self.bet {
            if self.point == 0 {
                let ev = if pass { -0.0141 } else { -0.0136 } * stake as f64;
                let ev = if self.shooter == p { ev } else { -ev };
                v += (ev * 100.0).round() as i32;
                let risk = stake
                    * if diff < 0 {
                        8
                    } else if diff > 0 {
                        -8
                    } else {
                        0
                    };
                v += if self.shooter == p { risk } else { -risk };
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Craps — runda {}/{ROUNDS}. Jetoane: J1 {}, J2 {} (start {START}).\nShooter: J{}.\n",
            self.done + 1,
            self.chips[0],
            self.chips[1],
            self.shooter.idx() + 1
        );
        if let Some((pass, stake)) = self.bet {
            s.push_str(&format!(
                "Pariu: {} {stake}. {}\n",
                if pass { "pass" } else { "don't pass" },
                if self.point > 0 {
                    format!("Punct: {}.", self.point)
                } else {
                    "Come-out.".into()
                }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima aruncare: {}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet { pass, stake } => format!("{} {stake}", if *pass { "pass" } else { "dont" }),
            Move::Roll => "arunc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn come_out_rules() {
        let mut g = Craps::with_seed(1);
        g.apply(&Move::Bet {
            pass: true,
            stake: 10,
        });
        g.point = 6;
        g.settle(Some(true));
        assert_eq!(g.chips, [110, 90]);
        assert_eq!(g.shooter, Player::Two);
        g.bet = Some((false, 5));
        g.settle(Some(true)); // pass câștigă, shooterul (J2) a jucat don't pass
        assert_eq!(g.chips, [115, 85]);
        g.bet = Some((true, 20));
        g.settle(None);
        assert_eq!(g.chips, [115, 85]);
    }

    #[test]
    fn random_games_end_with_conserved_chips() {
        for seed in 1..=30u64 {
            let mut g = Craps::with_seed(seed);
            let mut rng = Rng::new(seed + 7);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 5000);
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
            }
        }
    }

    #[test]
    fn ai_choice_does_not_depend_on_future_rolls() {
        let a = Craps::with_seed(1);
        let b = Craps::with_seed(99);
        let ma = polytabletop_core::GameObj::ai_move(&a, 5);
        let mb = polytabletop_core::GameObj::ai_move(&b, 5);
        assert_eq!(ma, mb);
    }
}
