//! Menajeria — cărucioare cu animale și țarcuri (variantă originală, 2 jucători).
//!
//! Cinci specii (8 plăcuțe fiecare). Trei cărucioare de câte 3 locuri. Pe tură: `trage` (se
//! dezvăluie următoarea plăcuță, apoi o pui cu `pune k` într-un cărucior cu loc) sau `ia k`
//! (iei animalele din căruciorul k în țarcurile tale și ieși din runda curentă). Fiecare are 3
//! țarcuri de capacități 3, 4 și 5, fiecare cu o singură specie; animalele se așază automat:
//! întâi în țarcul speciei lor, altfel în cel mai mare țarc gol; ce nu încape se pierde.
//! Runda se încheie când amândoi ați luat un cărucior. Jocul se termină când se golește pachetul
//! (la finalul rundei). Scor: fiecare țarc valorează pătratul numărului de animale (plin: bonus +2).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const CAPS: [u8; 3] = [3, 4, 5];

#[derive(Clone)]
pub struct Menajerie {
    deck: Vec<u8>,
    truck: [Vec<u8>; 3],
    pen: [[(u8, u8); 3]; 2], // (specie+1, număr)
    taken: [bool; 2],
    pending: Option<u8>,
    side: Player,
    over: bool,
    ply: u32,
}

impl Menajerie {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..40).map(|i| (i % 5) as u8).collect();
        shuf(&mut deck, &mut rng);
        Menajerie {
            deck,
            truck: Default::default(),
            pen: [[(0, 0); 3]; 2],
            taken: [false; 2],
            pending: None,
            side: Player::One,
            over: false,
            ply: 0,
        }
    }
    fn place(&mut self, p: usize, sp: u8) {
        for i in 0..3 {
            let (s, n) = self.pen[p][i];
            if s == sp + 1 && n < CAPS[i] {
                self.pen[p][i].1 += 1;
                return;
            }
        }
        for i in (0..3).rev() {
            if self.pen[p][i].0 == 0 {
                self.pen[p][i] = (sp + 1, 1);
                return;
            }
        }
    }
    fn score(&self, p: usize) -> i32 {
        let mut s = 0;
        for i in 0..3 {
            let n = self.pen[p][i].1 as i32;
            s += n * n + if self.pen[p][i].1 == CAPS[i] { 2 } else { 0 };
        }
        s
    }
    fn can_draw(&self) -> bool {
        !self.deck.is_empty() && self.truck.iter().any(|t| t.len() < 3)
    }
}

impl Default for Menajerie {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Menajerie {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        if self.pending.is_some() {
            for k in 0..3 {
                if self.truck[k].len() < 3 {
                    v.push(format!("pune {}", k + 1));
                }
            }
            return v;
        }
        if self.can_draw() {
            v.push("trage".into());
        }
        for k in 0..3 {
            if !self.truck[k].is_empty() && !self.taken[me] {
                v.push(format!("ia {}", k + 1));
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
        if m == "trage" {
            self.pending = self.deck.pop();
            return;
        }
        if let Some(k) = m.strip_prefix("pune ") {
            let t = self.pending.take().unwrap();
            self.truck[k.parse::<usize>().unwrap() - 1].push(t);
        } else if let Some(k) = m.strip_prefix("ia ") {
            let t = std::mem::take(&mut self.truck[k.parse::<usize>().unwrap() - 1]);
            for sp in t {
                self.place(me, sp);
            }
            self.taken[me] = true;
        } else {
            self.taken[me] = true;
        }
        if self.taken[0] && self.taken[1] {
            self.taken = [false; 2];
            if self.deck.is_empty() {
                self.over = true;
            }
        }
        // cine a luat deja nu mai joacă în rundă
        let mut next = self.side.other();
        if self.taken[pi(next)] && !self.taken[pi(self.side)] {
            next = self.side;
        }
        self.side = next;
        if self.pending.is_none() {
            // jucătorul care a luat deja nu mai are mutări
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.over || self.ply > 400 {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.score(a) - self.score(1 - a)
    }
    fn show(&self) -> String {
        let nm = ["leu", "zebră", "elefant", "maimuță", "cămilă"];
        let mut s = format!(
            "Menajeria — pachet {}{}\n",
            self.deck.len(),
            self.pending
                .map(|t| format!(" | plăcuță dezvăluită: {}", nm[t as usize]))
                .unwrap_or_default()
        );
        for k in 0..3 {
            let t: Vec<&str> = self.truck[k].iter().map(|&x| nm[x as usize]).collect();
            s.push_str(&format!(
                "Cărucior {} (3 locuri): {}\n",
                k + 1,
                t.join(", ")
            ));
        }
        for p in 0..2 {
            let pens: Vec<String> = (0..3)
                .map(|i| {
                    if self.pen[p][i].0 == 0 {
                        format!("[gol/{}]", CAPS[i])
                    } else {
                        format!(
                            "[{} {}/{}]",
                            nm[self.pen[p][i].0 as usize - 1],
                            self.pen[p][i].1,
                            CAPS[i]
                        )
                    }
                })
                .collect();
            s.push_str(&format!(
                "J{}: scor {} | {}{}\n",
                p + 1,
                self.score(p),
                pens.join(" "),
                if self.taken[p] {
                    " | a luat cărucior"
                } else {
                    ""
                }
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (trage | pune k | ia k)\n",
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
        crate::brand_util::check(Menajerie::with_seed, 500);
    }
}
