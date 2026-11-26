//! Turnuri de Piatră — cavaleri pe castele în creștere (variantă originală pe tablă 6×6).
//!
//! Fiecare jucător are 4 cavaleri. Tura (o acțiune): `pune c3` (cavaler pe o celulă de înălțime
//! ≤ 1, fără cavaler, în afara celor ocupate), `mut a1>a2` (cavaler spre o celulă vecină ortogonal,
//! fără cavaler, cu cel mult o treaptă mai sus), `etaj c3` (adaugi o treaptă pe o celulă vecină
//! unui cavaler al tău, fără cavaler pe ea; max 4). Câștigă cine ajunge primul cu un cavaler pe
//! înălțimea 4; după 80 de acțiuni câștigă cine are suma înălțimilor cavalerilor mai mare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

const N: usize = 6;

#[derive(Clone)]
pub struct TurnuriPiatra {
    h: [[u8; N]; N],
    k: [[u8; N]; N], // 0 liber, 1 = J1, 2 = J2
    to_place: [u8; 2],
    side: Player,
    ply: u32,
    winner: Option<Player>,
}

fn nm(r: usize, c: usize) -> String {
    format!("{}{}", (b'a' + c as u8) as char, r + 1)
}
fn pm(s: &str) -> (usize, usize) {
    let b = s.as_bytes();
    ((b[1] - b'1') as usize, (b[0] - b'a') as usize)
}

impl TurnuriPiatra {
    pub fn new() -> Self {
        Self::with_seed(0)
    }
    pub fn with_seed(_s: u64) -> Self {
        TurnuriPiatra {
            h: [[0; N]; N],
            k: [[0; N]; N],
            to_place: [4, 4],
            side: Player::One,
            ply: 0,
            winner: None,
        }
    }
    fn nbrs(r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (dr, dc) in [
            (-1i32, 0i32),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ] {
            let (a, b) = (r as i32 + dr, c as i32 + dc);
            if (0..N as i32).contains(&a) && (0..N as i32).contains(&b) {
                v.push((a as usize, b as usize));
            }
        }
        v
    }
    fn sum(&self, p: u8) -> i32 {
        let mut s = 0;
        for r in 0..N {
            for c in 0..N {
                if self.k[r][c] == p {
                    s += self.h[r][c] as i32 * self.h[r][c] as i32 + 1;
                }
            }
        }
        s
    }
}

impl Default for TurnuriPiatra {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for TurnuriPiatra {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side) as u8 + 1;
        let mut v = Vec::new();
        for r in 0..N {
            for c in 0..N {
                if self.k[r][c] == 0 && self.to_place[pi(self.side)] > 0 && self.h[r][c] <= 1 {
                    v.push(format!("pune {}", nm(r, c)));
                }
                if self.k[r][c] == me {
                    for (a, b) in Self::nbrs(r, c) {
                        if self.k[a][b] == 0 && self.h[a][b] <= self.h[r][c] + 1 {
                            v.push(format!("mut {}>{}", nm(r, c), nm(a, b)));
                        }
                        if self.k[a][b] == 0 && self.h[a][b] < 4 {
                            let s = format!("etaj {}", nm(a, b));
                            if !v.contains(&s) {
                                v.push(s);
                            }
                        }
                    }
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
        if let Some(x) = m.strip_prefix("pune ") {
            let (r, c) = pm(x);
            self.k[r][c] = me as u8 + 1;
            self.to_place[me] -= 1;
        } else if let Some(x) = m.strip_prefix("etaj ") {
            let (r, c) = pm(x);
            self.h[r][c] += 1;
        } else if let Some(x) = m.strip_prefix("mut ") {
            let (a, b) = x.split_once('>').unwrap();
            let (ar, ac) = pm(a);
            let (br, bc) = pm(b);
            self.k[ar][ac] = 0;
            self.k[br][bc] = me as u8 + 1;
            if self.h[br][bc] >= 4 {
                self.winner = Some(self.side);
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.ply >= 80 {
            return Some(by_score(self.sum(1), self.sum(2)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p) as u8 + 1;
        self.sum(a) - self.sum(3 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from(
            "Turnuri de Piatră — înălțime; cavaler: A (J1) / B (J2)\n   a  b  c  d  e  f\n",
        );
        for r in 0..N {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..N {
                let ch = match self.k[r][c] {
                    1 => 'A',
                    2 => 'B',
                    _ => ' ',
                };
                s.push_str(&format!("{}{} ", self.h[r][c], ch));
            }
            s.push('\n');
        }
        s.push_str(&format!("De pus: J1 {}, J2 {} | acțiunea {}/80 | La mutare: {}\nMutări: pune c3 | mut a1>a2 | etaj c3 | pas\n", self.to_place[0], self.to_place[1], self.ply, pname(self.side)));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(TurnuriPiatra::with_seed, 200);
    }
    #[test]
    fn climbing_to_four_wins() {
        let mut g = TurnuriPiatra::new();
        g.h[0][1] = 4;
        g.h[0][0] = 3;
        g.k[0][0] = 1;
        g.step("mut a1>b1");
        assert_eq!(g.result(), Some(Outcome::Win(Player::One)));
    }
}
