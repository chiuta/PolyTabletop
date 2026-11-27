//! Motor „bătălie de miniaturi” (familia T2, variante originale pentru doi jucători).
//!
//! Tablă pătrățică sau hexagonală cu teren (blocaj `#`, acoperire `+`, teren greu `~`, obiectiv
//! `*`). Fiecare unitate se activează o dată pe rundă, alternativ: mutarea `uN r,c` mută unitatea
//! N în (r,c) (sau o lasă pe loc), iar `uN r,c x uM` mută și atacă unitatea M din raza de
//! acțiune. Atacul se rezolvă prin mutarea forțată `tira` (zaruri d6: lovituri, apoi salvări);
//! AI-ul nu o execută, deci nu vede aruncările viitoare. Informație complet publică.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::type_complexity
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct Ut {
    pub name: &'static str,
    pub hp: i8,
    pub mv: i8,
    /// 1 = corp la corp
    pub rng: i8,
    pub dice: i8,
    /// lovește la d6 ≥ tn
    pub tn: i8,
    /// salvare la d6 ≥ save (7 = fără)
    pub save: i8,
    pub dmg: i8,
    pub cost: i8,
}

pub const FLANK: u16 = 1;
pub const CHARGE: u16 = 2;
pub const MORALE: u16 = 4;
pub const RETAL: u16 = 8;
pub const MOVEFIRE: u16 = 16;
pub const DIAG: u16 = 32;

pub struct Cfg {
    pub title: &'static str,
    pub hex: bool,
    pub w: i8,
    pub h: i8,
    pub types: &'static [Ut],
    /// armata lui J1: (tip, rând, coloană); J2 e oglindita prin centru
    pub army: &'static [(u8, i8, i8)],
    /// teren: (rând, coloană, fel) 1 blocaj, 2 acoperire, 3 obiectiv, 4 teren greu
    pub terrain: &'static [(i8, i8, u8)],
    pub flags: u16,
    pub rounds: u32,
}

#[derive(Clone)]
struct U {
    t: usize,
    owner: usize,
    r: i8,
    c: i8,
    hp: i8,
    act: bool,
    shaken: bool,
    moved: i8,
}

#[derive(Clone)]
pub struct Mn {
    cfg: &'static Cfg,
    u: Vec<U>,
    turn: usize,
    round: u32,
    score: [i32; 2],
    pend: Option<(usize, usize)>,
    rng: Rng,
}

fn p6(tn: i32) -> i32 {
    ((7 - tn) * 100 / 6).clamp(0, 100)
}

impl Mn {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let mut u = Vec::new();
        for o in 0..2 {
            for &(t, r, c) in cfg.army {
                let (r, c) = if o == 0 {
                    (r, c)
                } else {
                    (cfg.h - 1 - r, cfg.w - 1 - c)
                };
                u.push(U {
                    t: t as usize,
                    owner: o,
                    r,
                    c,
                    hp: cfg.types[t as usize].hp,
                    act: false,
                    shaken: false,
                    moved: 0,
                });
            }
        }
        Mn {
            cfg,
            u,
            turn: 0,
            round: 0,
            score: [0; 2],
            pend: None,
            rng: Rng::new(seed),
        }
    }
    fn has(&self, f: u16) -> bool {
        self.cfg.flags & f != 0
    }
    fn ut(&self, i: usize) -> &'static Ut {
        &self.cfg.types[self.u[i].t]
    }
    fn alive(&self, i: usize) -> bool {
        self.u[i].hp > 0
    }
    fn terr(&self, r: i8, c: i8) -> u8 {
        self.cfg
            .terrain
            .iter()
            .find(|x| x.0 == r && x.1 == c)
            .map(|x| x.2)
            .unwrap_or(0)
    }
    fn inside(&self, r: i8, c: i8) -> bool {
        r >= 0 && c >= 0 && r < self.cfg.h && c < self.cfg.w
    }
    fn dist(&self, a: (i8, i8), b: (i8, i8)) -> i32 {
        let dr = (a.0 - b.0) as i32;
        let dc = (a.1 - b.1) as i32;
        if self.cfg.hex {
            (dr.abs() + dc.abs() + (dr + dc).abs()) / 2
        } else if self.has(DIAG) {
            dr.abs().max(dc.abs())
        } else {
            dr.abs() + dc.abs()
        }
    }
    fn nbrs(&self, r: i8, c: i8) -> Vec<(i8, i8)> {
        let mut v = Vec::new();
        if self.cfg.hex {
            for (dq, dr) in HEX {
                v.push((r + dr as i8, c + dq as i8));
            }
        } else {
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                v.push((r + dr, c + dc));
            }
            if self.has(DIAG) {
                for (dr, dc) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    v.push((r + dr, c + dc));
                }
            }
        }
        v.into_iter().filter(|&(a, b)| self.inside(a, b)).collect()
    }
    fn occupied(&self, r: i8, c: i8) -> bool {
        (0..self.u.len()).any(|j| self.alive(j) && self.u[j].r == r && self.u[j].c == c)
    }
    /// Celulele atinse de unitatea `i` (cu costul în puncte de mișcare).
    fn reach(&self, i: usize) -> Vec<((i8, i8), i32)> {
        let mv = self.ut(i).mv as i32;
        let start = (self.u[i].r, self.u[i].c);
        let mut best: Vec<((i8, i8), i32)> = vec![(start, 0)];
        let mut k = 0;
        while k < best.len() {
            let (p, d) = best[k];
            k += 1;
            for q in self.nbrs(p.0, p.1) {
                let t = self.terr(q.0, q.1);
                if t == 1 || self.occupied(q.0, q.1) {
                    continue;
                }
                let nd = d + if t == 4 { 2 } else { 1 };
                if nd > mv {
                    continue;
                }
                match best.iter().position(|x| x.0 == q) {
                    Some(j) => {
                        if best[j].1 > nd {
                            best[j].1 = nd;
                            // reparcurgem vecinii
                            if j < k {
                                k = j;
                            }
                        }
                    }
                    None => best.push((q, nd)),
                }
            }
        }
        best
    }
    fn has_left(&self, p: usize) -> bool {
        (0..self.u.len()).any(|j| self.u[j].owner == p && self.alive(j) && !self.u[j].act)
    }
    fn count(&self, p: usize) -> usize {
        (0..self.u.len())
            .filter(|&j| self.u[j].owner == p && self.alive(j))
            .count()
    }
    fn objectives(&self) -> Vec<(i8, i8)> {
        self.cfg
            .terrain
            .iter()
            .filter(|x| x.2 == 3)
            .map(|x| (x.0, x.1))
            .collect()
    }
    fn control(&self, o: (i8, i8)) -> Option<usize> {
        let mut n = [0; 2];
        for j in 0..self.u.len() {
            if self.alive(j) && self.dist((self.u[j].r, self.u[j].c), o) <= 1 {
                n[self.u[j].owner] += 1;
            }
        }
        match n[0].cmp(&n[1]) {
            std::cmp::Ordering::Greater => Some(0),
            std::cmp::Ordering::Less => Some(1),
            _ => None,
        }
    }
    fn end_round(&mut self) {
        for o in self.objectives() {
            if let Some(p) = self.control(o) {
                self.score[p] += 2;
            }
        }
        self.round += 1;
        for j in 0..self.u.len() {
            self.u[j].act = false;
            self.u[j].moved = 0;
            if self.has(MORALE) {
                let mx = self.cfg.types[self.u[j].t].hp as i32;
                self.u[j].shaken = self.u[j].hp > 0 && self.u[j].hp as i32 * 3 <= mx;
            }
        }
        self.turn = (self.round % 2) as usize;
        if !self.has_left(self.turn) {
            self.turn = 1 - self.turn;
        }
    }
    fn advance(&mut self) {
        let o = 1 - self.turn;
        if self.has_left(o) {
            self.turn = o;
        } else if !self.has_left(self.turn) {
            self.end_round();
        }
    }
    /// Dați zarurile (atacator `a`, apărător `d`). Întoarce rănile cauzate.
    fn strike(&mut self, a: usize, d: usize, retal: bool) {
        let ua = self.ut(a);
        let ud = self.ut(d);
        let mut dice = ua.dice as i32;
        if retal {
            dice = (dice + 1) / 2;
        }
        let dpos = (self.u[d].r, self.u[d].c);
        if self.has(FLANK) {
            let friend = (0..self.u.len()).any(|j| {
                j != a
                    && self.alive(j)
                    && self.u[j].owner == self.u[a].owner
                    && self.dist((self.u[j].r, self.u[j].c), dpos) <= 1
            });
            if friend {
                dice += 1;
            }
        }
        if !retal && self.has(CHARGE) && ua.rng == 1 && self.u[a].moved >= 2 {
            dice += 1;
        }
        if !retal && self.has(MOVEFIRE) && ua.rng > 1 && self.u[a].moved > 0 {
            dice -= 1;
        }
        if self.u[a].shaken {
            dice -= 1;
        }
        let dice = dice.max(1);
        let mut hits = 0;
        for _ in 0..dice {
            if 1 + self.rng.below(6) as i32 >= ua.tn as i32 {
                hits += 1;
            }
        }
        let mut save = ud.save as i32;
        if self.terr(dpos.0, dpos.1) == 2 {
            save = if save >= 7 { 6 } else { (save - 1).max(2) };
        }
        let mut wounds = 0;
        for _ in 0..hits {
            if save >= 7 || (1 + self.rng.below(6) as i32) < save {
                wounds += ua.dmg as i32;
            }
        }
        let dealt = wounds.min(self.u[d].hp as i32);
        self.u[d].hp -= dealt as i8;
        self.score[self.u[a].owner] += dealt * ud.cost as i32 / ud.hp as i32;
        if self.u[d].hp <= 0 {
            self.score[self.u[a].owner] += ud.cost as i32;
        } else if self.has(MORALE) && self.u[d].hp as i32 * 3 <= ud.hp as i32 {
            self.u[d].shaken = true;
        }
    }
    /// Valoarea așteptată (în „puncte”) a unui atac `a` -> `d`.
    fn expect(&self, a: usize, d: usize) -> i32 {
        let ua = self.ut(a);
        let ud = self.ut(d);
        let mut dice = ua.dice as i32;
        let dpos = (self.u[d].r, self.u[d].c);
        if self.has(FLANK)
            && (0..self.u.len()).any(|j| {
                j != a
                    && self.alive(j)
                    && self.u[j].owner == self.u[a].owner
                    && self.dist((self.u[j].r, self.u[j].c), dpos) <= 1
            })
        {
            dice += 1;
        }
        if self.has(CHARGE) && ua.rng == 1 && self.u[a].moved >= 2 {
            dice += 1;
        }
        if self.has(MOVEFIRE) && ua.rng > 1 && self.u[a].moved > 0 {
            dice -= 1;
        }
        if self.u[a].shaken {
            dice -= 1;
        }
        let dice = dice.max(1);
        let mut save = ud.save as i32;
        if self.terr(dpos.0, dpos.1) == 2 {
            save = if save >= 7 { 6 } else { (save - 1).max(2) };
        }
        let pass = 100 - p6(save); // șansa ca rana să treacă
        let wounds100 = dice * p6(ua.tn as i32) * pass / 100 * ua.dmg as i32;
        let w = wounds100.min(self.u[d].hp as i32 * 100);
        w * ud.cost as i32 / ud.hp as i32 / 10
    }
    fn value(&self, p: usize) -> i32 {
        (0..self.u.len())
            .filter(|&j| self.u[j].owner == p && self.alive(j))
            .map(|j| self.ut(j).cost as i32 * self.u[j].hp as i32 * 10 / self.ut(j).hp as i32)
            .sum()
    }
    fn pull(&self, p: usize) -> i32 {
        // distanța medie spre obiectiv / cel mai apropiat inamic (mai mică = mai bine)
        let obj = self.objectives();
        let mut tot = 0;
        for j in 0..self.u.len() {
            if self.u[j].owner != p || !self.alive(j) {
                continue;
            }
            let me = (self.u[j].r, self.u[j].c);
            let mut best = i32::MAX;
            for k in 0..self.u.len() {
                if self.u[k].owner != p && self.alive(k) {
                    best = best.min(self.dist(me, (self.u[k].r, self.u[k].c)));
                }
            }
            for &o in &obj {
                best = best.min(self.dist(me, o) + 1);
            }
            if best != i32::MAX {
                tot += best;
            }
        }
        tot
    }
}

impl Mini for Mn {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.turn == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["tira".into()];
        }
        let mut v = Vec::new();
        for i in 0..self.u.len() {
            if self.u[i].owner != self.turn || !self.alive(i) || self.u[i].act {
                continue;
            }
            let ut = self.ut(i);
            for ((r, c), _) in self.reach(i) {
                v.push(format!("u{i} {r},{c}"));
                for j in 0..self.u.len() {
                    if self.u[j].owner != self.turn
                        && self.alive(j)
                        && self.dist((r, c), (self.u[j].r, self.u[j].c)) <= ut.rng as i32
                    {
                        v.push(format!("u{i} {r},{c} x u{j}"));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        if m == "tira" {
            let (a, d) = self.pend.take().unwrap();
            self.strike(a, d, false);
            if self.has(RETAL)
                && self.alive(d)
                && self.alive(a)
                && self.ut(a).rng == 1
                && self.ut(d).rng == 1
            {
                self.strike(d, a, true);
            }
            self.advance();
            return;
        }
        let parts: Vec<&str> = m.split(' ').collect();
        let i: usize = parts[0][1..].parse().unwrap();
        let (r, c) = parts[1].split_once(',').unwrap();
        let (r, c): (i8, i8) = (r.parse().unwrap(), c.parse().unwrap());
        let cost = self
            .reach(i)
            .iter()
            .find(|x| x.0 == (r, c))
            .map(|x| x.1)
            .unwrap_or(0);
        self.u[i].r = r;
        self.u[i].c = c;
        self.u[i].moved = cost as i8;
        self.u[i].act = true;
        if parts.len() == 4 {
            let j: usize = parts[3][1..].parse().unwrap();
            self.pend = Some((i, j));
        } else {
            self.advance();
        }
    }
    fn result(&self) -> Option<Outcome> {
        let a = self.count(0);
        let b = self.count(1);
        if a == 0 && b == 0 {
            return Some(Outcome::Draw);
        }
        if a == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if b == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.round >= self.cfg.rounds {
            let x = (self.score[0], self.value(0));
            let y = (self.score[1], self.value(1));
            return Some(match x.cmp(&y) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut s = 12 * (self.score[i] - self.score[o]) + (self.value(i) - self.value(o)) * 2;
        for ob in self.objectives() {
            match self.control(ob) {
                Some(x) if x == i => s += 14,
                Some(_) => s -= 14,
                None => {}
            }
        }
        s += (self.pull(o) - self.pull(i)) / 2;
        if let Some((a, d)) = self.pend {
            let e = self.expect(a, d) * 6;
            if self.u[a].owner == i {
                s += e;
            } else {
                s -= e;
            }
        }
        s
    }
    fn show(&self) -> String {
        let cfg = self.cfg;
        let mut s = format!(
            "{} — runda {}/{} | puncte J1 {} / J2 {} | la mutare J{}\n",
            cfg.title,
            (self.round + 1).min(cfg.rounds),
            cfg.rounds,
            self.score[0],
            self.score[1],
            self.turn + 1
        );
        for r in 0..cfg.h {
            if cfg.hex {
                s.push_str(&" ".repeat(r as usize));
            }
            for c in 0..cfg.w {
                let ch = if let Some(j) = (0..self.u.len())
                    .find(|&j| self.alive(j) && self.u[j].r == r && self.u[j].c == c)
                {
                    let k = j % cfg.army.len();
                    if self.u[j].owner == 0 {
                        (b'A' + k as u8) as char
                    } else {
                        (b'a' + k as u8) as char
                    }
                } else {
                    match self.terr(r, c) {
                        1 => '#',
                        2 => '+',
                        3 => '*',
                        4 => '~',
                        _ => '.',
                    }
                };
                s.push(ch);
                s.push(' ');
            }
            s.push_str(&format!("  r{r}\n"));
        }
        s.push_str("coloane 0..: ");
        for c in 0..cfg.w {
            s.push_str(&format!("{c} "));
        }
        s.push('\n');
        for j in 0..self.u.len() {
            if !self.alive(j) {
                continue;
            }
            let ut = self.ut(j);
            let k = j % cfg.army.len();
            let ch = if self.u[j].owner == 0 {
                (b'A' + k as u8) as char
            } else {
                (b'a' + k as u8) as char
            };
            s.push_str(&format!(
                "  {ch} = u{j} J{} {} PV {}/{} mișc {} rază {} zaruri {} (≥{}) salv {}{}{}\n",
                self.u[j].owner + 1,
                ut.name,
                self.u[j].hp,
                ut.hp,
                ut.mv,
                ut.rng,
                ut.dice,
                ut.tn,
                if ut.save >= 7 {
                    "-".to_string()
                } else {
                    format!("{}+", ut.save)
                },
                if self.u[j].act { " [activată]" } else { "" },
                if self.u[j].shaken { " [șovăie]" } else { "" }
            ));
        }
        if self.pend.is_some() {
            s.push_str("Atacul așteaptă aruncarea: tira\n");
        }
        s
    }
}

#[cfg(test)]
impl Mn {
    fn reseed(&mut self, s: u64) {
        self.rng = Rng::new(s);
    }
}

#[cfg(test)]
pub fn check_cfg(cfg: &'static Cfg) {
    crate::t2_util::check_fn(&|s| Mn::with_seed(cfg, s), 900, Mn::reseed);
}
