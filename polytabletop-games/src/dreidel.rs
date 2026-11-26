//! Dreidel (Orientul Mijlociu / Europa), titirezul cu patru fețe, 2 jucători.
//!
//! * Fiecare jucător începe cu 10 jetoane. La început (și ori de câte ori potul e gol sau are un singur
//!   jeton) fiecare pune 1 jeton în pot.
//! * Pe rând, fiecare învârte titirezul (`invart`): **nun** = nimic; **gimel** = iei tot potul;
//!   **hei** = iei jumătate din pot (rotunjit în sus); **shin** = pui 1 jeton în pot.
//! * Cine nu mai are jetoane când trebuie să plătească pierde. După 300 de învârtiri câștigă cine are
//!   mai multe jetoane (egal = remiză).
//!
//! Jocul de noroc pur: singura mutare este `invart` (rezultatul se alege la învârtire, AI-ul nu îl vede).

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 10;
const MAX_SPINS: u16 = 300;

#[derive(Clone)]
pub struct Dreidel {
    rng: Rng,
    chips: [i32; 2],
    pot: i32,
    side: Player,
    spins: u16,
    loser: Option<Player>,
    log: String,
}

impl Default for Dreidel {
    fn default() -> Self {
        Self::new()
    }
}

impl Dreidel {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Dreidel {
            rng: Rng::new(seed),
            chips: [START; 2],
            pot: 0,
            side: Player::One,
            spins: 0,
            loser: None,
            log: String::new(),
        };
        g.ante();
        g
    }
    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }
    fn ante(&mut self) {
        if self.pot > 1 {
            return;
        }
        for p in 0..2 {
            if self.chips[p] == 0 {
                self.loser = Some(if p == 0 { Player::One } else { Player::Two });
                return;
            }
            self.chips[p] -= 1;
            self.pot += 1;
        }
    }
}

impl Game for Dreidel {
    type Move = ();

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<()> {
        if self.outcome().is_some() {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn apply(&mut self, _m: &()) {
        let me = self.side.idx();
        let face = self.rng.below(4);
        self.spins += 1;
        match face {
            0 => self.log = "nun: nimic.".into(),
            1 => {
                self.chips[me] += self.pot;
                self.log = format!("gimel: iei tot potul ({}).", self.pot);
                self.pot = 0;
            }
            2 => {
                let half = (self.pot + 1) / 2;
                self.chips[me] += half;
                self.pot -= half;
                self.log = format!("hei: iei jumătate ({half}).");
            }
            _ => {
                if self.chips[me] == 0 {
                    self.loser = Some(self.side);
                } else {
                    self.chips[me] -= 1;
                    self.pot += 1;
                }
                self.log = "shin: pui 1 jeton în pot.".into();
            }
        }
        self.side = self.side.other();
        if self.loser.is_none() {
            self.ante();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(l) = self.loser {
            return Some(Outcome::Win(l.other()));
        }
        if self.spins >= MAX_SPINS {
            return Some(by_score(self.chips[0], self.chips[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.chips[p.idx()] - self.chips[1 - p.idx()]
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Dreidel — jetoane: J1 {}, J2 {}; pot {}; învârtiri {}/{MAX_SPINS}\n",
            self.chips[0], self.chips[1], self.pot, self.spins
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&end_line(self.outcome(), self.side));
        s
    }

    fn format_move(&self, _m: &()) -> String {
        "invart".into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chips_plus_pot_are_conserved_and_games_end() {
        for seed in 1..=30u64 {
            let mut g = Dreidel::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                g.apply(&());
                assert_eq!(g.chips[0] + g.chips[1] + g.pot, 2 * START);
                n += 1;
                assert!(n <= 301);
            }
        }
    }

    #[test]
    fn single_move_and_ai() {
        let g = Dreidel::with_seed(2);
        assert_eq!(
            polytabletop_core::GameObj::legal_moves(&g),
            vec!["invart".to_string()]
        );
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&g, 1).as_deref(),
            Some("invart")
        );
    }
}
