//! Provocarea Minții — șiruri de completat cu miză aleasă (variantă originală, 2 jucători).
//!
//! Cel la rând alege dificultatea `dif d` (1 = progresie aritmetică, 2 = înmulțire cu 2 sau 3,
//! 3 = adunări alternate +x, +y); apare un șir public de patru termeni, iar el răspunde `rasp n` (n = 0–200)
//! cu termenul următor. Corect: +d puncte; greșit: adversarul primește 1. 8 runde (rolurile alternează).
//! Fără informație ascunsă; AI-ul rezolvă șirul public (deci nu greșește: limita cunoscută a variantei).
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROUNDS: u32 = 8;

#[derive(Clone)]
pub struct Mintii {
    seq: Option<([i32; 4], i32)>,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

/// Rezolvă un șir din cele trei familii (aritmetic, geometric, alternat).
pub fn solve(t: [i32; 4]) -> i32 {
    let d = [t[1] - t[0], t[2] - t[1], t[3] - t[2]];
    if d[0] == d[1] && d[1] == d[2] {
        return t[3] + d[0];
    }
    if t[0] != 0 && t[1] % t[0] == 0 && t[2] == t[1] * (t[1] / t[0]) && t[3] == t[2] * (t[1] / t[0])
    {
        return t[3] * (t[1] / t[0]);
    }
    t[3] + d[1]
}

impl Mintii {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Mintii {
            seq: None,
            round: 0,
            pts: [0; 2],
            rng: Rng::new(seed),
        }
    }
    fn mover(&self) -> usize {
        (self.round % 2) as usize
    }
    fn gen(&mut self, d: i32) -> [i32; 4] {
        match d {
            1 => {
                let a = 1 + self.rng.below(20) as i32;
                let k = 2 + self.rng.below(8) as i32;
                [a, a + k, a + 2 * k, a + 3 * k]
            }
            2 => {
                let r = 2 + self.rng.below(2) as i32;
                let a = 1 + self.rng.below(if r == 2 { 6 } else { 2 }) as i32;
                [a, a * r, a * r * r, a * r * r * r]
            }
            _ => {
                let a = 1 + self.rng.below(20) as i32;
                let x = 2 + self.rng.below(8) as i32;
                let mut y = 1 + self.rng.below(9) as i32;
                if y == x {
                    y += 1;
                }
                [a, a + x, a + x + y, a + 2 * x + y]
            }
        }
    }
}

impl Default for Mintii {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Mintii {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.mover() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.seq {
            None => (1..=3).map(|d| format!("dif {d}")).collect(),
            Some(_) => (0..=200).map(|n| format!("rasp {n}")).collect(),
        }
    }
    fn step(&mut self, m: &str) {
        match self.seq {
            None => {
                let d: i32 = m[4..].parse().unwrap();
                let t = self.gen(d);
                self.seq = Some((t, d));
            }
            Some((t, d)) => {
                let n: i32 = m[5..].parse().unwrap();
                let me = self.mover();
                if n == solve(t) {
                    self.pts[me] += d;
                } else {
                    self.pts[1 - me] += 1;
                }
                self.seq = None;
                self.round += 1;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 100 * (self.pts[a] - self.pts[1 - a]);
        match self.seq {
            Some((_, d)) if a == self.mover() => base + 10 * d,
            _ => base,
        }
    }
    fn show(&self) -> String {
        let q = match self.seq {
            Some((t, d)) => format!(
                "Șir (dificultatea {d}): {}, {}, {}, {}, ?",
                t[0], t[1], t[2], t[3]
            ),
            None => "Alege dificultatea (dif 1–3)".to_string(),
        };
        format!(
            "Provocarea Minții — runda {}/{} | puncte J1 {}, J2 {}\n{}\nLa mutare: {} (dif d | rasp n)\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.pts[0],
            self.pts[1],
            q,
            pname(self.side())
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Mintii::with_seed, 100);
    }
    #[test]
    fn solver_solves_every_family() {
        let mut g = Mintii::with_seed(1);
        for d in 1..=3 {
            for _ in 0..200 {
                let t = g.gen(d);
                let n = solve(t);
                assert!((0..=200).contains(&n), "{t:?} -> {n}");
            }
        }
        assert_eq!(solve([2, 4, 8, 16]), 32);
        assert_eq!(solve([3, 6, 9, 12]), 15);
        assert_eq!(solve([5, 7, 10, 12]), 15);
    }
    #[test]
    fn ai_answers_correctly() {
        let mut g = Mintii::with_seed(2);
        g.step("dif 3");
        let t = g.seq.unwrap().0;
        assert_eq!(ai_of(&g, 1), Some(format!("rasp {}", solve(t))));
    }
}
