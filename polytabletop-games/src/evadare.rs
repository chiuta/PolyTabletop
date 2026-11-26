//! Evadarea din Temniță — pirați care fug printr-un tunel (variantă originală, mâini deschise).
//!
//! Tunelul are 36 de pătrate cu 6 simboluri (câte 6 din fiecare) și o barcă la capăt. Fiecare
//! jucător are 6 pirați la start și 6 cărți-simbol în mână (deschisă). Tura are până la 3 acțiuni:
//! `avans p,S` (pirat p ajunge pe cel mai apropiat pătrat liber înainte cu simbolul S; joci o carte
//! S; după ultimul pătrat intră în barcă), `retras p` (piratul p se întoarce pe cel mai apropiat
//! pătrat ocupat din spate și trage o carte), `gata` (încheie tura). La final de tură îți refaci mâna
//! la 6. Câștigă primul care îmbarcă toți cei 6 pirați. (după 400 de mutări: cine are mai mulți în barcă).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const SYM: [char; 6] = ['c', 'p', 'r', 'k', 's', 'b'];
const LEN: i32 = 36;

#[derive(Clone)]
pub struct Evadare {
    sq: Vec<u8>,
    pos: [[i32; 6]; 2], // -1 start, 0..35, 36 barcă
    hand: [Vec<u8>; 2],
    deck: Vec<u8>,
    discard: Vec<u8>,
    left: u8,
    side: Player,
    ply: u32,
}

impl Evadare {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut sq: Vec<u8> = (0..36).map(|i| (i % 6) as u8).collect();
        shuf(&mut sq, &mut rng);
        let mut deck: Vec<u8> = (0..102).map(|i| (i % 6) as u8).collect();
        shuf(&mut deck, &mut rng);
        let h1 = (0..6).map(|_| deck.pop().unwrap()).collect();
        let h2 = (0..6).map(|_| deck.pop().unwrap()).collect();
        Evadare {
            sq,
            pos: [[-1; 6]; 2],
            hand: [h1, h2],
            deck,
            discard: Vec::new(),
            left: 3,
            side: Player::One,
            ply: 0,
        }
    }
    fn occupied(&self, s: i32) -> bool {
        (0..LEN).contains(&s) && self.pos.iter().any(|p| p.contains(&s))
    }
    fn draw(&mut self, me: usize, n: usize) {
        for _ in 0..n {
            if self.deck.is_empty() {
                self.deck = std::mem::take(&mut self.discard);
                let mut rng = Rng::new(self.ply as u64 + 17);
                shuf(&mut self.deck, &mut rng);
            }
            if let Some(c) = self.deck.pop() {
                self.hand[me].push(c);
            }
        }
    }
    fn aboard(&self, p: usize) -> usize {
        self.pos[p].iter().filter(|&&x| x == LEN).count()
    }
    fn progress(&self, p: usize) -> i32 {
        self.pos[p].iter().map(|&x| x + 1).sum()
    }
    fn target(&self, from: i32, sym: u8) -> i32 {
        for s in (from + 1)..LEN {
            if self.sq[s as usize] == sym && !self.occupied(s) {
                return s;
            }
        }
        LEN
    }
    fn back(&self, from: i32) -> Option<i32> {
        (0..from).rev().find(|&s| self.occupied(s))
    }
}

impl Default for Evadare {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Evadare {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for p in 0..6 {
            let at = self.pos[me][p];
            if at >= LEN {
                continue;
            }
            for s in 0..6u8 {
                if self.hand[me].contains(&s) {
                    let t = format!("avans {},{}", p + 1, SYM[s as usize]);
                    if !v.contains(&t) {
                        let tg = self.target(at, s);
                        // dedup piratii identici nu-i filtrăm; dar nu avansăm dacă destinația e aceeași ca pentru alt pirat — ok
                        let _ = tg;
                        v.push(t);
                    }
                }
            }
            if at >= 0 && self.back(at).is_some() {
                v.push(format!("retras {}", p + 1));
            }
        }
        if self.left < 3 || v.is_empty() {
            v.push("gata".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if m == "gata" {
            self.left = 0;
        } else if let Some(r) = m.strip_prefix("avans ") {
            let (p, s) = r.split_once(',').unwrap();
            let p = p.parse::<usize>().unwrap() - 1;
            let s = SYM.iter().position(|&c| c.to_string() == s).unwrap() as u8;
            let i = self.hand[me].iter().position(|&x| x == s).unwrap();
            self.hand[me].remove(i);
            self.discard.push(s);
            self.pos[me][p] = self.target(self.pos[me][p], s);
            self.left -= 1;
        } else if let Some(r) = m.strip_prefix("retras ") {
            let p = r.parse::<usize>().unwrap() - 1;
            if let Some(b) = self.back(self.pos[me][p]) {
                self.pos[me][p] = b;
                self.draw(me, 1);
            }
            self.left -= 1;
        }
        if self.left == 0 || self.aboard(me) == 6 {
            let need = 6usize.saturating_sub(self.hand[me].len());
            self.draw(me, need);
            self.left = 3;
            self.side = self.side.other();
        }
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.aboard(p) == 6 {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.ply >= 400 {
            return Some(by_score(
                self.aboard(0) as i32 * 100 + self.progress(0),
                self.aboard(1) as i32 * 100 + self.progress(1),
            ));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.progress(a) - self.progress(1 - a)
    }
    fn show(&self) -> String {
        let mut s =
            String::from("Evadarea din Temniță — simboluri c p r k s b; pirați J1=a..f, J2=A..F\n");
        let mut line = String::new();
        let mut owners = String::new();
        for i in 0..LEN {
            line.push(SYM[self.sq[i as usize] as usize]);
            let mut o = '.';
            for p in 0..2 {
                for k in 0..6 {
                    if self.pos[p][k] == i {
                        o = if p == 0 {
                            (b'a' + k as u8) as char
                        } else {
                            (b'A' + k as u8) as char
                        };
                    }
                }
            }
            owners.push(o);
        }
        s.push_str(&format!("{}\n{}\n", line, owners));
        for p in 0..2 {
            let h: Vec<String> = self.hand[p]
                .iter()
                .map(|&c| SYM[c as usize].to_string())
                .collect();
            s.push_str(&format!(
                "J{}: în barcă {} | la start {} | mână: {}\n",
                p + 1,
                self.aboard(p),
                self.pos[p].iter().filter(|&&x| x == -1).count(),
                h.join(" ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({} acțiuni rămase) — avans p,S | retras p | gata\n",
            pname(self.side),
            self.left
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Evadare::with_seed, 900);
    }
}
