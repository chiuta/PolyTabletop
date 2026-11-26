//! Morra (Mora), Italia: fiecare jucător arată simultan 1–5 degete și strigă suma presupusă
//! a degetelor ambilor. Dacă exact unul ghicește suma, ia un punct; dacă ghicesc amândoi sau
//! niciunul, runda se reia. Primul la 7 puncte câștigă.
//! Alegerea (degete/ghicire) e simultană: J1 alege ascuns, apoi J2; rezultatul se stabilește
//! la pasul forțat `dezvaluie` (arbitru = J2). AI-ul nu vede alegerea adversarului (evaluare
//! constantă), deci joacă uniform. Mutări: `3/7` (3 degete, suma presupusă 7: între degete+1
//! și degete+5), `dezvaluie`.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: u8 = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Show(u8, u8),
    Reveal,
}

#[derive(Clone)]
pub struct Morra {
    pick: [Option<(u8, u8)>; 2],
    pts: [u8; 2],
    rounds: u16,
    viewer: Viewer,
    log: String,
}

impl Default for Morra {
    fn default() -> Self {
        Self::new()
    }
}

impl Morra {
    pub fn new() -> Self {
        Morra {
            pick: [None; 2],
            pts: [0; 2],
            rounds: 0,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.pts.iter().any(|&p| p >= TARGET) || self.rounds >= 120
    }
}

impl Game for Morra {
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
        let mut v = Vec::new();
        for f in 1..=5u8 {
            for g in f + 1..=f + 5 {
                v.push(Move::Show(f, g));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Show(f, g) => {
                if self.pick[0].is_none() {
                    self.pick[0] = Some((f, g));
                } else {
                    self.pick[1] = Some((f, g));
                }
            }
            Move::Reveal => {
                let (a, b) = (self.pick[0].unwrap(), self.pick[1].unwrap());
                let sum = a.0 + b.0;
                self.rounds += 1;
                let (ra, rb) = (a.1 == sum, b.1 == sum);
                self.log = format!(
                    "degete {}+{}={} (J1 a zis {}, J2 a zis {}): {}",
                    a.0,
                    b.0,
                    sum,
                    a.1,
                    b.1,
                    match (ra, rb) {
                        (true, false) => {
                            self.pts[0] += 1;
                            "punct pentru J1"
                        }
                        (false, true) => {
                            self.pts[1] += 1;
                            "punct pentru J2"
                        }
                        _ => "nimeni (sau amândoi) — se reia",
                    }
                );
                self.pick = [None; 2];
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Morra — primul la {TARGET}. Scor: J1 {} – J2 {} (runda {}).\n",
            self.pts[0],
            self.pts[1],
            self.rounds + 1
        );
        let me = self.viewer.who(self.to_move());
        match (self.pick[0], self.pick[1]) {
            (Some(a), Some(b)) => s.push_str(&format!(
                "J1: {} degete, suma {}; J2: {} degete, suma {}. Scrie «dezvaluie».\n",
                a.0, a.1, b.0, b.1
            )),
            (Some(a), None) => {
                if let Viewer::HotSeat = self.viewer {
                    s.push_str(&handover_banner(Player::Two));
                }
                if me == Player::One {
                    s.push_str(&format!("J1 a ales {}/{} (ascuns pentru J2).\n", a.0, a.1));
                } else {
                    s.push_str("J1 a ales în ascuns. Urmează J2.\n");
                }
            }
            _ => {}
        }
        if !self.log.is_empty() {
            s.push_str(&format!("Ultima rundă: {}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Show(f, g) => format!("{f}/{g}"),
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
    fn only_one_correct_guess_scores() {
        let mut g = Morra::new();
        for m in [Move::Show(2, 5), Move::Show(3, 5), Move::Reveal] {
            g.apply(&m);
        }
        assert_eq!(g.pts, [0, 0]); // ambii au ghicit 5
        for m in [Move::Show(2, 5), Move::Show(3, 8), Move::Reveal] {
            g.apply(&m);
        }
        assert_eq!(g.pts, [1, 0]);
    }

    #[test]
    fn guess_range_and_random_games_end() {
        let g = Morra::new();
        assert_eq!(g.legal_moves().len(), 25);
        for seed in 1..=10u64 {
            let mut g = Morra::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    #[test]
    fn second_player_does_not_see_first_choice() {
        let mut g = Morra::new();
        g.apply(&Move::Show(4, 7));
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains("4/7"));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("4/7"));
    }
}
