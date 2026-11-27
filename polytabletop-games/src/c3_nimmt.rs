//! Randurile cu Boi — variantă originală de cărți cu rânduri (lista C3, 2 jucători).
//!
//! Pachet 1..104 cu „boi” (penalizări): 55 = 7, multiplu de 11 = 5, de 10 = 3, de 5 = 2, altfel 1.
//! Patru rânduri pornesc cu câte o carte. Fiecare are 10 cărți, mâini deschise; pe rând joacă
//! `joc i`: cartea merge la rândul al cărui capăt e cel mai mare număr mai mic decât ea; dacă ar fi
//! a 6-a carte din rând, jucătorul ia cele 5 cărți (boii lor) și cartea începe rândul. Dacă
//! e mai mică decât toate capetele, alege `rand r` rândul pe care îl ia. După 20 de cărți câștigă
//! cine are mai puțini boi. (Varianta „pe rând” înlocuiește alegerea simultană a jocului de bază.)
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Nimmt {
    rows: [Vec<u8>; 4],
    hands: [Vec<u8>; 2],
    bulls: [i32; 2],
    pend: Option<u8>,
    side: usize,
    played: u32,
}

fn bulls(c: u8) -> i32 {
    if c == 55 {
        7
    } else if c.is_multiple_of(11) {
        5
    } else if c.is_multiple_of(10) {
        3
    } else if c.is_multiple_of(5) {
        2
    } else {
        1
    }
}

pub fn new() -> Nimmt {
    with_seed(seed_now())
}

pub fn with_seed(seed: u64) -> Nimmt {
    let mut rng = Rng::new(seed);
    let mut deck: Vec<u8> = (1..=104).collect();
    shuf(&mut deck, &mut rng);
    let rows = [
        vec![deck.pop().unwrap()],
        vec![deck.pop().unwrap()],
        vec![deck.pop().unwrap()],
        vec![deck.pop().unwrap()],
    ];
    let mut h0: Vec<u8> = (0..10).map(|_| deck.pop().unwrap()).collect();
    let mut h1: Vec<u8> = (0..10).map(|_| deck.pop().unwrap()).collect();
    h0.sort();
    h1.sort();
    Nimmt {
        rows,
        hands: [h0, h1],
        bulls: [0; 2],
        pend: None,
        side: 0,
        played: 0,
    }
}

impl Nimmt {
    fn target(&self, c: u8) -> Option<usize> {
        let mut best: Option<usize> = None;
        for r in 0..4 {
            let e = *self.rows[r].last().unwrap();
            if e < c && best.is_none_or(|b| *self.rows[b].last().unwrap() < e) {
                best = Some(r);
            }
        }
        best
    }
    fn take(&mut self, r: usize, c: u8) {
        let b: i32 = self.rows[r].iter().map(|&x| bulls(x)).sum();
        self.bulls[self.side] += b;
        self.rows[r] = vec![c];
    }
}

impl Mini for Nimmt {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        2
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return (0..4).map(|r| format!("rand {}", r + 1)).collect();
        }
        (0..self.hands[self.side].len())
            .map(|i| format!("joc {}", i + 1))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if let Some(c) = self.pend.take() {
            let r = m[5..].parse::<usize>().unwrap() - 1;
            self.take(r, c);
            self.played += 1;
            self.side = 1 - self.side;
            return;
        }
        let i = m[4..].parse::<usize>().unwrap() - 1;
        let c = self.hands[self.side].remove(i);
        match self.target(c) {
            None => {
                self.pend = Some(c);
                return;
            }
            Some(r) => {
                if self.rows[r].len() >= 5 {
                    self.take(r, c);
                } else {
                    self.rows[r].push(c);
                }
            }
        }
        self.played += 1;
        self.side = 1 - self.side;
    }
    fn result(&self) -> Option<Outcome> {
        if self.played >= 20 {
            Some(by_score(self.bulls[1], self.bulls[0]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        (self.bulls[1 - a] - self.bulls[a]) * 10
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Randurile cu Boi — boi J1 {} | J2 {}\n",
            self.bulls[0], self.bulls[1]
        );
        for (r, row) in self.rows.iter().enumerate() {
            let c: Vec<String> = row.iter().map(|c| c.to_string()).collect();
            s.push_str(&format!("Rândul {}: {}\n", r + 1, c.join(" ")));
        }
        for h in 0..2 {
            let c: Vec<String> = self.hands[h]
                .iter()
                .map(|c| format!("{}({})", c, bulls(*c)))
                .collect();
            s.push_str(&format!("Mâna J{} (deschisă): {}\n", h + 1, c.join(" ")));
        }
        s.push_str(&format!(
            "La mutare: {} — joc i | rand r\n",
            pname(self.side())
        ));
        s
    }
}

fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 100);
    }
}
