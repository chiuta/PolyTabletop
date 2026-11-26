//! Hochei pe masă (variantă proprie, pe ture, fără fizică în timp real): masa are 7×11 celule,
//! porțile (3 celule late) sunt la mijlocul capetelor. Fiecare jucător are o paletă în jumătatea
//! lui (J1 jos, J2 sus). O tură: muți paleta cu un pas (în 8 direcții) sau o lași pe loc;
//! dacă după mutare pucul e în celula paletei sau într-una vecină, îl lovești: pucul primește
//! viteza de la palete spre puc (dublată) plus mișcarea paletei, cel mult 2 pe fiecare axă.
//! Apoi pucul parcurge pe rând atâtea celule cât e viteza cea mai mare, ricoșează de pereți
//! și de paleta adversă; trece de linia de capăt pe poartă = gol. Viteza scade cu 1 pe axă
//! în fiecare tură. Primul la 3 goluri câștigă; după 300 de semi-mutări câștigă cine are mai
//! multe goluri. Mutări: n, s, e, v, ne, nv, se, sv (paleta) sau `stau`.
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 7;
const H: i32 = 11;
const DIRS: [(&str, i32, i32); 9] = [
    ("stau", 0, 0),
    ("n", 0, 1),
    ("s", 0, -1),
    ("e", 1, 0),
    ("v", -1, 0),
    ("ne", 1, 1),
    ("nv", -1, 1),
    ("se", 1, -1),
    ("sv", -1, -1),
];

#[derive(Clone)]
pub struct HocheiMasa {
    pad: [(i32, i32); 2],
    puck: (i32, i32),
    vel: (i32, i32),
    score: [u8; 2],
    side: Player,
    ply: u32,
}

impl Default for HocheiMasa {
    fn default() -> Self {
        Self::new()
    }
}

impl HocheiMasa {
    pub fn new() -> Self {
        HocheiMasa {
            pad: [(3, 1), (3, 9)],
            puck: (3, 5),
            vel: (0, 0),
            score: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn reset(&mut self) {
        self.pad = [(3, 1), (3, 9)];
        self.puck = (3, 5);
        self.vel = (0, 0);
    }
    fn goal_x(x: i32) -> bool {
        (2..=4).contains(&x)
    }
    /// Mută pucul după viteza curentă; întoarce Some(jucătorul care a dat gol).
    fn glide(&mut self) -> Option<usize> {
        let n = self.vel.0.abs().max(self.vel.1.abs());
        for k in 0..n {
            let sx = if k < self.vel.0.abs() {
                self.vel.0.signum()
            } else {
                0
            };
            let sy = if k < self.vel.1.abs() {
                self.vel.1.signum()
            } else {
                0
            };
            let (mut nx, mut ny) = (self.puck.0 + sx, self.puck.1 + sy);
            if ny < 0 || ny >= H {
                if Self::goal_x(self.puck.0) {
                    return Some(if ny < 0 { 1 } else { 0 });
                }
                self.vel.1 = -self.vel.1;
                ny = self.puck.1;
            }
            if nx < 0 || nx >= W {
                self.vel.0 = -self.vel.0;
                nx = self.puck.0;
            }
            if self.pad.contains(&(nx, ny)) {
                self.vel = (-self.vel.0, -self.vel.1);
                continue;
            }
            self.puck = (nx, ny);
        }
        None
    }
}

impl Game for HocheiMasa {
    type Move = u8;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<u8> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let (lo, hi) = if p == 0 { (0, 4) } else { (6, 10) };
        let mut v = vec![];
        for (k, &(_, dx, dy)) in DIRS.iter().enumerate() {
            let (nx, ny) = (self.pad[p].0 + dx, self.pad[p].1 + dy);
            if nx < 0 || nx >= W || ny < lo || ny > hi || (nx, ny) == self.pad[1 - p] {
                continue;
            }
            v.push(k as u8);
        }
        v
    }
    fn apply(&mut self, m: &u8) {
        let p = self.side.idx();
        let (_, dx, dy) = DIRS[*m as usize];
        self.pad[p] = (self.pad[p].0 + dx, self.pad[p].1 + dy);
        let (px, py) = self.pad[p];
        let (ux, uy) = (self.puck.0 - px, self.puck.1 - py);
        if ux.abs() <= 1 && uy.abs() <= 1 {
            let fwd = if p == 0 { 1 } else { -1 };
            let (hx, hy) = if (ux, uy) == (0, 0) {
                (0, fwd)
            } else {
                (ux, uy)
            };
            self.vel = ((hx * 2 + dx).clamp(-2, 2), (hy * 2 + dy).clamp(-2, 2));
            if self.vel == (0, 0) {
                self.vel = (0, fwd);
            }
        }
        if let Some(scorer) = self.glide() {
            self.score[scorer] += 1;
            self.reset();
        } else {
            self.vel = (
                self.vel.0 - self.vel.0.signum(),
                self.vel.1 - self.vel.1.signum(),
            );
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.score[0] >= 3 {
            return Some(Outcome::Win(Player::One));
        }
        if self.score[1] >= 3 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.ply >= 300 {
            return Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.score[me] as i32 - self.score[1 - me] as i32) * 1000;
        // puc spre poarta adversă = bine
        let adv = if me == 0 {
            self.puck.1
        } else {
            H - 1 - self.puck.1
        };
        s += adv * 6;
        let dist = (self.pad[me].0 - self.puck.0).abs() + (self.pad[me].1 - self.puck.1).abs();
        s -= dist;
        let vdir = if me == 0 { self.vel.1 } else { -self.vel.1 };
        s + vdir * 15
    }
    fn render(&self) -> String {
        let mut s = grid(W as usize, H as usize, |x, y| {
            let c = (x as i32, y as i32);
            if c == self.puck {
                '*'
            } else if c == self.pad[0] {
                'A'
            } else if c == self.pad[1] {
                'B'
            } else if (y == 0 || y == H as usize - 1) && Self::goal_x(c.0) {
                '='
            } else {
                '.'
            }
        });
        s.push_str(&format!(
            "A = J1 (jos), B = J2 (sus), * = puc {}, viteza puc ({:+},{:+}); scor {}–{}\n",
            sq(self.puck.0 as usize, self.puck.1 as usize),
            self.vel.0,
            self.vel.1,
            self.score[0],
            self.score[1]
        ));
        s
    }
    fn format_move(&self, m: &u8) -> String {
        DIRS[*m as usize].0.to_string()
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 8,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = hkey(27, self.side, std::iter::empty());
        for (i, v) in [
            self.pad[0].0,
            self.pad[0].1,
            self.pad[1].0,
            self.pad[1].1,
            self.puck.0,
            self.puck.1,
            self.vel.0 + 3,
            self.vel.1 + 3,
            self.score[0] as i32,
            self.score[1] as i32,
        ]
        .iter()
        .enumerate()
        {
            h ^= polytabletop_core::hash_mix(
                (i as u64) << 8 | (*v as u64 & 0xff) + 1000 * (i as u64 + 1),
            );
        }
        Some(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn hit_and_goal() {
        let mut g = HocheiMasa::new();
        g.pad[0] = (3, 4);
        g.puck = (3, 5);
        g.apply(&0); // stau: puc adiacent, este lovit în sus (0,1)*2
        assert!(g.puck.1 > 5);
        // gol direct
        let mut h = HocheiMasa::new();
        h.puck = (3, 9);
        h.vel = (0, 2);
        h.pad[1] = (0, 9);
        h.pad[0] = (0, 1);
        h.side = Player::One;
        h.apply(&0);
        assert_eq!(h.score[0], 1);
        assert_eq!(h.puck, (3, 5));
    }
    #[test]
    fn random_and_ai() {
        check(|_| HocheiMasa::new(), 400);
    }
}
