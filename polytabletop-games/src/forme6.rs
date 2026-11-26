//! Forme și culori (variantă proprie de aliniere de plăcuțe, 2 jucători, informație ascunsă):
//! 108 de plăcuți (6 culori × 6 forme × 3 exemplare). Fiecare jucător ține 6 plăcuți ascunse;
//! punga e ascunsă. O tură: pui o plăcuță lipită de cele de pe masă (prima plăcuță a jocului
//! oriunde) astfel încât fiecare rând și coloană care o conține (de cel mult 6 plăcuțe) să aibă
//! fie aceeași culoare, fie aceeași formă, fără repetarea aceleiași plăcuțe în linie. Punctele:
//! pentru fiecare linie de cel puțin 2 plăcuțe care conține plăcuța pusă primești câte puncte
//! are lungimea ei (+6 bonus pentru o linie de 6). Apoi tragi din pungă. Sau `schimb R3`:
//! dai o plăcuță în pungă și tragi alta (fără punctaj). Jocul se termină când punga e goală și
//! cineva rămâne fără plăcuțe (+6 bonus) sau când amândoi trec. Câștigă scorul mai mare.
//! Culori: R O Y G B P; forme 1–6. Mutări: `R3@2,-1` (x,y față de prima plăcuță), `schimb R3`,
//! `pas`.
#![allow(clippy::all)]

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::BTreeMap;

const COL: [char; 6] = ['R', 'O', 'Y', 'G', 'B', 'P'];

type Tile = (u8, u8); // culoare, formă (0..6)

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Place(Tile, i8, i8),
    Swap(Tile),
    Pass,
}

#[derive(Clone)]
pub struct Forme6 {
    board: BTreeMap<(i8, i8), Tile>,
    hand: [Vec<Tile>; 2],
    bag: Vec<Tile>,
    score: [i32; 2],
    side: Player,
    prev: Player,
    passes: u8,
    over: bool,
    viewer: Viewer,
    ply: u32,
}

impl Default for Forme6 {
    fn default() -> Self {
        Self::new()
    }
}

fn tname(t: Tile) -> String {
    format!("{}{}", COL[t.0 as usize], t.1 + 1)
}

impl Forme6 {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag = vec![];
        for _ in 0..3 {
            for c in 0..6 {
                for s in 0..6 {
                    bag.push((c, s));
                }
            }
        }
        for i in (1..bag.len()).rev() {
            bag.swap(i, rng.below(i + 1));
        }
        let h0: Vec<Tile> = bag.drain(..6).collect();
        let h1: Vec<Tile> = bag.drain(..6).collect();
        Forme6 {
            board: BTreeMap::new(),
            hand: [h0, h1],
            bag,
            score: [0; 2],
            side: Player::One,
            prev: Player::One,
            passes: 0,
            over: false,
            viewer: Viewer::Fixed(Player::One),
            ply: 0,
        }
    }
    /// Linia (în direcția dx,dy) prin (x,y) cu plăcuța `t` presupusă pusă acolo.
    fn line(&self, x: i8, y: i8, dx: i8, dy: i8, t: Tile) -> Vec<Tile> {
        let mut v = vec![];
        let (mut cx, mut cy) = (x - dx, y - dy);
        let mut back = vec![];
        while let Some(&b) = self.board.get(&(cx, cy)) {
            back.push(b);
            cx -= dx;
            cy -= dy;
        }
        back.reverse();
        v.extend(back);
        v.push(t);
        let (mut cx, mut cy) = (x + dx, y + dy);
        while let Some(&b) = self.board.get(&(cx, cy)) {
            v.push(b);
            cx += dx;
            cy += dy;
        }
        v
    }
    fn valid_line(l: &[Tile]) -> bool {
        if l.len() > 6 {
            return false;
        }
        if l.len() < 2 {
            return true;
        }
        let same_c = l.iter().all(|t| t.0 == l[0].0);
        let same_s = l.iter().all(|t| t.1 == l[0].1);
        if !(same_c || same_s) {
            return false;
        }
        for (i, a) in l.iter().enumerate() {
            if l[i + 1..].contains(a) {
                return false;
            }
        }
        true
    }
    /// Punctele pentru plasare sau None dacă e ilegală.
    fn try_place(&self, t: Tile, x: i8, y: i8) -> Option<i32> {
        if self.board.contains_key(&(x, y)) {
            return None;
        }
        if self.board.is_empty() {
            return ((x, y) == (0, 0)).then_some(1);
        }
        let touch = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .any(|&(dx, dy)| self.board.contains_key(&(x + dx, y + dy)));
        if !touch {
            return None;
        }
        let mut pts = 0;
        for (dx, dy) in [(1, 0), (0, 1)] {
            let l = self.line(x, y, dx, dy, t);
            if !Self::valid_line(&l) {
                return None;
            }
            if l.len() >= 2 {
                pts += l.len() as i32 + if l.len() == 6 { 6 } else { 0 };
            }
        }
        Some(pts.max(1))
    }
    fn refill(&mut self, p: usize) {
        while self.hand[p].len() < 6 && !self.bag.is_empty() {
            let t = self.bag.pop().unwrap();
            self.hand[p].push(t);
        }
    }
}

impl Game for Forme6 {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        let mut tiles = self.hand[p].clone();
        tiles.sort();
        tiles.dedup();
        let (xs, ys): (Vec<i8>, Vec<i8>) = if self.board.is_empty() {
            (vec![0], vec![0])
        } else {
            let (x0, x1) = (
                self.board.keys().map(|k| k.0).min().unwrap() - 1,
                self.board.keys().map(|k| k.0).max().unwrap() + 1,
            );
            let (y0, y1) = (
                self.board.keys().map(|k| k.1).min().unwrap() - 1,
                self.board.keys().map(|k| k.1).max().unwrap() + 1,
            );
            ((x0..=x1).collect(), (y0..=y1).collect())
        };
        for &t in &tiles {
            for &x in &xs {
                for &y in &ys {
                    if self.try_place(t, x, y).is_some() {
                        v.push(Mv::Place(t, x, y));
                    }
                }
            }
            if !self.bag.is_empty() {
                v.push(Mv::Swap(t));
            }
        }
        if v.iter().all(|m| !matches!(m, Mv::Place(..))) && self.bag.is_empty() {
            v.push(Mv::Pass);
        }
        if v.is_empty() {
            v.push(Mv::Pass);
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        self.ply += 1;
        match m {
            Mv::Place(t, x, y) => {
                let pts = self.try_place(*t, *x, *y).unwrap();
                self.board.insert((*x, *y), *t);
                let k = self.hand[p].iter().position(|h| h == t).unwrap();
                self.hand[p].remove(k);
                self.score[p] += pts;
                self.refill(p);
                self.passes = 0;
                if self.hand[p].is_empty() && self.bag.is_empty() {
                    self.score[p] += 6;
                    self.over = true;
                }
            }
            Mv::Swap(t) => {
                let k = self.hand[p].iter().position(|h| h == t).unwrap();
                let out = self.hand[p].remove(k);
                self.refill(p);
                self.bag.insert(0, out);
                self.passes = 0;
            }
            Mv::Pass => {
                self.passes += 1;
                if self.passes >= 2 {
                    self.over = true;
                }
            }
        }
        if self.ply > 400 {
            self.over = true;
        }
        self.prev = self.side;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        // doar scor public + varietatea mâinii proprii
        let me = p.idx();
        let mut variety = self.hand[me].clone();
        variety.sort();
        variety.dedup();
        (self.score[me] - self.score[1 - me]) * 10 + variety.len() as i32
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !self.over {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        if self.board.is_empty() {
            s.push_str("Masa e goală: prima plăcuță se pune pe 0,0.\n");
        } else {
            let (x0, x1) = (
                self.board.keys().map(|k| k.0).min().unwrap() - 1,
                self.board.keys().map(|k| k.0).max().unwrap() + 1,
            );
            let (y0, y1) = (
                self.board.keys().map(|k| k.1).min().unwrap() - 1,
                self.board.keys().map(|k| k.1).max().unwrap() + 1,
            );
            s.push_str("    ");
            for x in x0..=x1 {
                s.push_str(&format!("{:>3}", x));
            }
            s.push('\n');
            for y in (y0..=y1).rev() {
                s.push_str(&format!("{:>3} ", y));
                for x in x0..=x1 {
                    match self.board.get(&(x, y)) {
                        Some(&t) => s.push_str(&format!("{:>3}", tname(t))),
                        None => s.push_str("  ."),
                    }
                }
                s.push('\n');
            }
        }
        s.push_str(&format!(
            "Scor: J1 {}, J2 {}; în pungă {} plăcuțe\n",
            self.score[0],
            self.score[1],
            self.bag.len()
        ));
        let v = self.viewer.who(self.side);
        let mut h = self.hand[v.idx()].clone();
        h.sort();
        s.push_str(&format!(
            "Piesele tale (J{}): {}\n",
            v.idx() + 1,
            h.iter().map(|&t| tname(t)).collect::<Vec<_>>().join(" ")
        ));
        s.push_str(&format!(
            "J{} are {} plăcuțe\n",
            v.other().idx() + 1,
            self.hand[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Place(t, x, y) => format!("{}@{},{}", tname(*t), x, y),
            Mv::Swap(t) => format!("schimb {}", tname(*t)),
            Mv::Pass => "pas".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn line_rules_and_score() {
        let mut g = Forme6::with_seed(1);
        g.board.insert((0, 0), (0, 0)); // R1
        g.board.insert((1, 0), (0, 1)); // R2
                                        // a treia: R3 pe (2,0): linie R1 R2 R3 => 3 puncte
        assert_eq!(g.try_place((0, 2), 2, 0), Some(3));
        // altă culoare și altă formă: ilegal
        assert_eq!(g.try_place((1, 3), 2, 0), None);
        // aceeași plăcuță de două ori în linie: ilegal
        assert_eq!(g.try_place((0, 0), 2, 0), None);
        // nelipită
        assert_eq!(g.try_place((0, 2), 5, 5), None);
    }
    #[test]
    fn hidden_hands() {
        let g = Forme6::with_seed(2);
        let mut a = g.clone();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone();
        b.set_viewer(Some(Player::Two));
        let l1 = a
            .render()
            .lines()
            .find(|l| l.starts_with("Piesele tale"))
            .unwrap()
            .to_string();
        let l2 = b
            .render()
            .lines()
            .find(|l| l.starts_with("Piesele tale"))
            .unwrap()
            .to_string();
        assert_ne!(
            l1.split_once(": ").unwrap().1,
            l2.split_once(": ").unwrap().1
        );
    }
    #[test]
    fn random_and_ai() {
        check(Forme6::with_seed, 500);
    }
}
