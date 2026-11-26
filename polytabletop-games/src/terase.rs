//! Terase (variantă proprie de construcție pe tablă 5×5): fiecare jucător are doi constructori
//! (J1 pornește pe b2 și d4, J2 pe b4 și d2). O tură: muți un constructor pe o celulă vecină
//! liberă cu cel mult un nivel mai sus decât cel curent, apoi ridici un nivel pe o celulă
//! vecină liberă (după nivelul 3 urmează cupola, care blochează celula). Câștigi dacă
//! urci un constructor pe nivelul 3; pierzi dacă nu mai poți face o tură completă.
//! Mutări: b2-b3+c3 (constructorul de pe b2 merge pe b3 și construiește pe c3).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 5;

#[derive(Clone)]
pub struct Terase {
    h: [u8; 25], // 0..3 niveluri, 4 cupolă
    w: [u8; 25], // 0 / 1 / 2 constructor
    side: Player,
    win: Option<Player>,
}

impl Default for Terase {
    fn default() -> Self {
        Self::new()
    }
}

fn nbrs(i: usize) -> impl Iterator<Item = usize> {
    let (x, y) = ((i % 5) as i32, (i / 5) as i32);
    (-1..=1)
        .flat_map(move |dy| (-1..=1).map(move |dx| (dx, dy)))
        .filter_map(move |(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            ((dx != 0 || dy != 0) && nx >= 0 && ny >= 0 && nx < N && ny < N)
                .then(|| (ny * N + nx) as usize)
        })
}

impl Terase {
    pub fn new() -> Self {
        let mut t = Terase {
            h: [0; 25],
            w: [0; 25],
            side: Player::One,
            win: None,
        };
        t.w[6] = 1; // b2
        t.w[18] = 1; // d4
        t.w[16] = 2; // b4? (x1,y3)=16
        t.w[8] = 2; // d2 (x3,y1)=8
        t
    }
}

impl Game for Terase {
    type Move = (u8, u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.win.is_some() {
            return vec![];
        }
        let me = self.side.idx() as u8 + 1;
        let mut v = vec![];
        for f in 0..25 {
            if self.w[f] != me {
                continue;
            }
            for t in nbrs(f) {
                if self.w[t] != 0 || self.h[t] >= 4 || self.h[t] > self.h[f] + 1 {
                    continue;
                }
                if self.h[t] == 3 {
                    v.push((f as u8, t as u8, t as u8)); // câștig: construcția nu mai contează
                    continue;
                }
                for b in nbrs(t) {
                    if (self.w[b] == 0 || b == f) && self.h[b] < 4 {
                        v.push((f as u8, t as u8, b as u8));
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let me = self.side.idx() as u8 + 1;
        let (f, t, b) = (m.0 as usize, m.1 as usize, m.2 as usize);
        self.w[f] = 0;
        self.w[t] = me;
        if self.h[t] == 3 {
            self.win = Some(self.side);
        } else {
            self.h[b] += 1;
        }
        self.side = self.side.other();
        if self.win.is_none() && self.legal_moves().is_empty() {
            self.win = Some(self.side.other());
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        self.win.map(Outcome::Win)
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        for i in 0..25 {
            if self.w[i] == 0 {
                continue;
            }
            let hv = self.h[i] as i32;
            let mut val = hv * hv * 12;
            let reach = nbrs(i)
                .filter(|&n| self.w[n] == 0 && self.h[n] < 4 && self.h[n] as i32 <= hv + 1)
                .count() as i32;
            val += reach * 3;
            let c = (i % 5) as i32 - 2;
            let r = (i / 5) as i32 - 2;
            val += 6 - (c.abs() + r.abs()) * 2;
            s += if self.w[i] == me { val } else { -val };
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(5, 5, |x, y| {
            let i = y * 5 + x;
            if self.w[i] != 0 {
                if self.w[i] == 1 {
                    'X'
                } else {
                    'O'
                }
            } else if self.h[i] == 4 {
                '#'
            } else {
                (b'0' + self.h[i]) as char
            }
        });
        s.push_str("Cifră = nivel, # = cupolă, X/O = constructori (nivelul de sub ei:");
        for i in 0..25 {
            if self.w[i] != 0 {
                s.push_str(&format!(" {}={}", sq(i % 5, i / 5), self.h[i]));
            }
        }
        s.push_str(")\n");
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        let p = |i: u8| sq(i as usize % 5, i as usize / 5);
        if m.1 == m.2 {
            format!("{}-{}", p(m.0), p(m.1))
        } else {
            format!("{}-{}+{}", p(m.0), p(m.1), p(m.2))
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 6,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            19,
            self.side,
            (0..25).map(|i| (i, (self.h[i] as u64 + 1) | (self.w[i] as u64) << 4)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn climbing_rules() {
        let mut g = Terase::new();
        g.h[7] = 2; // c2 prea sus pentru b2 (nivel 0)
        assert!(!g.legal_moves().iter().any(|m| m.0 == 6 && m.1 == 7));
        g.h[6] = 2;
        g.h[7] = 3;
        let mv = g.legal_moves();
        assert!(mv.iter().any(|m| m.0 == 6 && m.1 == 7 && m.2 == 7));
        g.apply(&(6, 7, 7));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Terase::new(), 300);
    }
}
