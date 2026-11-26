//! Banane-Cuvinte — cursă de cuvinte cu literele din mână (variantă originală, 2 jucători).
//!
//! 60 de litere în sac; fiecare începe cu 10 (mâinile sunt deschise). Pe tură: `cuvant xyz` — un cuvânt
//! din dicționarul românesc de joc (cel puțin 3 litere) făcut numai din literele tale, care se consumă;
//! apoi tragi o literă („coajă”). Sau `arunc l`: pui litera l înapoi în sac și tragi 3 (dacă sacul are
//! cel puțin 3 litere). Dacă nu poți nimic, `pas`. Câștigă primul care rămâne fără litere; după 80 de
//! mutări câștigă cine are mai puține litere (egal: remiză).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const DIST: [u8; 26] = [
    6, 2, 3, 3, 6, 1, 2, 1, 6, 1, 0, 3, 3, 4, 4, 3, 0, 4, 4, 4, 3, 1, 0, 1, 0, 1,
];

#[derive(Clone)]
pub struct Banane {
    rack: [[u8; 26]; 2],
    bag: Vec<u8>,
    side: Player,
    ply: u32,
    winner: Option<Player>,
    rng: Rng,
}

fn letter(c: u8) -> char {
    (b'a' + c) as char
}

impl Banane {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag: Vec<u8> = Vec::new();
        for (l, &n) in DIST.iter().enumerate() {
            for _ in 0..n {
                bag.push(l as u8);
            }
        }
        shuf(&mut bag, &mut rng);
        let mut g = Banane {
            rack: [[0; 26]; 2],
            bag,
            side: Player::One,
            ply: 0,
            winner: None,
            rng,
        };
        for p in 0..2 {
            for _ in 0..10 {
                g.draw(p);
            }
        }
        g
    }
    fn draw(&mut self, p: usize) {
        if let Some(l) = self.bag.pop() {
            self.rack[p][l as usize] += 1;
        }
    }
    fn count(&self, p: usize) -> i32 {
        self.rack[p].iter().map(|&x| x as i32).sum()
    }
    fn can(&self, p: usize, w: &[u8]) -> bool {
        let mut need = [0u8; 26];
        for &c in w {
            need[c as usize] += 1;
        }
        (0..26).all(|i| need[i] <= self.rack[p][i])
    }
}

impl Default for Banane {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Banane {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v: Vec<String> = Vec::new();
        for w in words() {
            if w.len() >= 3 && self.can(me, w) {
                v.push(format!(
                    "cuvant {}",
                    w.iter().map(|&c| letter(c)).collect::<String>()
                ));
            }
        }
        if self.bag.len() >= 3 {
            for l in 0..26 {
                if self.rack[me][l] > 0 {
                    v.push(format!("arunc {}", letter(l as u8)));
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
        if let Some(w) = m.strip_prefix("cuvant ") {
            for c in w.bytes() {
                self.rack[me][(c - b'a') as usize] -= 1;
            }
            if self.count(me) == 0 {
                self.winner = Some(self.side);
            } else {
                self.draw(me);
            }
        } else if let Some(l) = m.strip_prefix("arunc ") {
            let c = (l.as_bytes()[0] - b'a') as usize;
            self.rack[me][c] -= 1;
            let i = self.rng.below(self.bag.len() + 1);
            self.bag.insert(i, c as u8);
            for _ in 0..3 {
                self.draw(me);
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.ply >= 80 {
            return Some(by_score(-self.count(0), -self.count(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.count(1 - a) - self.count(a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Banane-Cuvinte — sac {} litere | mutarea {}/80\n",
            self.bag.len(),
            self.ply + 1
        );
        for p in 0..2 {
            let r: String = (0..26)
                .flat_map(|l| std::iter::repeat_n(letter(l as u8), self.rack[p][l] as usize))
                .collect();
            s.push_str(&format!("J{} ({} litere): {}\n", p + 1, self.count(p), r));
        }
        s.push_str(&format!(
            "La mutare: {} (cuvant xyz | arunc l | pas)\n",
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
        crate::brand_util::check(Banane::with_seed, 200);
    }
    #[test]
    fn words_use_only_rack() {
        let g = Banane::with_seed(4);
        for m in g.moves() {
            if let Some(w) = m.strip_prefix("cuvant ") {
                let v: Vec<u8> = w.bytes().map(|c| c - b'a').collect();
                assert!(g.can(0, &v) && is_word(&v));
            }
        }
    }
}
