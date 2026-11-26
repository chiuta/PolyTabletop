//! Chicago (zaruri, Europa/SUA), 2 jucători, din regulile publice.
//!
//! * 11 runde; în runda `r` ținta este `r + 1` (2, 3, …, 12). La rândul tău arunci două zaruri (`arunc`); dacă
//!   suma e egală cu ținta, ia **atâtea puncte cât e ținta** (nu se dublează), altfel 0.
//! * Variantă „rearuncare” (Double Roll): dacă prima aruncare nu a nimerit ținta poți **păstra un zar** (`tin 1`
//!   sau `tin 2`, cel din stânga/dreapta) și rearunci celălalt o singură dată, sau renunți (`gata`).
//! * Aruncările sunt mutări forțate (`arunc`, `rearunc`), deci AI-ul nu cunoaște rezultatul înainte să aleagă.
//!   Câștigă cine are mai multe puncte după 11 runde.
//!
//! Mutări: `arunc`, `tin 1`, `tin 2`, `gata`, `rearunc`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 11;

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Roll,
    Keep(u8),
    Stop,
    Reroll,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Roll,
    Decide,
    Reroll(u8),
}

#[derive(Clone)]
pub struct Chicago {
    rng: Rng,
    round: u8,
    side: Player,
    phase: Phase,
    dice: [u8; 2],
    score: [i32; 2],
    log: String,
}

impl Default for Chicago {
    fn default() -> Self {
        Self::new()
    }
}

impl Chicago {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Chicago {
            rng: Rng::new(seed),
            round: 0,
            side: Player::One,
            phase: Phase::Roll,
            dice: [0, 0],
            score: [0, 0],
            log: String::new(),
        }
    }

    fn target(&self) -> i32 {
        self.round as i32 + 2
    }

    fn end_turn(&mut self) {
        self.phase = Phase::Roll;
        if self.side == Player::Two {
            self.round += 1;
        }
        self.side = self.side.other();
    }

    fn settle(&mut self) -> bool {
        let sum = self.dice[0] as i32 + self.dice[1] as i32;
        if sum == self.target() {
            self.score[self.side.idx()] += sum;
            self.log = format!(
                "J{}: {} + {} = {} (ținta) → +{}",
                self.side.idx() + 1,
                self.dice[0],
                self.dice[1],
                sum,
                sum
            );
            true
        } else {
            self.log = format!(
                "J{}: {} + {} = {} (ținta {}) — ratat",
                self.side.idx() + 1,
                self.dice[0],
                self.dice[1],
                sum,
                self.target()
            );
            false
        }
    }
}

impl Game for Chicago {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.round >= ROUNDS {
            return Vec::new();
        }
        match self.phase {
            Phase::Roll => vec![Move::Roll],
            Phase::Decide => vec![Move::Keep(1), Move::Keep(2), Move::Stop],
            Phase::Reroll(_) => vec![Move::Reroll],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Roll => {
                self.dice = [self.rng.d6(), self.rng.d6()];
                if self.settle() {
                    self.end_turn();
                } else {
                    self.phase = Phase::Decide;
                }
            }
            Move::Keep(k) => self.phase = Phase::Reroll(*k),
            Move::Stop => self.end_turn(),
            Move::Reroll => {
                if let Phase::Reroll(k) = self.phase {
                    let new = self.rng.d6();
                    self.dice[2 - k as usize] = new;
                    self.settle();
                    self.end_turn();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 100;
        if self.side == p {
            if let Phase::Reroll(k) = self.phase {
                let kept = self.dice[k as usize - 1] as i32;
                let need = self.target() - kept;
                if (1..=6).contains(&need) {
                    v += self.target() * 100 / 6;
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Chicago — runda {}/{ROUNDS}, ținta {}; scor J1 {} – J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.target(),
            self.score[0],
            self.score[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                if self.phase == Phase::Decide {
                    s.push_str(&format!(
                        "Zaruri: [{}] [{}] — păstrezi unul (tin 1/tin 2) sau «gata».\n",
                        self.dice[0], self.dice[1]
                    ));
                }
                s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Keep(k) => format!("tin {k}"),
            Move::Stop => "gata".into(),
            Move::Reroll => "rearunc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eleven_rounds() {
        for seed in 1..=20u64 {
            let mut g = Chicago::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 200);
            }
            assert_eq!(g.round, 11);
        }
    }

    #[test]
    fn hitting_the_target_scores_the_target() {
        let mut g = Chicago::with_seed(1);
        g.round = 5; // ținta 7
        g.dice = [3, 4];
        assert!(g.settle());
        assert_eq!(g.score[0], 7);
        g.dice = [3, 3];
        assert!(!g.settle());
        assert_eq!(g.score[0], 7);
    }
}
