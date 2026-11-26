#![allow(clippy::doc_lazy_continuation)]
//! Statuile (Statues / „Un, doi, trei, stop”), joc de copii cu alergare și înghețare, pentru doi
//! jucători.
//!
//! * Fiecare are un alergător care trebuie să ajungă la linia de 12. La rundele impare aleargă J1
//!   (J2 e cel care strigă „statuie!”), la cele pare aleargă J2.
//! * Alergătorul alege în ascuns câți pași face (1–3); cel care strigă alege în ascuns după câți
//!   pași strigă „stop” (1–3). Dacă pașii alergătorului sunt cel mult cât a strigat celălalt,
//!   avansează; dacă a făcut mai mulți, e prins în mișcare și se întoarce la start.
//! * Primul care ajunge la 12 sau mai mult câștigă; după 40 de runde câștigă cine e mai avansat.
//! Alegerea e simultană: J1 alege primul, J2 al doilea, rezultatul vine la pasul `dezvaluie`
//! (arbitru J2). Robotul nu vede alegerea adversarului (evaluare constantă): joacă uniform.
//! Mutări: `1`, `2`, `3`, `dezvaluie`.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const GOAL: u8 = 12;
const MAX_ROUNDS: u16 = 40;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    N(u8),
    Reveal,
}

#[derive(Clone)]
pub struct Statuile {
    pick: [Option<u8>; 2],
    pos: [u8; 2],
    round: u16,
    viewer: Viewer,
    log: String,
}

impl Default for Statuile {
    fn default() -> Self {
        Self::new()
    }
}

impl Statuile {
    pub fn new() -> Self {
        Statuile {
            pick: [None; 2],
            pos: [0; 2],
            round: 0,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    fn runner(&self) -> usize {
        (self.round % 2) as usize
    }

    fn over(&self) -> bool {
        self.pos.iter().any(|&p| p >= GOAL) || self.round >= MAX_ROUNDS
    }
}

impl Game for Statuile {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.pick[0].is_none() {
            Player::One
        } else {
            Player::Two
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pick[1].is_some() {
            return vec![Move::Reveal];
        }
        (1..=3).map(Move::N).collect()
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::N(k) => {
                if self.pick[0].is_none() {
                    self.pick[0] = Some(k);
                } else {
                    self.pick[1] = Some(k);
                }
            }
            Move::Reveal => {
                let r = self.runner();
                let (steps, stop) = (self.pick[r].unwrap(), self.pick[1 - r].unwrap());
                if steps <= stop {
                    self.pos[r] = (self.pos[r] + steps).min(GOAL);
                    self.log = format!(
                        "J{} a făcut {steps} pași (stop după {stop}): avansează la {}",
                        r + 1,
                        self.pos[r]
                    );
                } else {
                    self.pos[r] = 0;
                    self.log = format!("J{} a făcut {steps} pași, dar stop a venit după {stop}: prins, înapoi la start", r + 1);
                }
                self.pick = [None; 2];
                self.round += 1;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.pos[0].cmp(&self.pos[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let r = self.runner();
        let mut s = format!(
            "Statuile — linia de sosire {GOAL}. J1 la {}, J2 la {} (runda {}).\nAleargă J{}, strigă „stop” J{}.\n",
            self.pos[0],
            self.pos[1],
            self.round + 1,
            r + 1,
            2 - r
        );
        let me = self.viewer.who(self.to_move());
        match (self.pick[0], self.pick[1]) {
            (Some(a), Some(b)) => {
                s.push_str(&format!("Alegeri: J1 {a}, J2 {b}. Scrie «dezvaluie».\n"))
            }
            (Some(a), None) => {
                if let Viewer::HotSeat = self.viewer {
                    s.push_str(&handover_banner(Player::Two));
                }
                if me == Player::One {
                    s.push_str(&format!("J1 a ales {a} (ascuns pentru J2).\n"));
                } else {
                    s.push_str("J1 a ales în ascuns. Urmează J2.\n");
                }
            }
            _ => {}
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::N(k) => k.to_string(),
            Move::Reveal => "dezvaluie".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn caught_runner_goes_back_and_roles_alternate() {
        let mut g = Statuile::new();
        for m in [Move::N(2), Move::N(3), Move::Reveal] {
            g.apply(&m); // J1 aleargă cu 2, stop la 3: avansează
        }
        assert_eq!(g.pos, [2, 0]);
        for m in [Move::N(1), Move::N(3), Move::Reveal] {
            g.apply(&m); // acum aleargă J2 cu 3, J1 strigă la 1: prins
        }
        assert_eq!(g.pos, [2, 0]);
        assert_eq!(g.runner(), 0);
    }

    #[test]
    fn games_end_and_choice_is_hidden() {
        for seed in 1..=15u64 {
            let mut g = Statuile::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
        let mut g = Statuile::new();
        g.apply(&Move::N(3));
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains("a ales 3"));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("a ales 3"));
    }
}
