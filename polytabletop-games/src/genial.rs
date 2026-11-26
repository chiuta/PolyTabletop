//! Mozaicul Priceput — domino hexagonal cu șase culori (variantă originală, mâini deschise).
//!
//! Tablă hexagonală cu 91 de celule (coordonate axiale q,r, raza 5); la colțuri stau cele șase
//! culori. Fiecare are 4 plăcuțe-domino de două culori în mână (mâinile sunt vizibile) și
//! pune pe rând una pe două celule libere vecine: `pune i@q,r,d` = plăcuța i, prima culoare în
//! celula (q,r), a doua în vecinul din direcția d (0–5, în sensul E, NE, NV, V, SV, SE).
//! Scor: pentru fiecare jumătate de plăcuță numeri celulele consecutive de aceeași culoare, în cele
//! 5 direcții libere, și le adaugi culorii respective (maxim 18). Câștigă cine are cea mai bună
//! culoare cea mai slabă (apoi a doua cea mai slabă…).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const R: i32 = 5;
const LETTERS: [char; 7] = ['.', 'R', 'G', 'B', 'Y', 'V', 'O'];

#[derive(Clone)]
pub struct Genial {
    cell: [[u8; 11]; 11],
    score: [[i32; 7]; 2],
    hand: [Vec<(u8, u8)>; 2],
    bag: Vec<(u8, u8)>,
    side: Player,
    open_pairs: bool,
}

fn inb(q: i32, r: i32) -> bool {
    q.abs() <= R && r.abs() <= R && (q + r).abs() <= R
}

impl Genial {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag = Vec::new();
        for a in 1..=6u8 {
            for _ in 0..5 {
                bag.push((a, a));
            }
            for b in a + 1..=6 {
                for _ in 0..6 {
                    bag.push((a, b));
                }
            }
        }
        shuf(&mut bag, &mut rng);
        let mut cell = [[0u8; 11]; 11];
        let corners = [(5, 0), (0, 5), (-5, 5), (-5, 0), (0, -5), (5, -5)];
        for (i, &(q, r)) in corners.iter().enumerate() {
            cell[(q + R) as usize][(r + R) as usize] = i as u8 + 1;
        }
        let h1: Vec<_> = (0..4).map(|_| bag.pop().unwrap()).collect();
        let h2: Vec<_> = (0..4).map(|_| bag.pop().unwrap()).collect();
        let mut g = Genial {
            cell,
            score: [[0; 7]; 2],
            hand: [h1, h2],
            bag,
            side: Player::One,
            open_pairs: true,
        };
        g.open_pairs = g.any_pair();
        g
    }
    fn at(&self, q: i32, r: i32) -> u8 {
        if inb(q, r) {
            self.cell[(q + R) as usize][(r + R) as usize]
        } else {
            255
        }
    }
    fn any_pair(&self) -> bool {
        for q in -R..=R {
            for r in -R..=R {
                if inb(q, r) && self.at(q, r) == 0 {
                    for d in 0..6 {
                        if self.at(q + HEX[d].0, r + HEX[d].1) == 0 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn run(&self, q: i32, r: i32, d: usize, col: u8) -> i32 {
        let (mut n, mut cq, mut cr) = (0, q + HEX[d].0, r + HEX[d].1);
        while self.at(cq, cr) == col {
            n += 1;
            cq += HEX[d].0;
            cr += HEX[d].1;
        }
        n
    }
    fn low(&self, p: usize) -> Vec<i32> {
        let mut v: Vec<i32> = (1..=6).map(|c| self.score[p][c]).collect();
        v.sort();
        v
    }
}

impl Default for Genial {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Genial {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let p = pi(self.side);
        let mut v = Vec::new();
        for (i, &(a, b)) in self.hand[p].iter().enumerate() {
            for q in -R..=R {
                for r in -R..=R {
                    if !inb(q, r) || self.at(q, r) != 0 {
                        continue;
                    }
                    for d in 0..(if a == b { 3 } else { 6 }) {
                        if self.at(q + HEX[d].0, r + HEX[d].1) == 0 {
                            v.push(format!("pune {}@{},{},{}", i + 1, q, r, d));
                        }
                    }
                }
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let p = pi(self.side);
        if m != "pas" {
            let rest = m.strip_prefix("pune ").unwrap();
            let (i, xs) = rest.split_once('@').unwrap();
            let x: Vec<i32> = xs.split(',').map(|s| s.parse().unwrap()).collect();
            let (a, b) = self.hand[p].remove(i.parse::<usize>().unwrap() - 1);
            let (q, r, d) = (x[0], x[1], x[2] as usize);
            let (q2, r2) = (q + HEX[d].0, r + HEX[d].1);
            self.cell[(q + R) as usize][(r + R) as usize] = a;
            self.cell[(q2 + R) as usize][(r2 + R) as usize] = b;
            for dd in 0..6 {
                if dd != d {
                    let n = self.run(q, r, dd, a);
                    self.score[p][a as usize] = (self.score[p][a as usize] + n).min(18);
                }
                if dd != (d + 3) % 6 {
                    let n = self.run(q2, r2, dd, b);
                    self.score[p][b as usize] = (self.score[p][b as usize] + n).min(18);
                }
            }
            if let Some(t) = self.bag.pop() {
                self.hand[p].push(t);
            }
            self.open_pairs = self.any_pair();
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.open_pairs {
            return None;
        }
        let (a, b) = (self.low(0), self.low(1));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn eval(&self, p: Player) -> i32 {
        let (a, b) = (self.low(pi(p)), self.low(1 - pi(p)));
        let w = [12, 6, 3, 2, 1, 1];
        (0..6).map(|i| w[i] * (a[i] - b[i])).sum()
    }
    fn show(&self) -> String {
        let mut s = String::from("Mozaicul Priceput — culori R G B Y V O; celula (q,r)\n");
        for r in -R..=R {
            let qmin = (-R).max(-R - r);
            let qmax = R.min(R - r);
            s.push_str(&format!("r={:>2} q={:>2}..{:<2} ", r, qmin, qmax));
            s.push_str(&" ".repeat(r.unsigned_abs() as usize));
            for q in qmin..=qmax {
                s.push(LETTERS[self.at(q, r) as usize]);
                s.push(' ');
            }
            s.push('\n');
        }
        for p in 0..2 {
            let sc: Vec<String> = (1..=6)
                .map(|c| format!("{}{}", LETTERS[c], self.score[p][c]))
                .collect();
            let h: Vec<String> = self.hand[p]
                .iter()
                .enumerate()
                .map(|(i, &(a, b))| {
                    format!("{}:{}{}", i + 1, LETTERS[a as usize], LETTERS[b as usize])
                })
                .collect();
            s.push_str(&format!(
                "J{} scor {} | plăcuțe {}\n",
                p + 1,
                sc.join(" "),
                h.join(" ")
            ));
        }
        s.push_str(&format!("La mutare: {} (pune i@q,r,d)\n", pname(self.side)));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Genial::with_seed, 200);
    }
    #[test]
    fn board_has_91_cells() {
        let n = (-R..=R)
            .flat_map(|q| (-R..=R).map(move |r| (q, r)))
            .filter(|&(q, r)| inb(q, r))
            .count();
        assert_eq!(n, 91);
    }
}
