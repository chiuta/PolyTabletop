//! Culori în șir (variantă proprie de turnuri colorate pe tablă 8×8): fiecare celulă are una din
//! opt culori (fiecare culoare apare o dată pe fiecare rând și coloană). Fiecare jucător are
//! opt turnuri, câte unul din fiecare culoare, pe rândul lui. J1 mută primul orice turn; după
//! aceea, culoarea celulei pe care a aterizat ultimul turn hotărăște ce turn trebuie să mute
//! adversarul (cel de culoarea aceea). Un turn merge doar înainte, pe verticală sau în diagonală,
//! oricât, fără să sară peste piese. Câștigă cine ajunge primul pe rândul adversarului. Dacă
//! turnul cerut e blocat, jucătorul trece, iar culoarea celulei lui hotărăște turnul
//! adversarului; dacă și acela e blocat, pierde cel care trece a doua oară la rând.
//! Culorile: R, O, Y, G, B, P, K, W. Mutări: `a1-a5`, `pas`.
#![allow(clippy::all)]

use crate::b1_util::{hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COL: [char; 8] = ['R', 'O', 'Y', 'G', 'B', 'P', 'K', 'W'];

fn color(x: usize, y: usize) -> usize {
    (x * 3 + y * 5) % 8
}

#[derive(Clone)]
pub struct CuloriPas {
    b: [u8; 64], // 0 gol, altfel (culoare+1)<<2 | jucător (1/2)
    side: Player,
    need: Option<usize>, // culoarea turnului ce trebuie mutat
    passes: u8,
    win: Option<Player>,
    ply: u32,
}

impl Default for CuloriPas {
    fn default() -> Self {
        Self::new()
    }
}

impl CuloriPas {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        for x in 0..8 {
            b[x] = ((color(x, 0) as u8 + 1) << 2) | 1;
            b[56 + x] = ((color(x, 7) as u8 + 1) << 2) | 2;
        }
        CuloriPas {
            b,
            side: Player::One,
            need: None,
            passes: 0,
            win: None,
            ply: 0,
        }
    }
    fn moves_of(&self, i: usize, p: usize, v: &mut Vec<(u8, u8)>) {
        let dy: i32 = if p == 0 { 1 } else { -1 };
        for dx in -1..=1 {
            let (mut x, mut y) = ((i % 8) as i32 + dx, (i / 8) as i32 + dy);
            while x >= 0 && y >= 0 && x < 8 && y < 8 && self.b[(y * 8 + x) as usize] == 0 {
                v.push((i as u8, (y * 8 + x) as u8));
                x += dx;
                y += dy;
            }
        }
    }
}

impl Game for CuloriPas {
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.win.is_some() || self.ply >= 400 {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        for i in 0..64 {
            if self.b[i] as usize & 3 != p + 1 {
                continue;
            }
            if let Some(c) = self.need {
                if (self.b[i] >> 2) as usize - 1 != c {
                    continue;
                }
            }
            self.moves_of(i, p, &mut v);
        }
        if v.is_empty() {
            v.push((255, 255));
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let p = self.side.idx();
        if m.0 == 255 {
            self.passes += 1;
            if self.passes >= 2 {
                self.win = Some(self.side.other());
            } else {
                // turnul blocat stă pe o celulă; culoarea ei hotărăște turnul adversarului
                let c = self.need.unwrap_or(0);
                let pos = (0..64).find(|&i| {
                    self.b[i] as usize & 3 == p + 1 && (self.b[i] >> 2) as usize - 1 == c
                });
                self.need = pos.map(|i| color(i % 8, i / 8));
            }
            self.ply += 1;
            self.side = self.side.other();
            return;
        }
        self.passes = 0;
        self.b[m.1 as usize] = self.b[m.0 as usize];
        self.b[m.0 as usize] = 0;
        let (x, y) = (m.1 as usize % 8, m.1 as usize / 8);
        if (p == 0 && y == 7) || (p == 1 && y == 0) {
            self.win = Some(self.side);
        }
        self.need = Some(color(x, y));
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.ply >= 400 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = 0;
        for i in 0..64 {
            let v = self.b[i] as usize & 3;
            if v == 0 {
                continue;
            }
            let adv = if v == 1 {
                (i / 8) as i32
            } else {
                7 - (i / 8) as i32
            };
            let w = adv * adv + adv * 3;
            s += if v == me + 1 { w } else { -w };
        }
        s
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..8).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..8 {
                let c = COL[color(x, y)];
                let v = self.b[y * 8 + x];
                let t = if v == 0 {
                    c
                } else {
                    COL[(v >> 2) as usize - 1]
                };
                match v & 3 {
                    1 => s.push_str(&format!("[{}] ", t)),
                    2 => s.push_str(&format!("({}) ", t)),
                    _ => s.push_str(&format!(" {}  ", c.to_ascii_lowercase())),
                }
            }
            s.push('\n');
        }
        s.push_str("   a   b   c   d   e   f   g   h\n[X] = turn J1, (X) = turn J2 (litera = culoarea turnului); litera mică = culoarea celulei goale.\n");
        match self.need {
            Some(c) => s.push_str(&format!("Turnul de mutat: culoarea {}.\n", COL[c])),
            None => s.push_str("Prima mutare: orice turn.\n"),
        }
        s
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        if m.0 == 255 {
            "pas".into()
        } else {
            format!(
                "{}-{}",
                sq(m.0 as usize % 8, m.0 as usize / 8),
                sq(m.1 as usize % 8, m.1 as usize / 8)
            )
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 8,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            25,
            self.side,
            self.b
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v as u64))
                .chain(std::iter::once((
                    200,
                    self.need.map_or(0, |c| c as u64 + 1) + 16 * self.passes as u64,
                ))),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn colors_are_latin() {
        for y in 0..8 {
            let mut r: Vec<usize> = (0..8).map(|x| color(x, y)).collect();
            r.sort();
            assert_eq!(r, (0..8).collect::<Vec<_>>());
            let mut c: Vec<usize> = (0..8).map(|x| color(y, x)).collect();
            c.sort();
            assert_eq!(c, (0..8).collect::<Vec<_>>());
        }
    }
    #[test]
    fn forced_color_and_goal() {
        let mut g = CuloriPas::new();
        g.apply(&(0, 8)); // a1-a2: aterizează pe culoarea lui a2
        let need = g.need.unwrap();
        for m in g.legal_moves() {
            assert_eq!((g.b[m.0 as usize] >> 2) as usize - 1, need);
        }
        let mut h = CuloriPas::new();
        h.b = [0; 64];
        h.b[48] = (1 << 2) | 1;
        h.b[63] = (2 << 2) | 2;
        h.apply(&(48, 56));
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| CuloriPas::new(), 500);
    }
}
