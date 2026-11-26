//! Piese de Mozaic — puzzle în doi (variantă originală, inspirată de jocurile de plăcuțe și forme).
//!
//! Tablă 5×6 cu 6 găuri aleatoare (24 de celule libere). Rezerva comună are 6 piese de câte
//! 4 celule (I, O, T, S, L, J). Pe rând pui oricare piesă nefolosită, în orice orientare,
//! pe celule libere: `pune T@r,c,o` (r rând 1–5, c coloană 1–6 = celula de sus-stânga a
//! piesei, o = orientarea). Cine nu mai poate pune nicio piesă pierde.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [char; 6] = ['I', 'O', 'T', 'S', 'L', 'J'];

fn base(i: usize) -> Vec<(i8, i8)> {
    match i {
        0 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        1 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        2 => vec![(0, 0), (0, 1), (0, 2), (1, 1)],
        3 => vec![(0, 1), (0, 2), (1, 0), (1, 1)],
        4 => vec![(0, 0), (1, 0), (2, 0), (2, 1)],
        _ => vec![(0, 1), (1, 1), (2, 1), (2, 0)],
    }
}

#[derive(Clone)]
pub struct PietrePuzzle {
    cell: [[u8; 6]; 5], // 0 liber, 255 gaură, 1+ = piesa
    used: [bool; 6],
    side: Player,
    stuck: Option<Player>,
    shapes: Vec<Vec<Vec<(i8, i8)>>>,
}

impl PietrePuzzle {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cell = [[0u8; 6]; 5];
        let mut h = 0;
        while h < 6 {
            let (r, c) = (rng.below(5), rng.below(6));
            if cell[r][c] == 0 {
                cell[r][c] = 255;
                h += 1;
            }
        }
        let shapes = (0..6).map(|i| orientations(&base(i), true)).collect();
        let mut g = PietrePuzzle {
            cell,
            used: [false; 6],
            side: Player::One,
            stuck: None,
            shapes,
        };
        g.check_stuck();
        g
    }
    fn placements(&self) -> Vec<(usize, usize, i8, i8)> {
        let mut v = Vec::new();
        for i in 0..6 {
            if self.used[i] {
                continue;
            }
            for (o, sh) in self.shapes[i].iter().enumerate() {
                for r in 0..5i8 {
                    for c in 0..6i8 {
                        if sh.iter().all(|&(dr, dc)| {
                            let (rr, cc) = (r + dr, c + dc);
                            rr < 5 && cc < 6 && self.cell[rr as usize][cc as usize] == 0
                        }) {
                            v.push((i, o, r, c));
                        }
                    }
                }
            }
        }
        v
    }
    fn check_stuck(&mut self) {
        if self.placements().is_empty() {
            self.stuck = Some(self.side);
        }
    }
}

impl Default for PietrePuzzle {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for PietrePuzzle {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        self.placements()
            .iter()
            .map(|&(i, o, r, c)| format!("pune {}@{},{},{}", NAMES[i], r + 1, c + 1, o))
            .collect()
    }
    fn step(&mut self, m: &str) {
        let r = m.strip_prefix("pune ").unwrap();
        let (n, rest) = r.split_once('@').unwrap();
        let i = NAMES
            .iter()
            .position(|&c| c.to_string().eq_ignore_ascii_case(n))
            .unwrap();
        let v: Vec<i8> = rest.split(',').map(|x| x.parse().unwrap()).collect();
        for &(dr, dc) in &self.shapes[i][v[2] as usize] {
            self.cell[(v[0] - 1 + dr) as usize][(v[1] - 1 + dc) as usize] = i as u8 + 1;
        }
        self.used[i] = true;
        self.side = self.side.other();
        self.check_stuck();
    }
    fn result(&self) -> Option<Outcome> {
        self.stuck.map(|p| Outcome::Win(p.other()))
    }
    fn eval(&self, p: Player) -> i32 {
        let n = self.placements().len() as i32;
        if self.side == p {
            n.min(40)
        } else {
            -(n.min(40))
        }
    }
    fn show(&self) -> String {
        let mut s =
            String::from("Piese de Mozaic — # = gaură, litera = piesă pusă\n   1 2 3 4 5 6\n");
        for r in 0..5 {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..6 {
                s.push(match self.cell[r][c] {
                    0 => '.',
                    255 => '#',
                    k => NAMES[k as usize - 1],
                });
                s.push(' ');
            }
            s.push('\n');
        }
        let rest: String = (0..6)
            .filter(|&i| !self.used[i])
            .map(|i| NAMES[i].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        s.push_str(&format!(
            "Rămase: {} | La mutare: {} (pune P@r,c,o)\n",
            rest,
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
        crate::brand_util::check(PietrePuzzle::with_seed, 100);
    }
}
