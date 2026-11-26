//! Cvartetul Isteț — cursă pe o pistă cu probe de patru feluri (variantă originală, 2 jucători).
//!
//! Pista are 24 de pătrate; fiecare pătrat are un fel de probă (0 creativă, 1 cunoștințe, 2 cuvinte,
//! 3 scenă). Mutare: `incerc c r` (felul c = 0..3, riscul r = 1..3). Șansa de reușită crește cu
//! îndemânarea ta în felul ales și scade cu riscul și când felul nu e cel al pătratului tău. Reușita
//! (mutarea forțată `tira`, decisă de zar) te avansează cu 2, 3 sau 5 pătrate și îți crește
//! îndemânarea; eșecul îl avansează pe adversar cu 1. Primul la pătratul 24 câștigă.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const END: i32 = 24;

#[derive(Clone)]
pub struct Cvartet {
    pos: [i32; 2],
    skill: [[i32; 4]; 2],
    turn: usize,
    pending: Option<(usize, usize)>,
    plies: u32,
    rng: Rng,
}

pub fn new() -> Cvartet {
    Cvartet::with_seed(seed_now())
}

impl Cvartet {
    pub fn with_seed(seed: u64) -> Self {
        Cvartet {
            pos: [0; 2],
            skill: [[0; 4]; 2],
            turn: 0,
            pending: None,
            plies: 0,
            rng: Rng::new(seed),
        }
    }
    fn pct(&self, p: usize, c: usize, r: usize) -> i32 {
        let mut x = 88 - 24 * (r as i32 - 1) + 9 * self.skill[p][c];
        if c as i32 != self.pos[p] % 4 {
            x -= 15;
        }
        x.clamp(5, 95)
    }
    fn adv(r: usize) -> i32 {
        [0, 2, 3, 5][r]
    }
}

impl Mini for Cvartet {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.turn == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.pending.is_some() {
            return vec!["tira".into()];
        }
        let mut v = Vec::new();
        for c in 0..4 {
            for r in 1..=3 {
                v.push(format!("incerc {c} {r}"));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let p = self.turn;
        if m == "tira" {
            let (c, r) = self.pending.take().unwrap();
            if (self.rng.below(100) as i32) < self.pct(p, c, r) {
                self.pos[p] += Self::adv(r);
                self.skill[p][c] = (self.skill[p][c] + 1).min(3);
            } else {
                self.pos[1 - p] += 1;
            }
            if self.pos[0] < END && self.pos[1] < END {
                self.turn = 1 - p;
            }
            return;
        }
        let mut it = m.split(' ');
        it.next();
        let c: usize = it.next().unwrap().parse().unwrap();
        let r: usize = it.next().unwrap().parse().unwrap();
        self.pending = Some((c, r));
    }
    fn result(&self) -> Option<Outcome> {
        if self.pos[0] >= END && self.pos[1] >= END {
            return Some(Outcome::Draw);
        }
        if self.pos[0] >= END {
            Some(Outcome::Win(Player::One))
        } else if self.pos[1] >= END {
            Some(Outcome::Win(Player::Two))
        } else if self.plies > 400 {
            Some(by_score(self.pos[0], self.pos[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut e = 10 * (self.pos[i] - self.pos[o])
            + 2 * (self.skill[i].iter().sum::<i32>() - self.skill[o].iter().sum::<i32>());
        if let Some((c, r)) = self.pending {
            let t = self.turn;
            let x = self.pct(t, c, r);
            let g = x * 10 * Self::adv(r) / 100 - (100 - x) * 10 / 100 + x * 2 / 100;
            e += if t == i { g } else { -g };
        }
        e
    }
    fn show(&self) -> String {
        let mut s = String::from("Cvartetul Isteț — cursă pe 24 de pătrate\n");
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: pătrat {} (proba curentă {}), îndemânări {:?}\n",
                p + 1,
                self.pos[p],
                self.pos[p] % 4,
                self.skill[p]
            ));
        }
        if let Some((c, r)) = self.pending {
            s.push_str(&format!(
                "Încercare în așteptare: felul {c}, risc {r} ({}%)\n",
                self.pct(self.turn, c, r)
            ));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cvartet_ok() {
        check(Cvartet::with_seed, 900);
    }
}
