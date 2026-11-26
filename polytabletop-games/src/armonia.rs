//! Armonia florilor (variantă proprie de poziționare pe tablă 7×7): fiecare jucător are trei feluri
//! de piese a câte trei: trandafir (t, se mută 1), lotus (l, se mută până la 2) și orhidee
//! (o, se mută până la 3). Trandafirul bate lotusul, lotusul bate orhideea, orhideea bate
//! trandafirul; două piese de același fel se anulează. O tură: pui o piesă din mână pe un punct
//! gol din cele două rânduri ale tale sau lângă o piesă proprie, SAU muți o piesă proprie
//! ortogonal, fără să sară peste piese, pe un punct gol sau pe o piesă adversă pe care o bate
//! (o iei; la același fel dispar amândouă). Două piese proprii de feluri diferite formează o
//! „armonie” când sunt pe același rând sau pe aceeași coloană fără nicio piesă între ele.
//! Câștigi când un lanț de armonii îți leagă coloana a de coloana g (J1) sau rândul 1 de
//! rândul 7 (J2). După 160 de semi-mutări: remiză.
//! Mutări: `+t c3` (pune), `c3-c5` (mută).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 7;
const TY: [char; 3] = ['t', 'l', 'o'];

#[derive(Clone)]
pub struct Armonia {
    b: [u8; 49], // 0 gol, owner<<2 | (tip+1)
    hand: [[u8; 3]; 2],
    side: Player,
    ply: u32,
}

impl Default for Armonia {
    fn default() -> Self {
        Self::new()
    }
}

fn own(c: u8) -> usize {
    (c >> 2) as usize
}
fn ty(c: u8) -> usize {
    (c & 3) as usize - 1
}
/// 1 dacă a bate b, -1 invers, 0 egal
fn beats(a: usize, b: usize) -> i32 {
    if a == b {
        0
    } else if (a + 1) % 3 == b {
        1
    } else {
        -1
    }
}

impl Armonia {
    pub fn new() -> Self {
        Armonia {
            b: [0; 49],
            hand: [[3; 3]; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn range(t: usize) -> i32 {
        t as i32 + 1
    }
    fn harmonic(&self, i: usize, j: usize) -> bool {
        let (a, b) = (self.b[i], self.b[j]);
        if a == 0 || b == 0 || own(a) != own(b) || ty(a) == ty(b) {
            return false;
        }
        let (xi, yi, xj, yj) = (i as i32 % N, i as i32 / N, j as i32 % N, j as i32 / N);
        if xi == xj {
            let (lo, hi) = (yi.min(yj), yi.max(yj));
            ((lo + 1)..hi).all(|y| self.b[(y * N + xi) as usize] == 0)
        } else if yi == yj {
            let (lo, hi) = (xi.min(xj), xi.max(xj));
            ((lo + 1)..hi).all(|x| self.b[(yi * N + x) as usize] == 0)
        } else {
            false
        }
    }
    fn wins(&self, p: usize) -> bool {
        let edge = |i: usize| {
            if p == 0 {
                i as i32 % N == 0
            } else {
                i as i32 / N == 0
            }
        };
        let goal = |i: usize| {
            if p == 0 {
                i as i32 % N == N - 1
            } else {
                i as i32 / N == N - 1
            }
        };
        let mine: Vec<usize> = (0..49)
            .filter(|&i| self.b[i] != 0 && own(self.b[i]) == p)
            .collect();
        let mut seen = vec![false; 49];
        let mut st: Vec<usize> = mine.iter().copied().filter(|&i| edge(i)).collect();
        for &i in &st {
            seen[i] = true;
        }
        while let Some(i) = st.pop() {
            if goal(i) {
                return true;
            }
            for &j in &mine {
                if !seen[j] && self.harmonic(i, j) {
                    seen[j] = true;
                    st.push(j);
                }
            }
        }
        false
    }
    fn harmony_count(&self, p: usize) -> i32 {
        let mine: Vec<usize> = (0..49)
            .filter(|&i| self.b[i] != 0 && own(self.b[i]) == p)
            .collect();
        let mut n = 0;
        for (k, &i) in mine.iter().enumerate() {
            for &j in &mine[k + 1..] {
                if self.harmonic(i, j) {
                    n += 1;
                }
            }
        }
        n
    }
}

impl Game for Armonia {
    type Move = (u8, u8, u8); // (tip+1 pentru pus sau 0, de la/ținta, la)
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        for i in 0..49 {
            if self.b[i] == 0 {
                let y = i as i32 / N;
                let home = if p == 0 { y <= 1 } else { y >= N - 2 };
                let (x, y) = (i as i32 % N, y);
                let adj = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(dx, dy)| {
                    let (nx, ny) = (x + dx, y + dy);
                    nx >= 0
                        && ny >= 0
                        && nx < N
                        && ny < N
                        && self.b[(ny * N + nx) as usize] != 0
                        && own(self.b[(ny * N + nx) as usize]) == p
                });
                if home || adj {
                    for t in 0..3 {
                        if self.hand[p][t] > 0 {
                            v.push((t as u8 + 1, i as u8, 0));
                        }
                    }
                }
            } else if own(self.b[i]) == p {
                let t = ty(self.b[i]);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (mut x, mut y) = (i as i32 % N + dx, i as i32 / N + dy);
                    let mut k = 1;
                    while x >= 0 && y >= 0 && x < N && y < N && k <= Self::range(t) {
                        let j = (y * N + x) as usize;
                        if self.b[j] == 0 {
                            v.push((0, i as u8, j as u8));
                        } else {
                            if own(self.b[j]) != p && beats(t, ty(self.b[j])) >= 0 {
                                v.push((0, i as u8, j as u8));
                            }
                            break;
                        }
                        x += dx;
                        y += dy;
                        k += 1;
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let p = self.side.idx();
        if m.0 > 0 {
            let t = m.0 as usize - 1;
            self.hand[p][t] -= 1;
            self.b[m.1 as usize] = (p as u8) << 2 | m.0;
        } else {
            let (f, t) = (m.1 as usize, m.2 as usize);
            let a = self.b[f];
            let d = self.b[t];
            self.b[f] = 0;
            if d == 0 {
                self.b[t] = a;
            } else if ty(a) == ty(d) {
                self.b[t] = 0;
            } else {
                self.b[t] = a;
            }
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        // câștigă cel care tocmai a mutat (ultimul), apoi celălalt
        let last = self.side.other().idx();
        if self.wins(last) {
            return Some(Outcome::Win(self.side.other()));
        }
        if self.wins(1 - last) {
            return Some(Outcome::Win(self.side));
        }
        if self.ply >= 160 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.harmony_count(me) - self.harmony_count(1 - me)) * 25;
        for i in 0..49 {
            let c = self.b[i];
            if c == 0 {
                continue;
            }
            let o = own(c);
            let prog = if o == 0 { i as i32 % N } else { i as i32 / N };
            let adv = prog;
            let w = 10 + if adv == 0 || adv == N - 1 { 6 } else { 0 };
            s += if o == me { w } else { -w };
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(7, 7, |x, y| {
            let c = self.b[y * 7 + x];
            if c == 0 {
                '.'
            } else if own(c) == 0 {
                TY[ty(c)].to_ascii_uppercase()
            } else {
                TY[ty(c)]
            }
        });
        s.push_str("T/L/O = J1 (trandafir, lotus, orhidee), t/l/o = J2. J1 leagă coloana a de g; J2 rândul 1 de 7.\n");
        for p in 0..2 {
            s.push_str(&format!(
                "În mână J{}: t×{} l×{} o×{}\n",
                p + 1,
                self.hand[p][0],
                self.hand[p][1],
                self.hand[p][2]
            ));
        }
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        if m.0 > 0 {
            format!(
                "+{} {}",
                TY[m.0 as usize - 1],
                sq(m.1 as usize % 7, m.1 as usize / 7)
            )
        } else {
            format!(
                "{}-{}",
                sq(m.1 as usize % 7, m.1 as usize / 7),
                sq(m.2 as usize % 7, m.2 as usize / 7)
            )
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 3,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = hkey(
            29,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        );
        for p in 0..2 {
            for t in 0..3 {
                h ^= polytabletop_core::hash_mix(
                    60000 + (p * 3 + t) as u64 * 5 + self.hand[p][t] as u64,
                );
            }
        }
        Some(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn cycle_and_win() {
        assert_eq!(beats(0, 1), 1);
        assert_eq!(beats(1, 2), 1);
        assert_eq!(beats(2, 0), 1);
        assert_eq!(beats(1, 0), -1);
        // lanț a3 (t) - c3 (l) ... g3
        let mut g = Armonia::new();
        let put = |g: &mut Armonia, x: usize, t: u8| g.b[3 * 7 + x] = t; // owner 0
        put(&mut g, 0, 1);
        put(&mut g, 2, 2);
        put(&mut g, 4, 3);
        assert!(!g.wins(0));
        put(&mut g, 6, 1);
        assert!(g.wins(0));
        // piesă între ele rupe armonia
        g.b[3 * 7 + 3] = 4 | 1;
        assert!(!g.wins(0));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Armonia::new(), 300);
    }
}
