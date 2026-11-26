//! Jocuri cu ață (Cat's cradle), jocul figurilor de ață pentru doi, 2 jucători.
//!
//! Figurile se preiau pe rând, în ordinea clasică: leagăn → patul soldatului → lumânări → ieslea →
//! diamante → ochi de pisică → pește în farfurie → ceas → leagăn…
//! * Fiecare jucător preia figura de la celălalt: `atent` (reușești în 85 %, 1 punct) sau `rapid`
//!   (reușești în 60 %, 2 puncte), apoi `preiau`. Dacă ața se încurcă, pierzi 1 punct și figura revine la leagăn.
//! * Când închizi un ciclu (ajungi la leagăn din ceas) primești 3 puncte bonus.
//! * Primul la 12 puncte câștigă. (Îndemânarea e înlocuită de noroc; alegi tu riscul.)

use crate::q2_util::end_line;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const FIGURES: [&str; 8] = [
    "leagăn",
    "patul soldatului",
    "lumânări",
    "ieslea",
    "diamante",
    "ochi de pisică",
    "pește în farfurie",
    "ceas",
];
const GOAL: i32 = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Careful,
    Quick,
    Take,
}

#[derive(Clone)]
pub struct JocuriAta {
    rng: Rng,
    fig: usize,
    pts: [i32; 2],
    side: Player,
    pending: Option<Move>,
    log: String,
}

fn odds(m: Move) -> (i32, i32) {
    if m == Move::Quick {
        (60, 2)
    } else {
        (85, 1)
    }
}

impl Default for JocuriAta {
    fn default() -> Self {
        Self::new()
    }
}

impl JocuriAta {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        JocuriAta {
            rng: Rng::new(seed),
            fig: 0,
            pts: [0; 2],
            side: Player::One,
            pending: None,
            log: String::new(),
        }
    }
    pub fn points(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }
}

impl Game for JocuriAta {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            vec![Move::Take]
        } else {
            vec![Move::Careful, Move::Quick]
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Careful | Move::Quick => {
                self.pending = Some(*m);
                self.log.clear();
            }
            Move::Take => {
                let k = self.pending.take().unwrap_or(Move::Careful);
                let (pct, gain) = odds(k);
                let me = self.side.idx();
                if (self.rng.below(100) as i32) < pct {
                    self.fig = (self.fig + 1) % FIGURES.len();
                    let mut g = gain;
                    if self.fig == 0 {
                        g += 3;
                    }
                    self.pts[me] += g;
                    self.log = format!("Reușit: «{}» (+{g}).", FIGURES[self.fig]);
                } else {
                    self.pts[me] -= 1;
                    self.fig = 0;
                    self.log = "Ața s-a încurcat (−1); înapoi la leagăn.".into();
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pts[0] >= GOAL || self.pts[1] >= GOAL {
            Some(if self.pts[0] >= self.pts[1] {
                Outcome::Win(Player::One)
            } else {
                Outcome::Win(Player::Two)
            })
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.pts[me] - self.pts[1 - me]) * 100;
        if let (Some(k), true) = (self.pending, self.side == p) {
            let (pct, gain) = odds(k);
            let closes = if (self.fig + 1).is_multiple_of(FIGURES.len()) {
                3
            } else {
                0
            };
            let g = gain + closes;
            v += pct * g - (100 - pct);
            if self.pts[me] + g >= GOAL {
                v += pct * 10;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Jocuri cu ață — figura: «{}» (următoarea: «{}»). Puncte: J1 {}, J2 {} (la {GOAL})\n",
            FIGURES[self.fig],
            FIGURES[(self.fig + 1) % FIGURES.len()],
            self.pts[0],
            self.pts[1]
        );
        if let Some(k) = self.pending {
            s.push_str(&format!(
                "J{}: ai ales {} — scrie «preiau».\n",
                self.side.idx() + 1,
                if k == Move::Quick { "rapid" } else { "atent" }
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
            Move::Careful => "atent",
            Move::Quick => "rapid",
            Move::Take => "preiau",
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
            let mut g = JocuriAta::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
        }
    }

    #[test]
    fn ai_independent_of_hidden_rng() {
        let a = JocuriAta::with_seed(1);
        let mut b = a.clone();
        b.rng = Rng::new(3);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 7),
            polytabletop_core::GameObj::ai_move(&b, 7)
        );
    }
}
