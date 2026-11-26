#![allow(clippy::doc_lazy_continuation)]
//! Șotron (Hopscotch), joc de copii pentru doi jucători: cursă pe pătratele 1–10.
//!
//! * Fiecare jucător are un pion pe pătratul pe care îl are de „luat”. La tura ta arunci piatra:
//!   `prudent` (țintești pătratul următor: reușești în 80% din cazuri și avansezi 1) sau
//!   `indraznet` (țintești peste un pătrat: reușești în 50% și avansezi 2; la ratare cazi un pătrat
//!   înapoi, dacă nu ești la început). Cine duce piatra pe pătratul 10 și se întoarce (avansează
//!   dincolo de 10) primul câștigă.
//! * Aruncarea se hotărăște la pasul forțat `sar` (arbitrul = J2); AI-ul alege după numărul
//!   așteptat de ture rămase, calculat prin programare dinamică, fără să vadă zarul.
//! Mutări: `prudent`, `indraznet`, `sar`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const GOAL: usize = 11;
const P_SAFE: f64 = 0.8;
const P_BOLD: f64 = 0.5;

/// Numărul așteptat de ture până la țintă, cu politica optimă, din fiecare poziție.
fn expected_turns() -> &'static [f64; GOAL + 1] {
    static T: OnceLock<[f64; GOAL + 1]> = OnceLock::new();
    T.get_or_init(|| {
        let mut e = [0.0f64; GOAL + 1];
        for _ in 0..400 {
            for pos in 1..GOAL {
                let left = GOAL - pos;
                let safe = 1.0 + P_SAFE * e[pos + 1] + (1.0 - P_SAFE) * e[pos];
                let mut best = safe;
                if left >= 2 {
                    let back = if pos > 1 { pos - 1 } else { pos };
                    let bold = 1.0 + P_BOLD * e[(pos + 2).min(GOAL)] + (1.0 - P_BOLD) * e[back];
                    best = best.min(bold);
                }
                e[pos] = best;
            }
        }
        e
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Safe,
    Bold,
    Jump,
}

#[derive(Clone)]
pub struct Sotron {
    rng: Rng,
    pos: [usize; 2],
    side: Player,
    pending: Option<bool>,
    last: String,
}

impl Default for Sotron {
    fn default() -> Self {
        Self::new()
    }
}

impl Sotron {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Sotron {
            rng: Rng::new(seed),
            pos: [1; 2],
            side: Player::One,
            pending: None,
            last: String::new(),
        }
    }
}

impl Game for Sotron {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            Player::Two
        } else {
            self.side
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Jump];
        }
        let mut v = vec![Move::Safe];
        if self.pos[self.side.idx()] + 2 <= GOAL {
            v.push(Move::Bold);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let i = self.side.idx();
        match *m {
            Move::Safe => self.pending = Some(false),
            Move::Bold => self.pending = Some(true),
            Move::Jump => {
                let bold = self.pending.take().unwrap();
                let p = if bold { P_BOLD } else { P_SAFE };
                let ok = (self.rng.below(1000) as f64) < p * 1000.0;
                if ok {
                    self.pos[i] += if bold { 2 } else { 1 };
                    self.last = format!(
                        "J{}: piatra a nimerit, pătratul {}",
                        i + 1,
                        self.pos[i].min(10)
                    );
                } else {
                    if bold && self.pos[i] > 1 {
                        self.pos[i] -= 1;
                    }
                    self.last = format!("J{}: piatra a ratat", i + 1);
                }
                self.pos[i] = self.pos[i].min(GOAL);
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pos[0] >= GOAL {
            Some(Outcome::Win(Player::One))
        } else if self.pos[1] >= GOAL {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let t = expected_turns();
        let exp = |i: usize| -> f64 {
            let pos = self.pos[i];
            match self.pending {
                Some(bold) if i == self.side.idx() => {
                    let (pr, nxt, back) = if bold {
                        (
                            P_BOLD,
                            (pos + 2).min(GOAL),
                            if pos > 1 { pos - 1 } else { pos },
                        )
                    } else {
                        (P_SAFE, pos + 1, pos)
                    };
                    pr * t[nxt] + (1.0 - pr) * t[back]
                }
                _ => t[pos],
            }
        };
        ((exp(p.other().idx()) - exp(p.idx())) * 100.0) as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Șotron — cursă pe pătratele 1–10 (dincolo de 10 = acasă)\n");
        for i in 0..2 {
            s.push_str(&format!(
                "J{}: pătratul {}\n",
                i + 1,
                if self.pos[i] >= GOAL {
                    "acasă".into()
                } else {
                    self.pos[i].to_string()
                }
            ));
        }
        if self.outcome().is_none() {
            s.push_str(&format!(
                "La rând: J{} — `prudent` (80%, +1) sau `indraznet` (50%, +2, la ratare −1).\n",
                self.side.idx() + 1
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Safe => "prudent",
            Move::Bold => "indraznet",
            Move::Jump => "sar",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn races_end_and_expected_turns_decrease() {
        let t = expected_turns();
        assert!(t[1] > t[5] && t[5] > t[10] && t[11] == 0.0);
        for seed in 1..=30u64 {
            let mut g = Sotron::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 5000);
            }
        }
    }

    #[test]
    fn ai_ignores_the_future_throw() {
        let (a, b) = (Sotron::with_seed(1), Sotron::with_seed(2));
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
