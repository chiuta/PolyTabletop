//! Trei tipuri (variantă proprie de stive pe tablă 6×6): fiecare jucător are 18 piese de trei
//! tipuri: A (3), B (6) și C (9), așezate simetric (oglindit pe rândurile 3–4). O piesă sau
//! stivă se mută pe linie dreaptă (rând, coloană, diagonală) pe prima celulă ocupată
//! întâlnită (nu poate sări peste piese, dar poate trece peste celule goale). Pe o stivă
//! adversară cu înălțime cel mult cât a ta o iei (stiva ta o înlocuiește); pe o stivă proprie
//! te alipești (înălțimile se adună, tipul de sus e al celei mutate). O tură: o captură
//! obligatorie, apoi opțional o a doua acțiune (altă captură, alipire sau `fin`). Pierzi
//! dacă nu mai ai niciun tip (A, B sau C) la vârful vreunei stive sau dacă la începutul turei
//! nu poți captura. Remiză după 150 de ture.
//! Mutări: `a1-a4`, `fin`.
#![allow(clippy::all)]

use crate::b1_util::{hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 6;

#[derive(Clone)]
pub struct TreiTipuri {
    h: [u8; 36],
    top: [u8; 36], // 0 gol, owner<<2 | tip(1..3)
    side: Player,
    second: bool,
    turns: u32,
    win: Option<Player>,
}

impl Default for TreiTipuri {
    fn default() -> Self {
        Self::new()
    }
}

impl TreiTipuri {
    pub fn new() -> Self {
        let mut types = vec![];
        for (t, n) in [(1u8, 3), (2, 6), (3, 9)] {
            for _ in 0..n {
                types.push(t);
            }
        }
        let mut top = [0u8; 36];
        let mut h = [0u8; 36];
        let mut k = 0;
        for y in 0..3 {
            for x in 0..6 {
                let t = types[(k * 7) % 18];
                k += 1;
                top[y * 6 + x] = t;
                h[y * 6 + x] = 1;
                top[(5 - y) * 6 + x] = 4 | t;
                h[(5 - y) * 6 + x] = 1;
            }
        }
        TreiTipuri {
            h,
            top,
            side: Player::One,
            second: false,
            turns: 0,
            win: None,
        }
    }
    fn own(c: u8) -> usize {
        (c >> 2) as usize
    }
    fn gen(&self, p: usize, caps: bool, stack: bool) -> Vec<(u8, u8)> {
        let mut v = vec![];
        for i in 0..36 {
            let c = self.top[i];
            if c == 0 || Self::own(c) != p {
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let (mut x, mut y) = ((i % 6) as i32 + dx, (i / 6) as i32 + dy);
                    while x >= 0 && y >= 0 && x < N && y < N {
                        let t = (y * N + x) as usize;
                        if self.top[t] != 0 {
                            let mine = Self::own(self.top[t]) == p;
                            if (mine && stack) || (!mine && caps && self.h[i] >= self.h[t]) {
                                v.push((i as u8, t as u8));
                            }
                            break;
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
        v
    }
    fn has_all_types(&self, p: usize) -> bool {
        (1..=3).all(|t| {
            self.top
                .iter()
                .any(|&c| c != 0 && Self::own(c) == p && c & 3 == t)
        })
    }
    fn count(&self, p: usize) -> i32 {
        (0..36)
            .filter(|&i| self.top[i] != 0 && Self::own(self.top[i]) == p)
            .map(|i| self.h[i] as i32)
            .sum()
    }
}

impl Game for TreiTipuri {
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.win.is_some() || self.turns >= 300 {
            return vec![];
        }
        let p = self.side.idx();
        if !self.second {
            let v = self.gen(p, true, false);
            if v.is_empty() {
                return vec![(255, 255)]; // nu poate captura: pierde (marcat în apply)
            }
            v
        } else {
            let mut v = self.gen(p, true, true);
            v.push((254, 254));
            v
        }
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let p = self.side.idx();
        if m.0 == 255 {
            self.win = Some(self.side.other());
            return;
        }
        if m.0 != 254 {
            let (f, t) = (m.0 as usize, m.1 as usize);
            let own_t = self.top[t] != 0 && Self::own(self.top[t]) == p;
            if own_t {
                self.h[t] += self.h[f];
            } else {
                self.h[t] = self.h[f];
            }
            self.top[t] = self.top[f];
            self.top[f] = 0;
            self.h[f] = 0;
            if !self.has_all_types(1 - p) {
                self.win = Some(self.side);
                return;
            }
            if !self.has_all_types(p) {
                self.win = Some(self.side.other());
                return;
            }
        }
        if !self.second && m.0 != 254 {
            self.second = true;
        } else {
            self.second = false;
            self.side = self.side.other();
            self.turns += 1;
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.turns >= 300 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.count(me) - self.count(1 - me)) * 8;
        for who in 0..2 {
            let mut v = 0;
            for t in 1..=3u8 {
                let n = (0..36)
                    .filter(|&i| {
                        self.top[i] != 0 && Self::own(self.top[i]) == who && self.top[i] & 3 == t
                    })
                    .count() as i32;
                v += match n {
                    0 => -1000,
                    1 => -60,
                    2 => -20,
                    _ => n.min(6) * 2,
                };
            }
            s += if who == me { v } else { -v };
        }
        s + (self.gen(me, true, false).len() as i32 - self.gen(1 - me, true, false).len() as i32)
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..6).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..6 {
                let c = self.top[y * 6 + x];
                if c == 0 {
                    s.push_str(" .  ");
                } else {
                    let ch = ['?', 'A', 'B', 'C'][(c & 3) as usize];
                    let ch = if Self::own(c) == 0 {
                        ch
                    } else {
                        ch.to_ascii_lowercase()
                    };
                    s.push_str(&format!("{}{:<2} ", ch, self.h[y * 6 + x]));
                }
            }
            s.push('\n');
        }
        s.push_str("   a   b   c   d   e   f\nMAJUSCULE = J1, minuscule = J2; numărul = înălțimea stivei.\n");
        if self.second {
            s.push_str("A doua acțiune (opțională): altă captură, alipire sau fin.\n");
        }
        s
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        if m.0 == 255 {
            "predau".into()
        } else if m.0 == 254 {
            "fin".into()
        } else {
            format!(
                "{}-{}",
                sq(m.0 as usize % 6, m.0 as usize / 6),
                sq(m.1 as usize % 6, m.1 as usize / 6)
            )
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 5,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            24,
            self.side,
            (0..36)
                .map(|i| (i, (self.top[i] as u64) << 8 | self.h[i] as u64))
                .chain(std::iter::once((100, self.second as u64))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn setup_counts() {
        let g = TreiTipuri::new();
        for p in 0..2 {
            for t in 1..=3u8 {
                let n = (0..36)
                    .filter(|&i| {
                        g.top[i] != 0 && TreiTipuri::own(g.top[i]) == p && g.top[i] & 3 == t
                    })
                    .count();
                assert_eq!(n, [3, 6, 9][t as usize - 1]);
            }
        }
        assert!(!g.legal_moves().is_empty());
    }
    #[test]
    fn capture_and_stack() {
        let mut g = TreiTipuri::new();
        g.top = [0; 36];
        g.h = [0; 36];
        g.top[0] = 1;
        g.h[0] = 2;
        g.top[3] = 4 | 1;
        g.h[3] = 1;
        g.top[30] = 1;
        g.h[30] = 1;
        g.top[35] = 4 | 2;
        g.h[35] = 1;
        g.top[7] = 2; // tip B pentru J1
        g.h[7] = 1;
        g.top[8] = 3;
        g.h[8] = 1; // tip C J1
        g.top[20] = 4 | 3;
        g.h[20] = 1;
        g.top[33] = 4 | 1;
        g.h[33] = 1;
        let m = g.legal_moves();
        assert!(m.contains(&(0, 3)));
        g.apply(&(0, 3));
        assert_eq!(g.h[3], 2);
        assert!(g.second);
        // alipire: 3 -> 30? nu aliniat; verificăm doar că fin e posibil
        assert!(g.legal_moves().contains(&(254, 254)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| TreiTipuri::new(), 700);
    }
}
