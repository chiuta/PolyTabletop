//! Cinci pietricele (Jacks / Knucklebones), joc de îndemânare pentru copii, 2 jucători, cu zarul
//! în locul mâinii: reușita unei încercări e decisă de noroc, iar tu alegi riscul.
//!
//! * 5 etape: „uni”, „doi”, „trei”, „patru” și „tot” (aruncarea și prinderea celor cinci pietricele).
//! * În tura ta alegi `calm` (reușești în 70 %, avansezi o etapă) sau `iute` (reușești în 45 %, avansezi
//!   două etape), apoi `incerc`. După o reușită continui; la o greșeală trece rândul.
//! * Câștigă primul care termină etapa a 5-a.
//!
//! AI: alege `iute` numai când poate termina jocul cu ea și `calm` nu poate; nu vede rezultatul viitor.

use crate::q2_util::end_line;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const GOAL: u8 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Calm,
    Fast,
    Try,
}

#[derive(Clone)]
pub struct CinciPietricele {
    rng: Rng,
    stage: [u8; 2],
    side: Player,
    pending: Option<Move>,
    log: String,
}

impl Default for CinciPietricele {
    fn default() -> Self {
        Self::new()
    }
}

fn odds(m: Move) -> (i32, u8) {
    if m == Move::Fast {
        (45, 2)
    } else {
        (70, 1)
    }
}

impl CinciPietricele {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        CinciPietricele {
            rng: Rng::new(seed),
            stage: [0; 2],
            side: Player::One,
            pending: None,
            log: String::new(),
        }
    }
    pub fn stages(&self) -> (u8, u8) {
        (self.stage[0], self.stage[1])
    }
}

impl Game for CinciPietricele {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            vec![Move::Try]
        } else {
            vec![Move::Calm, Move::Fast]
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Calm | Move::Fast => {
                self.pending = Some(*m);
                self.log.clear();
            }
            Move::Try => {
                let kind = self.pending.take().unwrap_or(Move::Calm);
                let (pct, gain) = odds(kind);
                let me = self.side.idx();
                if (self.rng.below(100) as i32) < pct {
                    self.stage[me] = (self.stage[me] + gain).min(GOAL);
                    self.log = format!("Reușit! J{} la etapa {}.", me + 1, self.stage[me]);
                } else {
                    self.log = format!("Ai scăpat o pietricică. Rândul trece la J{}.", 2 - me);
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.stage[0] >= GOAL {
            Some(Outcome::Win(Player::One))
        } else if self.stage[1] >= GOAL {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.stage[me] as i32 - self.stage[1 - me] as i32) * 100;
        if let (Some(k), true) = (self.pending, self.side == p) {
            let (pct, gain) = odds(k);
            let reach = self.stage[me] + gain >= GOAL;
            v += pct * gain as i32 + pct * 6 / 10 - (100 - pct) * 15 / 10;
            if reach {
                v += pct * 10;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Cinci pietricele — etapa: J1 {}/{GOAL}, J2 {}/{GOAL}\n",
            self.stage[0], self.stage[1]
        );
        if let Some(k) = self.pending {
            s.push_str(&format!(
                "J{}: ai ales {} — scrie «incerc».\n",
                self.side.idx() + 1,
                if k == Move::Fast { "iute" } else { "calm" }
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&end_line(self.outcome(), self.side));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Calm => "calm",
            Move::Fast => "iute",
            Move::Try => "incerc",
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
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = CinciPietricele::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn ai_picks_by_odds_not_by_peeking() {
        let mut a = CinciPietricele::with_seed(1);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1).as_deref(),
            Some("calm")
        );
        a.stage[0] = 3; // iute termină jocul (3+2), calm nu
        let mut b = a.clone();
        b.rng = Rng::new(9);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
