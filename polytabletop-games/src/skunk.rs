//! Skunk (SUA), joc de zaruri „împinge-ți norocul”, 2 jucători.
//!
//! * 5 runde (S-K-U-N-K). În tura ta arunci două zaruri cât vrei (`arunc`) sau te oprești (`opresc`).
//!   Fără niciun 1: adaugi suma la punctele turei. **Un singur 1** (skunk): pierzi punctele turei;
//!   **doi de 1**: pierzi tot totalul tău. Când te oprești, punctele turei intră în total.
//! * După un skunk confirmi cu `ok`. Câștigă totalul cel mai mare după 5 runde (egal = remiză).
//!
//! AI: la evaluare nu citește fețele aruncate; folosește media pe cele 36 de rezultate posibile.

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Stop,
    Ack,
}

#[derive(Clone)]
pub struct Skunk {
    rng: Rng,
    total: [i32; 2],
    turn: i32,
    round: u8,
    side: Player,
    /// ultima aruncare (a rămas pe masă) și punctele turei înainte de ea
    rolled: Option<(u8, u8, i32)>,
    saved_total: i32,
    over: bool,
    log: String,
}

impl Default for Skunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Skunk {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Skunk {
            rng: Rng::new(seed),
            total: [0; 2],
            turn: 0,
            round: 1,
            side: Player::One,
            rolled: None,
            saved_total: 0,
            over: false,
            log: String::new(),
        }
    }
    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }
    fn skunked(&self) -> bool {
        matches!(self.rolled, Some((a, b, _)) if a == 1 || b == 1)
    }
    fn end_turn(&mut self) {
        self.turn = 0;
        self.rolled = None;
        if self.side == Player::One {
            self.side = Player::Two;
        } else {
            self.side = Player::One;
            self.round += 1;
            if self.round > ROUNDS {
                self.over = true;
            }
        }
    }
}

impl Game for Skunk {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.skunked() {
            vec![Move::Ack]
        } else if self.turn == 0 && self.rolled.is_none() {
            vec![Move::Roll]
        } else {
            vec![Move::Roll, Move::Stop]
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let me = self.side.idx();
        match m {
            Move::Roll => {
                let (a, b) = (self.rng.d6(), self.rng.d6());
                let pre = self.turn;
                self.saved_total = self.total[me];
                self.rolled = Some((a, b, pre));
                if a == 1 && b == 1 {
                    self.total[me] = 0;
                    self.turn = 0;
                    self.log = "Dublu 1! Pierzi tot totalul.".into();
                } else if a == 1 || b == 1 {
                    self.turn = 0;
                    self.log = format!("Aruncare {a}+{b}: skunk! Pierzi punctele turei.");
                } else {
                    self.turn += (a + b) as i32;
                    self.log = format!("Aruncare {a}+{b}: +{}.", a + b);
                }
            }
            Move::Stop => {
                self.total[me] += self.turn;
                self.log = format!(
                    "J{} se oprește cu {} (total {}).",
                    me + 1,
                    self.turn,
                    self.total[me]
                );
                self.end_turn();
            }
            Move::Ack => self.end_turn(),
        }
        if !self.skunked() && !matches!(m, Move::Roll) {
            self.rolled = None;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over {
            Some(by_score(self.total[0], self.total[1]))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (q, me) = (p.other().idx(), p.idx());
        if let Some((_, _, pre)) = self.rolled {
            // starea de după aruncare: media pe cele 36 de rezultate, fără a citi fețele
            let roller = self.side.idx();
            let tot = self.saved_total;
            let mut acc = 0.0;
            for a in 1..=6i32 {
                for b in 1..=6i32 {
                    let v = if a == 1 && b == 1 {
                        0
                    } else if a == 1 || b == 1 {
                        tot
                    } else {
                        tot + pre + a + b
                    };
                    acc += v as f64 / 36.0;
                }
            }
            let mine = acc as i32;
            return if roller == me {
                (mine - self.total[q]) * 10
            } else {
                (self.total[me] - mine) * 10
            };
        }
        (self.total[me] - self.total[q]) * 10 + if self.side == p { self.turn * 10 } else { 0 }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Skunk — runda {}/{ROUNDS}. Total: J1 {}, J2 {}.\n",
            self.round.min(ROUNDS),
            self.total[0],
            self.total[1]
        );
        if !self.over {
            s.push_str(&format!(
                "Punctele turei lui J{}: {}\n",
                self.side.idx() + 1,
                self.turn
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
            Move::Roll => "arunc",
            Move::Stop => "opresc",
            Move::Ack => "ok",
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
    fn full_random_games() {
        for seed in 1..=30u64 {
            let mut g = Skunk::with_seed(seed);
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
    fn ai_does_not_peek_at_the_dice() {
        let mut a = Skunk::with_seed(1);
        a.turn = 12;
        a.rolled = None;
        let mut b = a.clone();
        b.rng = Rng::new(42);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 3),
            polytabletop_core::GameObj::ai_move(&b, 3)
        );
        // aceleași decizii cu 5 și 30 de puncte de tură: oprește la 30
        a.turn = 30;
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 3).as_deref(),
            Some("opresc")
        );
        a.turn = 0;
        a.total = [0, 0];
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 3).as_deref(),
            Some("arunc")
        );
    }
}
