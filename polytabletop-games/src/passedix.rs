#![allow(clippy::doc_lazy_continuation)]
//! Passe-dix (Franța, secolul XVII), joc de zaruri cu trei zaruri pentru doi jucători.
//!
//! * Se aruncă trei zaruri: total peste 10 (11–18) = „passe”, 10 sau mai puțin (3–10) = „manque”.
//!   O arătură cu trei zaruri identice se aruncă din nou (varianta „Paschen”/băncii).
//! * „Banca” (J1 la început, apoi alternativ) alege miza (5, 10, 20) și partea (`passe` sau
//!   `manque`); celălalt jucător acoperă partea opusă. Câștigă cine are partea ieșită.
//! * 20 de runde, fiecare jucător pornește cu 100 de jetoane; mai multe jetoane la final câștigă.
//! Mutări: `passe 10`, `manque 5`…, apoi pasul forțat `arunc`. Șanse egale (50%): AI-ul alege
//! miza după scor, fără să vadă zarurile.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 100;
const ROUNDS: u8 = 20;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bet { passe: bool, stake: i32 },
    Roll,
}

#[derive(Clone)]
pub struct PasseDix {
    rng: Rng,
    chips: [i32; 2],
    banker: Player,
    bet: Option<(bool, i32)>,
    done: u8,
    last: String,
}

impl Default for PasseDix {
    fn default() -> Self {
        Self::new()
    }
}

impl PasseDix {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        PasseDix {
            rng: Rng::new(seed),
            chips: [START; 2],
            banker: Player::One,
            bet: None,
            done: 0,
            last: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.done >= ROUNDS || self.chips.iter().any(|&c| c <= 0)
    }
}

impl Game for PasseDix {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.bet.is_some() {
            Player::Two
        } else {
            self.banker
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
        for passe in [true, false] {
            for &stake in STAKES.iter().filter(|&&s| s <= cap) {
                v.push(Move::Bet { passe, stake });
            }
            if cap < STAKES[0] {
                v.push(Move::Bet { passe, stake: cap });
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Bet { passe, stake } => self.bet = Some((passe, stake)),
            Move::Roll => {
                let (passe, stake) = self.bet.take().unwrap();
                let d = loop {
                    let d = [self.rng.d6(), self.rng.d6(), self.rng.d6()];
                    if !(d[0] == d[1] && d[1] == d[2]) {
                        break d;
                    }
                };
                let t: u8 = d.iter().sum();
                let passe_out = t > 10;
                let b = self.banker.idx();
                let (w, l) = if passe_out == passe {
                    (b, 1 - b)
                } else {
                    (1 - b, b)
                };
                self.chips[w] += stake;
                self.chips[l] -= stake;
                self.last = format!(
                    "{}+{}+{} = {t}: {}, J{} câștigă {stake}",
                    d[0],
                    d[1],
                    d[2],
                    if passe_out { "passe" } else { "manque" },
                    w + 1
                );
                self.done += 1;
                self.banker = self.banker.other();
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

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let diff = self.chips[me] - self.chips[1 - me];
        let mut v = diff * 100;
        if let Some((_, stake)) = self.bet {
            let risk = stake
                * if diff < 0 {
                    8
                } else if diff > 0 {
                    -8
                } else {
                    0
                };
            v += if self.banker == p { risk } else { -risk };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Passe-dix — runda {}/{ROUNDS}. Jetoane: J1 {}, J2 {} (start {START}).\nBanca: J{}.\n",
            self.done + 1,
            self.chips[0],
            self.chips[1],
            self.banker.idx() + 1
        );
        if let Some((passe, stake)) = self.bet {
            s.push_str(&format!(
                "Pariu: {} {stake}. Scrie «arunc».\n",
                if passe {
                    "passe (11–18)"
                } else {
                    "manque (3–10)"
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
            Move::Bet { passe, stake } => {
                format!("{} {stake}", if *passe { "passe" } else { "manque" })
            }
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
    fn bets_and_conservation() {
        let g = PasseDix::with_seed(3);
        assert_eq!(g.legal_moves().len(), 6);
        for seed in 1..=30u64 {
            let mut g = PasseDix::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 2000);
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
            }
        }
    }

    #[test]
    fn roll_never_triple_and_ai_blind() {
        let mut g = PasseDix::with_seed(8);
        for _ in 0..200 {
            if g.outcome().is_some() {
                break;
            }
            g.apply(&Move::Bet {
                passe: true,
                stake: 5,
            });
            g.apply(&Move::Roll);
            assert!(!g.last.contains("1+1+1"));
        }
        let (a, b) = (PasseDix::with_seed(1), PasseDix::with_seed(2));
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 4),
            polytabletop_core::GameObj::ai_move(&b, 4)
        );
    }
}
