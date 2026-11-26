//! Cartierul Înalt — majorități în cartiere de zgârie-nori (variantă originală).
//!
//! Orașul are 16 cartiere (4×4). Fiecare tură joci una din cărțile tale (mâinile sunt deschise,
//! 4 cărți pe rundă) — fiecare carte indică un cartier — și adaugi un etaj al tău acolo
//! (`etaj n`, n = 1..16 cartierul; maxim 6 etaje, cartier plin = carte pierdută: `etaj` nu e
//! posibil), sau, în loc de etaj, îți muți un etaj de pe vârful unui cartier vecin în cel al
//! cărții (`muta a>n`: preiei etajul de sus al oricui, doar dacă e al tău). La finalul fiecărei
//! din cele 3 runde se punctează fiecare cartier: cine are mai multe etaje (la egalitate, cine are
//! etajul cel mai de sus) primește un punct pe etaj din cartier. Câștigă cine are mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct ZgarieNori {
    stack: Vec<Vec<u8>>, // proprietar pe etaje (0/1)
    hand: [Vec<u8>; 2],
    deck: Vec<u8>,
    pts: [i32; 2],
    played: u32,
    round: u32,
    side: Player,
}

impl ZgarieNori {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..48).map(|i| (i % 16) as u8).collect();
        shuf(&mut deck, &mut rng);
        let mut g = ZgarieNori {
            stack: vec![Vec::new(); 16],
            hand: [Vec::new(), Vec::new()],
            deck,
            pts: [0; 2],
            played: 0,
            round: 0,
            side: Player::One,
        };
        g.deal();
        g
    }
    fn deal(&mut self) {
        for p in 0..2 {
            for _ in 0..4 {
                if let Some(c) = self.deck.pop() {
                    self.hand[p].push(c);
                }
            }
        }
    }
    fn lead(&self, d: usize) -> Option<usize> {
        let (a, b) = (
            self.stack[d].iter().filter(|&&x| x == 0).count(),
            self.stack[d].iter().filter(|&&x| x == 1).count(),
        );
        if a > b {
            Some(0)
        } else if b > a {
            Some(1)
        } else {
            self.stack[d].last().map(|&x| x as usize)
        }
    }
    fn hold(&self, p: usize) -> i32 {
        (0..16)
            .filter(|&d| self.lead(d) == Some(p))
            .map(|d| self.stack[d].len() as i32)
            .sum()
    }
}

impl Default for ZgarieNori {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for ZgarieNori {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        let mut seen = [false; 16];
        for &c in &self.hand[me] {
            let c = c as usize;
            if seen[c] {
                continue;
            }
            seen[c] = true;
            if self.stack[c].len() < 6 {
                v.push(format!("etaj {}", c + 1));
                let (r, k) = (c / 4, c % 4);
                for (dr, dk) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                    let (nr, nk) = (r as i32 + dr, k as i32 + dk);
                    if (0..4).contains(&nr) && (0..4).contains(&nk) {
                        let a = (nr * 4 + nk) as usize;
                        if self.stack[a].last() == Some(&(me as u8)) {
                            v.push(format!("muta {}>{}", a + 1, c + 1));
                        }
                    }
                }
            } else {
                v.push(format!("arunc {}", c + 1));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let (cmd, rest) = m.split_once(' ').unwrap();
        let card = match cmd {
            "etaj" | "arunc" => {
                let c = rest.parse::<usize>().unwrap() - 1;
                if cmd == "etaj" {
                    self.stack[c].push(me as u8);
                }
                c
            }
            _ => {
                let (a, b) = rest.split_once('>').unwrap();
                let (a, b) = (
                    a.parse::<usize>().unwrap() - 1,
                    b.parse::<usize>().unwrap() - 1,
                );
                let x = self.stack[a].pop().unwrap();
                self.stack[b].push(x);
                b
            }
        };
        let i = self.hand[me]
            .iter()
            .position(|&x| x as usize == card)
            .unwrap();
        self.hand[me].remove(i);
        self.played += 1;
        self.side = self.side.other();
        if self.hand[0].is_empty() && self.hand[1].is_empty() {
            for d in 0..16 {
                if let Some(w) = self.lead(d) {
                    self.pts[w] += self.stack[d].len() as i32;
                }
            }
            self.round += 1;
            if self.round < 3 {
                self.deal();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 3 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * (self.pts[a] - self.pts[1 - a]) + self.hold(a) - self.hold(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Cartierul Înalt — runda {}/3 | puncte J1 {}, J2 {}\n",
            (self.round + 1).min(3),
            self.pts[0],
            self.pts[1]
        );
        for r in 0..4 {
            for k in 0..4 {
                let d = r * 4 + k;
                let st: String = self.stack[d]
                    .iter()
                    .map(|&x| if x == 0 { 'a' } else { 'b' })
                    .collect();
                s.push_str(&format!("{:>2}:{:<7}", d + 1, st));
            }
            s.push('\n');
        }
        for p in 0..2 {
            let h: Vec<String> = self.hand[p].iter().map(|c| (c + 1).to_string()).collect();
            s.push_str(&format!(
                "J{} ({}) cărți: {}\n",
                p + 1,
                if p == 0 { 'a' } else { 'b' },
                h.join(" ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (etaj n | muta a>n | arunc n)\n",
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
        crate::brand_util::check(ZgarieNori::with_seed, 100);
    }
}
