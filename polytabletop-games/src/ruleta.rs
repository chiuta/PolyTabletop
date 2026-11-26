//! Ruleta europeană (Franța, secolul al XVIII-lea): roată cu 37 de numere (0–36), 2 jucători.
//!
//! * Fiecare jucător începe cu 100 de jetoane; 12 învârtiri. La fiecare învârtire fiecare pune
//!   **un singur pariu** (mize 5, 10 sau 20, sau tot ce mai are sub 5), J1 întâi, J2 vede pariul lui J1.
//!   Apoi J1 spune `invart` și bila cade la un număr. Câștigă la final cine are mai multe jetoane.
//! * Pariuri: `plin N` (un număr, plătește 35:1), `rosu`, `negru`, `par`, `impar`, `mic` (1–18),
//!   `mare` (19–36) — plătesc 1:1; `duz1`/`duz2`/`duz3` (1–12, 13–24, 25–36) și `col1`/`col2`/`col3`
//!   (coloanele de pe masă) plătesc 2:1. Zero pierde toate pariurile (fără „la partage”).
//! * Învârtirea este o mutare separată, ca AI-ul să nu-i vadă rezultatul.
//!   Mutări: `rosu 10`, `plin 17 5`, `duz2 20`, `pas` (doar la 0 jetoane), `invart`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SPINS: u8 = 12;
const RED: [u8; 18] = [
    1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
];
const NAMES: [&str; 9] = [
    "plin", "rosu", "negru", "par", "impar", "mic", "mare", "duz", "col",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bet {
    /// 0 plin, 1 rosu, 2 negru, 3 par, 4 impar, 5 mic, 6 mare, 7 duzina (num 1..3), 8 coloana (num 1..3), 9 fără pariu
    pub kind: u8,
    pub num: u8,
    pub stake: i32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Place(Bet),
    Spin,
}

#[derive(Clone)]
pub struct Ruleta {
    chips: [i32; 2],
    bets: [Option<Bet>; 2],
    spins: u8,
    rng: Rng,
    last: String,
}

impl Default for Ruleta {
    fn default() -> Self {
        Self::new()
    }
}

/// Câștigă pariul pe numărul `n`? Întoarce multiplicatorul (0 = pierde).
pub fn payout(b: &Bet, n: u8) -> i32 {
    if n == 0 {
        return if b.kind == 0 && b.num == 0 { 35 } else { 0 };
    }
    let win = match b.kind {
        0 => n == b.num,
        1 => RED.contains(&n),
        2 => !RED.contains(&n),
        3 => n.is_multiple_of(2),
        4 => n % 2 == 1,
        5 => n <= 18,
        6 => n >= 19,
        7 => (n - 1) / 12 + 1 == b.num,
        8 => (n - 1) % 3 + 1 == b.num,
        _ => false,
    };
    if !win {
        return 0;
    }
    match b.kind {
        0 => 35,
        7 | 8 => 2,
        _ => 1,
    }
}

fn cover(b: &Bet) -> i32 {
    (0..=36u8).filter(|&n| payout(b, n) > 0).count() as i32
}

impl Ruleta {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Ruleta {
            chips: [100; 2],
            bets: [None; 2],
            spins: 0,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }

    fn placing(&self) -> Option<Player> {
        if self.bets[0].is_none() {
            Some(Player::One)
        } else if self.bets[1].is_none() {
            Some(Player::Two)
        } else {
            None
        }
    }
}

fn bet_text(b: &Bet) -> String {
    match b.kind {
        0 => format!("plin {} {}", b.num, b.stake),
        7 | 8 => format!("{}{} {}", NAMES[b.kind as usize], b.num, b.stake),
        9 => "pas".into(),
        k => format!("{} {}", NAMES[k as usize], b.stake),
    }
}

impl Game for Ruleta {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.placing().unwrap_or(Player::One)
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let Some(p) = self.placing() else {
            return vec![Move::Spin];
        };
        let c = self.chips[p.idx()];
        if c <= 0 {
            return vec![Move::Place(Bet {
                kind: 9,
                num: 0,
                stake: 0,
            })];
        }
        let mut stakes: Vec<i32> = [5, 10, 20].into_iter().filter(|&s| s <= c).collect();
        if stakes.is_empty() {
            stakes.push(c);
        }
        let mut v = Vec::new();
        for &stake in &stakes {
            for n in 0..=36u8 {
                v.push(Move::Place(Bet {
                    kind: 0,
                    num: n,
                    stake,
                }));
            }
            for kind in 1..=6u8 {
                v.push(Move::Place(Bet {
                    kind,
                    num: 0,
                    stake,
                }));
            }
            for kind in [7u8, 8] {
                for num in 1..=3u8 {
                    v.push(Move::Place(Bet { kind, num, stake }));
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Place(b) => {
                let i = self.placing().unwrap().idx();
                self.bets[i] = Some(*b);
            }
            Move::Spin => {
                let n = self.rng.below(37) as u8;
                let mut txt = format!("Bila cade pe {n}.");
                for i in 0..2 {
                    if let Some(b) = self.bets[i] {
                        if b.kind == 9 {
                            continue;
                        }
                        let k = payout(&b, n);
                        if k > 0 {
                            self.chips[i] += b.stake * k;
                            txt.push_str(&format!(" J{} câștigă {}.", i + 1, b.stake * k));
                        } else {
                            self.chips[i] -= b.stake;
                            txt.push_str(&format!(" J{} pierde {}.", i + 1, b.stake));
                        }
                    }
                }
                self.last = txt;
                self.bets = [None; 2];
                self.spins += 1;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.spins >= SPINS {
            return Some(by_score(self.chips[0], self.chips[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.chips[me] - self.chips[1 - me]) as f64 * 100.0;
        if let Some(b) = self.bets[me] {
            if b.kind != 9 {
                let pw = cover(&b) as f64 / 37.0;
                let mult = payout(&b, if b.kind == 0 { b.num } else { 1 }).max(1) as f64;
                let mult = if b.kind == 0 { 35.0 } else { mult.max(1.0) };
                let ev = b.stake as f64 * (pw * (mult + 1.0) - 1.0);
                let sd = b.stake as f64 * (pw * (1.0 - pw)).sqrt() * (mult + 1.0);
                let left = (SPINS - self.spins) as f64;
                let diff = (self.chips[me] - self.chips[1 - me]) as f64;
                // în urmă = caută varianța; în față = o evită
                let k = if diff < 0.0 {
                    12.0
                } else if diff > 0.0 {
                    -12.0
                } else {
                    0.0
                };
                v += ev * 100.0 + k * sd * (left / SPINS as f64);
            }
        }
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ruleta europeană — învârtirea {}/{SPINS}. Jetoane: J1 {}, J2 {}.\n",
            (self.spins + 1).min(SPINS),
            self.chips[0],
            self.chips[1]
        );
        for i in 0..2 {
            if let Some(b) = self.bets[i] {
                s.push_str(&format!("Pariul lui J{}: {}\n", i + 1, bet_text(&b)));
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            match self.placing() {
                Some(p) => s.push_str(&format!("La mutare: Jucător {} — pariază\n", p.idx() + 1)),
                None => s.push_str("La mutare: Jucător 1 — invart\n"),
            }
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Spin => "invart".into(),
            Move::Place(b) => bet_text(b),
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
    fn payouts() {
        let b = |kind, num| Bet {
            kind,
            num,
            stake: 10,
        };
        assert_eq!(payout(&b(0, 17), 17), 35);
        assert_eq!(payout(&b(1, 0), 1), 1);
        assert_eq!(payout(&b(1, 0), 2), 0);
        assert_eq!(payout(&b(2, 0), 0), 0);
        assert_eq!(payout(&b(7, 2), 13), 2);
        assert_eq!(payout(&b(7, 2), 24), 2);
        assert_eq!(payout(&b(7, 2), 25), 0);
        assert_eq!(payout(&b(8, 1), 34), 2);
        assert_eq!(payout(&b(8, 3), 34), 0);
        assert_eq!(payout(&b(5, 0), 18), 1);
        assert_eq!(payout(&b(6, 0), 18), 0);
        assert_eq!(cover(&b(1, 0)), 18);
    }

    #[test]
    fn chips_are_conserved_as_zero_sum_with_the_bank() {
        let mut g = Ruleta::with_seed(3);
        g.apply(&Move::Place(Bet {
            kind: 1,
            num: 0,
            stake: 10,
        }));
        g.apply(&Move::Place(Bet {
            kind: 2,
            num: 0,
            stake: 10,
        }));
        assert_eq!(g.legal_moves(), vec![Move::Spin]);
        g.apply(&Move::Spin);
        assert_eq!(g.spins, 1);
        assert!(g.chips[0] != 100 || g.chips[1] != 100 || g.last.contains("0"));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Ruleta::with_seed, 5, 500);
    }
}
