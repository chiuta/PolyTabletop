//! Tiddlywinks (Marea Britanie) — adaptare pe ture, fără îndemânare, cu informație perfectă.
//!
//! În jocul real jetoanele („winks”) sar cu un squidger; aici saltul se alege direct:
//! * Masa are 11×7 câmpuri; găleata („pot”) e în centru (f4). Fiecare jucător are 4 winks.
//! * O tură: alegi un wink propriu **liber** (nu e acoperit) și un câmp la cel mult 3 pătrate
//!   distanță (în orice direcție, `a1-c3`). Aterizarea pe un wink (propriu sau advers) îl acoperă
//!   („squop”): winkul acoperit nu se poate mișca până e descoperit. Aterizarea în găleată scoate winkul din joc (potted).
//! * Se termină când un jucător rămâne fără winks libere sau după 80 de mutări.
//!   Scor = 3 × winks potted + câte winks adverse acoperă direct un wink al tău.
//!   Mai multe puncte câștigă, egal = remiză.
//!
//! Adaptare originală inspirată de jocul tradițional; scorul real (tiddlies pe poziție) nu e copiat.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 11;
const H: i32 = 7;
const POT: (i32, i32) = (5, 3);
const PLY_CAP: u16 = 80;

fn cell(x: i32, y: i32) -> usize {
    (y * W + x) as usize
}
fn name(i: usize) -> String {
    format!(
        "{}{}",
        (b'a' + (i as i32 % W) as u8) as char,
        i as i32 / W + 1
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Tiddlywinks {
    /// teancuri de winks pe câmp; vârful = ultimul; valori 0 (J1) / 1 (J2)
    piles: Vec<Vec<u8>>,
    potted: [u8; 2],
    side: Player,
    ply: u16,
}

impl Default for Tiddlywinks {
    fn default() -> Self {
        Self::new()
    }
}

impl Tiddlywinks {
    pub fn new() -> Self {
        let mut piles = vec![Vec::new(); (W * H) as usize];
        for y in 1..=4 {
            piles[cell(0, y)].push(0);
            piles[cell(W - 1, y + 1)].push(1);
        }
        Tiddlywinks {
            piles,
            potted: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }

    pub fn points(&self) -> (i32, i32) {
        (self.points_of(0), self.points_of(1))
    }

    fn points_of(&self, p: usize) -> i32 {
        let mut s = 3 * self.potted[p] as i32;
        for pile in &self.piles {
            for w in pile.windows(2) {
                if w[1] as usize == p && w[0] as usize != p {
                    s += 1;
                }
            }
        }
        s
    }

    fn free_winks(&self, p: usize) -> Vec<usize> {
        (0..self.piles.len())
            .filter(|&i| self.piles[i].last() == Some(&(p as u8)))
            .collect()
    }
}

impl Game for Tiddlywinks {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        for i in self.free_winks(self.side.idx()) {
            let (x, y) = (i as i32 % W, i as i32 / W);
            for ty in (y - 3).max(0)..=(y + 3).min(H - 1) {
                for tx in (x - 3).max(0)..=(x + 3).min(W - 1) {
                    if (tx, ty) != (x, y) {
                        v.push(Mv {
                            from: i as u8,
                            to: cell(tx, ty) as u8,
                        });
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx() as u8;
        let w = self.piles[m.from as usize].pop().unwrap();
        if m.to as usize == cell(POT.0, POT.1) {
            self.potted[p as usize] += 1;
        } else {
            self.piles[m.to as usize].push(w);
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        let done =
            self.ply >= PLY_CAP || self.free_winks(0).is_empty() || self.free_winks(1).is_empty();
        if !done {
            return None;
        }
        let (a, b) = self.points();
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = self.points();
        let mut d = if p == Player::One { a - b } else { b - a } * 100;
        // apropierea de găleată a winks-urilor libere
        for q in [p, p.other()] {
            let sgn = if q == p { 1 } else { -1 };
            for i in self.free_winks(q.idx()) {
                let (x, y) = (i as i32 % W, i as i32 / W);
                let dist = (x - POT.0).abs().max((y - POT.1).abs());
                d += sgn * (6 - dist).max(0) * 4;
            }
        }
        d
    }
    fn move_score(&self, m: &Mv) -> i32 {
        if m.to as usize == cell(POT.0, POT.1) {
            100
        } else if self.piles[m.to as usize].last().is_some() {
            50
        } else {
            0
        }
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..H).rev() {
            s.push_str(&format!("{}  ", y + 1));
            for x in 0..W {
                let i = cell(x, y);
                let ch = if (x, y) == POT {
                    'O'
                } else {
                    match (self.piles[i].last(), self.piles[i].len()) {
                        (None, _) => '.',
                        (Some(0), 1) => 'X',
                        (Some(1), 1) => 'Y',
                        (Some(0), _) => 'x',
                        (_, _) => 'y',
                    }
                };
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g h i j k\nX/Y = winks J1/J2, minuscule = acoperă un alt wink, O = găleata (f4).\n");
        let (a, b) = self.points();
        s.push_str(&format!(
            "Scor: J1 {a} – {b} J2 (potted {}–{}). La mutare: Jucătorul {}.",
            self.potted[0],
            self.potted[1],
            self.side.idx() + 1
        ));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        format!("{}-{}", name(m.from as usize), name(m.to as usize))
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 250,
            max_depth: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn squop_and_pot() {
        let mut g = Tiddlywinks::new();
        g.piles = vec![Vec::new(); (W * H) as usize];
        g.piles[cell(2, 3)].push(0);
        g.piles[cell(3, 3)].push(1);
        GameObj::play(&mut g, "c4-d4").unwrap(); // acoperă winkul advers
        assert_eq!(g.points(), (1, 0));
        assert_eq!(g.piles[cell(3, 3)], vec![1, 0]);
        // J2 nu mai are winks libere: partida s-a terminat
        assert!(Game::outcome(&g).is_some());
        let mut h = Tiddlywinks::new();
        h.piles = vec![Vec::new(); (W * H) as usize];
        h.piles[cell(3, 3)].push(0);
        h.piles[cell(10, 5)].push(1);
        GameObj::play(&mut h, "d4-f4").unwrap();
        assert_eq!(h.potted[0], 1);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(Tiddlywinks::new()), 6, 200);
        ai_vs_random(|_| Box::new(Tiddlywinks::new()), 2, Player::One, 200);
        ai_vs_random(|_| Box::new(Tiddlywinks::new()), 3, Player::Two, 200);
    }
}
