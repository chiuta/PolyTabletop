//! Motor „zaruri din sac” (lista C5, 2 jucători): fiecare are un sac de zaruri; la începutul turei
//! mutarea forțată `trag` scoate `draw` zaruri și le aruncă. Fețele dau energie (se adună automat)
//! sau unități (`dep i` le așază pe teren, cel mult `field`). Cu energia cumperi zaruri din rezerva
//! publică (`cumpar t`); unitățile gata de luptă atacă (`atac i j`: j = unitatea adversă, sau 0
//! pentru lovitură directă când adversarul n-are unități). Moduri: `Score` (câștigi glorie cât costă
//! unitatea doborâtă, 1 pe lovitura directă) și `Life` (loviturile directe scad viața adversarului).
//! Sacul se amestecă la mutarea forțată: informație publică în rest.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BMode {
    Score,
    Life,
}

#[derive(Clone, Copy)]
pub enum Fc {
    Nrg(i32),
    Unit(i32, i32),
    Blank,
}

pub struct DieT {
    pub name: &'static str,
    pub cost: i32,
    pub stock: i32,
    pub faces: [Fc; 6],
}

pub struct BagSpec {
    pub title: &'static str,
    pub mode: BMode,
    pub draw: usize,
    pub field: usize,
    pub goal: i32,
    /// primele două tipuri sunt zarurile de pornire (câte `start` din fiecare)
    pub types: &'static [DieT],
    pub start: [usize; 2],
    pub cap: u32,
}

#[derive(Clone, Copy)]
struct Fd {
    t: u8,
    f: u8,
    ready: bool,
}

#[derive(Clone)]
pub struct Bag {
    sp: &'static BagSpec,
    bag: [Vec<u8>; 2],
    disc: [Vec<u8>; 2],
    hand: Vec<(u8, u8)>,
    field: [Vec<Fd>; 2],
    pool: Vec<i32>,
    nrg: i32,
    pts: [i32; 2],
    drawn: bool,
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Bag {
    pub fn new(sp: &'static BagSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static BagSpec, seed: u64) -> Self {
        let mk = || {
            let mut b = vec![0u8; sp.start[0]];
            b.extend(vec![1u8; sp.start[1]]);
            b
        };
        let mut pool: Vec<i32> = sp.types.iter().map(|t| t.stock).collect();
        pool[0] = 0;
        pool[1] = 0;
        Bag {
            sp,
            bag: [mk(), mk()],
            disc: [vec![], vec![]],
            hand: vec![],
            field: [vec![], vec![]],
            pool,
            nrg: 0,
            pts: [if sp.mode == BMode::Life { sp.goal } else { 0 }; 2],
            drawn: false,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn face(&self, t: u8, f: u8) -> Fc {
        self.sp.types[t as usize].faces[f as usize]
    }
    fn unit_stats(&self, d: &Fd) -> (i32, i32) {
        match self.face(d.t, d.f) {
            Fc::Unit(a, b) => (a, b),
            _ => (0, 1),
        }
    }
    fn field_power(&self, p: usize) -> i32 {
        self.field[p]
            .iter()
            .map(|d| {
                let (a, b) = self.unit_stats(d);
                a + b
            })
            .sum()
    }
    fn end_turn(&mut self) {
        let me = self.side;
        let mut h: Vec<u8> = self.hand.drain(..).map(|x| x.0).collect();
        self.disc[me].append(&mut h);
        for u in self.field[me].iter_mut() {
            u.ready = true;
        }
        self.nrg = 0;
        self.drawn = false;
        self.side = 1 - me;
    }
    fn attacks(&self) -> Vec<(usize, usize)> {
        let me = self.side;
        let mut v = Vec::new();
        for i in 0..self.field[me].len() {
            if !self.field[me][i].ready {
                continue;
            }
            if self.field[1 - me].is_empty() {
                v.push((i, 0));
            } else {
                for j in 0..self.field[1 - me].len() {
                    v.push((i, j + 1));
                }
            }
        }
        v
    }
    fn score(&self, p: usize) -> i32 {
        self.pts[p]
    }
}

impl Mini for Bag {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        if !self.drawn {
            return vec!["trag".into()];
        }
        let me = self.side;
        let mut v = vec!["fin".to_string()];
        if self.field[me].len() < self.sp.field {
            for (i, h) in self.hand.iter().enumerate() {
                if matches!(self.face(h.0, h.1), Fc::Unit(..)) {
                    v.push(format!("dep {}", i + 1));
                }
            }
        }
        for t in 0..self.sp.types.len() {
            if self.pool[t] > 0 && self.nrg >= self.sp.types[t].cost {
                v.push(format!("cumpar {}", t + 1));
            }
        }
        for (i, j) in self.attacks() {
            v.push(format!("atac {} {}", i + 1, j));
        }
        v
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let me = self.side;
        if !self.drawn {
            for _ in 0..self.sp.draw {
                if self.bag[me].is_empty() {
                    let mut d = std::mem::take(&mut self.disc[me]);
                    // amestecare cu zarul reînsămânțat
                    for i in (1..d.len()).rev() {
                        let j = self.dice.below(i + 1);
                        d.swap(i, j);
                    }
                    self.bag[me] = d;
                }
                if self.bag[me].is_empty() {
                    break;
                }
                let k = self.dice.below(self.bag[me].len());
                let t = self.bag[me].swap_remove(k);
                let f = self.dice.below(6) as u8;
                if let Fc::Nrg(n) = self.face(t, f) {
                    self.nrg += n;
                }
                self.hand.push((t, f));
            }
            self.drawn = true;
            return;
        }
        let mut it = m.split(' ');
        match it.next().unwrap() {
            "fin" => self.end_turn(),
            "dep" => {
                let i = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let (t, f) = self.hand.remove(i);
                self.field[me].push(Fd { t, f, ready: false });
            }
            "cumpar" => {
                let t = it.next().unwrap().parse::<usize>().unwrap() - 1;
                self.nrg -= self.sp.types[t].cost;
                self.pool[t] -= 1;
                self.disc[me].push(t as u8);
            }
            _ => {
                let i = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let j = it.next().unwrap().parse::<usize>().unwrap();
                self.field[me][i].ready = false;
                let (a, _) = self.unit_stats(&self.field[me][i].clone());
                if j == 0 {
                    match self.sp.mode {
                        BMode::Score => self.pts[me] += 1,
                        BMode::Life => self.pts[1 - me] -= a,
                    }
                } else {
                    let tgt = self.field[1 - me][j - 1];
                    let (_, d) = self.unit_stats(&tgt);
                    let att = self.field[me][i];
                    let (_, ad) = self.unit_stats(&att);
                    let (ta, _) = self.unit_stats(&tgt);
                    let mut dead_t = false;
                    let mut dead_a = false;
                    if a >= d {
                        dead_t = true;
                    }
                    if ta >= ad {
                        dead_a = true;
                    }
                    if dead_t {
                        let t = self.field[1 - me].remove(j - 1);
                        self.disc[1 - me].push(t.t);
                        if self.sp.mode == BMode::Score {
                            self.pts[me] += self.sp.types[t.t as usize].cost.max(1);
                        }
                    }
                    if dead_a {
                        let u = self.field[me].remove(i);
                        self.disc[me].push(u.t);
                    }
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        match self.sp.mode {
            BMode::Score => {
                if self.pts[0] >= self.sp.goal || self.pts[1] >= self.sp.goal {
                    return Some(by_score(self.pts[0], self.pts[1]));
                }
            }
            BMode::Life => {
                if self.pts[0] <= 0 || self.pts[1] <= 0 {
                    return Some(by_score(self.pts[0], self.pts[1]));
                }
            }
        }
        if self.plies >= self.sp.cap {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let mut v = 100 * (self.score(a) - self.score(o))
            + 25 * (self.field_power(a) - self.field_power(o));
        let sgn = if self.side == a { 1 } else { -1 };
        // energie nefolosită valorează puțin; zarurile cumpărate (în sac) valorează după cost
        v += sgn * 5 * self.nrg;
        let owned = |q: usize| -> i32 {
            self.bag[q]
                .iter()
                .chain(self.disc[q].iter())
                .map(|&t| self.sp.types[t as usize].cost * 12)
                .sum()
        };
        v + owned(a) - owned(o)
    }
    fn show(&self) -> String {
        let mut s = format!("{} — țintă {}\n", self.sp.title, self.sp.goal);
        for p in 0..2 {
            let f: Vec<String> = self.field[p]
                .iter()
                .map(|u| {
                    let (a, d) = self.unit_stats(u);
                    format!(
                        "{}({}/{}){}",
                        self.sp.types[u.t as usize].name,
                        a,
                        d,
                        if u.ready { "" } else { "*" }
                    )
                })
                .collect();
            s.push_str(&format!(
                "{}: {} {} | sac {} | aruncate {} | teren: {}\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                if self.sp.mode == BMode::Life {
                    "viață"
                } else {
                    "glorie"
                },
                self.pts[p],
                self.bag[p].len(),
                self.disc[p].len(),
                if f.is_empty() {
                    "-".into()
                } else {
                    f.join(" ")
                }
            ));
        }
        let h: Vec<String> = self
            .hand
            .iter()
            .enumerate()
            .map(|(i, &(t, f))| {
                let d = match self.face(t, f) {
                    Fc::Nrg(n) => format!("energie {n}"),
                    Fc::Unit(a, b) => format!("unitate {a}/{b}"),
                    Fc::Blank => "gol".into(),
                };
                format!("{}) {} {}", i + 1, self.sp.types[t as usize].name, d)
            })
            .collect();
        s.push_str(&format!(
            "Zaruri aruncate: {} | energie {}\n",
            h.join("; "),
            self.nrg
        ));
        let pl: Vec<String> = (0..self.sp.types.len())
            .filter(|&t| self.pool[t] > 0)
            .map(|t| {
                format!(
                    "{}) {} cost {} (stoc {})",
                    t + 1,
                    self.sp.types[t].name,
                    self.sp.types[t].cost,
                    self.pool[t]
                )
            })
            .collect();
        s.push_str(&format!("Rezerva: {}\n", pl.join("; ")));
        s.push_str(&format!(
            "La mutare: {} (trag | dep i | cumpar t | atac i j | fin)\n",
            pname(self.side())
        ));
        s
    }
}
