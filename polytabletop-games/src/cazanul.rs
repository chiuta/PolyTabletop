//! Cazanul Vrăjit — pariu cu pungi de ingrediente (variantă originală de „push-your-luck”).
//!
//! Fiecare are o pungă de ingrediente (conținutul e public). În rundă, pe rând, tragi la întâmplare
//! ingrediente (`scot`, mutare forțată) și te oprești când vrei (`opresc`) sau mai tragi (`mai`).
//! Ingredientele albe (a1, a2, a3) ard: dacă suma lor trece de 7, cazanul explodează. Celelalte
//! (r2 = rubin, valoare 2; z3 = zmeură, valoare 3; ambele nu explodează) doar adună valoare.
//! Fără explozie: primești monede = suma valorilor și 1 punct la 4 de valoare. Cu explozie alegi
//! `bani` (monede = suma fără ultimul ingredient) sau `puncte` (punctele de mai sus). Apoi cumperi
//! cu monedele: `cumpar a3` (3), `cumpar r2` (5), `cumpar z3` (8), `gata`. 6 runde; câștigă cine
//! are mai multe puncte (+1 la 5 monede rămase).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const KINDS: [&str; 5] = ["a1", "a2", "a3", "r2", "z3"];
const VAL: [i32; 5] = [1, 2, 3, 2, 3];
const COST: [i32; 5] = [0, 0, 3, 5, 8];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Pull,
    Decide,
    Choose,
    Buy,
}

#[derive(Clone)]
pub struct Cazanul {
    bag: [[u8; 5]; 2],
    drawn: [u8; 5],
    last: Option<usize>,
    coins: [i32; 2],
    pts: [i32; 2],
    ph: Ph,
    round: u32,
    first: usize,
    side: Player,
    turn_in_round: u8,
    rng: Rng,
}

impl Cazanul {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Cazanul {
            bag: [[4, 2, 0, 1, 0]; 2],
            drawn: [0; 5],
            last: None,
            coins: [0; 2],
            pts: [0; 2],
            ph: Ph::Pull,
            round: 0,
            first: 0,
            side: Player::One,
            turn_in_round: 0,
            rng: Rng::new(seed),
        }
    }
    fn total(&self) -> i32 {
        (0..5).map(|k| self.drawn[k] as i32 * VAL[k]).sum()
    }
    fn white(&self) -> i32 {
        (0..3).map(|k| self.drawn[k] as i32 * VAL[k]).sum()
    }
    fn exploded(&self) -> bool {
        self.white() > 7
    }
    fn vp(&self, total: i32) -> i32 {
        total / 4
    }
    fn left(&self, p: usize) -> u32 {
        self.bag[p].iter().map(|&x| x as u32).sum()
    }
}

impl Default for Cazanul {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cazanul {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        match self.ph {
            Ph::Pull => vec!["scot".into()],
            Ph::Decide => {
                let mut v = vec!["opresc".to_string()];
                if self.left(me) > 0 {
                    v.push("mai".into());
                }
                v
            }
            Ph::Choose => vec!["bani".into(), "puncte".into()],
            Ph::Buy => {
                let mut v = vec!["gata".to_string()];
                for k in 2..5 {
                    if self.coins[me] >= COST[k] {
                        v.push(format!("cumpar {}", KINDS[k]));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        match (self.ph, m) {
            (Ph::Pull, _) => {
                let n = self.left(me) as usize;
                let mut r = self.rng.below(n);
                let mut k = 0;
                loop {
                    if r < self.bag[me][k] as usize {
                        break;
                    }
                    r -= self.bag[me][k] as usize;
                    k += 1;
                }
                self.bag[me][k] -= 1;
                self.drawn[k] += 1;
                self.last = Some(k);
                self.ph = if self.exploded() {
                    Ph::Choose
                } else {
                    Ph::Decide
                };
            }
            (Ph::Decide, "mai") => self.ph = Ph::Pull,
            (Ph::Decide, _) => {
                self.coins[me] += self.total();
                self.pts[me] += self.vp(self.total());
                self.ph = Ph::Buy;
            }
            (Ph::Choose, "bani") => {
                let lastv = self.last.map(|k| VAL[k]).unwrap_or(0);
                self.coins[me] += self.total() - lastv;
                self.ph = Ph::Buy;
            }
            (Ph::Choose, _) => {
                self.pts[me] += self.vp(self.total());
                self.ph = Ph::Buy;
            }
            (Ph::Buy, "gata") => {
                // pune ingredientele extrase înapoi în pungă
                for k in 0..5 {
                    self.bag[me][k] += self.drawn[k];
                }
                self.drawn = [0; 5];
                self.last = None;
                self.turn_in_round += 1;
                if self.turn_in_round == 2 {
                    self.turn_in_round = 0;
                    self.round += 1;
                    self.first = 1 - self.first;
                    self.side = if self.first == 0 {
                        Player::One
                    } else {
                        Player::Two
                    };
                } else {
                    self.side = self.side.other();
                }
                self.ph = Ph::Pull;
            }
            (Ph::Buy, c) => {
                let k = KINDS.iter().position(|&x| x == &c[7..]).unwrap();
                self.coins[me] -= COST[k];
                self.bag[me][k] += 1;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 6 {
            let f = |i: usize| self.pts[i] + self.coins[i] / 5;
            return Some(by_score(f(0), f(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut mine = 6 * self.pts[a] + 2 * self.coins[a];
        if self.side == p && matches!(self.ph, Ph::Pull | Ph::Decide) {
            // valoare așteptată a ingredientelor din pungă (informație proprie, publică)
            let n = self.left(a) as i32;
            let t = self.total();
            let w = self.white();
            if self.ph == Ph::Pull {
                let mut boom = 0;
                let mut gain = 0;
                for k in 0..5 {
                    let c = self.bag[a][k] as i32;
                    if k < 3 && w + VAL[k] > 7 {
                        boom += c;
                    }
                    gain += c * VAL[k];
                }
                if n > 0 {
                    let p_boom = 100 * boom / n;
                    mine += (100 - p_boom) * (t + gain / n) / 100 / 2 - p_boom * t / 100 / 2;
                }
            } else {
                mine += t / 2;
            }
        }
        let theirs = 6 * self.pts[1 - a] + 2 * self.coins[1 - a];
        mine - theirs
    }
    fn show(&self) -> String {
        let mut s = format!("Cazanul Vrăjit — runda {}/6\n", (self.round + 1).min(6));
        for p in 0..2 {
            let b: Vec<String> = (0..5)
                .filter(|&k| self.bag[p][k] > 0)
                .map(|k| format!("{}×{}", KINDS[k], self.bag[p][k]))
                .collect();
            s.push_str(&format!(
                "J{}: puncte {} | monede {} | pungă: {}\n",
                p + 1,
                self.pts[p],
                self.coins[p],
                b.join(" ")
            ));
        }
        let d: Vec<String> = (0..5)
            .filter(|&k| self.drawn[k] > 0)
            .map(|k| format!("{}×{}", KINDS[k], self.drawn[k]))
            .collect();
        s.push_str(&format!("În cazan: {} | valoare {} | alb {}/7\nLa mutare: {} (faza {:?}: scot | mai | opresc | bani | puncte | cumpar X | gata)\n", d.join(" "), self.total(), self.white(), pname(self.side), self.ph));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Cazanul::with_seed, 600);
    }
}
