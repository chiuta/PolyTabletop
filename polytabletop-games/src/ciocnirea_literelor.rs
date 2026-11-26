//! Ciocnirea Literelor — cuvinte pe litere epuizabile (variantă originală, informație perfectă, 2 jucători).
//!
//! În fiecare rundă se stabilește o lungime L (4, 5 sau 6 litere). Pe rând jucătorii spun un cuvânt din
//! dicționarul PolyTabletop de lungime L, `cuv CUVANT`, a cărui literă de început nu a mai fost „apăsată”
//! în runda asta; litera se consumă, iar cuvântul nu se mai poate repeta. Cine nu are niciun cuvânt posibil
//! spune `nu am` și pierde runda (adversarul +1). 4 runde. AI-ul alege cuvântul care lasă adversarul cu cele mai
//! puține variante.
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};
use std::collections::BTreeSet;

const ROUNDS: u32 = 4;
const LENS: [usize; 3] = [4, 5, 6];

fn by_len(l: usize) -> &'static Vec<Vec<u8>> {
    static S: std::sync::OnceLock<Vec<Vec<Vec<u8>>>> = std::sync::OnceLock::new();
    &S.get_or_init(|| {
        LENS.iter()
            .map(|&l| words().iter().filter(|w| w.len() == l).cloned().collect())
            .collect()
    })[l - 4]
}

fn to_str(w: &[u8]) -> String {
    w.iter().map(|&c| (b'A' + c) as char).collect()
}

#[derive(Clone)]
pub struct Ciocnire {
    len: usize,
    used_letters: u32,
    used_words: BTreeSet<Vec<u8>>,
    round: u32,
    pts: [i32; 2],
    side: Player,
}

impl Ciocnire {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let _ = seed;
        Ciocnire {
            len: LENS[0],
            used_letters: 0,
            used_words: BTreeSet::new(),
            round: 0,
            pts: [0; 2],
            side: Player::One,
        }
    }
    fn avail(&self) -> Vec<&'static Vec<u8>> {
        by_len(self.len)
            .iter()
            .filter(|w| self.used_letters & (1 << w[0]) == 0 && !self.used_words.contains(*w))
            .collect()
    }
}

impl Default for Ciocnire {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Ciocnire {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let a = self.avail();
        if a.is_empty() {
            return vec!["nu am".into()];
        }
        a.iter().map(|w| format!("cuv {}", to_str(w))).collect()
    }
    fn step(&mut self, m: &str) {
        if m == "nu am" {
            self.pts[pi(self.side.other())] += 1;
            self.round += 1;
            self.len = LENS[(self.round as usize) % 3];
            self.used_letters = 0;
            self.used_words.clear();
            self.side = if self.round.is_multiple_of(2) {
                Player::One
            } else {
                Player::Two
            };
            return;
        }
        let w: Vec<u8> = m[4..]
            .bytes()
            .map(|b| b.to_ascii_uppercase() - b'A')
            .collect();
        self.used_letters |= 1 << w[0];
        self.used_words.insert(w);
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 10000 * (self.pts[a] - self.pts[1 - a]);
        // după mutarea mea, adversarul are `n` variante: cu cât mai puține, cu atât mai bine
        let n = self.avail().len() as i32;
        if self.side == p {
            base
        } else if n == 0 {
            base + 5000
        } else {
            base - n.min(2000)
        }
    }
    fn show(&self) -> String {
        let letters: String = (0..26)
            .filter(|&c| self.used_letters & (1 << c) != 0)
            .map(|c| (b'A' + c as u8) as char)
            .collect();
        format!(
            "Ciocnirea Literelor — runda {}/{} | cuvinte de {} litere | puncte J1 {}, J2 {}\nLitere consumate: {} | cuvinte spuse: {}\nVariante disponibile: {}\nLa mutare: {} (cuv CUVANT | nu am)\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.len,
            self.pts[0],
            self.pts[1],
            if letters.is_empty() { "-".into() } else { letters },
            self.used_words.iter().map(|w| to_str(w)).collect::<Vec<_>>().join(" "),
            self.avail().len(),
            pname(self.side)
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Ciocnire::with_seed, 200);
    }
    #[test]
    fn dictionary_has_words_of_every_length() {
        for l in LENS {
            assert!(by_len(l).len() > 30, "lungimea {l}");
        }
    }
}
