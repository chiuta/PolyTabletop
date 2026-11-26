//! Perechi în piramidă (variantă proprie de duel pe un aranjament de plăcuțe): 36 de plăcuți
//! (9 modele × 4) sunt așezate pe trei niveluri: 6×4 jos, 4×2 la mijloc, 2×2 sus (nivelurile
//! superioare stau deasupra celor de dedesubt). O plăcuță e liberă dacă nu are nimic deasupra
//! și are liberă măcar una din laturile stânga/dreapta. Pe rând, fiecare jucător ia o pereche de
//! plăcuțe libere cu același model și o păstrează (1 punct). Când nu mai există nicio pereche
//! liberă, câștigă cine are mai multe perechi (egal = remiză).
//! Plăcuțe: nivel + coloană + rând, ex. `0a1`. Mutări: `0a1+0f4`.
#![allow(clippy::all)]

use crate::b1_util::hkey;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone)]
pub struct PerechiDuel {
    t: [[[u8; 4]; 6]; 3], // [nivel][x][y] model 1..9, 0 gol
    score: [u8; 2],
    side: Player,
}

fn pos(l: usize, x: usize, y: usize) -> String {
    format!("{}{}{}", l, (b'a' + x as u8) as char, y + 1)
}

impl Default for PerechiDuel {
    fn default() -> Self {
        Self::new()
    }
}

impl PerechiDuel {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cells = vec![];
        for x in 0..6 {
            for y in 0..4 {
                cells.push((0, x, y));
            }
        }
        for x in 1..5 {
            for y in 1..3 {
                cells.push((1, x, y));
            }
        }
        for x in 2..4 {
            for y in 1..3 {
                cells.push((2, x, y));
            }
        }
        let mut tiles: Vec<u8> = (0..36).map(|i| (i % 9) as u8 + 1).collect();
        for i in (1..tiles.len()).rev() {
            tiles.swap(i, rng.below(i + 1));
        }
        let mut t = [[[0u8; 4]; 6]; 3];
        for (k, (l, x, y)) in cells.into_iter().enumerate() {
            t[l][x][y] = tiles[k];
        }
        PerechiDuel {
            t,
            score: [0; 2],
            side: Player::One,
        }
    }
    fn free(&self, l: usize, x: usize, y: usize) -> bool {
        if self.t[l][x][y] == 0 {
            return false;
        }
        if l < 2 && self.t[l + 1][x][y] != 0 {
            return false;
        }
        let left = x == 0 || self.t[l][x - 1][y] == 0;
        let right = x == 5 || self.t[l][x + 1][y] == 0;
        left || right
    }
    fn free_list(&self) -> Vec<(usize, usize, usize)> {
        let mut v = vec![];
        for l in 0..3 {
            for x in 0..6 {
                for y in 0..4 {
                    if self.free(l, x, y) {
                        v.push((l, x, y));
                    }
                }
            }
        }
        v
    }
}

impl Game for PerechiDuel {
    type Move = ((u8, u8, u8), (u8, u8, u8));
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Self::Move> {
        let f = self.free_list();
        let mut v = vec![];
        for (i, &a) in f.iter().enumerate() {
            for &b in &f[i + 1..] {
                if self.t[a.0][a.1][a.2] == self.t[b.0][b.1][b.2] {
                    v.push((
                        (a.0 as u8, a.1 as u8, a.2 as u8),
                        (b.0 as u8, b.1 as u8, b.2 as u8),
                    ));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Self::Move) {
        self.t[m.0 .0 as usize][m.0 .1 as usize][m.0 .2 as usize] = 0;
        self.t[m.1 .0 as usize][m.1 .1 as usize][m.1 .2 as usize] = 0;
        self.score[self.side.idx()] += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if !self.legal_moves().is_empty() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        (self.score[me] as i32 - self.score[1 - me] as i32) * 100
            + if self.side == p {
                self.legal_moves().len() as i32
            } else {
                -(self.legal_moves().len() as i32)
            }
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for l in 0..3 {
            s.push_str(&format!("Nivel {l}:\n"));
            for y in (0..4).rev() {
                s.push_str(&format!("{} ", y + 1));
                for x in 0..6 {
                    let v = self.t[l][x][y];
                    s.push(if v == 0 {
                        '.'
                    } else if self.free(l, x, y) {
                        char::from(b'0' + v)
                    } else {
                        char::from(b'a' + v - 1)
                    });
                    s.push(' ');
                }
                s.push('\n');
            }
        }
        s.push_str("  a b c d e f   (cifră = plăcuță liberă, literă = blocată: a=1 … i=9)\n");
        s.push_str(&format!(
            "Perechi: J1 {}, J2 {}\n",
            self.score[0], self.score[1]
        ));
        s
    }
    fn format_move(&self, m: &Self::Move) -> String {
        format!(
            "{}+{}",
            pos(m.0 .0 as usize, m.0 .1 as usize, m.0 .2 as usize),
            pos(m.1 .0 as usize, m.1 .1 as usize, m.1 .2 as usize)
        )
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 6,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = hkey(30, self.side, std::iter::empty());
        for l in 0..3 {
            for x in 0..6 {
                for y in 0..4 {
                    if self.t[l][x][y] != 0 {
                        h ^= polytabletop_core::hash_mix(((l * 24 + x * 4 + y) as u64) | 1 << 40);
                    }
                }
            }
        }
        Some(
            h ^ polytabletop_core::hash_mix(
                self.score[0] as u64 * 100 + self.score[1] as u64 + 1 << 50,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn free_rules() {
        let g = PerechiDuel::with_seed(1);
        // plăcuța de jos de la mijloc, acoperită de nivelul 1, nu e liberă
        assert!(!g.free(0, 2, 1));
        // colțul din stânga jos e liber
        assert!(g.free(0, 0, 0));
        assert!(!g.free(0, 2, 1) || g.t[1][2][1] == 0);
        let total: usize = (0..3)
            .map(|l| g.t[l].iter().flatten().filter(|&&v| v != 0).count())
            .sum();
        assert_eq!(total, 36);
    }
    #[test]
    fn random_and_ai() {
        check(PerechiDuel::with_seed, 40);
    }
}
