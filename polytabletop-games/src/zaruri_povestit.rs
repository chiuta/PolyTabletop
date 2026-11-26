//! Zaruri de Povestit — lanțuri de simboluri (variantă originală, 2 jucători, informație perfectă).
//!
//! Se aruncă 12 zaruri, fiecare cu unul din 6 simboluri (0 stea, 1 cheie, 2 barcă, 3 foc, 4 pasăre, 5 carte);
//! rezultatele sunt la vedere. Pe rând fiecare ia un zar și îl adaugă la lanțul lui, `ia k f` (la față)
//! sau `ia k s` (la spate); k = numărul zarului din rezervă. O legătură între simbolurile vecine a și b valorează
//! după diferența d = (b − a) mod 6: 0→1, 1→3, 2→0, 3→1, 4→0, 5→2 puncte (povestea „curge” mai bine din a în a+1).
//! Scorul = suma legăturilor din lanț. După epuizarea zarurilor câștigă scorul mai mare.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const LINK: [i32; 6] = [1, 3, 0, 1, 0, 2];
const NAMES: [&str; 6] = ["stea", "cheie", "barcă", "foc", "pasăre", "carte"];

#[derive(Clone)]
pub struct Zaruri {
    pool: Vec<u8>,
    chain: [Vec<u8>; 2],
    side: Player,
}

fn link(a: u8, b: u8) -> i32 {
    LINK[((b + 6 - a) % 6) as usize]
}

fn score(c: &[u8]) -> i32 {
    c.windows(2).map(|w| link(w[0], w[1])).sum()
}

impl Zaruri {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut pool: Vec<u8> = (0..12).map(|_| rng.below(6) as u8).collect();
        pool.sort_unstable();
        Zaruri {
            pool,
            chain: [Vec::new(), Vec::new()],
            side: Player::One,
        }
    }
}

impl Default for Zaruri {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Zaruri {
    fn depth(&self) -> u32 {
        3
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let mut v = Vec::new();
        for k in 0..self.pool.len() {
            if k > 0 && self.pool[k] == self.pool[k - 1] {
                continue;
            }
            v.push(format!("ia {} f", k + 1));
            if !self.chain[pi(self.side)].is_empty() {
                v.push(format!("ia {} s", k + 1));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let mut it = m[3..].split(' ');
        let k: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let front = it.next().unwrap() == "f";
        let d = self.pool.remove(k);
        let c = &mut self.chain[pi(self.side)];
        if front {
            c.insert(0, d);
        } else {
            c.push(d);
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.pool.is_empty() {
            return Some(by_score(score(&self.chain[0]), score(&self.chain[1])));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * (score(&self.chain[a]) - score(&self.chain[1 - a]))
    }
    fn show(&self) -> String {
        let name = |v: &Vec<u8>| {
            if v.is_empty() {
                "-".to_string()
            } else {
                v.iter()
                    .map(|&d| NAMES[d as usize])
                    .collect::<Vec<_>>()
                    .join(" → ")
            }
        };
        let pool: Vec<String> = self
            .pool
            .iter()
            .enumerate()
            .map(|(i, &d)| format!("{}={}", i + 1, NAMES[d as usize]))
            .collect();
        format!(
            "Zaruri de Povestit | rezerva: {}\nJ1 ({}): {}\nJ2 ({}): {}\nLa mutare: {} (ia k f | ia k s)\n",
            pool.join(", "),
            score(&self.chain[0]),
            name(&self.chain[0]),
            score(&self.chain[1]),
            name(&self.chain[1]),
            pname(self.side)
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Zaruri::with_seed, 40);
    }
    #[test]
    fn link_values() {
        assert_eq!(link(0, 1), 3);
        assert_eq!(link(5, 0), 3);
        assert_eq!(score(&[0, 1, 2]), 6);
    }
}
