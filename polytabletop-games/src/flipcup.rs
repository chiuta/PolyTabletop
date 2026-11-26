#![allow(clippy::doc_lazy_continuation)]
//! Flip Cup (SUA), jocul de petrecere cu paharul întors, ca duel de echipe de câte 5.
//!
//! * Două echipe (J1 și J2) de câte 5 jucători în relay: pe rând, fiecare membru trebuie să răstoarne
//!   paharul pe masă. O echipă are 5 răsturnări de făcut; câștigă prima echipă care le termină.
//! * Fiecare încercare costă timp. `repede` (reușește 55%, costă 2) sau `atent` (reușește 80%, costă 3);
//!   la ratare se repetă încercarea (același membru), dar timpul se pierde. Joacă la rând echipa
//!   cu ceasul mai mic (la egalitate J1). Câștigă echipa care își termină cele 5 răsturnări cu
//!   timpul cel mai mic (se joacă până când nici o încercare a celeilalte echipe nu mai poate termina
//!   înainte).
//! * Încercarea se hotărăște la pasul forțat `rastoarna` (arbitru J2); AI-ul alege după viteza
//!   așteptată, nu din zaruri.
//! Mutări: `repede`, `atent`, `rastoarna`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const FLIPS: u8 = 5;

fn params(careful: bool) -> (f64, u32) {
    if careful {
        (0.80, 3)
    } else {
        (0.55, 2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Fast,
    Careful,
    Flip,
}

#[derive(Clone)]
pub struct FlipCup {
    rng: Rng,
    done: [u8; 2],
    clock: [u32; 2],
    finish: [Option<u32>; 2],
    pending: Option<(usize, bool)>,
    last: String,
}

impl Default for FlipCup {
    fn default() -> Self {
        Self::new()
    }
}

impl FlipCup {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        FlipCup {
            rng: Rng::new(seed),
            done: [0; 2],
            clock: [0; 2],
            finish: [None; 2],
            pending: None,
            last: String::new(),
        }
    }

    /// Echipa care joacă: cea nefinalizată cu ceasul mai mic.
    fn next_team(&self) -> Option<usize> {
        let open: Vec<usize> = (0..2).filter(|&i| self.finish[i].is_none()).collect();
        let best_finish = self.finish.iter().flatten().min().copied();
        let t = open.into_iter().min_by_key(|&i| (self.clock[i], i))?;
        if let Some(f) = best_finish {
            if self.clock[t] >= f {
                return None;
            }
        }
        Some(t)
    }
}

impl Game for FlipCup {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            return Player::Two;
        }
        match self.next_team() {
            Some(1) => Player::Two,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.pending.is_some() {
            return vec![Move::Flip];
        }
        if self.next_team().is_none() {
            return Vec::new();
        }
        vec![Move::Fast, Move::Careful]
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Fast => self.pending = Some((self.next_team().unwrap(), false)),
            Move::Careful => self.pending = Some((self.next_team().unwrap(), true)),
            Move::Flip => {
                let (t, careful) = self.pending.take().unwrap();
                let (p, cost) = params(careful);
                self.clock[t] += cost;
                if (self.rng.below(1000) as f64) < p * 1000.0 {
                    self.done[t] += 1;
                    self.last = format!(
                        "Echipa J{}: pahar răsturnat ({}/{FLIPS}) la timpul {}",
                        t + 1,
                        self.done[t],
                        self.clock[t]
                    );
                    if self.done[t] == FLIPS {
                        self.finish[t] = Some(self.clock[t]);
                    }
                } else {
                    self.last = format!(
                        "Echipa J{}: pahar căzut pe margine, se repetă (timp {})",
                        t + 1,
                        self.clock[t]
                    );
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pending.is_some() || self.next_team().is_some() {
            return None;
        }
        match (self.finish[0], self.finish[1]) {
            (Some(a), Some(b)) => Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            }),
            (Some(_), None) => Some(Outcome::Win(Player::One)),
            (None, Some(_)) => Some(Outcome::Win(Player::Two)),
            _ => None,
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        // timp așteptat rămas = (răsturnări rămase) × (cost mediu pe răsturnare cu politica cea mai bună)
        let rate = |careful: bool| {
            let (pr, c) = params(careful);
            c as f64 / pr
        };
        let best = rate(false).min(rate(true));
        let eta = |i: usize| -> f64 {
            let mut t = self.clock[i] as f64 + (FLIPS - self.done[i]) as f64 * best;
            if let Some((pi, careful)) = self.pending {
                if pi == i {
                    let (pr, c) = params(careful);
                    t = self.clock[i] as f64 + c as f64 + (FLIPS - self.done[i]) as f64 * best
                        - pr * best;
                }
            }
            t
        };
        ((eta(p.other().idx()) - eta(p.idx())) * 100.0) as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Flip Cup — două echipe, 5 răsturnări fiecare\n");
        for i in 0..2 {
            s.push_str(&format!(
                "Echipa J{}: {}/{FLIPS} pahare, ceas {}{}\n",
                i + 1,
                self.done[i],
                self.clock[i],
                match self.finish[i] {
                    Some(f) => format!(" — gata la {f}"),
                    None => String::new(),
                }
            ));
        }
        if let Some(t) = self.next_team() {
            s.push_str(&format!(
                "Urmează echipa J{}: `repede` (55%, timp 2) sau `atent` (80%, timp 3).\n",
                t + 1
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Fast => "repede",
            Move::Careful => "atent",
            Move::Flip => "rastoarna",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_end_with_a_winner_by_time() {
        for seed in 1..=40u64 {
            let mut g = FlipCup::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 2000);
            }
            assert!(g.finish.iter().any(|f| f.is_some()));
        }
    }

    #[test]
    fn lower_clock_plays_and_ai_is_blind() {
        let mut g = FlipCup::with_seed(1);
        g.clock = [3, 2];
        assert_eq!(g.to_move(), Player::Two);
        let (a, b) = (FlipCup::with_seed(1), FlipCup::with_seed(8));
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
