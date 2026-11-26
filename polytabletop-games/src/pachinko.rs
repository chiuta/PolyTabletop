#![allow(clippy::needless_range_loop)]
//! Pachinko (Japonia) — flipperul cu bile, în variantă de duel pe ture.
//!
//! * Fiecare jucător pornește cu 20 de bile. La rândul tău lansezi o bilă într-una din cele 9
//!   coloane de start (`lansez 5`), apoi `cade` (forțat): bila coboară printre 8 rânduri de cuie,
//!   abătându-se la stânga sau la dreapta la fiecare (la margine sare înapoi spre interior) și
//!   ajunge într-unul din cele 9 buzunare de jos.
//! * Buzunarele plătesc, de la stânga la dreapta, 3 2 1 2 6 2 1 2 3 bile câștigate (bila lansată e
//!   consumată). Fiecare jucător face 15 lansări; câștigă cine are mai multe bile la sfârșit.
//! * Din cauza cuielor, o coloană de start impară ajunge doar în buzunare impare (2, 4, 6, 8), una
//!   pară doar în buzunare pare (1, 3, 5, 7, 9): alegerea coloanei contează.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COLS: usize = 9;
const ROWS: usize = 8;
const PAYOUT: [i32; COLS] = [3, 2, 1, 2, 6, 2, 1, 2, 3];
const SHOTS: u8 = 15;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Launch(u8),
    Fall,
}

#[derive(Clone)]
pub struct Pachinko {
    rng: Rng,
    balls: [i32; 2],
    shots: [u8; 2],
    pending: Option<u8>,
    side: Player,
    log: String,
}

impl Default for Pachinko {
    fn default() -> Self {
        Self::new()
    }
}

/// Distribuția buzunarelor pentru o coloană de start (în procente ×1000).
pub fn distribution(start: usize) -> [u32; COLS] {
    let mut d = [0u32; COLS];
    d[start] = 1_000_000;
    for _ in 0..ROWS {
        let mut n = [0u32; COLS];
        for c in 0..COLS {
            let v = d[c];
            if v == 0 {
                continue;
            }
            if c == 0 {
                n[1] += v;
            } else if c == COLS - 1 {
                n[COLS - 2] += v;
            } else {
                n[c - 1] += v / 2;
                n[c + 1] += v - v / 2;
            }
        }
        d = n;
    }
    d
}

/// Câștigul mediu (în sutimi de bilă) al unei lansări din coloana `start`.
pub fn expected(start: usize) -> i32 {
    let d = distribution(start);
    let tot: u64 = d.iter().map(|&x| x as u64).sum();
    let s: u64 = (0..COLS)
        .map(|c| d[c] as u64 * PAYOUT[c] as u64 * 100)
        .sum();
    (s / tot) as i32
}

impl Pachinko {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Pachinko {
            rng: Rng::new(seed),
            balls: [20, 20],
            shots: [0, 0],
            pending: None,
            side: Player::One,
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.shots[0] >= SHOTS && self.shots[1] >= SHOTS
    }
}

impl Game for Pachinko {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Fall];
        }
        (1..=COLS as u8).map(Move::Launch).collect()
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Launch(c) => {
                self.pending = Some(c - 1);
                self.balls[self.side.idx()] -= 1;
            }
            Move::Fall => {
                let mut c = self.pending.take().unwrap() as usize;
                for _ in 0..ROWS {
                    if c == 0 {
                        c = 1;
                    } else if c == COLS - 1 {
                        c = COLS - 2;
                    } else if self.rng.below(2) == 0 {
                        c -= 1;
                    } else {
                        c += 1;
                    }
                }
                let p = self.side.idx();
                self.balls[p] += PAYOUT[c];
                self.shots[p] += 1;
                self.log = format!(
                    "J{}: bila cade în buzunarul {} → +{}",
                    p + 1,
                    c + 1,
                    PAYOUT[c]
                );
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.balls[0].cmp(&self.balls[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), 1 - p.idx());
        let mut v = (self.balls[me] - self.balls[op]) * 100;
        if let Some(c) = self.pending {
            let e = expected(c as usize);
            v += if self.side == p { e } else { -e };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pachinko — bile: J1 {}, J2 {} | lansări: J1 {}/{}, J2 {}/{}\n",
            self.balls[0], self.balls[1], self.shots[0], SHOTS, self.shots[1], SHOTS
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&format!(
            "Buzunare: {}\n",
            PAYOUT
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ));
        if let Some(c) = self.pending {
            s.push_str(&format!(
                "Bila e lansată din coloana {}. Scrie «cade».\n",
                c + 1
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
            Move::Launch(c) => format!("lansez {c}"),
            Move::Fall => "cade".into(),
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
    fn parity_of_pockets() {
        let d = distribution(0);
        for c in 0..COLS {
            if c % 2 == 1 {
                assert_eq!(d[c], 0);
            }
        }
        let d = distribution(3);
        for c in 0..COLS {
            if c % 2 == 0 {
                assert_eq!(d[c], 0);
            }
        }
    }

    #[test]
    fn expected_values_reasonable() {
        for c in 0..COLS {
            let e = expected(c);
            assert!(e > 50 && e < 400, "col {c}: {e}");
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Pachinko::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 500 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
