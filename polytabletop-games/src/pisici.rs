//! Pisicile din Barcă — draft de pisici-poliomino și împachetare pe barcă (variantă originală, 2 jucători).
//!
//! 5 runde. În fiecare rundă se dezvăluie 4 pisici (formă și valoare) — mutarea forțată `scot`.
//! Se alege pe rând (primul alternează: 1-2-1-2): `ia k`; apoi pisica se așază pe barca ta 5×6:
//! `pune r,c,o` (colțul de sus-stânga al formei, orientarea o) sau `arunc` dacă nu încape/nu o
//! vrei (pierdută, −1 punct). Scor: fiecare celulă acoperită 1 punct + valoarea pisicii (1–3, o dată),
//! fiecare rând plin de 6 celule +3. Câștigă scorul mai mare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROWS: usize = 5;
const COLS: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Reveal,
    Pick,
    Place,
}

#[derive(Clone)]
pub struct Pisici {
    boat: [[[bool; COLS]; ROWS]; 2],
    row: Vec<(usize, i32)>,
    held: Option<(usize, i32)>,
    deck: Vec<(usize, i32)>,
    bonus: [i32; 2],
    round: u32,
    picks: u8,
    ph: Ph,
    shapes: Vec<Vec<Vec<(i8, i8)>>>,
}

impl Pisici {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<(usize, i32)> =
            (0..22).map(|i| (i % 11, 1 + rng.below(3) as i32)).collect();
        shuf(&mut deck, &mut rng);
        let shapes = (0..11).map(|i| orientations(&poly(i), true)).collect();
        Pisici {
            boat: [[[false; COLS]; ROWS]; 2],
            row: Vec::new(),
            held: None,
            deck,
            bonus: [0; 2],
            round: 0,
            picks: 0,
            ph: Ph::Reveal,
            shapes,
        }
    }
    fn picker(&self) -> usize {
        let first = (self.round % 2) as usize;
        if self.picks.is_multiple_of(2) {
            first
        } else {
            1 - first
        }
    }
    fn covered(&self, p: usize) -> i32 {
        self.boat[p].iter().flatten().filter(|&&x| x).count() as i32
    }
    fn full_rows(&self, p: usize) -> i32 {
        self.boat[p].iter().filter(|r| r.iter().all(|&x| x)).count() as i32
    }
    fn score(&self, p: usize) -> i32 {
        self.covered(p) + 3 * self.full_rows(p) + self.bonus[p]
    }
}

impl Default for Pisici {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Pisici {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        if self.picker() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Reveal => vec!["scot".into()],
            Ph::Pick => (0..self.row.len())
                .map(|k| format!("ia {}", k + 1))
                .collect(),
            Ph::Place => {
                let me = self.picker();
                let (sh, _) = self.held.unwrap();
                let mut v = Vec::new();
                for (o, cells) in self.shapes[sh].iter().enumerate() {
                    for r in 0..ROWS as i8 {
                        for c in 0..COLS as i8 {
                            if cells.iter().all(|&(dr, dc)| {
                                let (a, b) = (r + dr, c + dc);
                                a < ROWS as i8
                                    && b < COLS as i8
                                    && !self.boat[me][a as usize][b as usize]
                            }) {
                                v.push(format!("pune {},{},{}", r + 1, c + 1, o));
                            }
                        }
                    }
                }
                v.push("arunc".into());
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.picker();
        match self.ph {
            Ph::Reveal => {
                self.row = (0..4).filter_map(|_| self.deck.pop()).collect();
                self.picks = 0;
                self.ph = Ph::Pick;
            }
            Ph::Pick => {
                let k = m[3..].parse::<usize>().unwrap() - 1;
                self.held = Some(self.row.remove(k));
                self.ph = Ph::Place;
            }
            Ph::Place => {
                let (sh, val) = self.held.take().unwrap();
                if m == "arunc" {
                    self.bonus[me] -= 1;
                } else {
                    let v: Vec<i32> = m[5..].split(',').map(|x| x.parse().unwrap()).collect();
                    for &(dr, dc) in &self.shapes[sh][v[2] as usize] {
                        self.boat[me][(v[0] - 1 + dr as i32) as usize]
                            [(v[1] - 1 + dc as i32) as usize] = true;
                    }
                    self.bonus[me] += val;
                }
                self.picks += 1;
                if self.row.is_empty() {
                    self.round += 1;
                    self.ph = Ph::Reveal;
                } else {
                    self.ph = Ph::Pick;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 5 || (self.deck.is_empty() && self.ph == Ph::Reveal) {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.score(a) - self.score(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!("Pisicile din Barcă — runda {}/5\n", (self.round + 1).min(5));
        for p in 0..2 {
            s.push_str(&format!("Barca J{} (scor {}):\n", p + 1, self.score(p)));
            for r in 0..ROWS {
                s.push_str("  ");
                for c in 0..COLS {
                    s.push(if self.boat[p][r][c] { '#' } else { '.' });
                    s.push(' ');
                }
                s.push('\n');
            }
        }
        let rw: Vec<String> = self
            .row
            .iter()
            .enumerate()
            .map(|(k, &(sh, v))| {
                format!(
                    "{}: formă {} ({} celule, valoare {})",
                    k + 1,
                    sh,
                    poly(sh).len(),
                    v
                )
            })
            .collect();
        s.push_str(&format!("Rândul: {}\nÎn mână: {}\nLa mutare: {} (faza {:?}: scot | ia k | pune r,c,o | arunc)\n", rw.join(" | "), self.held.map(|(sh, v)| format!("formă {} val {}", sh, v)).unwrap_or("-".into()), pname(self.side()), self.ph));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Pisici::with_seed, 200);
    }
}
