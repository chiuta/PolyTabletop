#![allow(clippy::doc_lazy_continuation)]
//! Mexico (Mäxchen, Mia), joc de bluf cu două zaruri și cupă, pentru doi jucători.
//!
//! * Ordinea aruncărilor, de la cea mai mică la cea mai mare: 31, 32, 41, 42, 43, 51, 52, 53, 54,
//!   61, 62, 63, 64, 65, apoi dublele 11, 22, 33, 44, 55, 66 și în vârf 21 („Mexico”).
//! * Arunci sub cupă, te uiți doar tu și anunți o valoare **strict mai mare** decât ultima
//!   anunțată (poți minți; la început orice valoare). Celălalt fie `cred` (aruncă și el sub cupă și
//!   trebuie să anunțe mai mult), fie `contest`: se ridică cupa; dacă aruncarea reală e mai mică
//!   decât anunțul, mincinosul pierde o viață; dacă anunțul era adevărat (sau depășit), pierde cel
//!   care a contestat. Dacă s-a anunțat Mexico (21), contestarea e obligatorie și se pierd două vieți.
//! * Fiecare are 5 vieți; cine rămâne fără vieți pierde; cine a pierdut o viață aruncă primul.
//! Mutări: `arunc`, `anunt 65` (sau `anunt 21`), `cred`, `contest`, `dezvaluie`.
//! AI-ul vede doar propriile zaruri; la contestație estimează după probabilități, nu din zarurile
//! adversarului.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LIVES: u8 = 5;
pub const NAMES: [&str; 21] = [
    "31", "32", "41", "42", "43", "51", "52", "53", "54", "61", "62", "63", "64", "65", "11", "22",
    "33", "44", "55", "66", "21",
];

pub fn rank_of(a: u8, b: u8) -> u8 {
    let (hi, lo) = (a.max(b), a.min(b));
    if hi == 2 && lo == 1 {
        return 20;
    }
    if hi == lo {
        return 13 + hi;
    }
    let code = hi * 10 + lo;
    NAMES[..14]
        .iter()
        .position(|n| n.parse::<u8>().unwrap() == code)
        .unwrap() as u8
}

fn count(r: u8) -> f64 {
    if r <= 13 || r == 20 {
        2.0
    } else {
        1.0
    }
}

/// Probabilitatea ca o aruncare corectă să fie cel puțin `claim`.
fn p_true(claim: u8) -> f64 {
    (claim..=20).map(count).sum::<f64>() / 36.0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Roll,
    Claim,
    Respond,
    Reveal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Claim(u8),
    Believe,
    Contest,
    Reveal,
}

#[derive(Clone)]
pub struct Mexico {
    rng: Rng,
    lives: [u8; 2],
    roller: Player,
    dice: (u8, u8),
    claim: Option<u8>,
    phase: Phase,
    contested: bool,
    viewer: Viewer,
    log: String,
}

impl Default for Mexico {
    fn default() -> Self {
        Self::new()
    }
}

impl Mexico {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Mexico {
            rng: Rng::new(seed),
            lives: [LIVES; 2],
            roller: Player::One,
            dice: (1, 1),
            claim: None,
            phase: Phase::Roll,
            contested: false,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    fn actual(&self) -> u8 {
        rank_of(self.dice.0, self.dice.1)
    }

    fn over(&self) -> bool {
        self.lives.contains(&0)
    }
}

impl Game for Mexico {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Roll | Phase::Claim => self.roller,
            Phase::Respond => self.roller.other(),
            Phase::Reveal => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Roll => vec![Move::Roll],
            Phase::Claim => {
                let lo = self.claim.map_or(0, |c| c + 1);
                (lo..=20).map(Move::Claim).collect()
            }
            Phase::Respond => {
                if self.claim == Some(20) {
                    vec![Move::Contest]
                } else {
                    vec![Move::Believe, Move::Contest]
                }
            }
            Phase::Reveal => vec![Move::Reveal],
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Roll => {
                self.dice = (self.rng.d6(), self.rng.d6());
                self.phase = Phase::Claim;
            }
            Move::Claim(c) => {
                self.claim = Some(c);
                self.phase = Phase::Respond;
            }
            Move::Believe => {
                self.roller = self.roller.other();
                self.phase = Phase::Roll;
            }
            Move::Contest => {
                self.contested = true;
                self.phase = Phase::Reveal;
            }
            Move::Reveal => {
                let claim = self.claim.unwrap();
                let truth = self.actual() >= claim;
                let loser = if truth {
                    self.roller.other()
                } else {
                    self.roller
                };
                let loss = if claim == 20 { 2 } else { 1 };
                self.lives[loser.idx()] = self.lives[loser.idx()].saturating_sub(loss);
                self.log = format!(
                    "J{} anunțase {}, era {}: J{} pierde {loss} {}.",
                    self.roller.idx() + 1,
                    NAMES[claim as usize],
                    NAMES[self.actual() as usize],
                    loser.idx() + 1,
                    if loss == 1 { "viață" } else { "vieți" }
                );
                self.roller = loser;
                self.claim = None;
                self.contested = false;
                self.phase = Phase::Roll;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(if self.lives[0] == 0 {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Win(Player::One)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let lead = 100 * (self.lives[p.idx()] as i32 - self.lives[q.idx()] as i32);
        let Some(claim) = self.claim else { return lead };
        let pt = p_true(claim);
        let unit = if claim == 20 { 200.0 } else { 100.0 };
        let v = match self.phase {
            Phase::Respond if self.roller == p => {
                // p tocmai a anunțat și își cunoaște zarurile
                let truth = self.actual() >= claim;
                let pdoubt = (1.0 - pt).clamp(0.2, 0.9);
                if truth {
                    pdoubt * unit + (1.0 - pdoubt) * 10.0
                } else {
                    -pdoubt * unit + (1.0 - pdoubt) * (claim as f64 * 2.0)
                }
            }
            Phase::Reveal if self.roller != p => (1.0 - 2.0 * pt) * unit * 0.9,
            Phase::Roll if self.roller == p => -(1.0 - pt) * unit * 0.6,
            Phase::Reveal if self.roller == p => {
                if self.actual() >= claim {
                    unit
                } else {
                    -unit
                }
            }
            _ => 0.0,
        };
        lead + v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Mexico — vieți: J1 {}, J2 {}.\n",
            self.lives[0], self.lives[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let me = self.viewer.who(self.to_move());
        if let Some(c) = self.claim {
            s.push_str(&format!(
                "Ultimul anunț (J{}): {}.\n",
                self.roller.idx() + 1,
                NAMES[c as usize]
            ));
        }
        match self.phase {
            Phase::Roll => s.push_str(&format!(
                "J{} aruncă sub cupă (`arunc`).\n",
                self.roller.idx() + 1
            )),
            Phase::Claim | Phase::Respond => {
                if let Viewer::HotSeat = self.viewer {
                    if self.phase == Phase::Respond {
                        s.push_str(&handover_banner(self.roller.other()));
                    }
                }
                if me == self.roller {
                    s.push_str(&format!(
                        "Zarurile tale (ascunse pentru celălalt): {} {} = {}.\n",
                        self.dice.0,
                        self.dice.1,
                        NAMES[self.actual() as usize]
                    ));
                } else {
                    s.push_str("Zarurile din cupă sunt ascunse.\n");
                }
                if self.phase == Phase::Claim {
                    s.push_str("Anunță o valoare mai mare: `anunt 65`.\n");
                } else {
                    s.push_str("`cred` sau `contest`.\n");
                }
            }
            Phase::Reveal => s.push_str(&format!(
                "Cupa se ridică: {} {}. Scrie «dezvaluie».\n",
                self.dice.0, self.dice.1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Claim(c) => format!("anunt {}", NAMES[*c as usize]),
            Move::Believe => "cred".into(),
            Move::Contest => "contest".into(),
            Move::Reveal => "dezvaluie".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranking_and_probabilities() {
        assert_eq!(rank_of(2, 1), 20);
        assert_eq!(rank_of(3, 1), 0);
        assert_eq!(rank_of(6, 5), 13);
        assert_eq!(rank_of(1, 1), 14);
        assert!(rank_of(6, 6) < rank_of(1, 2));
        assert!((p_true(0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn lie_costs_a_life_and_truth_costs_the_doubter() {
        let mut g = Mexico::with_seed(1);
        g.dice = (3, 1);
        g.roller = Player::One;
        g.claim = Some(rank_of(6, 5));
        g.phase = Phase::Reveal;
        g.apply(&Move::Reveal);
        assert_eq!(g.lives, [LIVES - 1, LIVES]);
        assert_eq!(g.roller, Player::One);
        g.dice = (6, 5);
        g.claim = Some(rank_of(6, 5));
        g.phase = Phase::Reveal;
        g.apply(&Move::Reveal);
        assert_eq!(g.lives, [LIVES - 1, LIVES - 1]);
    }

    #[test]
    fn random_games_end_and_views_are_private() {
        for seed in 1..=30u64 {
            let mut g = Mexico::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
        let mut g = Mexico::with_seed(4);
        g.apply(&Move::Roll);
        g.set_viewer(Some(Player::Two));
        assert!(g.render().contains("ascunse"));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("Zarurile tale"));
    }
}
