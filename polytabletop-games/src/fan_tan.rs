#![allow(clippy::needless_range_loop)]
//! Fan-Tan (China) — jocul de pariuri cu mărgele: banca acoperă un morman de mărgele, se pariază
//! pe restul împărțirii la 4, apoi se numără câte patru.
//!
//! * Fiecare jucător pornește cu 100 de jetoane și face 10 pariuri (cu mize 5, 10 sau 20):
//!   `pun 3 10` = pariez pe „fan” 3 cu 10 jetoane. Moșul are între 40 și 199 de mărgele.
//! * După pariurile ambilor, `descopar`: mărgelele se numără câte patru; resturile 1, 2, 3 sau 4
//!   (restul 0 = fan 4) sunt „fan”-urile. Pariul ghicit plătește 3:1 minus comisionul de 5%
//!   al casei (rotunjit în jos); celelalte pierd miza.
//! * Câștigă cine are mai multe jetoane după 10 runde.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 10;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Bet(u8, i32),
    Reveal,
}

#[derive(Clone)]
pub struct FanTan {
    rng: Rng,
    chips: [i32; 2],
    bets: [Option<(u8, i32)>; 2],
    round: u8,
    side: Player,
    log: String,
}

impl Default for FanTan {
    fn default() -> Self {
        Self::new()
    }
}

/// Câștigul net la un pariu ghicit.
pub fn win_amount(stake: i32) -> i32 {
    stake * 3 * 95 / 100
}

impl FanTan {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        FanTan {
            rng: Rng::new(seed),
            chips: [100, 100],
            bets: [None, None],
            round: 1,
            side: Player::One,
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.round > ROUNDS
    }
}

impl Game for FanTan {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.bets[0].is_some() && self.bets[1].is_some() {
            return vec![Move::Reveal];
        }
        let mut v = Vec::new();
        for f in 1..=4u8 {
            for &s in &STAKES {
                v.push(Move::Bet(f, s));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Bet(f, s) => {
                self.bets[self.side.idx()] = Some((f, s));
                self.side = self.side.other();
            }
            Move::Reveal => {
                let beads = 40 + self.rng.below(160) as i32;
                let fan = if beads % 4 == 0 { 4 } else { (beads % 4) as u8 };
                let mut parts = Vec::new();
                for p in 0..2 {
                    let (f, s) = self.bets[p].take().unwrap();
                    let g = if f == fan { win_amount(s) } else { -s };
                    self.chips[p] += g;
                    parts.push(format!("J{} {:+}", p + 1, g));
                }
                self.log = format!("{} mărgele → fan {}; {}", beads, fan, parts.join(", "));
                self.round += 1;
                self.side = Player::One;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), 1 - p.idx());
        let diff = self.chips[me] - self.chips[op];
        let mut v = diff * 100;
        // speranța pariului: 0,25·win − 0,75·miză, aceeași pentru orice fan; în urmă mizezi mare
        for (i, b) in self.bets.iter().enumerate() {
            if let Some((_, s)) = b {
                let ev = (25 * win_amount(*s) - 75 * s) / 100 * 100 / 100;
                let sgn = if i == me { 1 } else { -1 };
                let risk = if diff < 0 { *s } else { -*s };
                v += sgn * (ev + if i == me { risk } else { 0 });
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Fan-Tan — runda {}/{} | jetoane: J1 {}, J2 {}\n",
            self.round.min(ROUNDS),
            ROUNDS,
            self.chips[0],
            self.chips[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        for p in 0..2 {
            if let Some((f, st)) = self.bets[p] {
                s.push_str(&format!("J{} pariază fan {} cu {}\n", p + 1, f, st));
            }
        }
        if self.over() {
            s.push_str("Joc încheiat.\n");
        } else if self.bets[0].is_some() && self.bets[1].is_some() {
            s.push_str("Scrie «descopar».\n");
        } else {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet(f, s) => format!("pun {f} {s}"),
            Move::Reveal => "descopar".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payout_has_commission() {
        assert_eq!(win_amount(10), 28);
        assert_eq!(win_amount(20), 57);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = FanTan::with_seed(seed);
            let mut rng = Rng::new(seed + 1);
            let mut n = 0;
            while g.outcome().is_none() && n < 500 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
            assert_eq!(n, 30);
        }
    }

    #[test]
    fn reveal_pays_or_takes() {
        let mut g = FanTan::with_seed(1);
        g.apply(&Move::Bet(4, 10));
        g.apply(&Move::Bet(1, 10));
        g.apply(&Move::Reveal);
        for c in g.chips {
            assert!(c == 90 || c == 100 + win_amount(10));
        }
        assert_eq!(g.round, 2);
    }
}
