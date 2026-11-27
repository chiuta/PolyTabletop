//! Două motoare „zaruri repartizate” (lista C5, 2 jucători, informație publică):
//! * `Adv` — în fiecare rundă ambii aruncă două zaruri (`arunc`) și, alternativ, ocupă sfetnici:
//!   `pun n d` (zarul d = n) sau `pun n 3` (suma celor două). Un sfetnic poate fi ocupat o dată pe
//!   rundă. Apoi se construiește (`constr b`). La rundele marcate vine o invazie: cine are prea puțini
//!   soldați pierde glorie.
//! * `Gal` — fiecare are mai multe zaruri-muncitori cu fețele explorare/așezare/producție/livrare/dezvoltare
//!   și joker. După aruncarea forțată, jokerii se fixează (`fixez d f`), un zar poate fi schimbat contra
//!   unui credit (`ajust d f`); `faze` rezolvă fazele; apoi `recrut` (zar nou contra credite) sau `fin`.
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

// ---------------------------------------------------------------- Adv
/// (lemn, piatră, aur, glorie, soldați)
pub type Rw = (i32, i32, i32, i32, i32);

pub struct AdvSpec {
    pub title: &'static str,
    pub rounds: u32,
    pub advisors: [Rw; 12],
    /// (lemn, piatră, aur, glorie)
    pub buildings: &'static [(i32, i32, i32, i32)],
    /// invazii: (runda după care vine, forța)
    pub invasions: &'static [(u32, i32)],
    pub loss: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum APh {
    Roll,
    Place,
    Build,
}

#[derive(Clone)]
pub struct Adv {
    sp: &'static AdvSpec,
    res: [[i32; 5]; 2], // lemn, piatră, aur, glorie, soldați
    d: [[u8; 2]; 2],
    used_d: [[bool; 2]; 2],
    taken: [bool; 12],
    built: Vec<Option<usize>>,
    passed: [bool; 2],
    ph: APh,
    round: u32,
    first: usize,
    side: usize,
    dice: Dice,
}

impl Adv {
    pub fn new(sp: &'static AdvSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static AdvSpec, seed: u64) -> Self {
        Adv {
            sp,
            res: [[0; 5]; 2],
            d: [[1; 2]; 2],
            used_d: [[false; 2]; 2],
            taken: [false; 12],
            built: vec![None; sp.buildings.len()],
            passed: [false; 2],
            ph: APh::Roll,
            round: 0,
            first: 0,
            side: 0,
            dice: Dice::new(seed),
        }
    }
    fn place_opts(&self) -> Vec<(usize, usize)> {
        let me = self.side;
        let mut v = Vec::new();
        for k in 0..2 {
            if !self.used_d[me][k] {
                let n = self.d[me][k] as usize;
                if !self.taken[n - 1] {
                    v.push((n, k + 1));
                }
            }
        }
        if !self.used_d[me][0] && !self.used_d[me][1] {
            let n = (self.d[me][0] + self.d[me][1]) as usize;
            if n <= 12 && !self.taken[n - 1] {
                v.push((n, 3));
            }
        }
        v
    }
    fn next_side(&mut self) {
        let o = 1 - self.side;
        if !self.passed[o] {
            self.side = o;
        }
    }
    fn gl(&self, p: usize) -> i32 {
        self.res[p][3]
    }
}

impl Mini for Adv {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl2(self.side)
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            APh::Roll => vec!["arunc".into()],
            APh::Place => {
                let mut v: Vec<String> = self
                    .place_opts()
                    .iter()
                    .map(|&(n, d)| format!("pun {n} {d}"))
                    .collect();
                v.push("pas".into());
                v
            }
            APh::Build => {
                let me = self.side;
                let mut v = vec!["gata".to_string()];
                for (b, bd) in self.sp.buildings.iter().enumerate() {
                    if self.built[b].is_none()
                        && self.res[me][0] >= bd.0
                        && self.res[me][1] >= bd.1
                        && self.res[me][2] >= bd.2
                    {
                        v.push(format!("constr {}", b + 1));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.side;
        match self.ph {
            APh::Roll => {
                for p in 0..2 {
                    for k in 0..2 {
                        self.d[p][k] = self.dice.below(6) as u8 + 1;
                    }
                }
                self.used_d = [[false; 2]; 2];
                self.taken = [false; 12];
                self.passed = [false; 2];
                self.side = self.first;
                self.ph = APh::Place;
            }
            APh::Place => {
                if m == "pas" {
                    self.passed[me] = true;
                } else {
                    let mut it = m[4..].split(' ').map(|x| x.parse::<usize>().unwrap());
                    let (n, d) = (it.next().unwrap(), it.next().unwrap());
                    self.taken[n - 1] = true;
                    if d == 3 {
                        self.used_d[me] = [true; 2];
                    } else {
                        self.used_d[me][d - 1] = true;
                    }
                    let r = self.sp.advisors[n - 1];
                    self.res[me][0] += r.0;
                    self.res[me][1] += r.1;
                    self.res[me][2] += r.2;
                    self.res[me][3] += r.3;
                    self.res[me][4] += r.4;
                    if self.used_d[me] == [true; 2] {
                        self.passed[me] = true;
                    }
                }
                if self.passed[0] && self.passed[1] {
                    self.ph = APh::Build;
                    self.passed = [false; 2];
                    self.side = self.first;
                } else {
                    self.next_side();
                }
            }
            APh::Build => {
                if m == "gata" {
                    self.passed[me] = true;
                } else {
                    let b = m[7..].parse::<usize>().unwrap() - 1;
                    let bd = self.sp.buildings[b];
                    self.res[me][0] -= bd.0;
                    self.res[me][1] -= bd.1;
                    self.res[me][2] -= bd.2;
                    self.res[me][3] += bd.3;
                    self.built[b] = Some(me);
                }
                if self.passed[0] && self.passed[1] {
                    self.round += 1;
                    if let Some(&(_, force)) = self.sp.invasions.iter().find(|x| x.0 == self.round)
                    {
                        for p in 0..2 {
                            if self.res[p][4] < force {
                                self.res[p][3] = (self.res[p][3] - self.sp.loss).max(0);
                            }
                        }
                    }
                    self.first = 1 - self.first;
                    self.side = self.first;
                    self.ph = APh::Roll;
                } else if !self.passed[1 - me] {
                    self.side = 1 - me;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.sp.rounds {
            return Some(by_score(self.gl(0), self.gl(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let raw =
            |q: usize| self.res[q][0] + self.res[q][1] + 2 * self.res[q][2] + 2 * self.res[q][4];
        100 * (self.gl(a) - self.gl(o)) + 20 * (raw(a) - raw(o))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds
        );
        for p in 0..2 {
            let r = &self.res[p];
            s.push_str(&format!(
                "{}: lemn {}, piatră {}, aur {}, glorie {}, soldați {} | zaruri {} {}\n",
                pname(pl2(p)),
                r[0],
                r[1],
                r[2],
                r[3],
                r[4],
                self.d[p][0],
                self.d[p][1]
            ));
        }
        let t: Vec<String> = (0..12)
            .filter(|&i| self.taken[i])
            .map(|i| (i + 1).to_string())
            .collect();
        s.push_str(&format!("Sfetnici ocupați: {}\n", t.join(",")));
        let bs: Vec<String> = self
            .sp
            .buildings
            .iter()
            .enumerate()
            .map(|(b, x)| {
                format!(
                    "{}) {}l {}p {}a → {}{}",
                    b + 1,
                    x.0,
                    x.1,
                    x.2,
                    x.3,
                    match self.built[b] {
                        Some(p) => format!(" [J{}]", p + 1),
                        None => String::new(),
                    }
                )
            })
            .collect();
        s.push_str(&format!("Clădiri: {}\n", bs.join("; ")));
        s.push_str(&format!(
            "La mutare: {} (arunc | pun n d | pas | constr b | gata)\n",
            pname(self.side())
        ));
        s
    }
}

// ---------------------------------------------------------------- Gal
pub struct GalSpec {
    pub title: &'static str,
    pub rounds: u32,
    pub dice0: usize,
    pub dice_max: usize,
    pub recruit: i32,
    pub settle_cost: i32,
    pub develop_cost: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GPh {
    Roll,
    Assign,
    Recruit,
}

const PH: [&str; 5] = ["explorare", "așezare", "producție", "livrare", "dezvoltare"];

#[derive(Clone)]
pub struct Gal {
    sp: &'static GalSpec,
    // cărți, lumi, mărfuri, credite, glorie
    st: [[i32; 5]; 2],
    nd: [usize; 2],
    face: [Vec<u8>; 2],
    fixed: [Vec<bool>; 2],
    ph: GPh,
    round: u32,
    first: usize,
    side: usize,
    done_first: bool,
    dice: Dice,
}

impl Gal {
    pub fn new(sp: &'static GalSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static GalSpec, seed: u64) -> Self {
        Gal {
            sp,
            st: [[2, 0, 0, 1, 0]; 2],
            nd: [sp.dice0; 2],
            face: [vec![], vec![]],
            fixed: [vec![], vec![]],
            ph: GPh::Roll,
            round: 0,
            first: 0,
            side: 0,
            done_first: false,
            dice: Dice::new(seed),
        }
    }
    fn resolve(&mut self) {
        for ph in 0..5u8 {
            for p in 0..2 {
                let n = self.face[p].iter().filter(|&&f| f == ph).count() as i32;
                let s = &mut self.st[p];
                for _ in 0..n {
                    match ph {
                        0 => s[0] += 1,
                        1 => {
                            if s[0] >= self.sp.settle_cost {
                                s[0] -= self.sp.settle_cost;
                                s[1] += 1;
                                s[4] += 1;
                            }
                        }
                        2 => {
                            if s[2] < s[1] {
                                s[2] += 1;
                            }
                        }
                        3 => {
                            if s[2] > 0 {
                                s[2] -= 1;
                                s[3] += 1;
                                s[4] += 1;
                            }
                        }
                        _ => {
                            if s[0] >= self.sp.develop_cost {
                                s[0] -= self.sp.develop_cost;
                                s[4] += 3;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Mini for Gal {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl2(self.side)
    }
    fn moves(&self) -> Vec<String> {
        let me = self.side;
        match self.ph {
            GPh::Roll => vec!["arunc".into()],
            GPh::Assign => {
                let mut v = vec!["gata".to_string()];
                let open: Vec<usize> = (0..self.nd[me])
                    .filter(|&d| self.face[me][d] == 5)
                    .collect();
                if let Some(&d) = open.first() {
                    for f in 0..5 {
                        v.push(format!("fixez {} {}", d + 1, f + 1));
                    }
                    v.retain(|x| x != "gata");
                } else if self.st[me][3] > 0 {
                    for d in 0..self.nd[me] {
                        if !self.fixed[me][d] {
                            for f in 0..5 {
                                if self.face[me][d] != f as u8 {
                                    v.push(format!("ajust {} {}", d + 1, f + 1));
                                }
                            }
                        }
                    }
                }
                v
            }
            GPh::Recruit => {
                let mut v = vec!["fin".to_string()];
                if self.nd[me] < self.sp.dice_max && self.st[me][3] >= self.sp.recruit {
                    v.push("recrut".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.side;
        match self.ph {
            GPh::Roll => {
                for p in 0..2 {
                    self.face[p] = (0..self.nd[p]).map(|_| self.dice.below(6) as u8).collect();
                    self.fixed[p] = vec![false; self.nd[p]];
                }
                self.ph = GPh::Assign;
                self.side = self.first;
                self.done_first = false;
            }
            GPh::Assign => {
                if m == "gata" {
                    if self.done_first {
                        self.resolve();
                        self.ph = GPh::Recruit;
                        self.side = self.first;
                        self.done_first = false;
                    } else {
                        self.done_first = true;
                        self.side = 1 - self.first;
                    }
                    return;
                }
                let mut it = m.split(' ');
                let cmd = it.next().unwrap();
                let d = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let f = it.next().unwrap().parse::<u8>().unwrap() - 1;
                if cmd == "ajust" {
                    self.st[me][3] -= 1;
                }
                self.face[me][d] = f;
                self.fixed[me][d] = true;
            }
            GPh::Recruit => {
                if m == "recrut" {
                    self.st[me][3] -= self.sp.recruit;
                    self.nd[me] += 1;
                } else if self.done_first {
                    self.round += 1;
                    self.first = 1 - self.first;
                    self.side = self.first;
                    self.ph = GPh::Roll;
                    self.done_first = false;
                } else {
                    self.done_first = true;
                    self.side = 1 - self.first;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.sp.rounds {
            return Some(by_score(self.st[0][4], self.st[1][4]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let raw = |q: usize| {
            self.st[q][0]
                + 2 * self.st[q][1]
                + self.st[q][2]
                + self.st[q][3]
                + 3 * self.nd[q] as i32
        };
        100 * (self.st[a][4] - self.st[o][4]) + 25 * (raw(a) - raw(o))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds
        );
        for p in 0..2 {
            let r = &self.st[p];
            let f: Vec<String> = self.face[p]
                .iter()
                .map(|&x| {
                    if x == 5 {
                        "joker".to_string()
                    } else {
                        PH[x as usize].to_string()
                    }
                })
                .collect();
            s.push_str(&format!(
                "{}: cărți {}, lumi {}, mărfuri {}, credite {}, glorie {}, zaruri {} [{}]\n",
                pname(pl2(p)),
                r[0],
                r[1],
                r[2],
                r[3],
                r[4],
                self.nd[p],
                f.join(", ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | fixez d f | ajust d f | gata | recrut | fin)\n",
            pname(self.side())
        ));
        s
    }
}
