//! Sărit coarda (Jump rope), joc pentru copii — pe doi, cu alternarea rolurilor.
//!
//! * 8 runde; în fiecare, un jucător sare și celălalt învârte coarda (rolurile alternează, J1 sare întâi).
//!   Cel care învârte alege viteza (`lent`, `mediu`, `rapid`): lent = +8% șanse la fiecare săritură,
//!   punctele ×1; mediu = punctele ×2; rapid = −12% șanse, punctele ×3.
//! * Săritorul alege pentru fiecare săritură un truc și apoi `sar` (rezolvarea e o mutare separată):
//!   `simplu` (1 punct, 92% șanse), `incrucisat` (3 puncte, 75%), `dublu` (5 puncte, 55%), `triplu`
//!   (9 puncte, 30%). Dacă reușește, își adaugă punctele (× viteza) la șirul curent; dacă greșește, pierde
//!   tot șirul. Poate spune `opresc` oricând ca să bage șirul în scor. Cel mult 8 sărituri pe rundă.
//! * Câștigă cine are mai multe puncte la sfârșit. AI-ul calculează strategia optimă (oprire/continuare)
//!   din probabilitățile publice; cel care învârte alege viteza care îi dă săritorului valoarea cea mai mică.
//!   Mutări: `lent`, `mediu`, `rapid`, `simplu`, `incrucisat`, `dublu`, `triplu`, `opresc`, `sar`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 8;
const MAX_JUMPS: u8 = 8;
const BASE_P: [f64; 4] = [0.92, 0.75, 0.55, 0.30];
const VAL: [i32; 4] = [1, 3, 5, 9];
const TRICKS: [&str; 4] = ["simplu", "incrucisat", "dublu", "triplu"];
const SPEEDS: [&str; 3] = ["lent", "mediu", "rapid"];
const ADJ: [f64; 3] = [0.08, 0.0, -0.12];
const MULT: [i32; 3] = [1, 2, 3];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Speed(u8),
    Trick(u8),
    Stop,
    Jump,
}

#[derive(Clone)]
pub struct Coarda {
    bank: [i32; 2],
    round: u8,
    speed: Option<u8>,
    run: i32,
    jumps: u8,
    pending: Option<u8>,
    rng: Rng,
    last: String,
}

impl Default for Coarda {
    fn default() -> Self {
        Self::new()
    }
}

pub fn p_success(trick: u8, speed: u8) -> f64 {
    (BASE_P[trick as usize] + ADJ[speed as usize]).clamp(0.05, 0.99)
}

/// Valoarea optimă (în puncte) a șirului pentru săritor, ținând cont de oprire.
pub fn best_value(speed: u8, jumps_left: u8, run: i32) -> f64 {
    if jumps_left == 0 {
        return run as f64;
    }
    let mut best = run as f64;
    for t in 0..4u8 {
        let p = p_success(t, speed);
        let v = p * best_value(
            speed,
            jumps_left - 1,
            run + VAL[t as usize] * MULT[speed as usize],
        );
        if v > best {
            best = v;
        }
    }
    best
}

impl Coarda {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Coarda {
            bank: [0; 2],
            round: 0,
            speed: None,
            run: 0,
            jumps: 0,
            pending: None,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn bank(&self) -> [i32; 2] {
        self.bank
    }

    fn jumper(&self) -> Player {
        if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }

    fn end_round(&mut self) {
        self.round += 1;
        self.speed = None;
        self.run = 0;
        self.jumps = 0;
        self.pending = None;
    }

    /// Speranța rundei curente pentru săritor, după ultima decizie.
    fn expectation(&self) -> Option<f64> {
        let s = self.speed?;
        match self.pending {
            Some(t) => Some(
                p_success(t, s)
                    * best_value(
                        s,
                        MAX_JUMPS - self.jumps - 1,
                        self.run + VAL[t as usize] * MULT[s as usize],
                    ),
            ),
            None => Some(best_value(s, MAX_JUMPS - self.jumps, self.run)),
        }
    }
}

impl Game for Coarda {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.speed.is_none() {
            self.jumper().other()
        } else {
            self.jumper()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.speed.is_none() {
            return (0..3).map(Move::Speed).collect();
        }
        if self.pending.is_some() {
            return vec![Move::Jump];
        }
        let mut v: Vec<Move> = (0..4).map(Move::Trick).collect();
        if self.run > 0 || self.jumps > 0 {
            v.push(Move::Stop);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let j = self.jumper().idx();
        match m {
            Move::Speed(s) => self.speed = Some(*s),
            Move::Trick(t) => self.pending = Some(*t),
            Move::Stop => {
                self.bank[j] += self.run;
                self.last = format!("J{} se oprește și bagă {} puncte.", j + 1, self.run);
                self.end_round();
            }
            Move::Jump => {
                let t = self.pending.take().unwrap();
                let s = self.speed.unwrap();
                let r = self.rng.below(10_000) as f64 / 10_000.0;
                self.jumps += 1;
                if r < p_success(t, s) {
                    self.run += VAL[t as usize] * MULT[s as usize];
                    self.last = format!(
                        "J{} reușește «{}» (șir {}).",
                        j + 1,
                        TRICKS[t as usize],
                        self.run
                    );
                    if self.jumps >= MAX_JUMPS {
                        self.bank[j] += self.run;
                        self.last.push_str(" Runda s-a încheiat: bagă șirul.");
                        self.end_round();
                    }
                } else {
                    self.last = format!(
                        "J{} greșește «{}» și pierde șirul de {}.",
                        j + 1,
                        TRICKS[t as usize],
                        self.run
                    );
                    self.end_round();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            return Some(by_score(self.bank[0], self.bank[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.bank[me] - self.bank[1 - me]) as f64 * 100.0;
        if let Some(e) = self.expectation() {
            // `expectation` e valoarea șirului pentru săritor (nebăgat încă în scor)
            v += if self.jumper() == p {
                e * 100.0
            } else {
                -e * 100.0
            };
        }
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Sărit coarda — runda {}/{ROUNDS}. Puncte: J1 {}, J2 {}. Sare: J{}, învârte: J{}.\n",
            (self.round + 1).min(ROUNDS),
            self.bank[0],
            self.bank[1],
            self.jumper().idx() + 1,
            self.jumper().other().idx() + 1
        );
        if let Some(sp) = self.speed {
            s.push_str(&format!(
                "Viteza: {} (puncte ×{}). Șir curent: {} după {} sărituri.\n",
                SPEEDS[sp as usize], MULT[sp as usize], self.run, self.jumps
            ));
        }
        if let Some(t) = self.pending {
            s.push_str(&format!("Truc ales: {}.\n", TRICKS[t as usize]));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            let what = if self.speed.is_none() {
                "alege viteza (lent / mediu / rapid)"
            } else if self.pending.is_some() {
                "sar"
            } else {
                "truc: simplu / incrucisat / dublu / triplu, sau opresc"
            };
            s.push_str(&format!(
                "La mutare: Jucător {} — {what}\n",
                self.to_move().idx() + 1
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Speed(s) => SPEEDS[*s as usize].into(),
            Move::Trick(t) => TRICKS[*t as usize].into(),
            Move::Stop => "opresc".into(),
            Move::Jump => "sar".into(),
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
    fn optimal_stopping_values() {
        // cu 0 sărituri rămase valoarea e șirul
        assert_eq!(best_value(1, 0, 7), 7.0);
        // șirul mare se oprește: cu șir 20 și viteză mare nu merită un «triplu»
        assert_eq!(best_value(2, 3, 20), 20.0);
        // de la zero merită să sari
        assert!(best_value(1, 8, 0) > 5.0);
        assert!(p_success(3, 2) < p_success(3, 0));
    }

    #[test]
    fn turner_picks_speed_then_jumper_plays() {
        let mut g = Coarda::with_seed(1);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Speed(1));
        assert_eq!(g.to_move(), Player::One);
        assert!(!g.legal_moves().contains(&Move::Stop));
        g.apply(&Move::Trick(0));
        assert_eq!(g.legal_moves(), vec![Move::Jump]);
        g.apply(&Move::Jump);
        // fie șir > 0, fie runda s-a terminat și acum sare J2
        assert!(g.run > 0 || g.round == 1);
    }

    #[test]
    fn ai_stops_with_a_big_run() {
        let mut g = Coarda::with_seed(1);
        g.apply(&Move::Speed(2));
        g.run = 30;
        g.jumps = 2;
        assert_eq!(polytabletop_core::ai::best_move(&g, 1), Some(Move::Stop));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Coarda::with_seed, 6, 600);
    }
}
