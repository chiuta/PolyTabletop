//! Două motoare „zaruri și foi de scor” (lista C5, 2 jucători):
//! * `Rq` — patru rânduri colorate (două urcătoare, două coborâtoare), marcaje doar spre dreapta;
//!   suma zarurilor albe o poate folosi oricine, apoi cel la rând folosește un zar alb + unul colorat.
//!   Un rând cu cel puțin `lockmin` marcaje se poate încheia (blocare). Penalizări pentru tura fără marcaj.
//! * `Rw` — trei rânduri de 9 căsuțe; cel la rând alege ce zaruri însumează (`dados m`), apoi amândoi
//!   scriu suma într-o căsuță: crescător pe rând și fără duplicat pe coloană.
//!
//! Zarurile sunt re-însămânțate la `clone()`; informație publică.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

fn pl2(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}
fn tri(n: i32) -> i32 {
    n * (n + 1) / 2
}

// ---------------------------------------------------------------- Rq
pub struct RqSpec {
    pub title: &'static str,
    pub lockmin: i32,
    pub pen_val: i32,
    pub pen_max: i32,
    pub locks_end: usize,
    pub names: [&'static str; 4],
    pub cap: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum QPh {
    Roll,
    ActW,
    PassW,
    ActC,
}

#[derive(Clone)]
pub struct Rq {
    sp: &'static RqSpec,
    marks: [[Vec<bool>; 4]; 2],
    last: [[i32; 4]; 2],
    locked: [bool; 4],
    pen: [i32; 2],
    w: [u8; 2],
    c: [u8; 4],
    ph: QPh,
    took: bool,
    to_lock: [bool; 4],
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Rq {
    pub fn new(sp: &'static RqSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static RqSpec, seed: u64) -> Self {
        let mk = || {
            [
                vec![false; 12],
                vec![false; 12],
                vec![false; 12],
                vec![false; 12],
            ]
        };
        Rq {
            sp,
            marks: [mk(), mk()],
            last: [[-1; 4]; 2],
            locked: [false; 4],
            pen: [0; 2],
            w: [1; 2],
            c: [1; 4],
            ph: QPh::Roll,
            took: false,
            to_lock: [false; 4],
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn cnt(&self, p: usize, r: usize) -> i32 {
        self.marks[p][r].iter().filter(|&&x| x).count() as i32
    }
    fn idx(r: usize, v: i32) -> i32 {
        if r < 2 {
            v - 2
        } else {
            12 - v
        }
    }
    fn can(&self, p: usize, r: usize, v: i32) -> bool {
        if self.locked[r] || !(2..=12).contains(&v) {
            return false;
        }
        let i = Self::idx(r, v);
        if i <= self.last[p][r] {
            return false;
        }
        i != 10 || self.cnt(p, r) >= self.sp.lockmin
    }
    fn put(&mut self, p: usize, r: usize, v: i32) {
        let i = Self::idx(r, v);
        self.marks[p][r][i as usize] = true;
        self.last[p][r] = i;
        if i == 10 {
            self.marks[p][r][11] = true;
            self.to_lock[r] = true;
        }
        if p == self.side {
            self.took = true;
        }
    }
    fn score(&self, p: usize) -> i32 {
        (0..4).map(|r| tri(self.cnt(p, r))).sum::<i32>() - self.sp.pen_val * self.pen[p]
    }
    fn potential(&self, p: usize) -> i32 {
        (0..4)
            .filter(|&r| !self.locked[r])
            .map(|r| 10 - self.last[p][r])
            .sum()
    }
    fn end_turn(&mut self) {
        if !self.took {
            self.pen[self.side] += 1;
        }
        for r in 0..4 {
            if self.to_lock[r] {
                self.locked[r] = true;
            }
        }
        self.to_lock = [false; 4];
        self.took = false;
        self.side = 1 - self.side;
        self.ph = QPh::Roll;
    }
    fn nlocked(&self) -> usize {
        self.locked.iter().filter(|&&x| x).count()
    }
}

impl Mini for Rq {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            QPh::PassW => pl2(1 - self.side),
            _ => pl2(self.side),
        }
    }
    fn moves(&self) -> Vec<String> {
        let mut v = Vec::new();
        match self.ph {
            QPh::Roll => return vec!["arunc".into()],
            QPh::ActW | QPh::PassW => {
                let p = if self.ph == QPh::ActW {
                    self.side
                } else {
                    1 - self.side
                };
                let s = (self.w[0] + self.w[1]) as i32;
                for r in 0..4 {
                    if self.can(p, r, s) {
                        v.push(format!("m {}", r + 1));
                    }
                }
            }
            QPh::ActC => {
                for w in 0..2 {
                    for r in 0..4 {
                        if self.can(self.side, r, self.w[w] as i32 + self.c[r] as i32) {
                            v.push(format!("c {} {}", w + 1, r + 1));
                        }
                    }
                }
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        match self.ph {
            QPh::Roll => {
                for i in 0..2 {
                    self.w[i] = self.dice.below(6) as u8 + 1;
                }
                for i in 0..4 {
                    self.c[i] = self.dice.below(6) as u8 + 1;
                }
                self.ph = QPh::ActW;
            }
            QPh::ActW | QPh::PassW => {
                let p = if self.ph == QPh::ActW {
                    self.side
                } else {
                    1 - self.side
                };
                if m != "pas" {
                    let r = m[2..].parse::<usize>().unwrap() - 1;
                    self.put(p, r, (self.w[0] + self.w[1]) as i32);
                }
                self.ph = if self.ph == QPh::ActW {
                    QPh::PassW
                } else {
                    QPh::ActC
                };
            }
            QPh::ActC => {
                if m != "pas" {
                    let mut it = m[2..].split(' ');
                    let w = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    let r = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    self.put(self.side, r, self.w[w] as i32 + self.c[r] as i32);
                }
                self.end_turn();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph == QPh::Roll
            && (self.nlocked() >= self.sp.locks_end
                || self.pen[0] >= self.sp.pen_max
                || self.pen[1] >= self.sp.pen_max
                || self.plies >= self.sp.cap)
        {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.score(a) - self.score(1 - a)) + 15 * (self.potential(a) - self.potential(1 - a))
    }
    fn show(&self) -> String {
        let mut s = format!("{} — rânduri: ↑ urcător, ↓ coborâtor\n", self.sp.title);
        for r in 0..4 {
            let (a, b) = (self.cnt(0, r), self.cnt(1, r));
            s.push_str(&format!(
                "{}. {} {}{}: J1 {} marcaje (ultimul {}), J2 {} marcaje (ultimul {})\n",
                r + 1,
                self.sp.names[r],
                if r < 2 { "↑" } else { "↓" },
                if self.locked[r] { " [blocat]" } else { "" },
                a,
                self.last[0][r] + 1,
                b,
                self.last[1][r] + 1
            ));
        }
        s.push_str(&format!(
            "Penalizări: J1 {}, J2 {} | scor J1 {} / J2 {}\n",
            self.pen[0],
            self.pen[1],
            self.score(0),
            self.score(1)
        ));
        if self.ph != QPh::Roll {
            s.push_str(&format!(
                "Zaruri albe: {} {} | colorate: {} {} {} {}\n",
                self.w[0], self.w[1], self.c[0], self.c[1], self.c[2], self.c[3]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | m r | c w r | pas)\n",
            pname(self.side())
        ));
        s
    }
}

// ---------------------------------------------------------------- Rw
pub struct RwSpec {
    pub title: &'static str,
    pub rows: usize,
    pub cols: usize,
    pub row_bonus: i32,
    pub col_bonus: i32,
    pub pen_max: i32,
    pub cap: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WPh {
    Roll,
    Choose,
    PlaceA,
    PlaceP,
}

#[derive(Clone)]
pub struct Rw {
    sp: &'static RwSpec,
    g: [Vec<u8>; 2],
    pen: [i32; 2],
    d: [u8; 3],
    sum: i32,
    ph: WPh,
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Rw {
    pub fn new(sp: &'static RwSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static RwSpec, seed: u64) -> Self {
        Rw {
            sp,
            g: [vec![0; sp.rows * sp.cols], vec![0; sp.rows * sp.cols]],
            pen: [0; 2],
            d: [1; 3],
            sum: 0,
            ph: WPh::Roll,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn ok(&self, p: usize, r: usize, c: usize, v: i32) -> bool {
        let w = self.sp.cols;
        if self.g[p][r * w + c] != 0 {
            return false;
        }
        for j in 0..w {
            let x = self.g[p][r * w + j] as i32;
            if x != 0 && ((j < c && x >= v) || (j > c && x <= v)) {
                return false;
            }
        }
        for i in 0..self.sp.rows {
            if i != r && self.g[p][i * w + c] as i32 == v {
                return false;
            }
        }
        true
    }
    fn legal(&self, p: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for r in 0..self.sp.rows {
            for c in 0..self.sp.cols {
                if self.ok(p, r, c, self.sum) {
                    v.push((r, c));
                }
            }
        }
        v
    }
    fn full_rows(&self, p: usize) -> usize {
        let w = self.sp.cols;
        (0..self.sp.rows)
            .filter(|&r| (0..w).all(|c| self.g[p][r * w + c] != 0))
            .count()
    }
    fn score(&self, p: usize) -> i32 {
        let w = self.sp.cols;
        let cells = self.g[p].iter().filter(|&&x| x != 0).count() as i32;
        let cols = (0..w)
            .filter(|&c| (0..self.sp.rows).all(|r| self.g[p][r * w + c] != 0))
            .count() as i32;
        cells + self.sp.row_bonus * self.full_rows(p) as i32 + self.sp.col_bonus * cols
            - self.pen[p]
    }
}

impl Mini for Rw {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            WPh::PlaceP => pl2(1 - self.side),
            _ => pl2(self.side),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            WPh::Roll => vec!["arunc".into()],
            WPh::Choose => (1..8).map(|m| format!("dados {m}")).collect(),
            WPh::PlaceA | WPh::PlaceP => {
                let p = if self.ph == WPh::PlaceA {
                    self.side
                } else {
                    1 - self.side
                };
                let mut v: Vec<String> = self
                    .legal(p)
                    .iter()
                    .map(|&(r, c)| format!("pun {} {}", r + 1, c + 1))
                    .collect();
                v.push("pas".into());
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        match self.ph {
            WPh::Roll => {
                for i in 0..3 {
                    self.d[i] = self.dice.below(6) as u8 + 1;
                }
                self.ph = WPh::Choose;
            }
            WPh::Choose => {
                let mask = m[6..].parse::<usize>().unwrap();
                self.sum = (0..3)
                    .filter(|i| mask >> i & 1 == 1)
                    .map(|i| self.d[i] as i32)
                    .sum();
                self.ph = WPh::PlaceA;
            }
            WPh::PlaceA | WPh::PlaceP => {
                let act = self.ph == WPh::PlaceA;
                let p = if act { self.side } else { 1 - self.side };
                if m == "pas" {
                    if act {
                        self.pen[p] += 1;
                    }
                } else {
                    let mut it = m[4..].split(' ');
                    let r = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    let c = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    self.g[p][r * self.sp.cols + c] = self.sum as u8;
                }
                if act {
                    self.ph = WPh::PlaceP;
                } else {
                    self.side = 1 - self.side;
                    self.ph = WPh::Roll;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph == WPh::Roll
            && (self.full_rows(0) > 0
                || self.full_rows(1) > 0
                || self.pen[0] >= self.sp.pen_max
                || self.pen[1] >= self.sp.pen_max
                || self.plies >= self.sp.cap)
        {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 100 * (self.score(a) - self.score(1 - a));
        if self.ph == WPh::PlaceA {
            // suma aleasă: câte locuri are cel la rând față de adversar
            v += 10
                * (self.legal(self.side).len() as i32 - self.legal(1 - self.side).len() as i32)
                * if self.side == a { 1 } else { -1 };
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{} — suma aleasă: {}\n", self.sp.title, self.sum);
        for p in 0..2 {
            s.push_str(&format!(
                "{} (scor {}, penalizări {}):\n",
                pname(pl2(p)),
                self.score(p),
                self.pen[p]
            ));
            for r in 0..self.sp.rows {
                let row: Vec<String> = (0..self.sp.cols)
                    .map(|c| match self.g[p][r * self.sp.cols + c] {
                        0 => " .".to_string(),
                        x => format!("{x:>2}"),
                    })
                    .collect();
                s.push_str(&format!("  {}: {}\n", r + 1, row.join(" ")));
            }
        }
        if self.ph != WPh::Roll {
            s.push_str(&format!(
                "Zaruri: {} {} {}\n",
                self.d[0], self.d[1], self.d[2]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | dados m | pun r c | pas)\n",
            pname(self.side())
        ));
        s
    }
}
