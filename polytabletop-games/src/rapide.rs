//! Rapidele — canoe pe râu spre prag (variantă originală de „cursă pe curent”).
//!
//! Râul are 22 de câmpuri (0–21); la 21 e cascada. Fiecare jucător are 3 canoe pe câmpul 0 și o
//! mână deschisă de 5 cărți-vâsle (1–4). Pe tură: `vasla c,k` mută canoea c cu k câmpuri în
//! aval (joci cartea k și tragi alta); aterizând pe un câmp cu piatră prețioasă (valoare 1–3) o
//! încarci. Pe malurile 12, 16 și 20 poți `acostez c`: descarci încărcătura (punctele tale) și
//! canoea iese din joc. La finalul rundei (după J2) curentul împinge toate canoele cu 1 câmp;
//! canoea care ajunge la 21 se pierde cu tot cu încărcătură. Se termină când nicio canoe nu mai e
//! pe râu sau după 60 de mutări. Câștigă cine are mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Rapide {
    canoe: [[i32; 3]; 2], // -1 = ieșită, 22 = pierdută
    cargo: [[i32; 3]; 2],
    gems: [i32; 22],
    hand: [Vec<u8>; 2],
    deck: Vec<u8>,
    score: [i32; 2],
    side: Player,
    ply: u32,
}

const BANKS: [i32; 3] = [12, 16, 20];

impl Rapide {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut gems = [0; 22];
        for i in 5..20 {
            if rng.below(100) < 40 {
                gems[i] = 1 + rng.below(3) as i32;
            }
        }
        let mut deck: Vec<u8> = Vec::new();
        for v in 1..=4u8 {
            for _ in 0..(if v == 4 { 6 } else { 10 }) {
                deck.push(v);
            }
        }
        shuf(&mut deck, &mut rng);
        let h1 = (0..5).map(|_| deck.pop().unwrap()).collect();
        let h2 = (0..5).map(|_| deck.pop().unwrap()).collect();
        Rapide {
            canoe: [[0; 3]; 2],
            cargo: [[0; 3]; 2],
            gems,
            hand: [h1, h2],
            deck,
            score: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn afloat(&self, p: usize) -> bool {
        self.canoe[p].iter().any(|&c| (0..21).contains(&c))
    }
    fn hold(&self, p: usize) -> i32 {
        (0..3)
            .filter(|&i| (0..21).contains(&self.canoe[p][i]))
            .map(|i| self.cargo[p][i])
            .sum()
    }
}

impl Default for Rapide {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Rapide {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for c in 0..3 {
            let pos = self.canoe[me][c];
            if !(0..21).contains(&pos) {
                continue;
            }
            if BANKS.contains(&pos) {
                v.push(format!("acostez {}", c + 1));
            }
            let mut seen = [false; 5];
            for &k in &self.hand[me] {
                if !seen[k as usize] {
                    seen[k as usize] = true;
                    v.push(format!("vasla {},{}", c + 1, k));
                }
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if let Some(c) = m.strip_prefix("acostez ") {
            let c: usize = c.parse::<usize>().unwrap() - 1;
            self.score[me] += self.cargo[me][c];
            self.cargo[me][c] = 0;
            self.canoe[me][c] = -1;
        } else if let Some(r) = m.strip_prefix("vasla ") {
            let (c, k) = r.split_once(',').unwrap();
            let (c, k): (usize, i32) = (c.parse::<usize>().unwrap() - 1, k.parse().unwrap());
            let i = self.hand[me].iter().position(|&x| x as i32 == k).unwrap();
            self.hand[me].remove(i);
            if let Some(n) = self.deck.pop() {
                self.hand[me].push(n);
            }
            let np = self.canoe[me][c] + k;
            if np >= 21 {
                self.canoe[me][c] = 22;
                self.cargo[me][c] = 0;
            } else {
                self.canoe[me][c] = np;
                self.cargo[me][c] += self.gems[np as usize];
                self.gems[np as usize] = 0;
            }
        }
        if self.side == Player::Two {
            for p in 0..2 {
                for c in 0..3 {
                    if (0..21).contains(&self.canoe[p][c]) {
                        self.canoe[p][c] += 1;
                        if self.canoe[p][c] >= 21 {
                            self.canoe[p][c] = 22;
                            self.cargo[p][c] = 0;
                        }
                    }
                }
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if (!self.afloat(0) && !self.afloat(1)) || self.ply >= 60 {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| {
            10 * self.score[i]
                + 6 * self.hold(i)
                + self.canoe[i]
                    .iter()
                    .filter(|&&c| (0..21).contains(&c))
                    .count() as i32
        };
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from(
            "Rapidele — râu 0..21 (21 = cascada), maluri la 12, 16, 20; ◆n = piatră\n",
        );
        for i in 0..22 {
            s.push_str(&format!("{:>3}", i));
        }
        s.push('\n');
        for i in 0..22 {
            s.push_str(&if self.gems[i] > 0 {
                format!("{:>3}", format!("◆{}", self.gems[i]))
            } else if BANKS.contains(&(i as i32)) {
                "  |".to_string()
            } else {
                "  .".to_string()
            });
        }
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: scor {} | canoe {:?} | încărcătură {:?} | mână {:?}\n",
                p + 1,
                self.score[p],
                self.canoe[p],
                self.cargo[p],
                self.hand[p]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (vasla c,k | acostez c)\n",
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
        crate::brand_util::check(Rapide::with_seed, 200);
    }
}
