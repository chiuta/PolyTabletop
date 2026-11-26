#![allow(clippy::doc_lazy_continuation)]
//! Bar billiards (Marea Britanie), masa de crâșmă cu găuri și popici, simulat pe lovituri, pentru doi
//! jucători.
//!
//! * Găurile valorează 10, 20, 30 (rândul din capăt), 50 (laterale), 100 (centru) și 200 (față, în
//!   spatele popicii negre). Bila roșie dublează punctele; o bilă albă le dă simplu.
//! * O „vizită” (break) continuă cât timp loviturile băgă o bilă; o lovitură fără punct închide
//!   vizita (punctele rămân). Un **fault** (popică doborâtă ș.a.) închide vizita și anulează
//!   punctele vizitei; popica neagră (în fața găurii de 200) anulează tot scorul jucătorului.
//! * Simulare: fiecare țintă are o șansă de reușită și de fault (de exemplu 200: 10% reușită,
//!   20% fault, din care jumătate cu popica neagră; bila roșie scade șansa cu o zecime). 8 vizite
//!   pe jucător; câștigă scorul mai mare.
//! * Lovitura se hotărăște la pasul forțat `lovitura` (arbitru J2); AI-ul alege după valoarea
//!   așteptată, nu din rezultat.
//! Mutări: `10 alba`, `100 rosie`, … (`10|20|30|50|100|200` × `alba|rosie`), `lovitura`, `stop`
//! (închei vizita păstrând punctele; nu din prima lovitură).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const VISITS: u8 = 8;
const MAX_STRIKES: u8 = 12;
/// (valoare, șansă de a băga, șansă de fault)
const TARGETS: [(i32, f64, f64); 6] = [
    (10, 0.60, 0.05),
    (20, 0.50, 0.05),
    (30, 0.42, 0.06),
    (50, 0.30, 0.12),
    (100, 0.20, 0.15),
    (200, 0.10, 0.20),
];

fn probs(t: usize, red: bool) -> (f64, f64) {
    let (_, p, f) = TARGETS[t];
    (if red { p * 0.9 } else { p }, f)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Shot(u8, bool),
    Strike,
    Stop,
}

#[derive(Clone)]
pub struct BarBilliards {
    rng: Rng,
    score: [i32; 2],
    visits: [u8; 2],
    side: Player,
    visit_pts: i32,
    strikes: u8,
    pending: Option<(u8, bool)>,
    last: String,
}

impl Default for BarBilliards {
    fn default() -> Self {
        Self::new()
    }
}

impl BarBilliards {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        BarBilliards {
            rng: Rng::new(seed),
            score: [0; 2],
            visits: [0; 2],
            side: Player::One,
            visit_pts: 0,
            strikes: 0,
            pending: None,
            last: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.visits[0] >= VISITS && self.visits[1] >= VISITS
    }

    fn end_visit(&mut self) {
        let i = self.side.idx();
        self.visits[i] += 1;
        self.visit_pts = 0;
        self.strikes = 0;
        let o = self.side.other();
        if self.visits[o.idx()] < VISITS {
            self.side = o;
        }
    }
}

impl Game for BarBilliards {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            Player::Two
        } else {
            self.side
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Strike];
        }
        let mut v = Vec::new();
        for t in 0..6u8 {
            for red in [false, true] {
                v.push(Move::Shot(t, red));
            }
        }
        if self.strikes > 0 {
            v.push(Move::Stop);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let i = self.side.idx();
        match *m {
            Move::Shot(t, red) => self.pending = Some((t, red)),
            Move::Stop => self.end_visit(),
            Move::Strike => {
                let (t, red) = self.pending.take().unwrap();
                let (p, f) = probs(t as usize, red);
                let val = TARGETS[t as usize].0 * if red { 2 } else { 1 };
                self.strikes += 1;
                let r = self.rng.below(10_000) as f64 / 10_000.0;
                if r < f {
                    let black = t == 5 && self.rng.below(2) == 0;
                    if black {
                        self.score[i] = 0;
                        self.last = format!("J{}: popica neagră! Scorul se anulează.", i + 1);
                    } else {
                        self.last = format!(
                            "J{}: fault, punctele vizitei ({}) se pierd.",
                            i + 1,
                            self.visit_pts
                        );
                        self.score[i] -= self.visit_pts;
                    }
                    self.end_visit();
                } else if r < f + p {
                    self.score[i] += val;
                    self.visit_pts += val;
                    self.last = format!(
                        "J{}: bilă în gaura de {}, +{val}",
                        i + 1,
                        TARGETS[t as usize].0
                    );
                    if self.strikes >= MAX_STRIKES {
                        self.end_visit();
                    }
                } else {
                    self.last = format!("J{}: bila nu a intrat, vizita se încheie.", i + 1);
                    self.end_visit();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = self.score[p.idx()] - self.score[p.other().idx()];
        if let Some((t, red)) = self.pending {
            let (pr, f) = probs(t as usize, red);
            let val = (TARGETS[t as usize].0 * if red { 2 } else { 1 }) as f64;
            let black_loss = if t == 5 {
                0.5 * self.score[self.side.idx()] as f64
            } else {
                0.0
            };
            let ev = pr * val - f * (self.visit_pts as f64 + black_loss) + pr * 6.0;
            v += if self.side == p {
                ev as i32
            } else {
                -(ev as i32)
            };
        }
        v * 10
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Bar billiards — vizita {}/{VISITS} (J1), {}/{VISITS} (J2). Scor: J1 {} – J2 {}.\n",
            (self.visits[0] + 1).min(VISITS),
            (self.visits[1] + 1).min(VISITS),
            self.score[0],
            self.score[1]
        );
        if !self.over() {
            s.push_str(&format!(
                "La masă: J{} (puncte în vizită {}, lovituri {}). Alege `100 rosie` etc.\n",
                self.side.idx() + 1,
                self.visit_pts,
                self.strikes
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Shot(t, red) => format!(
                "{} {}",
                TARGETS[*t as usize].0,
                if *red { "rosie" } else { "alba" }
            ),
            Move::Strike => "lovitura".into(),
            Move::Stop => "stop".into(),
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
    fn games_end_and_stop_needs_a_strike() {
        let g = BarBilliards::with_seed(1);
        assert!(!g.legal_moves().contains(&Move::Stop));
        for seed in 1..=30u64 {
            let mut g = BarBilliards::with_seed(seed);
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
    fn ai_blind_to_future_strikes() {
        let (a, b) = (BarBilliards::with_seed(1), BarBilliards::with_seed(5));
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 2),
            polytabletop_core::GameObj::ai_move(&b, 2)
        );
    }
}
