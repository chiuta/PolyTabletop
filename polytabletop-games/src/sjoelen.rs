//! Sjoelen (Olanda): „șuflecatul” discurilor pe o masă lungă spre patru porți numerotate 2, 3, 4, 1.
//!
//! * 30 de discuri; fiecare jucător are 3 runde. În runda 1 arunci toate cele 30, în rundele 2 și 3
//!   arunci din nou doar discurile care **nu** au intrat într-o poartă.
//! * Scor final: fiecare serie completă (câte un disc în fiecare poartă) valorează 20 de puncte;
//!   discurile rămase se numără după valoarea porții (poarta 2 = 2 puncte etc.).
//! * Alegi ținta (`stanga`, `centru`, `dreapta`, `atent`), apoi `arunc` (mutare separată). Probabilitățile
//!   fiecărui disc (poarta 2/3/4/1 sau pe lângă) sunt alegerea noastră: stanga 32/27/11/4/26 %,
//!   centru 9/25/25/9/32 %, dreapta 4/11/27/32/26 %, atent 14/22/22/14/28 %.
//! * Joacă J1 toate cele 3 runde, apoi J2. Evaluarea AI nu citește aruncarea ce urmează.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const VALUE: [i32; 4] = [2, 3, 4, 1];
const AIMS: [&str; 4] = ["stanga", "centru", "dreapta", "atent"];
const PROB: [[u32; 5]; 4] = [
    [32, 27, 11, 4, 26],
    [9, 25, 25, 9, 32],
    [4, 11, 27, 32, 26],
    [14, 22, 22, 14, 28],
];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Aim(u8),
    Throw,
}

#[derive(Clone)]
pub struct Sjoelen {
    gates: [[i32; 4]; 2],
    left: [i32; 2],
    round: [u8; 2],
    side: Player,
    aim: Option<u8>,
    rng: Rng,
    last: String,
}

impl Default for Sjoelen {
    fn default() -> Self {
        Self::new()
    }
}

pub fn score_of(g: &[f64; 4]) -> f64 {
    let sets = g.iter().cloned().fold(f64::MAX, f64::min).floor().max(0.0);
    let sets = sets.min(g.iter().cloned().fold(f64::MAX, f64::min));
    let mut s = sets * 20.0;
    for i in 0..4 {
        s += (g[i] - sets) * VALUE[i] as f64;
    }
    s
}

impl Sjoelen {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Sjoelen {
            gates: [[0; 4]; 2],
            left: [30; 2],
            round: [0; 2],
            side: Player::One,
            aim: None,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn total(&self, p: usize) -> i32 {
        let g = &self.gates[p];
        let sets = *g.iter().min().unwrap();
        let mut s = sets * 20;
        for i in 0..4 {
            s += (g[i] - sets) * VALUE[i];
        }
        s
    }

    fn est(&self, p: usize) -> f64 {
        let mut g = [0.0; 4];
        for (i, gi) in g.iter_mut().enumerate() {
            *gi = self.gates[p][i] as f64;
        }
        if let (Some(a), true) = (self.aim, self.side.idx() == p) {
            for i in 0..4 {
                g[i] += self.left[p] as f64 * PROB[a as usize][i] as f64 / 100.0;
            }
        }
        score_of(&g)
    }
}

impl Game for Sjoelen {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.aim.is_some() {
            return vec![Move::Throw];
        }
        (0..4).map(Move::Aim).collect()
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Aim(a) => self.aim = Some(*a),
            Move::Throw => {
                let a = self.aim.take().unwrap() as usize;
                let i = self.side.idx();
                let n = self.left[i];
                let mut got = [0i32; 4];
                for _ in 0..n {
                    let mut r = self.rng.below(100) as u32;
                    for (k, &p) in PROB[a].iter().enumerate() {
                        if r < p {
                            if k < 4 {
                                got[k] += 1;
                            }
                            break;
                        }
                        r -= p;
                    }
                }
                let sc: i32 = got.iter().sum();
                for (gate, add) in self.gates[i].iter_mut().zip(got.iter()) {
                    *gate += *add;
                }
                self.left[i] -= sc;
                self.round[i] += 1;
                self.last = format!(
                    "J{} (runda {}): porțile 2/3/4/1 primesc {:?}; {} discuri pe lângă.",
                    i + 1,
                    self.round[i],
                    got,
                    self.left[i]
                );
                if self.round[i] >= 3 {
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round[0] >= 3 && self.round[1] >= 3 {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        ((self.est(p.idx()) - self.est(p.other().idx())) * 100.0) as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Sjoelen — porți: 2 | 3 | 4 | 1\n");
        for i in 0..2 {
            s.push_str(&format!(
                "J{}: runda {}/3, porți {:?}, discuri rămase {}, scor {}\n",
                i + 1,
                self.round[i].min(3),
                self.gates[i],
                self.left[i],
                self.total(i)
            ));
        }
        if let Some(a) = self.aim {
            s.push_str(&format!("Ținta aleasă: {}.\n", AIMS[a as usize]));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — {}\n",
                self.side.idx() + 1,
                if self.aim.is_some() {
                    "arunc"
                } else {
                    "stanga / centru / dreapta / atent"
                }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(a) => AIMS[*a as usize].into(),
            Move::Throw => "arunc".into(),
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
    fn scoring_sets_and_leftovers() {
        let mut g = Sjoelen::with_seed(1);
        g.gates[0] = [3, 3, 2, 1];
        // o serie (20) + (2,2,1,0) discuri rămase: 2*2 + 2*3 + 1*4 = 14
        assert_eq!(g.total(0), 20 + 14);
        g.gates[1] = [0, 5, 5, 5];
        assert_eq!(g.total(1), 15 + 20 + 5);
    }

    #[test]
    fn discs_that_scored_are_not_thrown_again() {
        let mut g = Sjoelen::with_seed(7);
        g.apply(&Move::Aim(1));
        g.apply(&Move::Throw);
        let s: i32 = g.gates[0].iter().sum();
        assert_eq!(g.left[0], 30 - s);
        assert_eq!(g.round[0], 1);
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Sjoelen::with_seed, 5, 500);
    }
}
