#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::unnecessary_map_or,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
//! Lanțul de Bufete — duel economic în 8 runde (variantă originală, inspirată de genul „lanț de
//! restaurante”). Fiecare rundă: J1 apoi J2 aleg `pret p stoc k [rec]`; cei 6 clienți aleg cel mai
//! bun preț efectiv (preț − reclamă); fiecare unitate vândută aduce prețul. Stocul costă 1/unitate.
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

const HOUSE: [i32; 6] = [2, 3, 3, 4, 5, 5];

#[derive(Clone)]
pub struct Bufete {
    cash: [i32; 2],
    pend: Option<(i32, i32, i32)>,
    round: u32,
    side: Player,
}

impl Bufete {
    pub fn new() -> Self {
        Bufete {
            cash: [10, 10],
            pend: None,
            round: 0,
            side: Player::One,
        }
    }
    pub fn with_seed(_s: u64) -> Self {
        Self::new()
    }
    fn resolve(&mut self, a: (i32, i32, i32), b: (i32, i32, i32)) {
        let mut stock = [a.1, b.1];
        let pr = [a.0, b.0];
        let ad = [a.2, b.2];
        let mut earn = [0, 0];
        for (n, &h) in HOUSE.iter().enumerate() {
            let ok: Vec<usize> = (0..2).filter(|&s| stock[s] > 0 && pr[s] <= h).collect();
            let best = match ok.len() {
                0 => continue,
                1 => ok[0],
                _ => {
                    let e0 = pr[0] - ad[0];
                    let e1 = pr[1] - ad[1];
                    if e0 < e1 {
                        0
                    } else if e1 < e0 {
                        1
                    } else {
                        (n + self.round as usize) % 2
                    }
                }
            };
            stock[best] -= 1;
            earn[best] += pr[best];
        }
        for s in 0..2 {
            let (p, k, r) = if s == 0 { a } else { b };
            let _ = p;
            self.cash[s] += earn[s] - k - 2 * r;
        }
    }
}

impl Default for Bufete {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Bufete {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        2
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for p in 1..=5 {
            for k in 0..=5 {
                for r in 0..=1 {
                    if k + 2 * r <= self.cash[me] {
                        v.push(format!(
                            "pret {p} stoc {k}{}",
                            if r == 1 { " rec" } else { "" }
                        ));
                    }
                }
            }
        }
        if v.is_empty() {
            v.push("pret 1 stoc 0".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let t: Vec<&str> = m.split(' ').collect();
        let cur = (
            t[1].parse().unwrap(),
            t[3].parse().unwrap(),
            (t.len() > 4) as i32,
        );
        match self.pend.take() {
            None => {
                self.pend = Some(cur);
                self.side = Player::Two;
            }
            Some(first) => {
                self.resolve(first, cur);
                self.round += 1;
                self.side = Player::One;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        (self.round >= 8).then(|| by_score(self.cash[0], self.cash[1]))
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.cash[a] - self.cash[1 - a]
    }
    fn show(&self) -> String {
        format!(
            "Lanțul de Bufete — runda {}/8 | bani J1 {} J2 {} | clienți (preț maxim): {:?}\n{}La mutare: {} (pret p stoc k [rec]; rec costă 2)\n",
            self.round + 1,
            self.cash[0],
            self.cash[1],
            HOUSE,
            match self.pend {
                Some((p, k, r)) => format!("J1 a ales deja: preț {p}, stoc {k}, reclamă {r}\n"),
                None => String::new(),
            },
            pname(self.side)
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Bufete::with_seed, 100);
    }
}
