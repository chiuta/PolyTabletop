//! Urcuș și Coborâș — variantă cooperativă originală cu patru șiruri (lista C3, 2 jucători în echipă).
//!
//! Două șiruri urcă de la 1 (acceptă o carte mai mare sau exact top-10), două coboară de la 50
//! (acceptă o carte mai mică sau exact top+10). Cărțile 2..49; mâini deschise de 7. Într-o tură
//! joci cel puțin 2 cărți (1 când pachetul s-a golit): `joc i s` (cartea i pe șirul s=1..4) și
//! `fin` (încheie tura când ai jucat destul); apoi mutarea forțată `pesc` completează mâna. Dacă
//! nu mai poți juca numărul cerut, echipa pierde (remiză). Dacă toate cărțile sunt jucate, câștigă.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Urcus {
    piles: [u8; 4],
    deck: Vec<u8>,
    hands: [Vec<u8>; 2],
    played: u8,
    refill: bool,
    side: usize,
    total: u32,
}

pub fn new() -> Urcus {
    with_seed(seed_now())
}

pub fn with_seed(seed: u64) -> Urcus {
    let mut rng = Rng::new(seed);
    let mut deck: Vec<u8> = (2..=49).collect();
    shuf(&mut deck, &mut rng);
    let mut g = Urcus {
        piles: [1, 1, 50, 50],
        deck,
        hands: [Vec::new(), Vec::new()],
        played: 0,
        refill: false,
        side: 0,
        total: 0,
    };
    for h in 0..2 {
        for _ in 0..7 {
            let c = g.deck.pop().unwrap();
            g.hands[h].push(c);
        }
    }
    g
}

impl Urcus {
    fn ok(&self, s: usize, c: u8) -> bool {
        let t = self.piles[s];
        if s < 2 {
            c > t || c + 10 == t
        } else {
            c < t || c == t + 10
        }
    }
    fn need(&self) -> u8 {
        if self.deck.is_empty() {
            1
        } else {
            2
        }
    }
    fn plays(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (i, &c) in self.hands[self.side].iter().enumerate() {
            for s in 0..4 {
                if self.ok(s, c) {
                    v.push((i, s));
                }
            }
        }
        v
    }
    fn stuck(&self) -> bool {
        if self.refill || self.played >= self.need() || self.hands[self.side].is_empty() {
            return false;
        }
        self.plays().is_empty()
    }
}

impl Mini for Urcus {
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
        let mut v: Vec<String> = self
            .plays()
            .iter()
            .map(|&(i, s)| format!("joc {} {}", i + 1, s + 1))
            .collect();
        if self.played >= self.need() || self.hands[self.side].is_empty() {
            v.push("fin".into());
        }
        if v.is_empty() {
            v.push("fin".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        if self.refill {
            while self.hands[self.side].len() < 7 {
                match self.deck.pop() {
                    Some(c) => self.hands[self.side].push(c),
                    None => break,
                }
            }
            self.refill = false;
            self.side = 1 - self.side;
            self.played = 0;
            return;
        }
        if m == "fin" {
            self.refill = true;
            return;
        }
        let p: Vec<&str> = m.split(' ').collect();
        let i = p[1].parse::<usize>().unwrap() - 1;
        let s = p[2].parse::<usize>().unwrap() - 1;
        let c = self.hands[self.side].remove(i);
        self.piles[s] = c;
        self.played += 1;
        self.total += 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.deck.is_empty() && self.hands[0].is_empty() && self.hands[1].is_empty() {
            return Some(Outcome::Win(Player::One));
        }
        if self.stuck() {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, _p: Player) -> i32 {
        let dist = (self.piles[0] as i32 - 1)
            + (self.piles[1] as i32 - 1)
            + (50 - self.piles[2] as i32)
            + (50 - self.piles[3] as i32);
        self.total as i32 * 200 - dist
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Urcuș și Coborâș — pachet {} | șiruri: ↑{} ↑{} ↓{} ↓{}\n",
            self.deck.len(),
            self.piles[0],
            self.piles[1],
            self.piles[2],
            self.piles[3]
        );
        for h in 0..2 {
            let mut c = self.hands[h].clone();
            c.sort();
            let c: Vec<String> = c.iter().map(|c| c.to_string()).collect();
            s.push_str(&format!("Mâna J{} (deschisă): {}\n", h + 1, c.join(" ")));
        }
        s.push_str(&format!(
            "La mutare: {} (jucate {}/{}) — joc i s | fin\n",
            pname(self.side()),
            self.played,
            self.need()
        ));
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
