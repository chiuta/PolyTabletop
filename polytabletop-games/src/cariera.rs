//! Cariera Visată — formula succesului și zarul (variantă originală de joc de carieră, 2 jucători).
//!
//! Fiecare alege public o formulă de succes: `tinta 1` (faimă 36, avere 18, fericire 18), `tinta 2`
//! (18, 36, 18), `tinta 3` (18, 18, 36) sau `tinta 4` (24, 24, 24). Apoi, la fiecare tură, zarul
//! (`zar`, mutare forțată) îți dă valoarea d (1–6), iar tu alegi: `munca f|a|h` (+d la faimă, avere sau
//! fericire, dublu dacă cariera ta e potrivită) sau `schimb A|N|F` (treci la cariera de artist, antreprenor
//! sau frizer-filantrop: dublează faima, averea sau fericirea; schimbarea costă tura). Câștigă primul care
//! atinge toate cele trei ținte; după 160 de ture, cine e mai aproape (procent total atins).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const TARGETS: [[i32; 3]; 4] = [[36, 18, 18], [18, 36, 18], [18, 18, 36], [24, 24, 24]];

#[derive(Clone)]
pub struct Cariera {
    goal: [Option<usize>; 2],
    have: [[i32; 3]; 2],
    career: [usize; 2],
    die: Option<i32>,
    side: Player,
    ply: u32,
    rng: Rng,
    winner: Option<Player>,
}

impl Cariera {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Cariera {
            goal: [None; 2],
            have: [[0; 3]; 2],
            career: [0, 1],
            die: None,
            side: Player::One,
            ply: 0,
            rng: Rng::new(seed),
            winner: None,
        }
    }
    fn pct(&self, p: usize) -> i32 {
        match self.goal[p] {
            None => 0,
            Some(g) => (0..3)
                .map(|k| 100 * self.have[p][k].min(TARGETS[g][k]) / TARGETS[g][k])
                .sum(),
        }
    }
    fn done(&self, p: usize) -> bool {
        self.goal[p].is_some_and(|g| (0..3).all(|k| self.have[p][k] >= TARGETS[g][k]))
    }
}

impl Default for Cariera {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cariera {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        if self.goal[me].is_none() {
            return (1..=4).map(|k| format!("tinta {}", k)).collect();
        }
        if self.die.is_none() {
            return vec!["zar".into()];
        }
        let mut v = vec!["munca f".to_string(), "munca a".into(), "munca h".into()];
        for (i, c) in ["A", "N", "F"].iter().enumerate() {
            if self.career[me] != i {
                v.push(format!("schimb {}", c));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if let Some(t) = m.strip_prefix("tinta ") {
            self.goal[me] = Some(t.parse::<usize>().unwrap() - 1);
            self.side = self.side.other();
            return;
        }
        if m == "zar" {
            self.die = Some(self.rng.d6() as i32);
            return;
        }
        let d = self.die.take().unwrap();
        if let Some(x) = m.strip_prefix("munca ") {
            let k = ["f", "a", "h"].iter().position(|&c| c == x).unwrap();
            self.have[me][k] += d * if self.career[me] == k { 2 } else { 1 };
        } else if let Some(x) = m.strip_prefix("schimb ") {
            self.career[me] = ["A", "N", "F"].iter().position(|&c| c == x).unwrap();
        }
        self.ply += 1;
        if self.done(me) {
            self.winner = Some(self.side);
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.ply >= 160 {
            return Some(by_score(self.pct(0), self.pct(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.pct(a) - self.pct(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from("Cariera Visată — faimă / avere / fericire\n");
        for p in 0..2 {
            let g = self.goal[p]
                .map(|g| format!("{:?}", TARGETS[g]))
                .unwrap_or("nealeasă".into());
            s.push_str(&format!(
                "J{}: {:?} / ținta {} | carieră {} ({}%)\n",
                p + 1,
                self.have[p],
                g,
                ["Artist", "Antreprenor", "Filantrop"][self.career[p]],
                self.pct(p) / 3
            ));
        }
        s.push_str(&format!(
            "{}La mutare: {} (tinta k | zar | munca f|a|h | schimb A|N|F)\n",
            self.die
                .map(|d| format!("Zar: {}\n", d))
                .unwrap_or_default(),
            pname(self.side)
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Cariera::with_seed, 400);
    }
}
