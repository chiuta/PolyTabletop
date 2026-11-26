//! Poteca Pietrelor — cinci poteci de culori, cărți în ordine crescătoare (variantă originală).
//!
//! Cărțile au cinci culori și valori 1–10 (câte două exemplare); mâinile sunt deschise (8 cărți).
//! Pe tură: întâi joci o carte, apoi tragi. Joci: `joc C n` — pui cartea pe poteca culorii C
//! dacă n e mai mare decât ultima carte de pe ea — sau `arunc C n` în teancul culorii. Tragi:
//! `trage` din pachet (60 de cărți) sau `ia C` (cartea de sus din teancul culorii C, dar nu
//! cea aruncată chiar acum). Scor la final: fiecare potecă începută valorează −4, −3, −1, 2, 6, 11, 17, 24, 32,
//! 41 după câte cărți are (1..10). Jocul se termină când pachetul e gol. Câștigă scorul mai mare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const COL: [char; 5] = ['r', 'v', 'a', 'g', 'm'];
const F: [i32; 11] = [0, -4, -3, -1, 2, 6, 11, 17, 24, 32, 41];

#[derive(Clone)]
pub struct Poteca {
    track: [[Vec<u8>; 5]; 2],
    hand: [Vec<(u8, u8)>; 2],
    deck: Vec<(u8, u8)>,
    disc: [Vec<u8>; 5],
    side: Player,
    drawing: bool,
    last_disc: Option<usize>,
    ply: u32,
}

impl Poteca {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all: Vec<(u8, u8)> = Vec::new();
        for c in 0..5u8 {
            for n in 1..=10u8 {
                all.push((c, n));
                all.push((c, n));
            }
        }
        shuf(&mut all, &mut rng);
        all.truncate(76);
        let h1: Vec<_> = (0..8).map(|_| all.pop().unwrap()).collect();
        let h2: Vec<_> = (0..8).map(|_| all.pop().unwrap()).collect();
        Poteca {
            track: Default::default(),
            hand: [h1, h2],
            deck: all,
            disc: Default::default(),
            side: Player::One,
            drawing: false,
            last_disc: None,
            ply: 0,
        }
    }
    fn score(&self, p: usize) -> i32 {
        (0..5).map(|c| F[self.track[p][c].len().min(10)]).sum()
    }
}

impl Default for Poteca {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Poteca {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        if self.drawing {
            if !self.deck.is_empty() {
                v.push("trage".to_string());
            }
            for c in 0..5 {
                if Some(c) != self.last_disc && !self.disc[c].is_empty() {
                    v.push(format!("ia {}", COL[c]));
                }
            }
            if v.is_empty() {
                v.push("trage".into());
            }
            return v;
        }
        let mut seen: Vec<(u8, u8)> = Vec::new();
        for &(c, n) in &self.hand[me] {
            if seen.contains(&(c, n)) {
                continue;
            }
            seen.push((c, n));
            if self.track[me][c as usize].last().is_none_or(|&l| n > l) {
                v.push(format!("joc {} {}", COL[c as usize], n));
            }
            v.push(format!("arunc {} {}", COL[c as usize], n));
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if self.drawing {
            if m == "trage" {
                if let Some(c) = self.deck.pop() {
                    self.hand[me].push(c);
                }
            } else {
                let c = COL.iter().position(|&x| x.to_string() == m[3..]).unwrap();
                let n = self.disc[c].pop().unwrap();
                self.hand[me].push((c as u8, n));
            }
            self.drawing = false;
            self.last_disc = None;
            self.side = self.side.other();
            return;
        }
        let mut it = m.split(' ');
        let cmd = it.next().unwrap();
        let cs = it.next().unwrap();
        let c = COL.iter().position(|&x| x.to_string() == cs).unwrap();
        let n: u8 = it.next().unwrap().parse().unwrap();
        let i = self.hand[me]
            .iter()
            .position(|&x| x == (c as u8, n))
            .unwrap();
        self.hand[me].remove(i);
        if cmd == "joc" {
            self.track[me][c].push(n);
        } else {
            self.disc[c].push(n);
            self.last_disc = Some(c);
        }
        self.drawing = true;
    }
    fn result(&self) -> Option<Outcome> {
        if (self.deck.is_empty() || self.ply > 300) && !self.drawing {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * (self.score(a) - self.score(1 - a))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Poteca Pietrelor — pachet {} | culori r v a g m\n",
            self.deck.len()
        );
        for p in 0..2 {
            s.push_str(&format!("J{} scor {}:", p + 1, self.score(p)));
            for c in 0..5 {
                let t: Vec<String> = self.track[p][c].iter().map(|n| n.to_string()).collect();
                s.push_str(&format!("  {}[{}]", COL[c], t.join(",")));
            }
            let h: Vec<String> = self.hand[p]
                .iter()
                .map(|&(c, n)| format!("{}{}", COL[c as usize], n))
                .collect();
            s.push_str(&format!("\n   mână: {}\n", h.join(" ")));
        }
        let d: Vec<String> = (0..5)
            .map(|c| {
                format!(
                    "{}:{}",
                    COL[c],
                    self.disc[c]
                        .last()
                        .map(|n| n.to_string())
                        .unwrap_or("-".into())
                )
            })
            .collect();
        s.push_str(&format!(
            "Aruncate (sus): {} | La mutare: {} ({})\n",
            d.join(" "),
            pname(self.side),
            if self.drawing {
                "trage | ia C"
            } else {
                "joc C n | arunc C n"
            }
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Poteca::with_seed, 400);
    }
}
