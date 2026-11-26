//! Table skittles (Marea Britanie), popice de masă: o bilă legată cu ață se învârte în jurul unui stâlp
//! și doboară 9 popice. Variantă pentru 2 jucători, cu norocul în locul îndemânării.
//!
//! * Popicele stau pe o grilă 3×3 (coloane st / mij / dr). La fiecare aruncare alegi coloana către care
//!   lansezi bila (`stanga`, `mijloc`, `dreapta`), apoi `lansez`. Popicele din coloana țintă cad cu 60 %,
//!   din coloanele vecine cu 25 %, iar cele opuse (stânga ↔ dreapta) cu 8 %.
//! * O tură are 3 aruncări; fiecare popică doborâtă valorează 1 punct (cea din mijloc, 2). Dacă
//!   doboari toate 9 într-o tură („măturat”), iei 5 puncte bonus și popicele se ridică.
//! * După 3 aruncări popicele se ridică și rândul trece. 6 ture fiecare; cine are mai multe puncte câștigă.

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TURNS: u8 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Aim(u8),
    Release,
}

#[derive(Clone)]
pub struct Skittles {
    rng: Rng,
    up: [bool; 9],
    throws: u8,
    turn_pts: i32,
    score: [i32; 2],
    turns: [u8; 2],
    side: Player,
    pending: Option<u8>,
    log: String,
}

fn pin_pts(i: usize) -> i32 {
    if i == 4 {
        2
    } else {
        1
    }
}

/// Probabilitatea (în %) ca o popică din coloana `pc` să cadă după o aruncare în coloana `tc`.
fn pct(tc: u8, pc: u8) -> i32 {
    match (tc as i32 - pc as i32).abs() {
        0 => 60,
        1 => 25,
        _ => 8,
    }
}

impl Default for Skittles {
    fn default() -> Self {
        Self::new()
    }
}

impl Skittles {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Skittles {
            rng: Rng::new(seed),
            up: [true; 9],
            throws: 0,
            turn_pts: 0,
            score: [0; 2],
            turns: [0; 2],
            side: Player::One,
            pending: None,
            log: String::new(),
        }
    }
    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }
    fn over(&self) -> bool {
        self.turns == [TURNS, TURNS]
    }
    fn expected(&self, tc: u8) -> f64 {
        (0..9)
            .filter(|&i| self.up[i])
            .map(|i| pct(tc, (i % 3) as u8) as f64 / 100.0 * pin_pts(i) as f64)
            .sum()
    }
}

impl Game for Skittles {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            vec![Move::Release]
        } else {
            vec![Move::Aim(0), Move::Aim(1), Move::Aim(2)]
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Aim(c) => {
                self.pending = Some(c);
                self.log.clear();
            }
            Move::Release => {
                let tc = self.pending.take().unwrap_or(1);
                let me = self.side.idx();
                let mut down = 0;
                for i in 0..9 {
                    if self.up[i] && (self.rng.below(100) as i32) < pct(tc, (i % 3) as u8) {
                        self.up[i] = false;
                        self.turn_pts += pin_pts(i);
                        down += 1;
                    }
                }
                self.throws += 1;
                self.log = format!("Aruncarea: {down} popice căzute.");
                let swept = self.up.iter().all(|&u| !u);
                if swept {
                    self.turn_pts += 5;
                    self.log.push_str(" Măturat! +5.");
                }
                if self.throws >= 3 || swept {
                    self.score[me] += self.turn_pts;
                    self.log
                        .push_str(&format!(" Tura: {} puncte.", self.turn_pts));
                    self.turns[me] += 1;
                    self.turn_pts = 0;
                    self.throws = 0;
                    self.up = [true; 9];
                    if self.turns[1 - me] < TURNS {
                        self.side = self.side.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over() {
            Some(by_score(self.score[0], self.score[1]))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = ((self.score[me] - self.score[1 - me]) * 100) as f64;
        if self.side == p {
            v += self.turn_pts as f64 * 100.0;
            if let Some(c) = self.pending {
                v += self.expected(c) * 100.0;
            }
        }
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Table skittles — tura J1 {}/{TURNS}, J2 {}/{TURNS}. Puncte: J1 {}, J2 {}\n",
            self.turns[0], self.turns[1], self.score[0], self.score[1]
        );
        for r in 0..3 {
            s.push_str("  ");
            for c in 0..3 {
                s.push(if self.up[r * 3 + c] {
                    if r * 3 + c == 4 {
                        'K'
                    } else {
                        'I'
                    }
                } else {
                    '.'
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Aruncarea {}/3 a turei (puncte în tură: {}). Coloane: stanga / mijloc / dreapta\n",
            self.throws + 1,
            self.turn_pts
        ));
        if self.pending.is_some() {
            s.push_str("Ai ales ținta — scrie «lansez».\n");
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&end_line(self.outcome(), self.side));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(0) => "stanga".into(),
            Move::Aim(1) => "mijloc".into(),
            Move::Aim(_) => "dreapta".into(),
            Move::Release => "lansez".into(),
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
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = Skittles::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 400);
            }
        }
    }

    #[test]
    fn ai_aims_at_the_column_with_most_standing_pins_and_ignores_rng() {
        let mut a = Skittles::with_seed(1);
        a.up = [true, false, false, true, false, false, true, false, false]; // doar coloana stângă
        let mut b = a.clone();
        b.rng = Rng::new(8);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1).as_deref(),
            Some("stanga")
        );
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&b, 1).as_deref(),
            Some("stanga")
        );
    }
}
