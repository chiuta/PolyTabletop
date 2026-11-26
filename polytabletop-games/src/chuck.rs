#![allow(clippy::needless_range_loop)]
//! „Zarul norocos” (Chuck-a-luck) — joc tradițional de zaruri american/britanic, 2 jucători contra băncii.
//!
//! * Alegi un număr 1–6 și o miză (5, 10 sau 20 de jetoane): `pariez 3 10`. Apoi `arunc` cele
//!   trei zaruri. Pentru fiecare zar egal cu numărul tău primești miza (plătită 1:1, deci 1, 2 sau
//!   3 mize câștigate); dacă niciun zar nu se potrivește pierzi miza.
//! * Fiecare jucător joacă 12 ture, începe cu 100 de jetoane; câștigă cine are mai multe.
//!
//! Zarurile se aruncă la mutarea forțată `arunc` (AI-ul decide miza fără să le vadă).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TURNS: u8 = 12;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Bet(u8, i32),
    Roll,
}

#[derive(Clone)]
pub struct Chuck {
    rng: Rng,
    chips: [i32; 2],
    bet: Option<(u8, i32)>,
    side: Player,
    done: [u8; 2],
    log: String,
}

impl Default for Chuck {
    fn default() -> Self {
        Self::new()
    }
}

/// Câștigul net pentru numărul `n` și zarurile `d`.
pub fn payout(d: [u8; 3], n: u8, stake: i32) -> i32 {
    let k = d.iter().filter(|&&x| x == n).count() as i32;
    if k == 0 {
        -stake
    } else {
        k * stake
    }
}

impl Chuck {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Chuck {
            rng: Rng::new(seed),
            chips: [100, 100],
            bet: None,
            side: Player::One,
            done: [0, 0],
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.done[0] >= TURNS && self.done[1] >= TURNS
    }
}

impl Game for Chuck {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.bet.is_some() {
            return vec![Move::Roll];
        }
        let mut v = Vec::new();
        for n in 1..=6u8 {
            for &s in &STAKES {
                v.push(Move::Bet(n, s));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let p = self.side.idx();
        match *m {
            Move::Bet(n, s) => self.bet = Some((n, s)),
            Move::Roll => {
                let (n, s) = self.bet.take().unwrap();
                let d = [self.rng.d6(), self.rng.d6(), self.rng.d6()];
                let g = payout(d, n, s);
                self.chips[p] += g;
                self.log = format!(
                    "J{} pariază {} (miza {}): zaruri {} {} {} → {:+}",
                    p + 1,
                    n,
                    s,
                    d[0],
                    d[1],
                    d[2],
                    g
                );
                self.done[p] += 1;
                self.side = self.side.other();
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
        // Speranța matematică a pariului e aceeași pentru orice număr (−17/216 din miză);
        // în urmă iei riscuri, în față joci prudent.
        let (me, op) = (p.idx(), 1 - p.idx());
        let diff = self.chips[me] - self.chips[op];
        let mut v = diff * 100;
        if let Some((_, s)) = self.bet {
            let left = (TURNS - self.done[me].min(TURNS)) as i32;
            let risk = if diff < 0 { s } else { -s };
            v += risk * left.min(4) - s * 8 / 10;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Zarul norocos (Chuck-a-luck) — jetoane: J1 {}, J2 {} | ture: J1 {}/{}, J2 {}/{}\n",
            self.chips[0], self.chips[1], self.done[0], TURNS, self.done[1], TURNS
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if let Some((n, st)) = self.bet {
            s.push_str(&format!(
                "Pariu pus: numărul {n}, miza {st}. Scrie «arunc».\n"
            ));
        }
        if !self.over() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        } else {
            s.push_str("Joc încheiat.\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet(n, s) => format!("pariez {n} {s}"),
            Move::Roll => "arunc".into(),
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
    fn payouts() {
        assert_eq!(payout([3, 3, 3], 3, 10), 30);
        assert_eq!(payout([3, 1, 3], 3, 10), 20);
        assert_eq!(payout([3, 1, 2], 3, 5), 5);
        assert_eq!(payout([1, 1, 2], 3, 5), -5);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Chuck::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 1000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
            assert_eq!(g.done, [TURNS, TURNS]);
        }
    }
}
