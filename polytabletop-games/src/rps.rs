//! Piatră, hârtie, foarfece (China, „shoushiling”, sec. XVII), 2 jucători, primul la 5 runde câștigate.
//!
//! * Piatra sparge foarfeca, foarfeca taie hârtia, hârtia acoperă piatra; la egalitate runda se
//!   reia (dar se numără la limita de 15 runde). Primul la 5 victorii câștigă; la 15 runde câștigă cine
//!   are mai multe (egal = remiză).
//! * Alegerile sunt secrete: J1 alege, apoi J2 alege, apoi J1 spune `arat` (mutare separată).
//!   Un jucător nu poate vedea alegerea celuilalt înainte de `arat`.
//! * AI-ul nu vede alegerea ascunsă: contrează frecvența alegerilor tale din rundele deja arătate
//!   (informație publică) și, în lipsă de istoric, joacă la întâmplare.
//!   Mutări: `piatra`, `hartie`, `foarfeca`, `arat`.

use crate::cards::Viewer;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const NAMES: [&str; 3] = ["piatra", "hartie", "foarfeca"];
const TARGET: u8 = 5;
const MAX_ROUNDS: u8 = 15;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Reveal,
}

#[derive(Clone)]
pub struct Rps {
    picks: [Option<u8>; 2],
    wins: [u8; 2],
    hist: [[u8; 3]; 2],
    round: u8,
    viewer: Viewer,
    log: String,
}

impl Default for Rps {
    fn default() -> Self {
        Self::new()
    }
}

/// 1 dacă `a` bate `b`, −1 dacă pierde, 0 la egalitate.
pub fn duel(a: u8, b: u8) -> i32 {
    if a == b {
        0
    } else if (a + 1) % 3 == b {
        // 0 piatră bate 2 foarfece, 1 hârtie bate 0 piatră, 2 foarfece bate 1 hârtie
        -1
    } else {
        1
    }
}

impl Rps {
    pub fn new() -> Self {
        Rps {
            picks: [None; 2],
            wins: [0; 2],
            hist: [[0; 3]; 2],
            round: 0,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    pub fn wins(&self) -> [u8; 2] {
        self.wins
    }
}

impl Game for Rps {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match (self.picks[0], self.picks[1]) {
            (None, _) => Player::One,
            (Some(_), None) => Player::Two,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match (self.picks[0], self.picks[1]) {
            (Some(_), Some(_)) => vec![Move::Reveal],
            _ => (0..3).map(Move::Pick).collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pick(c) => {
                let i = self.to_move().idx();
                self.picks[i] = Some(*c);
            }
            Move::Reveal => {
                let (a, b) = (self.picks[0].take().unwrap(), self.picks[1].take().unwrap());
                self.hist[0][a as usize] += 1;
                self.hist[1][b as usize] += 1;
                self.round += 1;
                let r = duel(a, b);
                if r > 0 {
                    self.wins[0] += 1;
                } else if r < 0 {
                    self.wins[1] += 1;
                }
                self.log = format!(
                    "Runda {}: J1 {} – J2 {} → {}",
                    self.round,
                    NAMES[a as usize],
                    NAMES[b as usize],
                    match r {
                        1 => "J1 câștigă runda",
                        -1 => "J2 câștigă runda",
                        _ => "egal",
                    }
                );
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.wins[0] >= TARGET || self.wins[1] >= TARGET || self.round >= MAX_ROUNDS {
            return Some(by_score(self.wins[0] as i32, self.wins[1] as i32));
        }
        None
    }

    /// Doar istoricul public al alegerilor arătate; alegerea ascunsă a adversarului nu e citită.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.wins[me] as i32 - self.wins[1 - me] as i32) * 1000;
        if let Some(c) = self.picks[me] {
            if self.to_move() != p || self.picks[1 - me].is_none() {
                for k in 0..3u8 {
                    v += duel(c, k) * self.hist[1 - me][k as usize] as i32 * 10;
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Piatră, hârtie, foarfece — runda {}/{MAX_ROUNDS}; victorii: J1 {}, J2 {} (la {TARGET} câștigă)\n",
            (self.round + 1).min(MAX_ROUNDS),
            self.wins[0],
            self.wins[1]
        );
        let v = self.viewer.who(self.to_move());
        for i in 0..2 {
            if let Some(c) = self.picks[i] {
                if v.idx() == i {
                    s.push_str(&format!(
                        "Alegerea ta (J{}): {} — secretă.\n",
                        i + 1,
                        NAMES[c as usize]
                    ));
                } else {
                    s.push_str(&format!("J{} a ales (ascuns).\n", i + 1));
                }
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let fin = verdict(self.outcome());
        if fin.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {}{}\n",
                self.to_move().idx() + 1,
                if self.picks[0].is_some() && self.picks[1].is_some() {
                    " — arat"
                } else {
                    ""
                }
            ));
        } else {
            s.push_str(&fin);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(c) => NAMES[*c as usize].into(),
            Move::Reveal => "arat".into(),
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
    fn rules() {
        assert_eq!(duel(0, 2), 1); // piatra bate foarfeca
        assert_eq!(duel(2, 1), 1); // foarfeca bate hartia
        assert_eq!(duel(1, 0), 1); // hartia bate piatra
        assert_eq!(duel(0, 1), -1);
        assert_eq!(duel(1, 1), 0);
    }

    #[test]
    fn hidden_pick_and_blind_ai() {
        let mut a = Rps::new();
        let mut b = Rps::new();
        a.apply(&Move::Pick(0));
        b.apply(&Move::Pick(1));
        for seed in 1..10 {
            assert_eq!(
                polytabletop_core::GameObj::ai_move(&a, seed),
                polytabletop_core::GameObj::ai_move(&b, seed)
            );
        }
        let mut two = a.clone();
        two.set_viewer(Some(Player::Two));
        assert!(!two.render().contains("piatra"), "{}", two.render());
        let mut one = a.clone();
        one.set_viewer(Some(Player::One));
        assert!(one.render().contains("piatra"));
    }

    #[test]
    fn ai_counters_a_repetitive_opponent() {
        let mut g = Rps::new();
        for _ in 0..3 {
            g.apply(&Move::Pick(0)); // J1 piatră
            g.apply(&Move::Pick(1));
            g.apply(&Move::Reveal);
        }
        g.apply(&Move::Pick(0)); // J1 încă piatră ascunsă; J2 (AI) răspunde
        let best = polytabletop_core::ai::best_move(&g, 1).unwrap();
        assert_eq!(best, Move::Pick(1), "hârtia bate piatra repetată");
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Rps::new(), 6, 200);
    }
}
