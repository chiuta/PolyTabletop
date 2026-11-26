//! Bagatelle (Franța, sec. XVIII): masă înclinată cu cuie și 9 căsuțe cu puncte; fiecare jucător
//! lansează 9 bile, câștigă cine adună mai multe puncte.
//!
//! * Alegi culoarul de lansare (1–5) și puterea (1 = ușor, 2 = mediu, 3 = tare): `tir 3 2`.
//!   Apoi bila e lăsată să curgă (`lansez`, mutare separată).
//! * Bila coboară printre 7 rânduri de cuie: la fiecare rând merge drept cu o probabilitate care
//!   crește cu puterea (20% / 45% / 75%), altfel sare o căsuță în stânga sau în dreapta. O lovitură
//!   mai puternică are și risc de a sări peste masă (0% / 10% / 25%) și nu dă nimic.
//! * Căsuțele (stânga → dreapta): 0, 5, 1, 3, 10, 3, 1, 5, 0 puncte; culoarele pornesc din căsuțele
//!   0, 2, 4, 6, 8. Căsuța de 10 primește cel mult 3 bile, cele de 5 cel mult 4 (în total, ambii
//!   jucători): o bilă căzută într-o căsuță plină nu mai dă puncte.
//! * Probabilitățile sunt alegerea noastră (jocul fizic depinde de îndemânare); evaluarea AI folosește
//!   speranța exactă a loviturii alese, nu rezultatul ei.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const BALLS: u8 = 9;
const VALUES: [i32; 9] = [0, 5, 1, 3, 10, 3, 1, 5, 0];
const STRAIGHT: [f64; 3] = [0.20, 0.45, 0.75];
const OVER: [f64; 3] = [0.0, 0.10, 0.25];
/// Câte bile încap în fiecare căsuță (în total, ambii jucători); a 4-a în căsuța 10 nu mai punctează.
const CAP: [u8; 9] = [99, 4, 99, 99, 3, 99, 99, 4, 99];
const ROWS: usize = 7;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Aim(u8, u8),
    Release,
}

#[derive(Clone)]
pub struct Bagatelle {
    score: [i32; 2],
    shots: [u8; 2],
    filled: [u8; 9],
    side: Player,
    pending: Option<(u8, u8)>,
    rng: Rng,
    last: String,
}

impl Default for Bagatelle {
    fn default() -> Self {
        Self::new()
    }
}

/// Distribuția căsuței finale pentru (culoar 1..=5, putere 1..=3).
pub fn distribution(lane: u8, power: u8) -> [f64; 9] {
    let s = STRAIGHT[power as usize - 1];
    let mut d = [0.0; 9];
    d[(lane as usize - 1) * 2] = 1.0;
    for _ in 0..ROWS {
        let mut n = [0.0; 9];
        for x in 0..9 {
            n[x] += d[x] * s;
            let side = d[x] * (1.0 - s) / 2.0;
            n[if x == 0 { 1 } else { x - 1 }] += side;
            n[if x == 8 { 7 } else { x + 1 }] += side;
        }
        d = n;
    }
    d
}

pub fn expected_points(lane: u8, power: u8) -> f64 {
    expected_with(lane, power, &[0; 9])
}

/// Speranța loviturii știind câte bile sunt deja în fiecare căsuță (informație publică).
pub fn expected_with(lane: u8, power: u8, filled: &[u8; 9]) -> f64 {
    let d = distribution(lane, power);
    let e: f64 = (0..9)
        .map(|x| {
            if filled[x] < CAP[x] {
                d[x] * VALUES[x] as f64
            } else {
                0.0
            }
        })
        .sum();
    e * (1.0 - OVER[power as usize - 1])
}

impl Bagatelle {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Bagatelle {
            score: [0; 2],
            shots: [0; 2],
            filled: [0; 9],
            side: Player::One,
            pending: None,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn score(&self) -> [i32; 2] {
        self.score
    }
}

impl Game for Bagatelle {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Release];
        }
        let mut v = Vec::new();
        for lane in 1..=5 {
            for power in 1..=3 {
                v.push(Move::Aim(lane, power));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Aim(l, p) => self.pending = Some((*l, *p)),
            Move::Release => {
                let (lane, power) = self.pending.take().unwrap();
                let i = self.side.idx();
                let (s, o) = (STRAIGHT[power as usize - 1], OVER[power as usize - 1]);
                let r = self.rng.below(10_000) as f64 / 10_000.0;
                if r < o {
                    self.last = format!("J{}: bila sare peste masă (0 puncte).", i + 1);
                } else {
                    let mut x = (lane as usize - 1) * 2;
                    for _ in 0..ROWS {
                        let u = self.rng.below(10_000) as f64 / 10_000.0;
                        if u >= s {
                            let right = self.rng.below(2) == 1;
                            x = if right {
                                if x == 8 {
                                    7
                                } else {
                                    x + 1
                                }
                            } else if x == 0 {
                                1
                            } else {
                                x - 1
                            };
                        }
                    }
                    if self.filled[x] < CAP[x] {
                        self.filled[x] += 1;
                        self.score[i] += VALUES[x];
                        self.last = format!(
                            "J{}: bila cade în căsuța {} (+{}).",
                            i + 1,
                            x + 1,
                            VALUES[x]
                        );
                    } else {
                        self.last = format!(
                            "J{}: căsuța {} e plină, bila nu mai punctează.",
                            i + 1,
                            x + 1
                        );
                    }
                }
                self.shots[i] += 1;
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.shots[0] >= BALLS && self.shots[1] >= BALLS {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut d = (self.score[p.idx()] - self.score[p.other().idx()]) as f64;
        if let Some((l, pw)) = self.pending {
            let e = expected_with(l, pw, &self.filled);
            d += if self.side == p { e } else { -e };
        }
        (d * 100.0) as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Bagatelle — bile aruncate: J1 {}/{BALLS}, J2 {}/{BALLS}. Puncte: J1 {}, J2 {}.\n",
            self.shots[0], self.shots[1], self.score[0], self.score[1]
        );
        s.push_str("Căsuțe:  0  5  1  3 10  3  1  5  0\n");
        let f: Vec<String> = self.filled.iter().map(|x| format!("{x:>2}")).collect();
        s.push_str(&format!(
            "Bile în căsuțe: {} (10 se umple la 3, 5 la 4)\n",
            f.join(" ")
        ));
        s.push_str("Culoare: 1     2     3     4     5   (puterea 1–3)\n");
        if let Some((l, p)) = self.pending {
            s.push_str(&format!("Bila e pregătită: culoarul {l}, puterea {p}.\n"));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — {}\n",
                self.side.idx() + 1,
                if self.pending.is_some() {
                    "lansez"
                } else {
                    "tir C P"
                }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(l, p) => format!("tir {l} {p}"),
            Move::Release => "lansez".into(),
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
    fn distributions_sum_to_one_and_power_trades_spread_for_risk() {
        for l in 1..=5 {
            for p in 1..=3 {
                let s: f64 = distribution(l, p).iter().sum();
                assert!((s - 1.0).abs() < 1e-9);
            }
        }
        assert!(expected_points(3, 3) > expected_points(3, 2));
        assert!(expected_points(3, 2) > expected_points(1, 2));
        // o căsuță plină nu mai valorează nimic
        let mut f = [0u8; 9];
        f[4] = 3;
        assert!(expected_with(3, 3, &f) < expected_points(3, 3) * 0.6);
    }

    #[test]
    fn ai_prefers_the_best_expected_shot() {
        let mut g = Bagatelle::with_seed(1);
        g.filled[4] = 3;
        let best = polytabletop_core::ai::best_move(&g, 1).unwrap();
        let mut top = (0.0, Move::Release);
        for m in g.legal_moves() {
            if let Move::Aim(l, p) = m {
                let e = expected_with(l, p, &g.filled);
                if e > top.0 {
                    top = (e, Move::Aim(l, p));
                }
            }
        }
        assert_eq!(best, top.1);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Bagatelle::with_seed, 5, 500);
    }
}
