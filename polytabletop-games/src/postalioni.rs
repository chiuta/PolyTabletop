//! Poștalionii — rute de oraș în oraș pe o hartă de 12 orașe (variantă originală, mâini deschise).
//!
//! Harta e un cerc de 12 orașe plus 6 scurtături. Fiecare tură are două părți: întâi joci (o carte
//! din mână): `joc X` (orașul X se adaugă la ruta ta dacă e vecin cu un capăt și nu e deja în
//! rută; ruta goală acceptă orice), `nou X` (arunci ruta și începi alta cu X) sau `livrez`
//! (ruta de cel puțin 3 orașe se ia: 3, 6, 10, 15, 21 puncte pentru lungimile 3..7); apoi tragi:
//! `ia k` (una din cele 4 cărți deschise) sau `trage` din pachet. Pachetul are 3 cărți per oraș.
//! Jocul se termină când pachetul e gol; ruta neluată nu valorează nimic. Câștigă scorul mai mare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Postalioni {
    adj: [[bool; 12]; 12],
    route: [Vec<u8>; 2],
    hand: [Vec<u8>; 2],
    market: Vec<u8>,
    deck: Vec<u8>,
    pts: [i32; 2],
    drawing: bool,
    side: Player,
    ply: u32,
}

const VAL: [i32; 8] = [0, 0, 0, 3, 6, 10, 15, 21];

impl Postalioni {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut adj = [[false; 12]; 12];
        for i in 0..12 {
            adj[i][(i + 1) % 12] = true;
            adj[(i + 1) % 12][i] = true;
        }
        let mut k = 0;
        while k < 6 {
            let (a, b) = (rng.below(12), rng.below(12));
            if a != b && !adj[a][b] {
                adj[a][b] = true;
                adj[b][a] = true;
                k += 1;
            }
        }
        let mut deck: Vec<u8> = (0..36).map(|i| (i % 12) as u8).collect();
        shuf(&mut deck, &mut rng);
        let market: Vec<u8> = (0..4).map(|_| deck.pop().unwrap()).collect();
        let h1 = (0..3).map(|_| deck.pop().unwrap()).collect();
        let h2 = (0..3).map(|_| deck.pop().unwrap()).collect();
        Postalioni {
            adj,
            route: [Vec::new(), Vec::new()],
            hand: [h1, h2],
            market,
            deck,
            pts: [0; 2],
            drawing: false,
            side: Player::One,
            ply: 0,
        }
    }
    fn fits(&self, p: usize, c: u8) -> bool {
        let r = &self.route[p];
        if r.contains(&c) {
            return false;
        }
        r.is_empty()
            || self.adj[*r.last().unwrap() as usize][c as usize]
            || self.adj[r[0] as usize][c as usize]
    }
    fn len_score(&self, p: usize) -> i32 {
        VAL[self.route[p].len().min(7)]
    }
}

impl Default for Postalioni {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Postalioni {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        if self.drawing {
            for (k, _) in self.market.iter().enumerate() {
                v.push(format!("ia {}", k + 1));
            }
            if !self.deck.is_empty() {
                v.push("trage".into());
            }
            if v.is_empty() {
                v.push("trage".into());
            }
            return v;
        }
        let mut seen = Vec::new();
        for &c in &self.hand[me] {
            if seen.contains(&c) {
                continue;
            }
            seen.push(c);
            if self.fits(me, c) {
                v.push(format!("joc {}", c + 1));
            }
            v.push(format!("nou {}", c + 1));
        }
        if self.route[me].len() >= 3 {
            v.push("livrez".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if self.drawing {
            if m == "trage" {
                if let Some(c) = self.deck.pop() {
                    self.hand[me].push(c);
                }
            } else {
                let k = m[3..].parse::<usize>().unwrap() - 1;
                let c = self.market.remove(k);
                self.hand[me].push(c);
                if let Some(n) = self.deck.pop() {
                    self.market.push(n);
                }
            }
            self.drawing = false;
            self.side = self.side.other();
            return;
        }
        if m == "livrez" {
            self.pts[me] += self.len_score(me);
            self.route[me].clear();
        } else {
            let (cmd, a) = m.split_once(' ').unwrap();
            let c = a.parse::<u8>().unwrap() - 1;
            let i = self.hand[me].iter().position(|&x| x == c).unwrap();
            self.hand[me].remove(i);
            if cmd == "nou" {
                self.route[me].clear();
                self.route[me].push(c);
            } else if self.route[me].is_empty()
                || self.adj[*self.route[me].last().unwrap() as usize][c as usize]
            {
                self.route[me].push(c);
            } else {
                self.route[me].insert(0, c);
            }
        }
        self.drawing = true;
    }
    fn result(&self) -> Option<Outcome> {
        if (self.deck.is_empty() && !self.drawing) || self.ply > 400 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * (self.pts[a] - self.pts[1 - a]) + 4 * (self.len_score(a) - self.len_score(1 - a))
    }
    fn show(&self) -> String {
        let mut s = String::from("Poștalionii — orașe 1..12 în cerc + scurtături\n");
        let mut e = Vec::new();
        for a in 0..12 {
            for b in a + 1..12 {
                if self.adj[a][b] && (b != a + 1 && !(a == 0 && b == 11)) {
                    e.push(format!("{}-{}", a + 1, b + 1));
                }
            }
        }
        s.push_str(&format!("Scurtături: {}\n", e.join(" ")));
        let mk: Vec<String> = self.market.iter().map(|c| (c + 1).to_string()).collect();
        s.push_str(&format!(
            "Piață: {} | pachet {}\n",
            mk.join(" "),
            self.deck.len()
        ));
        for p in 0..2 {
            let r: Vec<String> = self.route[p].iter().map(|c| (c + 1).to_string()).collect();
            let h: Vec<String> = self.hand[p].iter().map(|c| (c + 1).to_string()).collect();
            s.push_str(&format!(
                "J{}: puncte {} | rută [{}] | mână [{}]\n",
                p + 1,
                self.pts[p],
                r.join("-"),
                h.join(" ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({})\n",
            pname(self.side),
            if self.drawing {
                "ia k | trage"
            } else {
                "joc X | nou X | livrez"
            }
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Postalioni::with_seed, 500);
    }
}
