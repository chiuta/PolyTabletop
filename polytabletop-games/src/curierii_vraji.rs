//! Curierii Vrăjitoarelor — cărți de rol curajoase sau fricoase (variantă originală, 2 jucători).
//!
//! Două vrăjitoare pe o hartă cu 8 locuri în cerc (+2 scurtături). Locurile 1, 4 și 7 sunt izvoare
//! de poțiuni (culorile 1, 2, 3); fiecare dintre celelalte are o cerere (culoare și valoare 2–4).
//! Rundă: fiecare joacă pe rând cele 4 roluri din mână (muta, colecteaza, livreaza, odihna), fiecare în
//! mod `f` (fricos: efect mic, sigur) sau `c` (curajos: efect mare). Un rol curajos al adversarului
//! rămâne în așteptare până răspunzi: dacă joci și tu același rol (în orice mod), curajosul lui se
//! anulează. Mutări: `muta:f>n` (1 pas) / `muta:c>n` (până la 3 pași), `colecteaza:f|c` (1 sau 3 poțiuni
//! la un izvor, capacitate 3), `livreaza:f|c` (1 sau toate poțiunile potrivite cererii locului),
//! `odihna:f|c` (+1 sau +3 puncte). 4 runde (ordinea alternează). Câștigă cine are mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROLES: [&str; 4] = ["muta", "colecteaza", "livreaza", "odihna"];

#[derive(Clone)]
pub struct Curierii {
    adj: [[bool; 8]; 8],
    demand: [(u8, u8); 8], // (culoare, valoare); culoare 0 = izvor
    at: [usize; 2],
    potions: [[u8; 4]; 2],
    pts: [i32; 2],
    hand: [[bool; 4]; 2],
    pending: Option<(usize, usize, bool, usize)>, // (jucător, rol, curajos, țintă)
    round: u32,
    plays: u8,
    side: Player,
}

impl Curierii {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut adj = [[false; 8]; 8];
        for i in 0..8 {
            adj[i][(i + 1) % 8] = true;
            adj[(i + 1) % 8][i] = true;
        }
        for (a, b) in [(0, 4), (2, 6)] {
            adj[a][b] = true;
            adj[b][a] = true;
        }
        let mut demand = [(0u8, 0u8); 8];
        for i in 0..8 {
            if i == 1 || i == 4 || i == 7 {
                demand[i] = (0, 0);
            } else {
                demand[i] = (1 + rng.below(3) as u8, 2 + rng.below(3) as u8);
            }
        }
        Curierii {
            adj,
            demand,
            at: [0, 4],
            potions: [[0; 4]; 2],
            pts: [0; 2],
            hand: [[true; 4]; 2],
            pending: None,
            round: 0,
            plays: 0,
            side: Player::One,
        }
    }
    fn spring(&self, n: usize) -> Option<u8> {
        match n {
            1 => Some(1),
            4 => Some(2),
            7 => Some(3),
            _ => None,
        }
    }
    fn reach(&self, from: usize, steps: usize) -> Vec<usize> {
        let mut seen = vec![from];
        let mut frontier = vec![from];
        for _ in 0..steps {
            let mut nf = Vec::new();
            for &x in &frontier {
                for y in 0..8 {
                    if self.adj[x][y] && !seen.contains(&y) {
                        seen.push(y);
                        nf.push(y);
                    }
                }
            }
            frontier = nf;
        }
        seen.retain(|&x| x != from);
        seen
    }
    fn carried(&self, p: usize) -> i32 {
        self.potions[p].iter().map(|&x| x as i32).sum()
    }
    fn apply_role(&mut self, p: usize, role: usize, brave: bool, target: usize) {
        match role {
            0 => self.at[p] = target,
            1 => {
                if let Some(c) = self.spring(self.at[p]) {
                    let n = if brave { 3 } else { 1 };
                    for _ in 0..n {
                        if self.carried(p) < 3 {
                            self.potions[p][c as usize] += 1;
                        }
                    }
                }
            }
            2 => {
                let (c, v) = self.demand[self.at[p]];
                if c > 0 {
                    let k = if brave {
                        self.potions[p][c as usize]
                    } else {
                        self.potions[p][c as usize].min(1)
                    };
                    self.potions[p][c as usize] -= k;
                    self.pts[p] += k as i32 * v as i32;
                }
            }
            _ => self.pts[p] += if brave { 3 } else { 1 },
        }
    }
    fn total_hand(&self, p: usize) -> usize {
        self.hand[p].iter().filter(|&&x| x).count()
    }
}

impl Default for Curierii {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Curierii {
    fn depth(&self) -> u32 {
        2
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for r in 0..4 {
            if !self.hand[me][r] {
                continue;
            }
            for (tag, br) in [("f", false), ("c", true)] {
                if r == 0 {
                    for t in self.reach(self.at[me], if br { 3 } else { 1 }) {
                        v.push(format!("muta:{}>{}", tag, t + 1));
                    }
                } else {
                    v.push(format!("{}:{}", ROLES[r], tag));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let (head, tail) = m
            .split_once('>')
            .map(|(a, b)| (a, b.parse::<usize>().unwrap() - 1))
            .unwrap_or((m, 0));
        let (name, tag) = head.split_once(':').unwrap();
        let role = ROLES.iter().position(|&x| x == name).unwrap();
        let brave = tag == "c";
        self.hand[me][role] = false;
        if let Some((pp, prole, pbrave, ptarget)) = self.pending.take() {
            if prole != role {
                self.apply_role(pp, prole, pbrave, ptarget);
            }
        }
        if brave {
            self.pending = Some((me, role, true, tail));
        } else {
            self.apply_role(me, role, false, tail);
        }
        self.plays += 1;
        self.side = self.side.other();
        if self.plays == 8 {
            if let Some((pp, prole, pbrave, ptarget)) = self.pending.take() {
                self.apply_role(pp, prole, pbrave, ptarget);
            }
            self.plays = 0;
            self.round += 1;
            self.hand = [[true; 4]; 2];
            self.side = if self.round.is_multiple_of(2) {
                Player::One
            } else {
                Player::Two
            };
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 4 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| 10 * self.pts[i] + 4 * self.carried(i) + self.total_hand(i) as i32;
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Curierii Vrăjitoarelor — runda {}/4\n",
            (self.round + 1).min(4)
        );
        for i in 0..8 {
            let d = if let Some(c) = self.spring(i) {
                format!("izvor{}", c)
            } else {
                format!("cerere c{}×{}", self.demand[i].0, self.demand[i].1)
            };
            s.push_str(&format!(
                "{}: {}{}{}\n",
                i + 1,
                d,
                if self.at[0] == i { " [J1]" } else { "" },
                if self.at[1] == i { " [J2]" } else { "" }
            ));
        }
        for p in 0..2 {
            let h: Vec<&str> = (0..4)
                .filter(|&r| self.hand[p][r])
                .map(|r| ROLES[r])
                .collect();
            s.push_str(&format!(
                "J{}: puncte {} | poțiuni {:?} | roluri: {}\n",
                p + 1,
                self.pts[p],
                &self.potions[p][1..],
                h.join(" ")
            ));
        }
        if let Some((pp, r, _, _)) = self.pending {
            s.push_str(&format!(
                "În așteptare: {} curajos de la J{}\n",
                ROLES[r],
                pp + 1
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (rol:f|c, muta:f>n)\n",
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
        crate::brand_util::check(Curierii::with_seed, 200);
    }
}
