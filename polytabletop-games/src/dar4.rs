//! Piesa dăruită (variantă proprie): 16 piese distincte (mărime, umplere, formă, culoare — patru
//! însușiri binare), tablă 4×4. În fiecare tură pui pe tablă piesa pe care ți-a dat-o adversarul
//! și apoi alegi tu piesa pe care i-o dai lui. Câștigi dacă piesa pusă completează un rând,
//! o coloană, o diagonală sau un pătrat 2×2 de patru piese cu cel puțin o însușire comună
//! (varianta cu pătrate). Tabla plină fără câștig = remiză. Mutări: b3 (punere) combinate cu
//! darul: `b3 dă 9` (piesele sunt notate 0–f, în hexazecimal); prima tură: `dă 5`.
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone)]
pub struct Dar4 {
    b: [i8; 16], // -1 gol, altfel piesa 0..15
    avail: u16,  // piese încă nedate
    given: Option<u8>,
    side: Player,
    win: Option<Player>,
    filled: u8,
}

impl Default for Dar4 {
    fn default() -> Self {
        Self::new()
    }
}

const LINES: [[usize; 4]; 10] = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11],
    [12, 13, 14, 15],
    [0, 4, 8, 12],
    [1, 5, 9, 13],
    [2, 6, 10, 14],
    [3, 7, 11, 15],
    [0, 5, 10, 15],
    [3, 6, 9, 12],
    // restul pătratelor 2×2 sunt generate separat
];

fn squares() -> Vec<[usize; 4]> {
    let mut v = vec![];
    for y in 0..3 {
        for x in 0..3 {
            let i = y * 4 + x;
            v.push([i, i + 1, i + 4, i + 5]);
        }
    }
    v
}

impl Dar4 {
    pub fn new() -> Self {
        Dar4 {
            b: [-1; 16],
            avail: 0xFFFF,
            given: None,
            side: Player::One,
            win: None,
            filled: 0,
        }
    }
    fn all_lines() -> Vec<[usize; 4]> {
        let mut v: Vec<[usize; 4]> = LINES.to_vec();
        v.extend(squares());
        v
    }
    fn wins(&self, line: &[usize; 4]) -> bool {
        let ps: Vec<i8> = line.iter().map(|&i| self.b[i]).collect();
        if ps.iter().any(|&p| p < 0) {
            return false;
        }
        let and = ps.iter().fold(15i8, |a, &p| a & p);
        let nor = ps.iter().fold(15i8, |a, &p| a & !p) & 15;
        and != 0 || nor != 0
    }
}

impl Game for Dar4 {
    // (celula 0..15 sau 255 la prima tură, piesa dată sau 255 la ultima punere)
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.win.is_some() || (self.filled == 16) {
            return vec![];
        }
        let gifts: Vec<u8> = (0..16).filter(|&p| self.avail >> p & 1 == 1).collect();
        match self.given {
            None => gifts.iter().map(|&g| (255, g)).collect(),
            Some(_) => {
                let mut v = vec![];
                for c in 0..16u8 {
                    if self.b[c as usize] >= 0 {
                        continue;
                    }
                    if self.filled == 15 || gifts.is_empty() {
                        v.push((c, 255));
                    } else {
                        for &g in &gifts {
                            v.push((c, g));
                        }
                    }
                }
                v
            }
        }
    }
    fn apply(&mut self, m: &(u8, u8)) {
        if m.0 != 255 {
            let p = self.given.unwrap();
            self.b[m.0 as usize] = p as i8;
            self.filled += 1;
            if Self::all_lines()
                .iter()
                .any(|l| l.contains(&(m.0 as usize)) && self.wins(l))
            {
                self.win = Some(self.side);
            }
        }
        if m.1 != 255 {
            self.avail &= !(1 << m.1);
            self.given = Some(m.1);
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.filled == 16 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        // aproape de câștig pentru cel care primește piesa; euristică ușoară
        let mut danger = 0;
        if let Some(g) = self.given {
            for l in Self::all_lines() {
                let ps: Vec<i8> = l.iter().map(|&i| self.b[i]).collect();
                if ps.iter().filter(|&&q| q < 0).count() == 1 {
                    let mut c = self.clone();
                    let e = l.iter().position(|&i| c.b[i] < 0).unwrap();
                    c.b[l[e]] = g as i8;
                    if c.wins(&l) {
                        danger += 1;
                    }
                }
            }
        }
        let s = (danger.min(3) * 10) as i32;
        if self.side == p {
            s
        } else {
            -s
        }
    }
    fn render(&self) -> String {
        let mut s = grid(4, 4, |x, y| {
            let v = self.b[y * 4 + x];
            if v < 0 {
                '.'
            } else {
                char::from_digit(v as u32, 16).unwrap()
            }
        });
        s.push_str("Disponibile: ");
        for p in 0..16 {
            if self.avail >> p & 1 == 1 {
                s.push(char::from_digit(p, 16).unwrap());
            }
        }
        match self.given {
            Some(g) => s.push_str(&format!(
                "\nPiesa de pus: {}\n",
                char::from_digit(g as u32, 16).unwrap()
            )),
            None => s.push_str("\nDă prima piesă (da <piesa>).\n"),
        }
        s.push_str("Însușiri: bit0 mărime, bit1 umplere, bit2 formă, bit3 culoare.\n");
        s
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        let g = if m.1 == 255 {
            String::new()
        } else {
            format!("da {}", char::from_digit(m.1 as u32, 16).unwrap())
        };
        if m.0 == 255 {
            g
        } else {
            format!("{} {}", sq(m.0 as usize % 4, m.0 as usize / 4), g)
                .trim()
                .to_string()
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
            16,
            self.side,
            self.b
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, (v + 2) as u64))
                .chain([
                    (100, self.avail as u64),
                    (101, self.given.map_or(0, |g| g as u64 + 1)),
                ]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn opening_and_win() {
        let g = Dar4::new();
        assert_eq!(g.legal_moves().len(), 16);
        let mut h = Dar4::new();
        h.b[0] = 1;
        h.b[1] = 3;
        h.b[2] = 5;
        h.avail = 0xFFFF & !(1 << 1 | 1 << 3 | 1 << 5 | 1 << 7);
        h.filled = 3;
        h.given = Some(7);
        h.apply(&(3, 9));
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn square_wins() {
        let mut h = Dar4::new();
        h.b[5] = 1;
        h.b[6] = 3;
        h.b[9] = 5;
        h.filled = 3;
        h.given = Some(7);
        h.avail = 0xFFFF & !(1 << 1 | 1 << 3 | 1 << 5 | 1 << 7);
        h.apply(&(10, 0));
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Dar4::new(), 40);
    }
}
