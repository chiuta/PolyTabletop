//! Zara (Italia medievală), 2 jucători — reconstituire modernă a jocului de zaruri cu numere strigate.
//!
//! Sursele păstrează doar ideea: un jucător strigă un număr, aruncă trei zaruri, câștigă dacă suma lor e
//! numărul strigat. Aici, în 10 runde:
//! * J1 strigă un număr între 4 și 17 (`n13`), apoi J2 strigă **alt** număr între 4 și 17; apoi se aruncă trei
//!   zaruri (`arunc`, mutare forțată).
//! * Cine a strigat suma aruncată primește **atâtea puncte cât numărul strigat** (3 și 18 nu pot fi strigate,
//!   fiind „azar”); dacă nimeni n-a nimerit, runda nu dă puncte.
//! * După 10 runde câștigă cine are mai multe puncte (egal = remiză). Rundele alternează cine strigă primul.
//!
//! Mutări: `n13`, `arunc`.
#![allow(clippy::manual_is_multiple_of)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 10;

/// Numărul de moduri (din 216) de a obține suma `s` cu trei zaruri.
pub fn ways(s: u8) -> i32 {
    let mut n = 0;
    for a in 1..=6u8 {
        for b in 1..=6u8 {
            for c in 1..=6u8 {
                if a + b + c == s {
                    n += 1;
                }
            }
        }
    }
    n
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Call(u8),
    Roll,
}

#[derive(Clone)]
pub struct Zara {
    rng: Rng,
    round: u8,
    calls: [Option<u8>; 2],
    score: [i32; 2],
    log: String,
}

impl Default for Zara {
    fn default() -> Self {
        Self::new()
    }
}

impl Zara {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Zara {
            rng: Rng::new(seed),
            round: 0,
            calls: [None, None],
            score: [0, 0],
            log: String::new(),
        }
    }

    fn first(&self) -> Player {
        if self.round % 2 == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
}

impl Game for Zara {
    type Move = Move;

    fn to_move(&self) -> Player {
        let f = self.first();
        if self.calls[f.idx()].is_none() {
            f
        } else if self.calls[f.other().idx()].is_none() {
            f.other()
        } else {
            f
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.round >= ROUNDS {
            return Vec::new();
        }
        let f = self.first();
        if self.calls[f.idx()].is_some() && self.calls[f.other().idx()].is_some() {
            return vec![Move::Roll];
        }
        let taken = self.calls[f.idx()];
        (4..=17)
            .filter(|n| Some(*n) != taken)
            .map(Move::Call)
            .collect()
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Call(n) => {
                let who = self.to_move().idx();
                self.calls[who] = Some(*n);
            }
            Move::Roll => {
                let d = [self.rng.d6(), self.rng.d6(), self.rng.d6()];
                let sum = d[0] + d[1] + d[2];
                let mut log = format!("Zaruri {}+{}+{} = {sum}.", d[0], d[1], d[2]);
                for p in 0..2 {
                    if self.calls[p] == Some(sum) {
                        self.score[p] += sum as i32;
                        log.push_str(&format!(" J{} a strigat {sum}: +{sum}.", p + 1));
                    }
                }
                self.log = log;
                self.calls = [None, None];
                self.round += 1;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        // speranța matematică (în sutimi de punct) a numerelor strigate în runda curentă
        let ev = |c: Option<u8>| c.map(|n| ways(n) * n as i32 * 100 / 216).unwrap_or(0);
        let me = ev(self.calls[p.idx()]);
        let you = ev(self.calls[p.other().idx()]);
        (self.score[p.idx()] - self.score[p.other().idx()]) * 100 + me - you
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Zara — runda {}/{ROUNDS}; scor J1 {} – J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        for p in 0..2 {
            if let Some(n) = self.calls[p] {
                s.push_str(&format!("J{} a strigat {n}\n", p + 1));
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {}\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Call(n) => format!("n{n}"),
            Move::Roll => "arunc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probabilities() {
        assert_eq!(ways(3), 1);
        assert_eq!(ways(10), 27);
        assert_eq!(ways(11), 27);
        assert_eq!((4..=17).map(ways).sum::<i32>(), 216 - 2);
    }

    #[test]
    fn calls_must_differ_and_games_end() {
        let mut g = Zara::with_seed(2);
        g.apply(&Move::Call(10));
        assert!(!g.legal_moves().contains(&Move::Call(10)));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Call(11));
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        let mut g = Zara::with_seed(5);
        let mut rng = Rng::new(1);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
            n += 1;
            assert!(n < 100);
        }
    }
}
