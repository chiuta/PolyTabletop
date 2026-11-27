//! „Labirintul cu Pereți Nevăzuți” — variantă originală C4 (2 jucători, informație publică).
//! Pe o tablă 5×5 pereții nu se văd: fiecare muchie e decisă lazy la prima încercare de trecere (30% perete,
//! zar re-însămânțat la `clone()`), apoi rămâne cunoscută de amândoi. `mut n|s|e|v` alege direcția, rezultatul se
//! află la mutarea forțată `arata`. Primul la comoara din centru câștigă; la plafon, cine e mai aproape.
use crate::c4_util::*;

const N: i32 = 5;
const DIRS: [(&str, i32, i32); 4] = [("n", -1, 0), ("s", 1, 0), ("e", 0, 1), ("v", 0, -1)];

#[derive(Clone)]
pub struct Labirint {
    dice: Dice,
    pos: [(i32, i32); 2],
    // muchii: 0 necunoscut, 1 liber, 2 perete; orizontale h[r][c] între (r,c) și (r+1,c); verticale v[r][c] între (r,c) și (r,c+1)
    h: [[u8; 5]; 5],
    v: [[u8; 5]; 5],
    turn: usize,
    pend: Option<usize>,
    plies: u32,
    last: String,
}

impl Labirint {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Labirint {
            dice: Dice::new(seed),
            pos: [(0, 0), (N - 1, N - 1)],
            h: [[0; 5]; 5],
            v: [[0; 5]; 5],
            turn: 0,
            pend: None,
            plies: 0,
            last: String::new(),
        }
    }
    fn slot(&mut self, r: i32, c: i32, d: usize) -> &mut u8 {
        let (dr, dc) = (DIRS[d].1, DIRS[d].2);
        match (dr, dc) {
            (1, 0) => &mut self.h[r as usize][c as usize],
            (-1, 0) => &mut self.h[(r - 1) as usize][c as usize],
            (0, 1) => &mut self.v[r as usize][c as usize],
            _ => &mut self.v[r as usize][(c - 1) as usize],
        }
    }
    fn inside(&self, r: i32, c: i32, d: usize) -> bool {
        let (nr, nc) = (r + DIRS[d].1, c + DIRS[d].2);
        (0..N).contains(&nr) && (0..N).contains(&nc)
    }
    fn dist(&self, w: usize) -> i32 {
        (self.pos[w].0 - 2).abs() + (self.pos[w].1 - 2).abs()
    }
    fn known_edge(&self, r: i32, c: i32, d: usize) -> u8 {
        let (dr, dc) = (DIRS[d].1, DIRS[d].2);
        match (dr, dc) {
            (1, 0) => self.h[r as usize][c as usize],
            (-1, 0) => self.h[(r - 1) as usize][c as usize],
            (0, 1) => self.v[r as usize][c as usize],
            _ => self.v[r as usize][(c - 1) as usize],
        }
    }
}

impl Default for Labirint {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Labirint {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["arata".into()];
        }
        let (r, c) = self.pos[self.turn];
        let mut v: Vec<String> = (0..4)
            .filter(|&d| self.inside(r, c, d) && self.known_edge(r, c, d) != 2)
            .map(|d| format!("mut {}", DIRS[d].0))
            .collect();
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m == "pas" {
            self.turn = 1 - me;
            self.plies += 1;
            return;
        }
        if m != "arata" {
            self.pend = Some(DIRS.iter().position(|d| d.0 == &m[4..]).unwrap());
            return;
        }
        let d = self.pend.take().unwrap();
        let (r, c) = self.pos[me];
        if *self.slot(r, c, d) == 0 {
            let wall = self.dice.below(100) < 30;
            *self.slot(r, c, d) = if wall { 2 } else { 1 };
        }
        if *self.slot(r, c, d) == 2 {
            self.last = format!("J{}: perete la {}", me + 1, DIRS[d].0);
        } else {
            self.pos[me] = (r + DIRS[d].1, c + DIRS[d].2);
            self.last = format!("J{} a mers spre {}", me + 1, DIRS[d].0);
        }
        self.turn = 1 - me;
        self.plies += 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend.is_some() {
            return None;
        }
        for w in 0..2 {
            if self.dist(w) == 0 {
                return Some(Outcome::Win(pl(w)));
            }
        }
        if self.plies > 250 {
            return Some(by_score(-self.dist(0), -self.dist(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.dist(1 - a) - self.dist(a))
    }
    fn show(&self) -> String {
        let mut s = String::from("Labirintul cu Pereți Nevăzuți (comoara la centru)\n");
        for r in 0..N {
            let mut row = String::new();
            for c in 0..N {
                let ch = if self.pos[0] == (r, c) && self.pos[1] == (r, c) {
                    "X"
                } else if self.pos[0] == (r, c) {
                    "1"
                } else if self.pos[1] == (r, c) {
                    "2"
                } else if (r, c) == (2, 2) {
                    "*"
                } else {
                    "."
                };
                row.push_str(ch);
                if c + 1 < N {
                    row.push(match self.v[r as usize][c as usize] {
                        2 => '|',
                        1 => ' ',
                        _ => '?',
                    });
                }
            }
            s.push_str(&row);
            s.push('\n');
            if r + 1 < N {
                let mut b = String::new();
                for c in 0..N {
                    b.push(match self.h[r as usize][c as usize] {
                        2 => '-',
                        1 => ' ',
                        _ => '?',
                    });
                    if c + 1 < N {
                        b.push(' ');
                    }
                }
                s.push_str(&b);
                s.push('\n');
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s.push_str(&format!(
            "La mutare: {} (mut n|s|e|v | arata)\n",
            pname(self.side())
        ));
        s
    }
}

pub fn make() -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Labirint::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        check(Labirint::with_seed, 700);
    }
}
