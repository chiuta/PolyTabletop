//! Over and Under Seven (Marea Britanie), 2 jucători, cu două zaruri.
//!
//! * Pariorul alege `sub`, `peste` (se plătește 1:1) sau `sapte` (se plătește 4:1) și miza (5, 10 sau
//!   20); banca (celălalt jucător) aruncă două zaruri (`arunc`). Suma 2–6 = sub, 8–12 = peste, 7 = șapte.
//!   Un pariu pierdut dă miza băncii, iar cel câștigat o primește din bancă. Banca schimbă mâinile la fiecare rundă.
//! * Pariul `sapte` cere ca banca să poată plăti de 4 ori miza.
//! * 50 de jetoane fiecare, 20 de mâini; câștigă cine are mai multe jetoane.

use crate::q2_util::{by_score, end_line, risk};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 50;
const HANDS: u8 = 20;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Under,
    Over,
    Seven,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Bet(Kind, i32),
    Roll,
}

#[derive(Clone)]
pub struct OverUnder {
    rng: Rng,
    chips: [i32; 2],
    banker: Player,
    bet: Option<(Kind, i32)>,
    hands: u8,
    last: String,
}

impl Default for OverUnder {
    fn default() -> Self {
        Self::new()
    }
}

impl OverUnder {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        OverUnder {
            rng: Rng::new(seed),
            chips: [START; 2],
            banker: Player::Two,
            bet: None,
            hands: 0,
            last: String::new(),
        }
    }
    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }
    fn better(&self) -> Player {
        self.banker.other()
    }
    fn over(&self) -> bool {
        self.hands >= HANDS || self.chips.iter().any(|&c| c < 5)
    }
}

impl Game for OverUnder {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.bet.is_some() {
            self.banker
        } else {
            self.better()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.bet.is_some() {
            return vec![Move::Roll];
        }
        let (b, k) = (self.better().idx(), self.banker.idx());
        let mut v = Vec::new();
        for kind in [Kind::Under, Kind::Over, Kind::Seven] {
            for s in STAKES {
                let pay = if kind == Kind::Seven { 4 * s } else { s };
                if s <= self.chips[b] && pay <= self.chips[k] {
                    v.push(Move::Bet(kind, s));
                }
            }
        }
        if v.is_empty() {
            v.push(Move::Bet(Kind::Under, 1));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Bet(k, s) => {
                self.bet = Some((k, s));
                self.last.clear();
            }
            Move::Roll => {
                let (kind, s) = self.bet.take().unwrap();
                let (d1, d2) = (self.rng.d6(), self.rng.d6());
                let sum = d1 + d2;
                let res = match sum {
                    7 => Kind::Seven,
                    2..=6 => Kind::Under,
                    _ => Kind::Over,
                };
                let (b, k) = (self.better().idx(), self.banker.idx());
                if res == kind {
                    let pay = if kind == Kind::Seven { 4 * s } else { s };
                    let pay = pay.min(self.chips[k]);
                    self.chips[b] += pay;
                    self.chips[k] -= pay;
                    self.last = format!("{d1}+{d2}={sum}: pariul reușește (+{pay})");
                } else {
                    let s = s.min(self.chips[b]);
                    self.chips[b] -= s;
                    self.chips[k] += s;
                    self.last = format!("{d1}+{d2}={sum}: pariul pierde ({s} la bancă)");
                }
                self.hands += 1;
                self.banker = self.banker.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over() {
            Some(by_score(self.chips[0], self.chips[1]))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let diff = self.chips[p.idx()] - self.chips[1 - p.idx()];
        let mut v = diff * 100;
        if let Some((k, s)) = self.bet {
            // speranța matematică a pariului (pierdere 1/6 din miză) și preferința de risc
            let ev = -(s * 100) / 6;
            let sign = if self.better() == p { 1 } else { -1 };
            v += sign * ev;
            let var = if k == Kind::Seven { 3 } else { 1 };
            let who = if self.better() == p { diff } else { -diff };
            v += sign * risk(s, who) * var / 2;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Over and Under Seven — mâna {}/{HANDS}. Jetoane: J1 {}, J2 {}. Banca: J{}\n",
            (self.hands + 1).min(HANDS),
            self.chips[0],
            self.chips[1],
            self.banker.idx() + 1
        );
        if let Some((k, st)) = self.bet {
            s.push_str(&format!("Pariu: {:?} cu miza {st} — banca aruncă.\n", k));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        s.push_str(&end_line(self.outcome(), self.to_move()));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet(k, s) => format!(
                "{} {s}",
                match k {
                    Kind::Under => "sub",
                    Kind::Over => "peste",
                    Kind::Seven => "sapte",
                }
            ),
            Move::Roll => "arunc".into(),
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
    fn payouts() {
        let mut g = OverUnder::with_seed(1);
        g.apply(&Move::Bet(Kind::Seven, 5));
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        assert_eq!(g.chips[0] + g.chips[1], 2 * START);
        assert!(g.chips != [START; 2]);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..=20u64 {
            let mut g = OverUnder::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())].clone());
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
                n += 1;
                assert!(n < 500);
            }
        }
    }

    #[test]
    fn ai_does_not_peek() {
        let a = OverUnder::with_seed(1);
        let mut b = a.clone();
        b.rng = Rng::new(5);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 4),
            polytabletop_core::GameObj::ai_move(&b, 4)
        );
    }
}
