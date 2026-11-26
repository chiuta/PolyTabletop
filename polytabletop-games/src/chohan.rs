//! Chō-han bakuchi (Japonia): două zaruri aruncate sub un pahar; se pariază pe suma lor,
//! **chō** (pară) sau **han** (impară).
//!
//! * Pentru doi jucători: în fiecare rundă „chemătorul” alege partea și miza (`cho 10` sau `han 5`,
//!   mize 5, 10 sau 20, cel mult jetoanele celui mai sărac); celălalt primește automat partea
//!   opusă cu aceeași miză (ca la mesele reale, unde partea de joc echilibrează pariurile).
//!   Apoi `agit` (mutare separată: zarurile sunt aruncate și paharul ridicat).
//! * Cine a nimerit partea ia miza de la celălalt. Rolul de chemător alternează. 10 runde,
//!   start 100 de jetoane; câștigă cine are mai multe la final (sau cine nu rămâne fără jetoane).
//!   Mutări: `cho 10`, `han 5`, `agit`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 10;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Call(bool, i32), // true = cho (pară)
    Shake,
}

#[derive(Clone)]
pub struct Chohan {
    chips: [i32; 2],
    caller: Player,
    call: Option<(bool, i32)>,
    round: u8,
    rng: Rng,
    last: String,
}

impl Default for Chohan {
    fn default() -> Self {
        Self::new()
    }
}

impl Chohan {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Chohan {
            chips: [100; 2],
            caller: Player::One,
            call: None,
            round: 0,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }
}

impl Game for Chohan {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.caller
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.call.is_some() {
            return vec![Move::Shake];
        }
        let cap = self.chips[0].min(self.chips[1]);
        let mut stakes: Vec<i32> = [5, 10, 20].into_iter().filter(|&s| s <= cap).collect();
        if stakes.is_empty() {
            stakes.push(cap);
        }
        let mut v = Vec::new();
        for s in stakes {
            v.push(Move::Call(true, s));
            v.push(Move::Call(false, s));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Call(side, s) => self.call = Some((*side, *s)),
            Move::Shake => {
                let (cho, stake) = self.call.take().unwrap();
                let (a, b) = (self.rng.d6(), self.rng.d6());
                let even = (a + b) % 2 == 0;
                let c = self.caller.idx();
                let caller_wins = even == cho;
                if caller_wins {
                    self.chips[c] += stake;
                    self.chips[1 - c] -= stake;
                } else {
                    self.chips[c] -= stake;
                    self.chips[1 - c] += stake;
                }
                self.last = format!(
                    "Zaruri {a}+{b} = {} → {}: J{} {} {stake}.",
                    a + b,
                    if even { "chō" } else { "han" },
                    c + 1,
                    if caller_wins { "câștigă" } else { "pierde" }
                );
                self.round += 1;
                self.caller = self.caller.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round >= ROUNDS || self.chips[0] <= 0 || self.chips[1] <= 0 {
            return Some(by_score(self.chips[0], self.chips[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let diff = self.chips[me] - self.chips[1 - me];
        let mut v = diff * 100;
        if let Some((_, stake)) = self.call {
            // jocul e corect (50/50): doar riscul contează; în urmă = varianță mai mare
            let left = (ROUNDS - self.round) as i32;
            let k = if diff < 0 {
                8
            } else if diff > 0 {
                -8
            } else {
                0
            };
            v += k * stake * left;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Chō-han — runda {}/{ROUNDS}. Jetoane: J1 {}, J2 {}. Chemător: J{}.\n",
            (self.round + 1).min(ROUNDS),
            self.chips[0],
            self.chips[1],
            self.caller.idx() + 1
        );
        if let Some((cho, st)) = self.call {
            s.push_str(&format!(
                "Chemarea: {} pe {st}; adversarul are partea opusă.\n",
                if cho { "chō (pară)" } else { "han (impară)" }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — {}\n",
                self.caller.idx() + 1,
                if self.call.is_some() {
                    "agit"
                } else {
                    "cho N / han N"
                }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Call(true, s) => format!("cho {s}"),
            Move::Call(false, s) => format!("han {s}"),
            Move::Shake => "agit".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn chips_move_between_players_only() {
        let mut g = Chohan::with_seed(4);
        g.apply(&Move::Call(true, 10));
        assert_eq!(g.legal_moves(), vec![Move::Shake]);
        g.apply(&Move::Shake);
        assert_eq!(g.chips[0] + g.chips[1], 200);
        assert_eq!(g.caller, Player::Two);
        assert_eq!(g.round, 1);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Chohan::with_seed, 6, 500);
    }
}
