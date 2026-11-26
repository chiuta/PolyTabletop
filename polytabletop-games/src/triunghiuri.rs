//! Triunghiurile Numerotate — domino triunghiular cu numere în colțuri (variantă originală, mâini deschise).
//!
//! Plăcuțele sunt triunghiuri cu câte un număr 0–4 în fiecare colț (35 de plăcuțe, toate
//! combinațiile). Fiecare primește 7 (mâinile sunt deschise). O plăcuță se așază pe o celulă
//! triunghiulară lipită de latura uneia deja puse, rotită 0–2 (în sens trigonometric) și cu fiecare
//! colț comun cu plăcuțele vecine având același număr: `pune i@x,y,s,k` (plăcuța i din mână, celula
//! (x,y), s = u pentru triunghi cu vârful în sus sau d în jos, k rotația). Scor: suma numerelor
//! plăcuței. `trage` dacă nu poți pune (apoi `pas` când pachetul e gol). Cine rămâne fără plăcuțe
//! ia 10 + numerele adversarului; blocaj general: cine are mai puține în mână ia diferența.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const R: i32 = 12;
const SZ: usize = 25;

type T3 = [u8; 3];

#[derive(Clone)]
pub struct Triunghiuri {
    vert: [[i8; SZ]; SZ],
    up: [[bool; SZ]; SZ],
    down: [[bool; SZ]; SZ],
    hand: [Vec<T3>; 2],
    pile: Vec<T3>,
    pts: [i32; 2],
    side: Player,
    passes: u8,
    first: bool,
    ply: u32,
    out: Option<usize>,
}

fn idx(x: i32) -> usize {
    (x + R) as usize
}

impl Triunghiuri {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all: Vec<T3> = Vec::new();
        for a in 0..5u8 {
            for b in a..5 {
                for c in b..5 {
                    all.push([a, b, c]);
                }
            }
        }
        shuf(&mut all, &mut rng);
        let h1 = (0..7).map(|_| all.pop().unwrap()).collect();
        let h2 = (0..7).map(|_| all.pop().unwrap()).collect();
        Triunghiuri {
            vert: [[-1; SZ]; SZ],
            up: [[false; SZ]; SZ],
            down: [[false; SZ]; SZ],
            hand: [h1, h2],
            pile: all,
            pts: [0; 2],
            side: Player::One,
            passes: 0,
            first: true,
            ply: 0,
            out: None,
        }
    }
    fn verts(x: i32, y: i32, up: bool) -> [(i32, i32); 3] {
        if up {
            [(x, y), (x + 1, y), (x, y + 1)]
        } else {
            [(x + 1, y), (x + 1, y + 1), (x, y + 1)]
        }
    }
    fn inside(x: i32, y: i32) -> bool {
        x > -R && y > -R && x < R - 1 && y < R - 1
    }
    fn occupied(&self, x: i32, y: i32, up: bool) -> bool {
        if !Self::inside(x, y) {
            return true;
        }
        if up {
            self.up[idx(x)][idx(y)]
        } else {
            self.down[idx(x)][idx(y)]
        }
    }
    fn nbrs(x: i32, y: i32, up: bool) -> [(i32, i32, bool); 3] {
        if up {
            [(x, y, false), (x, y - 1, false), (x - 1, y, false)]
        } else {
            [(x, y, true), (x, y + 1, true), (x + 1, y, true)]
        }
    }
    fn frontier(&self) -> Vec<(i32, i32, bool)> {
        let mut v = Vec::new();
        for x in -R + 1..R - 1 {
            for y in -R + 1..R - 1 {
                for up in [true, false] {
                    if self.occupied(x, y, up) {
                        continue;
                    }
                    if Self::nbrs(x, y, up)
                        .iter()
                        .any(|&(a, b, u)| self.occupied(a, b, u) && Self::inside(a, b))
                    {
                        v.push((x, y, up));
                    }
                }
            }
        }
        v
    }
    fn fits(&self, x: i32, y: i32, up: bool, t: &T3, k: usize) -> bool {
        let vs = Self::verts(x, y, up);
        (0..3).all(|i| {
            let cur = self.vert[idx(vs[i].0)][idx(vs[i].1)];
            cur < 0 || cur as u8 == t[(i + k) % 3]
        })
    }
    fn placements(&self, p: usize) -> Vec<(usize, i32, i32, bool, usize)> {
        let mut v = Vec::new();
        if self.first {
            for (i, _) in self.hand[p].iter().enumerate() {
                v.push((i, 0, 0, true, 0));
            }
            return v;
        }
        let fr = self.frontier();
        for (i, t) in self.hand[p].iter().enumerate() {
            if self.hand[p][..i].contains(t) {
                continue;
            }
            for &(x, y, up) in &fr {
                for k in 0..3 {
                    if self.fits(x, y, up, t, k) {
                        v.push((i, x, y, up, k));
                    }
                }
            }
        }
        v
    }
    fn sum(t: &T3) -> i32 {
        t.iter().map(|&x| x as i32).sum()
    }
    fn hand_sum(&self, p: usize) -> i32 {
        self.hand[p].iter().map(Self::sum).sum()
    }
    fn final_pts(&self) -> [i32; 2] {
        let mut s = self.pts;
        if let Some(w) = self.out {
            s[w] += 10 + self.hand_sum(1 - w);
        } else {
            let (a, b) = (self.hand_sum(0), self.hand_sum(1));
            if a < b {
                s[0] += b - a;
            } else {
                s[1] += a - b;
            }
        }
        s
    }
}

impl Default for Triunghiuri {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Triunghiuri {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v: Vec<String> = self
            .placements(me)
            .iter()
            .map(|&(i, x, y, up, k)| {
                format!(
                    "pune {}@{},{},{},{}",
                    i + 1,
                    x,
                    y,
                    if up { 'u' } else { 'd' },
                    k
                )
            })
            .collect();
        if v.is_empty() {
            if !self.pile.is_empty() {
                v.push("trage".into());
            } else {
                v.push("pas".into());
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if m == "trage" {
            if let Some(t) = self.pile.pop() {
                self.hand[me].push(t);
                self.pts[me] -= 1;
            }
            return;
        }
        if m == "pas" {
            self.passes += 1;
            self.side = self.side.other();
            return;
        }
        self.passes = 0;
        let (i, rest) = m[5..].split_once('@').unwrap();
        let i = i.parse::<usize>().unwrap() - 1;
        let f: Vec<&str> = rest.split(',').collect();
        let (x, y): (i32, i32) = (f[0].parse().unwrap(), f[1].parse().unwrap());
        let up = f[2] == "u";
        let k: usize = f[3].parse().unwrap();
        let t = self.hand[me].remove(i);
        let vs = Self::verts(x, y, up);
        for j in 0..3 {
            self.vert[idx(vs[j].0)][idx(vs[j].1)] = t[(j + k) % 3] as i8;
        }
        if up {
            self.up[idx(x)][idx(y)] = true;
        } else {
            self.down[idx(x)][idx(y)] = true;
        }
        self.pts[me] += Self::sum(&t);
        self.first = false;
        if self.hand[me].is_empty() {
            self.out = Some(me);
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.out.is_some() || self.passes >= 2 || self.ply > 300 {
            let s = self.final_pts();
            return Some(by_score(s[0], s[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let s = self.pts;
        3 * (s[a] - s[1 - a]) + (self.hand_sum(1 - a) - self.hand_sum(a))
    }
    fn show(&self) -> String {
        let mut s =
            String::from("Triunghiurile Numerotate — colțuri numerotate; celule (x,y,u|d)\n");
        let mut cells = Vec::new();
        for y in -R + 1..R - 1 {
            for x in -R + 1..R - 1 {
                for up in [true, false] {
                    if self.occupied(x, y, up) && Self::inside(x, y) {
                        let vs = Self::verts(x, y, up);
                        let n: Vec<String> = vs
                            .iter()
                            .map(|v| self.vert[idx(v.0)][idx(v.1)].to_string())
                            .collect();
                        cells.push(format!(
                            "({},{},{}):{}",
                            x,
                            y,
                            if up { 'u' } else { 'd' },
                            n.join("")
                        ));
                    }
                }
            }
        }
        s.push_str(&format!(
            "Puse: {}\n",
            if cells.is_empty() {
                "-".into()
            } else {
                cells.join(" ")
            }
        ));
        for p in 0..2 {
            let h: Vec<String> = self.hand[p]
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{}:{}{}{}", i + 1, t[0], t[1], t[2]))
                .collect();
            s.push_str(&format!(
                "J{}: puncte {} | mână {}\n",
                p + 1,
                self.pts[p],
                h.join(" ")
            ));
        }
        s.push_str(&format!(
            "Pachet: {} | La mutare: {} (pune i@x,y,s,k | trage | pas){}\n",
            self.pile.len(),
            pname(self.side),
            if self.first {
                " — prima plăcuță la (0,0,u)"
            } else {
                ""
            }
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Triunghiuri::with_seed, 400);
    }
    #[test]
    fn neighbours_are_symmetric() {
        for (x, y) in [(0, 0), (2, -3)] {
            for (a, b, u) in Triunghiuri::nbrs(x, y, true) {
                assert!(Triunghiuri::nbrs(a, b, u).contains(&(x, y, true)));
            }
        }
    }
}
