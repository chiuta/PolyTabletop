//! Balansul Pietrelor — echilibru pe o grindă (variantă originală, mecanică de „turn” instabil).
//!
//! O grindă cu 13 poziții (−6..6) în jurul unui pivot. Fiecare jucător are greutățile 1..6.
//! Pe rând pui o greutate pe o poziție liberă (`pune g@p`, ex. `pune 4@-3`). Momentul total
//! (suma greutate × poziție) nu are voie să depășească 18 în valoare absolută: cine răstoarnă
//! grinda pierde imediat. Dacă toate cele 12 greutăți stau, remiză.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

const LIM: i32 = 18;

#[derive(Clone)]
pub struct Balans {
    beam: [u8; 13],
    own: [[bool; 7]; 2],
    side: Player,
    tipped: Option<Player>,
}

impl Balans {
    pub fn new() -> Self {
        Self::with_seed(0)
    }
    pub fn with_seed(_seed: u64) -> Self {
        let mut own = [[false; 7]; 2];
        for p in 0..2 {
            for w in 1..=6 {
                own[p][w] = true;
            }
        }
        Balans {
            beam: [0; 13],
            own,
            side: Player::One,
            tipped: None,
        }
    }
    fn torque(&self) -> i32 {
        (0..13).map(|i| self.beam[i] as i32 * (i as i32 - 6)).sum()
    }
    fn safe_moves(&self, p: usize) -> i32 {
        let t = self.torque();
        let mut n = 0;
        for w in 1..=6 {
            if self.own[p][w] {
                for i in 0..13 {
                    if self.beam[i] == 0 && (t + w as i32 * (i as i32 - 6)).abs() <= LIM {
                        n += 1;
                    }
                }
            }
        }
        n
    }
}

impl Default for Balans {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Balans {
    fn depth(&self) -> u32 {
        2
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let p = pi(self.side);
        let mut v = Vec::new();
        for w in 1..=6 {
            if self.own[p][w] {
                for i in 0..13 {
                    if self.beam[i] == 0 {
                        v.push(format!("pune {}@{}", w, i as i32 - 6));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let r = m.strip_prefix("pune ").unwrap();
        let (w, pos) = r.split_once('@').unwrap();
        let w: usize = w.parse().unwrap();
        let pos: i32 = pos.parse().unwrap();
        self.beam[(pos + 6) as usize] = w as u8;
        self.own[pi(self.side)][w] = false;
        if self.torque().abs() > LIM {
            self.tipped = Some(self.side);
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(p) = self.tipped {
            return Some(Outcome::Win(p.other()));
        }
        if self.beam.iter().filter(|&&b| b > 0).count() == 12 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        self.safe_moves(pi(p)) - self.safe_moves(1 - pi(p))
    }
    fn show(&self) -> String {
        let mut s = String::from("Balansul Pietrelor — poziții: ");
        for i in 0..13 {
            s.push_str(&format!("{:>3}", i - 6));
        }
        s.push_str("\n                 greutăți: ");
        for i in 0..13 {
            if self.beam[i] == 0 {
                s.push_str("  .");
            } else {
                s.push_str(&format!("{:>3}", self.beam[i]));
            }
        }
        let rem = |p: usize| {
            (1..=6)
                .filter(|&w| self.own[p][w])
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        };
        s.push_str(&format!("\nMoment total: {} (limita ±{}). J1 are: {} | J2 are: {}\nLa mutare: {}. Mutare: pune g@p\n", self.torque(), LIM, rem(0), rem(1), pname(self.side)));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Balans::with_seed, 100);
    }
    #[test]
    fn tipping_loses() {
        let mut g = Balans::new();
        g.step("pune 3@6");
        assert_eq!(g.result(), None);
        g.step("pune 2@5");
        assert_eq!(g.result(), Some(Outcome::Win(Player::One)));
    }
}
