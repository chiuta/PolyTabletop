//! Knucklebones / Tali (Grecia și Roma antică): patru oscioare de oaie (talus) cu patru fețe
//! utile, valorile 1, 3, 4 și 6 (1 și 6 pe fețele înguste, deci rare: 10% fiecare; 3 și 4 câte 40%).
//!
//! * Aruncarea cea mai bună este **Venus** (cele patru fețe diferite: 1-3-4-6); cea mai proastă
//!   este **Canis** („câinele”: patru de 1). Altfel contează suma. Venus bate orice, Canis pierde în
//!   fața oricărei aruncări diferite.
//! * 10 runde; în fiecare rundă începătorul alternează: începătorul spune miza (`miza 1/2/3`) și aruncă
//!   (`arunc`), celălalt aruncă (`arunc`); aruncarea mai bună ia miza (egal = nimic). Start 20 de
//!   jetoane; câștigă cine are mai multe la final. Reguli reconstituite (sursele antice nu dau o
//!   singură formă a jocului).
//!   Mutări: `miza 2`, `arunc`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 10;
const FACES: [u8; 4] = [1, 3, 4, 6];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Stake(i32),
    Throw,
}

#[derive(Clone)]
pub struct Tali {
    chips: [i32; 2],
    starter: Player,
    stake: i32,
    throws: [Option<[u8; 4]>; 2],
    round: u8,
    rng: Rng,
    last: String,
}

impl Default for Tali {
    fn default() -> Self {
        Self::new()
    }
}

/// Rang: Venus 1000, Canis −1, altfel suma.
pub fn rank(t: &[u8; 4]) -> i32 {
    let mut s = *t;
    s.sort_unstable();
    if s == [1, 3, 4, 6] {
        1000
    } else if s == [1, 1, 1, 1] {
        -1
    } else {
        s.iter().map(|&x| x as i32).sum()
    }
}

fn name(t: &[u8; 4]) -> &'static str {
    match rank(t) {
        1000 => "Venus",
        -1 => "Canis",
        _ => "",
    }
}

impl Tali {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Tali {
            chips: [20; 2],
            starter: Player::One,
            stake: 0,
            throws: [None; 2],
            round: 0,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }
}

impl Game for Tali {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.stake == 0 {
            return self.starter;
        }
        let s = self.starter.idx();
        if self.throws[s].is_none() {
            self.starter
        } else {
            self.starter.other()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.stake == 0 {
            let cap = self.chips[0].min(self.chips[1]);
            return (1..=3).filter(|&s| s <= cap).map(Move::Stake).collect();
        }
        vec![Move::Throw]
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Stake(s) => self.stake = *s,
            Move::Throw => {
                let who = self.to_move().idx();
                let mut t = [0u8; 4];
                for x in t.iter_mut() {
                    let r = self.rng.below(100);
                    *x = if r < 10 {
                        FACES[0]
                    } else if r < 50 {
                        FACES[1]
                    } else if r < 90 {
                        FACES[2]
                    } else {
                        FACES[3]
                    };
                }
                self.throws[who] = Some(t);
                if let (Some(a), Some(b)) = (self.throws[0], self.throws[1]) {
                    let (ra, rb) = (rank(&a), rank(&b));
                    let txt = format!("J1 {:?} {} · J2 {:?} {}", a, name(&a), b, name(&b));
                    if ra > rb {
                        self.chips[0] += self.stake;
                        self.chips[1] -= self.stake;
                        self.last = format!("{txt} → J1 ia {}.", self.stake);
                    } else if rb > ra {
                        self.chips[1] += self.stake;
                        self.chips[0] -= self.stake;
                        self.last = format!("{txt} → J2 ia {}.", self.stake);
                    } else {
                        self.last = format!("{txt} → egal.");
                    }
                    self.stake = 0;
                    self.throws = [None; 2];
                    self.round += 1;
                    self.starter = self.starter.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round >= ROUNDS || self.chips[0] <= 0 || self.chips[1] <= 0 {
            return Some(by_score(self.chips[0], self.chips[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let diff = self.chips[me] - self.chips[1 - me];
        let mut v = diff * 100;
        if self.stake > 0 && self.throws.iter().all(|t| t.is_none()) {
            let k = if diff < 0 {
                6
            } else if diff > 0 {
                -6
            } else {
                0
            };
            v += k * self.stake * (ROUNDS - self.round) as i32;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tali — runda {}/{ROUNDS}. Jetoane: J1 {}, J2 {}. Începe J{}.\n",
            (self.round + 1).min(ROUNDS),
            self.chips[0],
            self.chips[1],
            self.starter.idx() + 1
        );
        if self.stake > 0 {
            s.push_str(&format!("Miza: {}.\n", self.stake));
            for i in 0..2 {
                if let Some(t) = self.throws[i] {
                    s.push_str(&format!("J{} a aruncat {:?} {}\n", i + 1, t, name(&t)));
                }
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — {}\n",
                self.to_move().idx() + 1,
                if self.stake == 0 {
                    "miza 1 / 2 / 3"
                } else {
                    "arunc"
                }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Stake(s) => format!("miza {s}"),
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
    fn ranking() {
        assert!(rank(&[6, 4, 3, 1]) > rank(&[6, 6, 6, 6]));
        assert!(rank(&[1, 1, 1, 1]) < rank(&[1, 1, 1, 3]));
        assert_eq!(name(&[1, 3, 4, 6]), "Venus");
        assert_eq!(name(&[1, 1, 1, 1]), "Canis");
    }

    #[test]
    fn round_resolves_after_both_throws() {
        let mut g = Tali::with_seed(3);
        g.apply(&Move::Stake(2));
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Throw);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Throw);
        assert_eq!(g.round, 1);
        assert_eq!(g.chips[0] + g.chips[1], 40);
        assert_eq!(g.starter, Player::Two);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Tali::with_seed, 6, 500);
    }
}
