//! Sferturi (variantă proprie): tablă 6×6 din patru sferturi 3×3. Pe rând pui o piesă pe o
//! celulă goală, apoi rotești obligatoriu un sfert (oricare) cu 90°, în ambele sensuri.
//! Cinci piese proprii în linie (verificate după rotire) câștigă; dacă rotirea face
//! linie pentru ambii jucători, e remiză. Tabla plină fără linie = remiză.
//! Mutări: b2 q2+ (pune pe b2, rotește sfertul 2 în sens orar), q3- = antiorar.
//! Sferturile: 1 = stânga-jos, 2 = dreapta-jos, 3 = stânga-sus, 4 = dreapta-sus.
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, run_through, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 6;

#[derive(Clone)]
pub struct Sferturi {
    b: [u8; 36],
    side: Player,
    res: Option<Outcome>,
    filled: u32,
}

impl Default for Sferturi {
    fn default() -> Self {
        Self::new()
    }
}

fn rotate(b: &mut [u8; 36], q: usize, cw: bool) {
    let (ox, oy) = ((q % 2) * 3, (q / 2) * 3);
    let old: Vec<u8> = (0..9).map(|k| b[(oy + k / 3) * N + ox + k % 3]).collect();
    for y in 0..3 {
        for x in 0..3 {
            // în sens orar (y în sus): (x,y) -> (y, 2-x)
            let (nx, ny) = if cw { (y, 2 - x) } else { (2 - y, x) };
            b[(oy + ny) * N + ox + nx] = old[y * 3 + x];
        }
    }
}

fn has5(b: &[u8; 36], who: u8) -> bool {
    for y in 0..N as i32 {
        for x in 0..N as i32 {
            if b[y as usize * N + x as usize] != who {
                continue;
            }
            for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                if run_through(6, 6, x, y, dx, dy, |a, c| {
                    b[c as usize * N + a as usize] == who
                }) >= 5
                {
                    return true;
                }
            }
        }
    }
    false
}

impl Sferturi {
    pub fn new() -> Self {
        Sferturi {
            b: [0; 36],
            side: Player::One,
            res: None,
            filled: 0,
        }
    }
}

impl Game for Sferturi {
    type Move = (u8, u8, bool);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, bool)> {
        if self.res.is_some() {
            return vec![];
        }
        let mut v = vec![];
        for i in 0..36 {
            if self.b[i] == 0 {
                for q in 0..4 {
                    v.push((i as u8, q as u8, true));
                    v.push((i as u8, q as u8, false));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, bool)) {
        let me = self.side.idx() as u8 + 1;
        self.b[m.0 as usize] = me;
        self.filled += 1;
        let after_place = has5(&self.b, me);
        rotate(&mut self.b, m.1 as usize, m.2);
        let (a, o) = (has5(&self.b, me), has5(&self.b, 3 - me));
        let _ = after_place;
        self.res = if a && o {
            Some(Outcome::Draw)
        } else if a {
            Some(Outcome::Win(self.side))
        } else if o {
            Some(Outcome::Win(self.side.other()))
        } else if self.filled == 36 {
            Some(Outcome::Draw)
        } else {
            None
        };
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        self.res
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        for y in 0..N as i32 {
            for x in 0..N as i32 {
                let v = self.b[y as usize * N + x as usize];
                if v == 0 {
                    continue;
                }
                for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                    let r = run_through(6, 6, x, y, dx, dy, |a, c| {
                        self.b[c as usize * N + a as usize] == v
                    });
                    let w = r * r * r;
                    s += if v == me { w } else { -w };
                }
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(N, N, |x, y| match self.b[y * N + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        });
        s.push_str("Sferturi: 1 stânga-jos, 2 dreapta-jos, 3 stânga-sus, 4 dreapta-sus\n");
        s
    }
    fn format_move(&self, m: &(u8, u8, bool)) -> String {
        format!(
            "{} q{}{}",
            sq(m.0 as usize % N, m.0 as usize / N),
            m.1 + 1,
            if m.2 { '+' } else { '-' }
        )
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 4,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            14,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn rotation_is_a_cycle() {
        let mut b = [0u8; 36];
        b[1] = 1;
        let o = b;
        for _ in 0..4 {
            rotate(&mut b, 0, true);
        }
        assert_eq!(b, o);
        rotate(&mut b, 0, true);
        rotate(&mut b, 0, false);
        assert_eq!(b, o);
    }
    #[test]
    fn five_wins() {
        let mut g = Sferturi::new();
        for x in 0..3 {
            g.b[3 * N + x] = 1;
        }
        g.b[3 * N + 4] = 1;
        g.filled = 5;
        // celula d4 liberă; sfertul 2 (jos-dreapta) nu atinge rândul 4
        let m = (3 * N + 3, 1, true);
        g.apply(&(m.0 as u8, m.1 as u8, m.2));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Sferturi::new(), 100);
    }
}
