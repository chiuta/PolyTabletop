//! Bambuzeria — grădinar, panda și obiective de bambus (variantă originală pe grilă 5×5).
//!
//! Grădina are 25 de parcele de trei culori (0, 1, 2), fiecare cu bambus de înălțime 0–4. Pe
//! tură faci o acțiune: `gradinar c3` (grădinarul se mută în linie dreaptă și crește cu 1 bambusul
//! parcelei și al vecinilor de aceeași culoare), `panda c3` (panda se mută în linie și mănâncă
//! un segment de bambus din parcela aceea, adăugat în depozitul culorii ei), `ploaie c3` (+1 pe
//! o parcelă; maxim 2 pe jucător). Trei obiective deschise per jucător cer câte 2–5 segmente de
//! anumite culori; când ai destule, se completează automat (cheltui segmentele, primești punctele).
//! Primul la 20 de puncte câștigă; după 60 de acțiuni câștigă cine are mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: usize = 5;

#[derive(Clone)]
pub struct Bambuzeria {
    col: [[u8; N]; N],
    h: [[u8; N]; N],
    gard: (usize, usize),
    panda: (usize, usize),
    store: [[i32; 3]; 2],
    goals: [Vec<([i32; 3], i32)>; 2],
    deck: Vec<([i32; 3], i32)>,
    rain: [i32; 2],
    pts: [i32; 2],
    side: Player,
    ply: u32,
}

fn nm(r: usize, c: usize) -> String {
    format!("{}{}", (b'a' + c as u8) as char, r + 1)
}
fn pm(s: &str) -> (usize, usize) {
    let b = s.as_bytes();
    ((b[1] - b'1') as usize, (b[0] - b'a') as usize)
}

impl Bambuzeria {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut col = [[0u8; N]; N];
        for r in 0..N {
            for c in 0..N {
                col[r][c] = rng.below(3) as u8;
            }
        }
        let mut deck = Vec::new();
        for _ in 0..24 {
            let mut need = [0i32; 3];
            let n = 2 + rng.below(4) as i32;
            for _ in 0..n {
                need[rng.below(3)] += 1;
            }
            deck.push((need, n + 1));
        }
        let g1: Vec<_> = (0..3).map(|_| deck.pop().unwrap()).collect();
        let g2: Vec<_> = (0..3).map(|_| deck.pop().unwrap()).collect();
        let mut h = [[0u8; N]; N];
        h[2][2] = 1;
        Bambuzeria {
            col,
            h,
            gard: (2, 2),
            panda: (2, 2),
            store: [[0; 3]; 2],
            goals: [g1, g2],
            deck,
            rain: [2, 2],
            pts: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn line_targets(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for r in 0..N {
            if r != from.0 {
                v.push((r, from.1));
            }
        }
        for c in 0..N {
            if c != from.1 {
                v.push((from.0, c));
            }
        }
        v
    }
    fn progress(&self, p: usize) -> i32 {
        let mut s = 0;
        for (need, _) in &self.goals[p] {
            let tot: i32 = need.iter().sum();
            let have: i32 = (0..3).map(|k| need[k].min(self.store[p][k])).sum();
            s += 10 * have / tot.max(1);
        }
        s
    }
}

impl Default for Bambuzeria {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Bambuzeria {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for (r, c) in self.line_targets(self.gard) {
            v.push(format!("gradinar {}", nm(r, c)));
        }
        for (r, c) in self.line_targets(self.panda) {
            if self.h[r][c] > 0 {
                v.push(format!("panda {}", nm(r, c)));
            }
        }
        if self.rain[me] > 0 {
            for r in 0..N {
                for c in 0..N {
                    if self.h[r][c] < 4 {
                        v.push(format!("ploaie {}", nm(r, c)));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        let (cmd, at) = m.split_once(' ').unwrap();
        let (r, c) = pm(at);
        match cmd {
            "gradinar" => {
                self.gard = (r, c);
                let k = self.col[r][c];
                self.h[r][c] = (self.h[r][c] + 1).min(4);
                for (a, b) in [
                    (r.wrapping_sub(1), c),
                    (r + 1, c),
                    (r, c.wrapping_sub(1)),
                    (r, c + 1),
                ] {
                    if a < N && b < N && self.col[a][b] == k {
                        self.h[a][b] = (self.h[a][b] + 1).min(4);
                    }
                }
            }
            "panda" => {
                self.panda = (r, c);
                self.h[r][c] -= 1;
                self.store[me][self.col[r][c] as usize] += 1;
            }
            _ => {
                self.rain[me] -= 1;
                self.h[r][c] += 1;
            }
        }
        // obiective
        let mut i = 0;
        while i < self.goals[me].len() {
            let (need, pts) = self.goals[me][i];
            if (0..3).all(|k| self.store[me][k] >= need[k]) {
                for k in 0..3 {
                    self.store[me][k] -= need[k];
                }
                self.pts[me] += pts;
                self.goals[me].remove(i);
                if let Some(n) = self.deck.pop() {
                    self.goals[me].push(n);
                }
            } else {
                i += 1;
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.pts[0] >= 20 || self.pts[1] >= 20 || self.ply >= 60 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * (self.pts[a] - self.pts[1 - a]) + self.progress(a) - self.progress(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from(
            "Bambuzeria — culoare/înălțime; G grădinar, P panda\n   a    b    c    d    e\n",
        );
        for r in 0..N {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..N {
                let mark = match (self.gard == (r, c), self.panda == (r, c)) {
                    (true, true) => "GP",
                    (true, false) => "G ",
                    (false, true) => "P ",
                    _ => "  ",
                };
                s.push_str(&format!("{}{}{} ", self.col[r][c], self.h[r][c], mark));
            }
            s.push('\n');
        }
        for p in 0..2 {
            let g: Vec<String> = self.goals[p]
                .iter()
                .map(|(n, pt)| format!("{:?}={}", n, pt))
                .collect();
            s.push_str(&format!(
                "J{}: puncte {} | depozit {:?} | ploi {} | obiective {}\n",
                p + 1,
                self.pts[p],
                self.store[p],
                self.rain[p],
                g.join(" ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (gradinar c3 | panda c3 | ploaie c3)\n",
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
        crate::brand_util::check(Bambuzeria::with_seed, 100);
    }
    #[test]
    fn panda_eats_and_goal_completes() {
        let mut g = Bambuzeria::with_seed(3);
        g.goals[0] = vec![([1, 0, 0], 2)];
        g.col[2][3] = 0;
        g.h[2][3] = 2;
        g.step("panda d3");
        assert_eq!(g.pts[0], 2);
        assert_eq!(g.h[2][3], 1);
    }
}
