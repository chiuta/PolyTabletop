//! Aruncări în Cupe — tragere cu mingi spre cupe în piramidă (variantă originală, 2 jucători, cu șansă).
//!
//! Fiecare jucător are 6 cupe în triunghi: rândul din față (cupele 1–3), mijloc (4–5) și spate (6). La rând,
//! aruncă de două ori: `tintesc c` alege o cupă a adversarului, iar mutarea forțată `arunc` decide (zar ascuns):
//! șanse de 65% față, 50% mijloc, 35% spate, +10% dacă adversarul mai are cel mult 3 cupe. Două reușite la rând
//! aduc o aruncare în plus. Cine rămâne fără cupe pierde. AI-ul alege după aceste șanse publice.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

fn row_chance(c: usize) -> i32 {
    match c {
        0..=2 => 65,
        3..=4 => 50,
        _ => 35,
    }
}

#[derive(Clone)]
pub struct Cupe {
    cups: [u8; 2],
    mover: usize,
    throws: u32,
    hits: u32,
    aim: Option<usize>,
    rng: Rng,
}

impl Cupe {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Cupe {
            cups: [63, 63],
            mover: 0,
            throws: 2,
            hits: 0,
            aim: None,
            rng: Rng::new(seed),
        }
    }
    fn chance(&self, c: usize) -> i32 {
        let left = self.cups[1 - self.mover].count_ones();
        (row_chance(c) + if left <= 3 { 10 } else { 0 }).min(95)
    }
}

impl Default for Cupe {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cupe {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.mover)
    }
    fn moves(&self) -> Vec<String> {
        if self.aim.is_some() {
            return vec!["arunc".into()];
        }
        (0..6)
            .filter(|&c| self.cups[1 - self.mover] & (1 << c) != 0)
            .map(|c| format!("tintesc {}", c + 1))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if let Some(c) = self.aim {
            let hit = (self.rng.below(100) as i32) < self.chance(c);
            if hit {
                self.cups[1 - self.mover] &= !(1 << c);
                self.hits += 1;
            } else {
                self.hits = 0;
            }
            self.throws -= 1;
            if hit && self.hits == 2 {
                self.throws += 1;
                self.hits = 0;
            }
            self.aim = None;
            if self.throws == 0 && self.cups[1 - self.mover] != 0 {
                self.mover = 1 - self.mover;
                self.throws = 2;
                self.hits = 0;
            }
            return;
        }
        self.aim = Some(m[8..].parse::<usize>().unwrap() - 1);
    }
    fn result(&self) -> Option<Outcome> {
        if self.cups[0] == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.cups[1] == 0 {
            return Some(Outcome::Win(Player::One));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 1000 * (self.cups[a].count_ones() as i32 - self.cups[1 - a].count_ones() as i32);
        match self.aim {
            Some(c) if self.mover == a => base + 10 * self.chance(c),
            _ => base,
        }
    }
    fn show(&self) -> String {
        let cups = |p: usize| {
            (0..6)
                .map(|c| {
                    if self.cups[p] & (1 << c) != 0 {
                        (c + 1).to_string()
                    } else {
                        ".".to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        format!(
            "Aruncări în Cupe | cupele J1: {} | cupele J2: {}\nAruncări rămase în tura asta: {}\nLa mutare: {} (tintesc c | arunc)\n",
            cups(0),
            cups(1),
            self.throws,
            pname(self.side())
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Cupe::with_seed, 600);
    }
    #[test]
    fn ai_prefers_the_front_row_and_ignores_the_dice() {
        let a = Cupe::with_seed(1);
        let mut b = a.clone();
        b.rng = Rng::new(77);
        for seed in 1..6 {
            let m = ai_of(&a, seed).unwrap();
            assert_eq!(Some(m.clone()), ai_of(&b, seed));
            assert!(["tintesc 1", "tintesc 2", "tintesc 3"].contains(&m.as_str()));
        }
    }
}
