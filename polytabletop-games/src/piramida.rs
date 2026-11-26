//! Piramida (variantă proprie de bile pe o piramidă pe patru niveluri): baza are 4×4 locuri, apoi
//! 3×3, 2×2 și un loc în vârf. Fiecare jucător are 15 bile. Un loc de pe un nivel superior se
//! poate folosi doar când cele patru locuri de sub el sunt ocupate. O tură: pui o bilă din
//! rezervă pe un loc disponibil, SAU muți o bilă proprie liberă (fără bile deasupra) pe un loc
//! disponibil mai sus (nu pe unul pe care ea însăși îl susține). Dacă bila pusă sau mutată
//! închide un pătrat 2×2 de bile proprii pe același nivel, poți ridica înapoi în rezervă una
//! sau două dintre bilele tale libere (sau niciuna). Pierzi dacă la rândul tău nu poți nici
//! pune, nici muta o bilă. Remiză după 250 de semi-mutări.
//! Locuri: nivelul 0 = a1–d4, nivelul 1 = e1–g3, nivelul 2 = h1–i2, vârf = j1.
//! Mutări: `b2` (pune), `b2^f1` (urcă bila de pe b2 pe f1), cu sufix `-a1` sau `-a1-b2` pentru
//! bilele ridicate: `b2-a1-c3`.
#![allow(clippy::all)]

use crate::b1_util::hkey;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SIZES: [usize; 4] = [4, 3, 2, 1];
const OFF: [usize; 5] = [0, 16, 25, 29, 30];

fn lvl(i: usize) -> usize {
    (0..4).find(|&l| i < OFF[l + 1]).unwrap()
}
fn xy(i: usize) -> (usize, usize) {
    let l = lvl(i);
    ((i - OFF[l]) % SIZES[l], (i - OFF[l]) / SIZES[l])
}
fn at(l: usize, x: usize, y: usize) -> usize {
    OFF[l] + y * SIZES[l] + x
}
fn name(i: usize) -> String {
    let l = lvl(i);
    let (x, y) = xy(i);
    let base = [b'a', b'e', b'h', b'j'][l];
    format!("{}{}", (base + x as u8) as char, y + 1)
}

#[derive(Clone)]
pub struct Piramida {
    b: [u8; 30],
    res: [u8; 2],
    side: Player,
    ply: u32,
}

impl Default for Piramida {
    fn default() -> Self {
        Self::new()
    }
}

impl Piramida {
    pub fn new() -> Self {
        Piramida {
            b: [0; 30],
            res: [15, 15],
            side: Player::One,
            ply: 0,
        }
    }
    fn supports(i: usize) -> Vec<usize> {
        let l = lvl(i);
        if l == 0 {
            return vec![];
        }
        let (x, y) = xy(i);
        vec![
            at(l - 1, x, y),
            at(l - 1, x + 1, y),
            at(l - 1, x, y + 1),
            at(l - 1, x + 1, y + 1),
        ]
    }
    fn available(&self, i: usize) -> bool {
        self.b[i] == 0 && Self::supports(i).iter().all(|&s| self.b[s] != 0)
    }
    fn covered(&self, i: usize) -> bool {
        let l = lvl(i);
        if l == 3 {
            return false;
        }
        let (x, y) = xy(i);
        for dy in 0..2usize {
            for dx in 0..2usize {
                if x >= dx && y >= dy {
                    let (ux, uy) = (x - dx, y - dy);
                    if ux < SIZES[l + 1] && uy < SIZES[l + 1] && self.b[at(l + 1, ux, uy)] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn free_own(&self, who: u8) -> Vec<usize> {
        (0..30)
            .filter(|&i| self.b[i] == who && !self.covered(i))
            .collect()
    }
    fn square_with(&self, i: usize) -> bool {
        let l = lvl(i);
        if l > 2 {
            return false;
        }
        let (x, y) = xy(i);
        let me = self.b[i];
        for sy in y.saturating_sub(1)..=y.min(SIZES[l] - 2) {
            for sx in x.saturating_sub(1)..=x.min(SIZES[l] - 2) {
                if [(0, 0), (1, 0), (0, 1), (1, 1)]
                    .iter()
                    .all(|&(dx, dy)| self.b[at(l, sx + dx, sy + dy)] == me)
                {
                    return true;
                }
            }
        }
        false
    }
}

impl Game for Piramida {
    // (de la sau 255, la, scoase [a,b] cu 255 = nimic)
    type Move = (u8, u8, u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8, u8)> {
        if self.ply >= 250 {
            return vec![];
        }
        let p = self.side.idx();
        let me = p as u8 + 1;
        let mut base: Vec<(u8, u8)> = vec![];
        if self.res[p] > 0 {
            for i in 0..30 {
                if self.available(i) {
                    base.push((255, i as u8));
                }
            }
        }
        for f in self.free_own(me) {
            for t in 0..30 {
                if lvl(t) > lvl(f) && self.available(t) && !Self::supports(t).contains(&f) {
                    base.push((f as u8, t as u8));
                }
            }
        }
        let mut v = vec![];
        for (f, t) in base {
            let mut c = self.clone();
            if f != 255 {
                c.b[f as usize] = 0;
            } else {
                c.res[p] -= 1;
            }
            c.b[t as usize] = me;
            v.push((f, t, 255, 255));
            if c.square_with(t as usize) {
                let fr = c.free_own(me);
                for (k, &a) in fr.iter().enumerate() {
                    v.push((f, t, a as u8, 255));
                    for &b in &fr[k + 1..] {
                        v.push((f, t, a as u8, b as u8));
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8, u8)) {
        let p = self.side.idx();
        let me = p as u8 + 1;
        if m.0 != 255 {
            self.b[m.0 as usize] = 0;
        } else {
            self.res[p] -= 1;
        }
        self.b[m.1 as usize] = me;
        for r in [m.2, m.3] {
            if r != 255 {
                self.b[r as usize] = 0;
                self.res[p] += 1;
            }
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.ply >= 250 {
            return Some(Outcome::Draw);
        }
        if self.legal_moves().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.res[me] as i32 - self.res[1 - me] as i32) * 25;
        for i in 0..30 {
            let v = self.b[i] as usize;
            if v != 0 {
                let w = 4 + lvl(i) as i32 * 3;
                s += if v == me + 1 { w } else { -w };
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for l in (0..4).rev() {
            s.push_str(&format!("nivel {}: ", l));
            for y in (0..SIZES[l]).rev() {
                s.push_str(&format!(
                    "[{}] ",
                    (0..SIZES[l])
                        .map(|x| {
                            let i = at(l, x, y);
                            match self.b[i] {
                                1 => 'X',
                                2 => 'O',
                                _ => {
                                    if self.available(i) {
                                        '_'
                                    } else {
                                        '.'
                                    }
                                }
                            }
                        })
                        .collect::<String>()
                ));
            }
            s.push('\n');
        }
        s.push_str("(rândul de sus al fiecărui nivel = ultimul rând; _ = loc disponibil, . = încă nesprijinit)\n");
        s.push_str(&format!(
            "Rezervă: J1 {}, J2 {}\n",
            self.res[0], self.res[1]
        ));
        s
    }
    fn format_move(&self, m: &(u8, u8, u8, u8)) -> String {
        let mut s = if m.0 == 255 {
            name(m.1 as usize)
        } else {
            format!("{}^{}", name(m.0 as usize), name(m.1 as usize))
        };
        for r in [m.2, m.3] {
            if r != 255 {
                s.push('-');
                s.push_str(&name(r as usize));
            }
        }
        s
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 4,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            26,
            self.side,
            self.b
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v as u64))
                .chain(std::iter::once((
                    100,
                    self.res[0] as u64 * 32 + self.res[1] as u64,
                ))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn availability_and_square() {
        let g = Piramida::new();
        assert_eq!(g.legal_moves().len(), 16);
        let mut h = Piramida::new();
        h.b[at(0, 0, 0)] = 1;
        h.b[at(0, 1, 0)] = 1;
        h.b[at(0, 0, 1)] = 1;
        let mv = h.legal_moves();
        // pe b2 (1,1) se închide pătratul: există opțiuni cu ridicare
        let b2 = at(0, 1, 1) as u8;
        assert!(mv.iter().any(|m| m.1 == b2 && m.2 != 255));
        assert!(!h.available(at(1, 0, 0)));
    }
    #[test]
    fn climbing_not_on_own_support() {
        let mut h = Piramida::new();
        for i in 0..4 {
            h.b[i] = 1;
        }
        for i in 4..8 {
            h.b[i] = 2;
        }
        // locul e1 (nivel 1, 0,0) e susținut de a1,b1,a2,b2 (indici 0,1,4,5)
        let mv = h.legal_moves();
        assert!(mv.iter().all(|m| !(m.0 == 0 && m.1 == at(1, 0, 0) as u8)));
        assert!(
            mv.iter().any(|m| m.0 == 3 && m.1 == at(1, 0, 0) as u8) || !h.available(at(1, 0, 0))
        );
    }
    #[test]
    fn random_and_ai() {
        check(|_| Piramida::new(), 400);
    }
}
