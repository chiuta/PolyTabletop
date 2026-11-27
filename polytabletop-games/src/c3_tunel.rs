//! Tunelul blocat — variantă cooperativă originală de construit drumuri (lista C3, 2 jucători în echipă).
//!
//! Pe o tablă 7×7 stă o carte de tunel cu deschideri (N, E, S, V). Echipa trebuie să închidă toate
//! deschiderile: o deschidere e „liberă” dacă dă într-un loc gol sau în afara tablei. Mutări:
//! `pun i r c o` (cartea i din mâna ta, în celula r c, rotită o×90°; deschiderile care se ating
//! trebuie să se potrivească, în afara tablei nu se admit deschideri, iar cartea se lipește de una
//! existentă), `arunc i` (aruncă cartea). Apoi mutarea forțată `pesc` trage o carte. Mâinile sunt
//! deschise (echipă). Câștigi când nu mai rămâne nicio deschidere liberă; pierzi (remiză) când
//! pachetul și mâinile s-au epuizat.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: usize = 7;

#[derive(Clone)]
pub struct Tunel {
    grid: [[Option<u8>; N]; N],
    deck: Vec<u8>,
    hands: [Vec<u8>; 2],
    refill: bool,
    side: usize,
    placed: u32,
}

fn rot(m: u8, o: u8) -> u8 {
    let mut m = m & 15;
    for _ in 0..o {
        m = ((m << 1) | (m >> 3)) & 15;
    }
    m
}

pub fn new() -> Tunel {
    with_seed(seed_now())
}

pub fn with_seed(seed: u64) -> Tunel {
    let mut rng = Rng::new(seed);
    let mut deck: Vec<u8> = Vec::new();
    // biți: 1=N 2=E 4=S 8=V
    for _ in 0..16 {
        deck.push([1, 2, 4, 8][rng.below(4)]);
    }
    for _ in 0..9 {
        deck.push([5, 10, 3, 6, 12, 9][rng.below(6)]);
    }
    for _ in 0..4 {
        deck.push([7, 11, 13, 14][rng.below(4)]);
    }
    deck.push(0);
    deck.push(0);
    shuf(&mut deck, &mut rng);
    let mut g = Tunel {
        grid: [[None; N]; N],
        deck,
        hands: [Vec::new(), Vec::new()],
        refill: false,
        side: 0,
        placed: 1,
    };
    g.grid[3][3] = Some(5 | 2);
    for h in 0..2 {
        for _ in 0..4 {
            let c = g.deck.pop().unwrap();
            g.hands[h].push(c);
        }
    }
    g
}

const D: [(i32, i32, u8, u8); 4] = [(-1, 0, 1, 4), (0, 1, 2, 8), (1, 0, 4, 1), (0, -1, 8, 2)];

impl Tunel {
    fn cell(&self, r: i32, c: i32) -> Option<Option<u8>> {
        if r < 0 || c < 0 || r >= N as i32 || c >= N as i32 {
            None
        } else {
            Some(self.grid[r as usize][c as usize])
        }
    }
    fn fits(&self, r: i32, c: i32, m: u8) -> bool {
        if self.cell(r, c) != Some(None) {
            return false;
        }
        let mut touch = false;
        for (dr, dc, bit, opp) in D {
            match self.cell(r + dr, c + dc) {
                None => {
                    if m & bit != 0 {
                        return false;
                    }
                }
                Some(None) => {}
                Some(Some(n)) => {
                    touch = true;
                    if (m & bit != 0) != (n & opp != 0) {
                        return false;
                    }
                }
            }
        }
        touch
    }
    fn open_ends(&self) -> i32 {
        let mut n = 0;
        for r in 0..N as i32 {
            for c in 0..N as i32 {
                if let Some(m) = self.grid[r as usize][c as usize] {
                    for (dr, dc, bit, _) in D {
                        if m & bit != 0 && matches!(self.cell(r + dr, c + dc), Some(None) | None) {
                            n += 1;
                        }
                    }
                }
            }
        }
        n
    }
}

impl Mini for Tunel {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.refill {
            return vec!["pesc".into()];
        }
        let mut v = Vec::new();
        for (i, &m) in self.hands[self.side].iter().enumerate() {
            let mut seen = Vec::new();
            for o in 0..4u8 {
                let rm = rot(m, o);
                if seen.contains(&rm) {
                    continue;
                }
                seen.push(rm);
                for r in 0..N as i32 {
                    for c in 0..N as i32 {
                        if self.fits(r, c, rm) {
                            v.push(format!("pun {} {} {} {}", i + 1, r + 1, c + 1, o));
                        }
                    }
                }
            }
            v.push(format!("arunc {}", i + 1));
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        if self.refill {
            if let Some(c) = self.deck.pop() {
                self.hands[self.side].push(c);
            }
            self.refill = false;
            self.side = 1 - self.side;
            return;
        }
        if m == "pas" {
            self.refill = true;
            return;
        }
        let p: Vec<&str> = m.split(' ').collect();
        let i = p[1].parse::<usize>().unwrap() - 1;
        let card = self.hands[self.side].remove(i);
        if p[0] == "pun" {
            let r = p[2].parse::<usize>().unwrap() - 1;
            let c = p[3].parse::<usize>().unwrap() - 1;
            let o = p[4].parse::<u8>().unwrap();
            self.grid[r][c] = Some(rot(card, o));
            self.placed += 1;
        }
        self.refill = true;
    }
    fn result(&self) -> Option<Outcome> {
        if self.open_ends() == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.deck.is_empty()
            && self.hands[0].is_empty()
            && self.hands[1].is_empty()
            && !self.refill
        {
            return Some(Outcome::Draw);
        }
        // mâna jucătorului la mutare e goală, dar al celuilalt nu: pasează automat prin pescuit gol
        None
    }
    fn eval(&self, _p: Player) -> i32 {
        -self.open_ends() * 40 + self.placed as i32
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Tunelul blocat — deschideri libere: {} | pachet: {}\n",
            self.open_ends(),
            self.deck.len()
        );
        for r in 0..N {
            for c in 0..N {
                match self.grid[r][c] {
                    None => s.push_str(" ."),
                    Some(m) => s.push_str(&format!(" {:X}", m)),
                }
            }
            s.push('\n');
        }
        s.push_str("(cod hexa: 1=N 2=E 4=S 8=V)\n");
        for h in 0..2 {
            let c: Vec<String> = self.hands[h].iter().map(|c| format!("{c:X}")).collect();
            s.push_str(&format!("Mâna J{} (deschisă): {}\n", h + 1, c.join(" ")));
        }
        s.push_str(&format!("La mutare: {}\n", pname(self.side())));
        s
    }
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 3000);
    }
}
