//! Motor „zaruri de împărțit și completat” (lista C6): cinci variante originale, 2 jucători.
//! `vitraliul_zarurilor`, `harta_cu_cifre`, `colorat_si_barat`, `drumuri_si_sine`, `eroul_din_zaruri`.
//!
//! La începutul fiecărei runde se aruncă zarurile comune (mutarea forțată `tira`); apoi jucătorii
//! aleg pe rând câte un zar și îl pun pe propria tablă (`pon …`) sau îl aruncă (`salt d`).
//! Zarurile rămase la sfârșitul rundei se pierd. `eval` nu citește generatorul.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Vitraliu,
    Harta,
    Colorat,
    Drumuri,
    Erou,
}

#[derive(Clone)]
pub struct Grid {
    kind: Kind,
    rng: u64,
    round: u32,
    rounds: u32,
    /// zaruri comune: (culoare, valoare)
    pool: Vec<(u8, u8)>,
    picks: usize,
    board: [Vec<i8>; 2],
    first: usize,
    rolled: bool,
    last: String,
}

const REGIONS: [&str; 5] = ["AABBC", "ADBCC", "DDECC", "DEEEC", "EEAAB"];
const HERO_W: [i32; 4] = [3, 2, 2, 1];

fn dims(k: Kind) -> usize {
    match k {
        Kind::Vitraliu | Kind::Harta => 16,
        Kind::Colorat | Kind::Drumuri => 25,
        Kind::Erou => 12,
    }
}

fn rot(m: u8) -> u8 {
    ((m << 1) | (m >> 3)) & 15
}

impl Grid {
    pub fn new(kind: Kind) -> Self {
        Self::with_seed(kind, seed_now())
    }
    pub fn with_seed(kind: Kind, seed: u64) -> Self {
        let rounds = match kind {
            Kind::Vitraliu => 4,
            Kind::Harta => 8,
            Kind::Colorat => 8,
            Kind::Drumuri => 7,
            Kind::Erou => 6,
        };
        let mut b = vec![-1i8; dims(kind)];
        if kind == Kind::Colorat {
            b = vec![0; 25];
        }
        if kind == Kind::Drumuri {
            b[12] = 15;
        }
        Grid {
            kind,
            rng: seed_state(seed),
            round: 0,
            rounds,
            pool: Vec::new(),
            picks: 0,
            board: [b.clone(), b],
            first: 0,
            rolled: false,
            last: String::new(),
        }
    }
    fn pool_size(&self) -> usize {
        if self.kind == Kind::Vitraliu {
            5
        } else {
            4
        }
    }
    fn side_idx(&self) -> usize {
        if self.picks % 2 == 0 {
            self.first
        } else {
            1 - self.first
        }
    }
    fn region(r: usize, c: usize) -> u8 {
        REGIONS[r].as_bytes()[c] - b'A'
    }

    // ---- legalitate pe tablă
    fn sag_ok(b: &[i8], r: usize, c: usize, col: u8, v: u8) -> bool {
        if b[r * 4 + c] != -1 {
            return false;
        }
        let any = b.iter().any(|&x| x != -1);
        let border = r == 0 || c == 0 || r == 3 || c == 3;
        if !any {
            return border;
        }
        let mut adj = false;
        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                if !(0..4).contains(&nr) || !(0..4).contains(&nc) {
                    continue;
                }
                let x = b[(nr * 4 + nc) as usize];
                if x == -1 {
                    continue;
                }
                adj = true;
                if dr == 0 || dc == 0 {
                    let (xc, xv) = ((x / 8) as u8, (x % 8) as u8);
                    if xc == col || xv == v {
                        return false;
                    }
                }
            }
        }
        adj
    }
    fn harta_ok(b: &[i8], r: usize, c: usize, v: u8) -> bool {
        if b[r * 4 + c] != -1 {
            return false;
        }
        for k in 0..4 {
            let x = b[r * 4 + k];
            if x != -1 && ((k < c && (x as u8) > v) || (k > c && (x as u8) < v)) {
                return false;
            }
            let y = b[k * 4 + c];
            if y != -1 && ((k < r && (y as u8) > v) || (k > r && (y as u8) < v)) {
                return false;
            }
        }
        true
    }
    fn paint_cells(b: &[i8], col: u8, num: usize) -> Vec<(usize, usize)> {
        // început + sens (spre dreapta): num celule libere consecutive, de aceeași regiune
        let mut out = Vec::new();
        let any = b.contains(&1);
        for r in 0..5 {
            for c in 0..=(5 - num) {
                let cells: Vec<(usize, usize)> = (0..num).map(|k| (r, c + k)).collect();
                if !cells
                    .iter()
                    .all(|&(rr, cc)| b[rr * 5 + cc] == 0 && Self::region(rr, cc) == col)
                {
                    continue;
                }
                let touch = if !any {
                    cells.iter().any(|&(_, cc)| cc == 2)
                } else {
                    cells.iter().any(|&(rr, cc)| {
                        [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)]
                            .iter()
                            .any(|&(dr, dc)| {
                                let (nr, nc) = (rr as i32 + dr, cc as i32 + dc);
                                (0..5).contains(&nr)
                                    && (0..5).contains(&nc)
                                    && b[(nr * 5 + nc) as usize] == 1
                            })
                    })
                };
                if touch {
                    out.push((r, c));
                }
            }
        }
        out
    }
    fn open(m: i8, dir: usize) -> bool {
        m >= 0 && (m as u8) >> dir & 1 == 1
    }
    fn rail_ok(b: &[i8], r: usize, c: usize, m: u8) -> bool {
        if b[r * 5 + c] != -1 {
            return false;
        }
        // N=0,E=1,S=2,W=3
        let nb = [(-1i32, 0i32, 2usize), (0, 1, 3), (1, 0, 0), (0, -1, 1)];
        for (d, &(dr, dc, back)) in nb.iter().enumerate() {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if !(0..5).contains(&nr) || !(0..5).contains(&nc) {
                continue;
            }
            let x = b[(nr * 5 + nc) as usize];
            if m >> d & 1 == 1 && Self::open(x, back) {
                return true;
            }
        }
        false
    }
    fn rail_net(b: &[i8]) -> (i32, i32) {
        let nb = [(-1i32, 0i32, 2usize), (0, 1, 3), (1, 0, 0), (0, -1, 1)];
        let mut seen = [false; 25];
        let mut st = vec![12usize];
        seen[12] = true;
        let (mut n, mut dead) = (0, 0);
        while let Some(i) = st.pop() {
            n += 1;
            let (r, c) = ((i / 5) as i32, (i % 5) as i32);
            for (d, &(dr, dc, back)) in nb.iter().enumerate() {
                if !Self::open(b[i], d) {
                    continue;
                }
                let (nr, nc) = (r + dr, c + dc);
                if !(0..5).contains(&nr) || !(0..5).contains(&nc) {
                    dead += 1;
                    continue;
                }
                let j = (nr * 5 + nc) as usize;
                if Self::open(b[j], back) {
                    if !seen[j] {
                        seen[j] = true;
                        st.push(j);
                    }
                } else {
                    dead += 1;
                }
            }
        }
        (n, dead)
    }
    fn hero_mod(sum: i32) -> i32 {
        match sum {
            ..=6 => -1,
            7..=9 => 0,
            10..=12 => 1,
            13..=15 => 2,
            _ => 3,
        }
    }

    /// Scorul curent al jucătorului `p` (în puncte întregi).
    pub fn score(&self, p: usize) -> i32 {
        let b = &self.board[p];
        match self.kind {
            Kind::Vitraliu => {
                let mut s = b.iter().filter(|&&x| x != -1).count() as i32;
                for r in 0..4 {
                    let v: Vec<i8> = (0..4).map(|c| b[r * 4 + c]).collect();
                    if v.iter().all(|&x| x != -1) {
                        let mut vals: Vec<i8> = v.iter().map(|x| x % 8).collect();
                        vals.sort_unstable();
                        vals.dedup();
                        if vals.len() == 4 {
                            s += 4;
                        }
                    }
                    let w: Vec<i8> = (0..4).map(|c| b[c * 4 + r]).collect();
                    if w.iter().all(|&x| x != -1) {
                        let mut cols: Vec<i8> = w.iter().map(|x| x / 8).collect();
                        cols.sort_unstable();
                        cols.dedup();
                        if cols.len() == 4 {
                            s += 5;
                        }
                    }
                }
                s
            }
            Kind::Harta => {
                let mut s = b.iter().filter(|&&x| x != -1).count() as i32;
                for k in 0..4 {
                    if (0..4).all(|c| b[k * 4 + c] != -1) {
                        s += 3;
                    }
                    if (0..4).all(|r| b[r * 4 + k] != -1) {
                        s += 3;
                    }
                }
                s
            }
            Kind::Colorat => {
                let mut s = b.iter().filter(|&&x| x == 1).count() as i32;
                for c in 0..5 {
                    if (0..5).all(|r| b[r * 5 + c] == 1) {
                        let other = self.board[1 - p].clone();
                        s += if (0..5).all(|r| other[r * 5 + c] == 1) {
                            2
                        } else {
                            4
                        };
                    }
                }
                for reg in 0..5u8 {
                    let all = (0..5)
                        .all(|r| (0..5).all(|c| Self::region(r, c) != reg || b[r * 5 + c] == 1));
                    if all {
                        s += 3;
                    }
                }
                s
            }
            Kind::Drumuri => {
                let (n, dead) = Self::rail_net(b);
                (n * 2 - dead).max(0)
            }
            Kind::Erou => {
                let mut s = 0;
                for a in 0..4 {
                    let sl = &b[a * 3..a * 3 + 3];
                    let sum: i32 = sl.iter().filter(|&&x| x != -1).map(|&x| x as i32).sum();
                    let n = sl.iter().filter(|&&x| x != -1).count() as i32;
                    if n == 3 {
                        s += HERO_W[a] * Self::hero_mod(sum) + 2;
                    } else {
                        s += n; // progres parțial
                    }
                }
                s
            }
        }
    }
}

impl Mini for Grid {
    fn side(&self) -> Player {
        pl(if self.rolled {
            self.side_idx()
        } else {
            self.first
        })
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if !self.rolled {
            return vec!["tira".into()];
        }
        let p = self.side_idx();
        let b = &self.board[p];
        let mut v = Vec::new();
        for (d, &(col, val)) in self.pool.iter().enumerate() {
            v.push(format!("salt {}", d + 1));
            match self.kind {
                Kind::Vitraliu => {
                    for r in 0..4 {
                        for c in 0..4 {
                            if Self::sag_ok(b, r, c, col, val) {
                                v.push(format!("pon {} {} {}", d + 1, r + 1, c + 1));
                            }
                        }
                    }
                }
                Kind::Harta => {
                    for r in 0..4 {
                        for c in 0..4 {
                            if Self::harta_ok(b, r, c, val) {
                                v.push(format!("pon {} {} {}", d + 1, r + 1, c + 1));
                            }
                        }
                    }
                }
                Kind::Colorat => {
                    for (r, c) in Self::paint_cells(b, col, val as usize) {
                        v.push(format!("pon {} {} {}", d + 1, r + 1, c + 1));
                    }
                }
                Kind::Drumuri => {
                    let base = [5u8, 3, 7, 15][col as usize];
                    let mut ms = vec![base];
                    for _ in 0..3 {
                        let n = rot(*ms.last().unwrap());
                        if !ms.contains(&n) {
                            ms.push(n);
                        }
                    }
                    for (o, &m) in ms.iter().enumerate() {
                        for r in 0..5 {
                            for c in 0..5 {
                                if Self::rail_ok(b, r, c, m) {
                                    v.push(format!("pon {} {} {} {}", d + 1, r + 1, c + 1, o));
                                }
                            }
                        }
                    }
                }
                Kind::Erou => {
                    for a in 0..4 {
                        if b[a * 3..a * 3 + 3].contains(&-1) {
                            v.push(format!("pon {} {}", d + 1, a + 1));
                        }
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        if !self.rolled {
            let n = self.pool_size();
            self.pool = (0..n)
                .map(|_| match self.kind {
                    Kind::Vitraliu => (die(&mut self.rng, 5), die(&mut self.rng, 6) + 1),
                    Kind::Harta | Kind::Erou => (0, die(&mut self.rng, 6) + 1),
                    Kind::Colorat => (die(&mut self.rng, 5), die(&mut self.rng, 5) + 1),
                    Kind::Drumuri => (weighted(&mut self.rng, &[3, 3, 2, 1]) as u8, 0),
                })
                .collect();
            self.rolled = true;
            self.picks = 0;
            return;
        }
        let p = self.side_idx();
        let mut it = m.split(' ');
        let kind = it.next().unwrap();
        let d: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let (col, val) = self.pool.remove(d);
        if kind == "pon" {
            let a: Vec<usize> = it.map(|x| x.parse::<usize>().unwrap()).collect();
            let b = &mut self.board[p];
            match self.kind {
                Kind::Vitraliu => b[(a[0] - 1) * 4 + a[1] - 1] = (col * 8 + val) as i8,
                Kind::Harta => b[(a[0] - 1) * 4 + a[1] - 1] = val as i8,
                Kind::Colorat => {
                    for k in 0..val as usize {
                        b[(a[0] - 1) * 5 + a[1] - 1 + k] = 1;
                    }
                }
                Kind::Drumuri => {
                    let base = [5u8, 3, 7, 15][col as usize];
                    let mut mm = base;
                    let mut ms = vec![base];
                    for _ in 0..3 {
                        mm = rot(mm);
                        if !ms.contains(&mm) {
                            ms.push(mm);
                        }
                    }
                    b[(a[0] - 1) * 5 + a[1] - 1] = ms[a[2]] as i8;
                }
                Kind::Erou => {
                    let at = a[0] - 1;
                    let k = b[at * 3..at * 3 + 3].iter().position(|&x| x == -1).unwrap();
                    b[at * 3 + k] = val as i8;
                }
            }
        }
        self.picks += 1;
        if self.picks >= 4 || self.pool.is_empty() {
            self.pool.clear();
            self.rolled = false;
            self.round += 1;
            self.first = 1 - self.first;
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.rounds {
            Some(by_score(self.score(0), self.score(1)))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.score(a) - self.score(1 - a))
    }
    fn show(&self) -> String {
        let name = match self.kind {
            Kind::Vitraliu => "Vitraliul zarurilor",
            Kind::Harta => "Harta cu cifre",
            Kind::Colorat => "Colorat și bărat",
            Kind::Drumuri => "Drumuri și șine",
            Kind::Erou => "Eroul din zaruri",
        };
        let mut s = format!(
            "{name} — runda {}/{} | la mutare {}\n",
            (self.round + 1).min(self.rounds),
            self.rounds,
            pname(self.side())
        );
        let w = match self.kind {
            Kind::Vitraliu | Kind::Harta => 4,
            Kind::Erou => 3,
            _ => 5,
        };
        for p in 0..2 {
            s.push_str(&format!("Tabla J{} (scor {}):\n", p + 1, self.score(p)));
            for row in self.board[p].chunks(w) {
                let line: Vec<String> = row
                    .iter()
                    .map(|&x| match self.kind {
                        Kind::Vitraliu if x != -1 => {
                            format!("{}{}", (b'a' + (x / 8) as u8) as char, x % 8)
                        }
                        Kind::Colorat => {
                            if x == 1 {
                                "X".into()
                            } else {
                                ".".into()
                            }
                        }
                        _ => {
                            if x == -1 {
                                ".".to_string()
                            } else {
                                x.to_string()
                            }
                        }
                    })
                    .collect();
                s.push_str(&format!("  {}\n", line.join(" ")));
            }
        }
        if self.kind == Kind::Colorat {
            s.push_str(&format!("Regiuni (litere): {}\n", REGIONS.join(" ")));
        }
        let pool: Vec<String> = self
            .pool
            .iter()
            .enumerate()
            .map(|(i, &(c, v))| match self.kind {
                Kind::Vitraliu => format!("{}:{}{}", i + 1, (b'a' + c) as char, v),
                Kind::Colorat => format!("{}:{}{}", i + 1, (b'A' + c) as char, v),
                Kind::Drumuri => format!(
                    "{}:{}",
                    i + 1,
                    ["dreaptă", "curbă", "T", "cruce"][c as usize]
                ),
                _ => format!("{}:{}", i + 1, v),
            })
            .collect();
        s.push_str(&format!("Zaruri comune: {}\n{}", pool.join(" "), self.last));
        s.push_str("Mutări: tira | pon d … | salt d\n");
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;

    fn full(k: Kind) {
        match k {
            Kind::Vitraliu => check(|s| Grid::with_seed(Kind::Vitraliu, s), 200),
            Kind::Harta => check(|s| Grid::with_seed(Kind::Harta, s), 300),
            Kind::Colorat => check(|s| Grid::with_seed(Kind::Colorat, s), 300),
            Kind::Drumuri => check(|s| Grid::with_seed(Kind::Drumuri, s), 300),
            Kind::Erou => check(|s| Grid::with_seed(Kind::Erou, s), 200),
        }
        let mut g = Grid::with_seed(k, 4);
        advance(&mut g, 40, |m| m.rolled);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn vitraliu() {
        full(Kind::Vitraliu);
    }
    #[test]
    fn harta() {
        full(Kind::Harta);
    }
    #[test]
    fn colorat() {
        full(Kind::Colorat);
    }
    #[test]
    fn drumuri() {
        full(Kind::Drumuri);
    }
    #[test]
    fn erou() {
        full(Kind::Erou);
    }
    #[test]
    fn drumuri_scor_start() {
        let g = Grid::with_seed(Kind::Drumuri, 1);
        // crucea de la centru: 1 piesă, 4 capete moarte
        assert_eq!(g.score(0), 0);
    }
    #[test]
    fn erou_mod() {
        assert_eq!(Grid::hero_mod(6), -1);
        assert_eq!(Grid::hero_mod(18), 3);
    }
}
