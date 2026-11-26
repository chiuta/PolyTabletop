//! Templele din Junglă — explorare și temple pe o grilă 5×5 (variantă originală).
//!
//! Fiecare jucător are 3 exploratori și câte 3 puncte de acțiune pe tură. Acțiuni (1 punct
//! fiecare): `mut a1>a2` (explorator pe celulă vecină), `templu c3` (+1 etaj pe celula unde ai un
//! explorator, max 5), `comoara c3` (iei comoara de pe celula unde ești, valoare 1–3). `pas` îți
//! termină tura. După fiecare 4 ture ale fiecărui jucător are loc o punctare (de 3 ori): la
//! fiecare templu primește etajele cine are mai mulți exploratori pe celula lui sau în vecinătatea
//! ortogonală (egalitate: nimeni). Câștigă cine are mai multe puncte (comorile contează).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: usize = 5;

#[derive(Clone)]
pub struct Templele {
    temple: [[u8; N]; N],
    treasure: [[u8; N]; N],
    expl: [[(u8, u8); 3]; 2],
    pts: [i32; 2],
    ap: u8,
    turns: u32,
    scorings: u32,
    side: Player,
}

fn nm(r: usize, c: usize) -> String {
    format!("{}{}", (b'a' + c as u8) as char, r + 1)
}
fn pm(s: &str) -> (usize, usize) {
    let b = s.as_bytes();
    ((b[1] - b'1') as usize, (b[0] - b'a') as usize)
}

impl Templele {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut treasure = [[0u8; N]; N];
        let mut k = 0;
        while k < 8 {
            let (r, c) = (rng.below(N), rng.below(N));
            if treasure[r][c] == 0 && (r, c) != (2, 2) {
                treasure[r][c] = 1 + rng.below(3) as u8;
                k += 1;
            }
        }
        Templele {
            temple: [[0; N]; N],
            treasure,
            expl: [[(2, 2); 3], [(2, 2); 3]],
            pts: [0; 2],
            ap: 3,
            turns: 0,
            scorings: 0,
            side: Player::One,
        }
    }
    fn influence(&self, p: usize, r: usize, c: usize) -> i32 {
        self.expl[p]
            .iter()
            .filter(|&&(a, b)| {
                let (a, b) = (a as i32, b as i32);
                (a - r as i32).abs() + (b - c as i32).abs() <= 1
            })
            .count() as i32
    }
    fn score_now(&self) -> [i32; 2] {
        let mut s = [0; 2];
        for r in 0..N {
            for c in 0..N {
                if self.temple[r][c] > 0 {
                    let (a, b) = (self.influence(0, r, c), self.influence(1, r, c));
                    if a > b {
                        s[0] += self.temple[r][c] as i32;
                    } else if b > a {
                        s[1] += self.temple[r][c] as i32;
                    }
                }
            }
        }
        s
    }
}

impl Default for Templele {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Templele {
    fn depth(&self) -> u32 {
        2
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        let mut seen: Vec<(u8, u8)> = Vec::new();
        for &(r, c) in &self.expl[me] {
            let (ru, cu) = (r as usize, c as usize);
            if !seen.contains(&(r, c)) {
                seen.push((r, c));
                if self.temple[ru][cu] < 5 {
                    v.push(format!("templu {}", nm(ru, cu)));
                }
                if self.treasure[ru][cu] > 0 {
                    v.push(format!("comoara {}", nm(ru, cu)));
                }
            }
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let (a, b) = (r as i32 + dr, c as i32 + dc);
                if (0..N as i32).contains(&a) && (0..N as i32).contains(&b) {
                    let s = format!("mut {}>{}", nm(ru, cu), nm(a as usize, b as usize));
                    if !v.contains(&s) {
                        v.push(s);
                    }
                }
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let mut end = m == "pas";
        if let Some(x) = m.strip_prefix("templu ") {
            let (r, c) = pm(x);
            self.temple[r][c] += 1;
        } else if let Some(x) = m.strip_prefix("comoara ") {
            let (r, c) = pm(x);
            self.pts[me] += self.treasure[r][c] as i32;
            self.treasure[r][c] = 0;
        } else if let Some(x) = m.strip_prefix("mut ") {
            let (a, b) = x.split_once('>').unwrap();
            let (ar, ac) = pm(a);
            let (br, bc) = pm(b);
            let i = self.expl[me]
                .iter()
                .position(|&e| e == (ar as u8, ac as u8))
                .unwrap();
            self.expl[me][i] = (br as u8, bc as u8);
        }
        if !end {
            self.ap -= 1;
            end = self.ap == 0;
        }
        if end {
            self.ap = 3;
            self.turns += 1;
            self.side = self.side.other();
            if self.turns.is_multiple_of(8) {
                let s = self.score_now();
                self.pts[0] += s[0];
                self.pts[1] += s[1];
                self.scorings += 1;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.scorings >= 3 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let s = self.score_now();
        6 * (self.pts[a] - self.pts[1 - a]) + 3 * (s[a] - s[1 - a])
    }
    fn show(&self) -> String {
        let mut s = format!("Templele din Junglă — punctare {}/3 | puncte J1 {}, J2 {} | acțiuni rămase: {}\n   a    b    c    d    e\n", self.scorings + 1, self.pts[0], self.pts[1], self.ap);
        for r in 0..N {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..N {
                let e1 = self.expl[0]
                    .iter()
                    .filter(|&&x| x == (r as u8, c as u8))
                    .count();
                let e2 = self.expl[1]
                    .iter()
                    .filter(|&&x| x == (r as u8, c as u8))
                    .count();
                let t = if self.treasure[r][c] > 0 {
                    format!("${}", self.treasure[r][c])
                } else {
                    "  ".into()
                };
                s.push_str(&format!(
                    "T{}{}{}{} ",
                    self.temple[r][c],
                    if e1 > 0 { e1.to_string() } else { "-".into() },
                    if e2 > 0 { e2.to_string() } else { "-".into() },
                    if t.trim().is_empty() { "".into() } else { t }
                ));
            }
            s.push('\n');
        }
        s.push_str(&format!("(Tn = etaje, apoi exploratori J1, J2, $comoară) | La mutare: {}\nMutări: mut a1>a2 | templu c3 | comoara c3 | pas\n", pname(self.side)));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Templele::with_seed, 400);
    }
}
