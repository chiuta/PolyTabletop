//! Ținuturi Negociate — licitație „prețul tău, alegerea mea” și peisaje proprii (variantă originală).
//!
//! 8 runde; rolul de „vânzător” alternează. Se dezvăluie 3 plăcuțe de teren (câmp, pădure, apă,
//! munte) — mutarea forțată `scot`. Vânzătorul pune prețuri 0–3 monede pe primele două: `pret a,b`;
//! a treia e gratuită. Cumpărătorul alege una (`ia k`, plătește prețul ei vânzătorului dacă are
//! monedele) și o așază în peisajul lui (`pune r,c`); vânzătorul alege una din celelalte două
//! (`ia k`, gratuit) și o așază. Peisajul are 5×5; prima plăcuță oriunde, apoi lângă una existentă
//! (pe laturi). Scor: fiecare grup conex de același teren cu n plăcuțe valorează n(n+1)/2 × valoarea
//! terenului (câmp 1, pădure 1, apă 2, munte 2) + monedele. Câștigă scorul mai mare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const W: [i32; 4] = [1, 1, 2, 2];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Reveal,
    Price,
    Buy,
    PlaceBuyer,
    Take,
    PlaceSeller,
}

#[derive(Clone)]
pub struct TinuturiNegociate {
    land: [[[i8; 5]; 5]; 2],
    offer: Vec<u8>,
    price: [i32; 3],
    held: Option<u8>,
    coins: [i32; 2],
    deck: Vec<u8>,
    round: u32,
    ph: Ph,
}

impl TinuturiNegociate {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..40).map(|i| (i % 4) as u8).collect();
        shuf(&mut deck, &mut rng);
        TinuturiNegociate {
            land: [[[-1; 5]; 5]; 2],
            offer: Vec::new(),
            price: [0; 3],
            held: None,
            coins: [3, 3],
            deck,
            round: 0,
            ph: Ph::Reveal,
        }
    }
    fn seller(&self) -> usize {
        (self.round % 2) as usize
    }
    fn score(&self, p: usize) -> i32 {
        let mut seen = [[false; 5]; 5];
        let mut s = self.coins[p];
        for r in 0..5 {
            for c in 0..5 {
                let t = self.land[p][r][c];
                if t < 0 || seen[r][c] {
                    continue;
                }
                let mut stack = vec![(r, c)];
                seen[r][c] = true;
                let mut n = 0;
                while let Some((a, b)) = stack.pop() {
                    n += 1;
                    for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                        let (x, y) = (a as i32 + dr, b as i32 + dc);
                        if (0..5).contains(&x)
                            && (0..5).contains(&y)
                            && !seen[x as usize][y as usize]
                            && self.land[p][x as usize][y as usize] == t
                        {
                            seen[x as usize][y as usize] = true;
                            stack.push((x as usize, y as usize));
                        }
                    }
                }
                s += n * (n + 1) / 2 * W[t as usize];
            }
        }
        s
    }
    fn spots(&self, p: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        let any = self.land[p].iter().flatten().any(|&t| t >= 0);
        for r in 0..5 {
            for c in 0..5 {
                if self.land[p][r][c] >= 0 {
                    continue;
                }
                if !any {
                    v.push((r, c));
                    continue;
                }
                let adj = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .any(|&(dr, dc)| {
                        let (x, y) = (r as i32 + dr, c as i32 + dc);
                        (0..5).contains(&x)
                            && (0..5).contains(&y)
                            && self.land[p][x as usize][y as usize] >= 0
                    });
                if adj {
                    v.push((r, c));
                }
            }
        }
        v
    }
}

impl Default for TinuturiNegociate {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for TinuturiNegociate {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        let s = self.seller();
        let who = match self.ph {
            Ph::Reveal | Ph::Price | Ph::Take | Ph::PlaceSeller => s,
            Ph::Buy | Ph::PlaceBuyer => 1 - s,
        };
        if who == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let s = self.seller();
        match self.ph {
            Ph::Reveal => vec!["scot".into()],
            Ph::Price => {
                let mut v = Vec::new();
                for a in 0..=3 {
                    for b in 0..=3 {
                        v.push(format!("pret {},{}", a, b));
                    }
                }
                v
            }
            Ph::Buy => (0..self.offer.len())
                .filter(|&k| self.price[k] <= self.coins[1 - s])
                .map(|k| format!("ia {}", k + 1))
                .collect(),
            Ph::Take => (0..self.offer.len())
                .map(|k| format!("ia {}", k + 1))
                .collect(),
            Ph::PlaceBuyer | Ph::PlaceSeller => {
                let p = if self.ph == Ph::PlaceBuyer { 1 - s } else { s };
                let sp = self.spots(p);
                if sp.is_empty() {
                    return vec!["arunc".into()];
                }
                sp.iter()
                    .map(|&(r, c)| format!("pune {},{}", r + 1, c + 1))
                    .collect()
            }
        }
    }
    fn step(&mut self, m: &str) {
        let s = self.seller();
        match self.ph {
            Ph::Reveal => {
                self.offer = (0..3).filter_map(|_| self.deck.pop()).collect();
                self.price = [0; 3];
                self.ph = Ph::Price;
            }
            Ph::Price => {
                let (a, b) = m[5..].split_once(',').unwrap();
                self.price = [a.parse().unwrap(), b.parse().unwrap(), 0];
                self.ph = Ph::Buy;
            }
            Ph::Buy => {
                let k = m[3..].parse::<usize>().unwrap() - 1;
                self.coins[1 - s] -= self.price[k];
                self.coins[s] += self.price[k];
                self.held = Some(self.offer.remove(k));
                self.price.copy_within(k + 1.., k);
                self.ph = Ph::PlaceBuyer;
            }
            Ph::PlaceBuyer | Ph::PlaceSeller => {
                let p = if self.ph == Ph::PlaceBuyer { 1 - s } else { s };
                if m != "arunc" {
                    let (r, c) = m[5..].split_once(',').unwrap();
                    self.land[p][r.parse::<usize>().unwrap() - 1]
                        [c.parse::<usize>().unwrap() - 1] = self.held.unwrap() as i8;
                }
                self.held = None;
                if self.ph == Ph::PlaceBuyer {
                    self.ph = Ph::Take;
                } else {
                    self.round += 1;
                    self.ph = Ph::Reveal;
                }
            }
            Ph::Take => {
                let k = m[3..].parse::<usize>().unwrap() - 1;
                self.held = Some(self.offer.remove(k));
                self.ph = Ph::PlaceSeller;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 8 {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.score(a) - self.score(1 - a)
    }
    fn show(&self) -> String {
        let nm = ['c', 'p', 'a', 'm'];
        let mut s = format!(
            "Ținuturi Negociate — runda {}/8 | vânzător: J{} | monede J1 {}, J2 {}\n",
            (self.round + 1).min(8),
            self.seller() + 1,
            self.coins[0],
            self.coins[1]
        );
        for p in 0..2 {
            s.push_str(&format!("J{} (scor {}):\n", p + 1, self.score(p)));
            for r in 0..5 {
                s.push_str("  ");
                for c in 0..5 {
                    s.push(if self.land[p][r][c] < 0 {
                        '.'
                    } else {
                        nm[self.land[p][r][c] as usize]
                    });
                    s.push(' ');
                }
                s.push('\n');
            }
        }
        let o: Vec<String> = self
            .offer
            .iter()
            .enumerate()
            .map(|(k, &t)| format!("{}:{}(preț {})", k + 1, nm[t as usize], self.price[k]))
            .collect();
        s.push_str(&format!("Oferta: {} | în mână: {}\n(c câmp, p pădure, a apă, m munte)\nLa mutare: {} (faza {:?}: scot | pret a,b | ia k | pune r,c)\n", o.join("  "), self.held.map(|t| nm[t as usize].to_string()).unwrap_or("-".into()), pname(self.side()), self.ph));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(TinuturiNegociate::with_seed, 400);
    }
}
