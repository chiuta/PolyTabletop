//! Agenții Tăcuți — agenți pe o stradă cu valori secrete (variantă originală de deducție, 2 jucători).
//!
//! Șase agenți (A–F) pornesc din câmpul 0 al unei străzi cu 10 pași. Fiecare jucător are în secret
//! o valoare 0–5 pentru fiecare agent (șase valori distincte). Pe rând muți ORICE agent cu 1–3
//! câmpuri înainte (`mut A,2`); mutările sunt publice, deci ghicești ce agenți îi plac adversarului.
//! După 24 de mutări (12 ale fiecăruia) scorul tău = Σ valoarea ta × poziția agentului. Câștigă scorul mai mare.
//! Informație ascunsă: vezi doar valorile tale.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Agenti {
    viewer: Option<Player>,
    val: [[i32; 6]; 2],
    pos: [i32; 6],
    moves_by: [[i32; 6]; 2],
    side: Player,
    ply: u32,
}

impl Agenti {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut v = [[0; 6]; 2];
        for p in 0..2 {
            let mut x: Vec<i32> = (0..6).collect();
            shuf(&mut x, &mut rng);
            v[p].copy_from_slice(&x);
        }
        Agenti {
            viewer: None,
            val: v,
            pos: [0; 6],
            moves_by: [[0; 6]; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn score(&self, p: usize) -> i32 {
        (0..6).map(|a| self.val[p][a] * self.pos[a]).sum()
    }
    /// Scor estimat al adversarului (doar din mutările lui publice).
    fn guess_opp(&self, o: usize) -> i32 {
        let tot: i32 = self.moves_by[o].iter().sum();
        (0..6)
            .map(|a| (25 + (self.moves_by[o][a] * 6 - tot) * 8 / 6).max(0) * self.pos[a])
            .sum::<i32>()
            / 10
    }
}

impl Default for Agenti {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Agenti {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let mut v = Vec::new();
        for a in 0..6 {
            for k in 1..=3 {
                if self.pos[a] + k <= 10 {
                    v.push(format!("mut {},{}", (b'A' + a as u8) as char, k));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let (a, k) = m[4..].split_once(',').unwrap();
        let a = (a.as_bytes()[0].to_ascii_uppercase() - b'A') as usize;
        self.pos[a] += k.parse::<i32>().unwrap();
        self.moves_by[me][a] += 1;
        self.ply += 1;
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.ply >= 24 || self.pos.iter().all(|&p| p >= 10) {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * self.score(a) - self.guess_opp(1 - a)
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side));
        let mut s = format!("Agenții Tăcuți — mutarea {}/24\n", self.ply + 1);
        for a in 0..6 {
            s.push_str(&format!(
                "{}: {}{}  (mutat de J1 ×{}, J2 ×{})\n",
                (b'A' + a as u8) as char,
                ".".repeat(self.pos[a] as usize),
                "@",
                self.moves_by[0][a],
                self.moves_by[1][a]
            ));
        }
        let v: Vec<String> = (0..6)
            .map(|a| format!("{}={}", (b'A' + a as u8) as char, self.val[viewer][a]))
            .collect();
        s.push_str(&format!(
            "Valorile tale (J{}): {}\nLa mutare: {} (mut X,k)\n",
            viewer + 1,
            v.join(" "),
            pname(self.side)
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Agenti::with_seed, 100);
    }
    #[test]
    fn opponent_values_are_not_rendered() {
        let g = Agenti::with_seed(5);
        let mut h = g.clone();
        h.view(Some(Player::One));
        let s = h.show();
        let line: String = (0..6)
            .map(|a| format!("{}={}", (b'A' + a as u8) as char, g.val[0][a]))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(s.contains(&line));
        let other: String = (0..6)
            .map(|a| format!("{}={}", (b'A' + a as u8) as char, g.val[1][a]))
            .collect::<Vec<_>>()
            .join(" ");
        if line != other {
            assert!(!s.contains(&other));
        }
    }
    #[test]
    fn ai_ignores_opponent_values() {
        let a = Agenti::with_seed(5);
        let mut b = a.clone();
        b.val[1].reverse();
        for seed in 1..6 {
            assert_eq!(
                crate::brand_util::ai_of(&a, seed),
                crate::brand_util::ai_of(&b, seed)
            );
        }
    }
}
