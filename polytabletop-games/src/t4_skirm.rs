//! Motor „miniaturi pe grilă” (lista T4, 2 jucători): unități cu mișcare, atac cu zaruri, teren,
//! obiective (eliminare, lider, controlul unor puncte, pradă, minge, spargerea liniei).
//! Fiecare joc are propria tablă, unități, tipuri de zar și reguli suplimentare (`Spec` în `t4_sdata`).
//! Zarurile sunt aruncate la aplicarea mutării cu un generator re-însămânțat la clonare:
//! căutarea AI nu vede niciodată aruncările viitoare. Informație complet deschisă.
#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]

use crate::brand_util::*;
use crate::w7_util::{cell_name, parse_cell, Dice};
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Grid {
    Sq8,
    Sq4,
    Hex,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Roll {
    /// n zaruri, fiecare ≥ ținta apărătorului lovește
    Count,
    /// 2d6 + n ≥ 5 + ținta
    Sum,
    /// cel mai bun zar al atacatorului contra celui mai bun al apărătorului
    Opp,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Obj {
    Elim,
    Leader,
    Hold,
    Loot,
    Ball,
    Breach,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ap {
    Fix(u8),
    Roll,
    Scale,
    Cmd,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pat {
    Open,
    Pillars,
    Rubble,
    Corridors,
    Ridge,
    Ruins,
    Hills,
}

pub const LEADER: u16 = 1;
pub const FLY: u16 = 2;
pub const BLAST: u16 = 4;
pub const HEAL: u16 = 8;
pub const MOBILE: u16 = 16;
pub const HEAVY: u16 = 32;
pub const DIAL: u16 = 64;

pub const CHARGE: u32 = 1;
pub const FLANK: u32 = 2;
pub const SUPPRESS: u32 = 4;
pub const MORALE: u32 = 8;
pub const ZOC: u32 = 16;
pub const COUNTER: u32 = 32;
pub const PUSH: u32 = 64;

pub struct Kind {
    pub name: &'static str,
    pub ch: char,
    pub hp: i8,
    pub mv: u8,
    pub rng: u8,
    pub dice: u8,
    pub tn: u8,
    pub dmg: u8,
    pub val: u8,
    pub fl: u16,
}

pub struct Spec {
    pub id: &'static str,
    pub w: i8,
    pub h: i8,
    pub grid: Grid,
    pub pat: Pat,
    pub kinds: &'static [Kind],
    pub army: [&'static [u8]; 2],
    pub ap: Ap,
    pub roll: Roll,
    pub obj: Obj,
    pub rounds: u8,
    pub goal: u8,
    pub mods: u32,
    pub loot: u8,
}

type P = (i8, i8);

fn nbrs(sp: &Spec, x: i8, y: i8) -> Vec<P> {
    const S8: [P; 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    const S4: [P; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    const HE: [P; 6] = [(1, 0), (-1, 0), (0, -1), (-1, -1), (0, 1), (-1, 1)];
    const HO: [P; 6] = [(1, 0), (-1, 0), (1, -1), (0, -1), (1, 1), (0, 1)];
    let d: &[P] = match sp.grid {
        Grid::Sq8 => &S8,
        Grid::Sq4 => &S4,
        Grid::Hex => {
            if y & 1 == 0 {
                &HE
            } else {
                &HO
            }
        }
    };
    d.iter()
        .map(|&(dx, dy)| (x + dx, y + dy))
        .filter(|&(a, b)| a >= 0 && b >= 0 && a < sp.w && b < sp.h)
        .collect()
}

fn dist(sp: &Spec, a: P, b: P) -> i32 {
    let (dx, dy) = ((a.0 - b.0) as i32, (a.1 - b.1) as i32);
    match sp.grid {
        Grid::Sq8 => dx.abs().max(dy.abs()),
        Grid::Sq4 => dx.abs() + dy.abs(),
        Grid::Hex => {
            let qa = a.0 as i32 - (a.1 as i32 - (a.1 as i32 & 1)) / 2;
            let qb = b.0 as i32 - (b.1 as i32 - (b.1 as i32 & 1)) / 2;
            let dq = qa - qb;
            dq.abs().max(dy.abs()).max((dq + dy).abs())
        }
    }
}

/// Trecere simetrică (la rotația de 180°) prin pereții orizontali.
fn gap(sp: &Spec, x: i8) -> bool {
    x == 1 || x == sp.w - 2 || x == sp.w / 2 || x == (sp.w - 1) / 2
}

fn pat_code(sp: &Spec, x: i8, y: i8) -> u8 {
    let h = sp.h;
    match sp.pat {
        Pat::Open => 0,
        Pat::Pillars => {
            if x % 3 == 1 && y % 3 == 2 {
                1
            } else if (x + y) % 7 == 0 {
                2
            } else {
                0
            }
        }
        Pat::Rubble => {
            if (x as i32 * 5 + y as i32 * 3) % 7 == 0 {
                2
            } else if (x + 2 * y) % 13 == 4 {
                1
            } else {
                0
            }
        }
        Pat::Corridors => {
            if y % 3 == 2 && !gap(sp, x) {
                1
            } else {
                0
            }
        }
        Pat::Ridge => {
            if y == h / 2 - 1 && !gap(sp, x) {
                1
            } else if (x + y) % 5 == 0 {
                2
            } else {
                0
            }
        }
        Pat::Ruins => {
            if x % 4 == 2 && y % 4 == 2 {
                1
            } else if (x + 2 * y) % 6 == 1 {
                2
            } else {
                0
            }
        }
        Pat::Hills => {
            if x % 4 == 3 && y % 4 == 1 {
                1
            } else if (x as i32 * 2 + y as i32) % 9 < 2 {
                3
            } else {
                0
            }
        }
    }
}

fn goals(sp: &Spec) -> Vec<P> {
    if sp.obj != Obj::Hold {
        return Vec::new();
    }
    let m = sp.h / 2;
    vec![(1, m), (sp.w / 2, m), (sp.w - 2, m)]
}

fn loot_cells(sp: &Spec) -> Vec<P> {
    let n = sp.loot as i8;
    let mut v = Vec::new();
    let pairs = (n + 1) / 2;
    for k in 0..pairs {
        let x = (k + 1) * sp.w / (pairs + 1);
        v.push((x, sp.h / 2 - 1));
        if v.len() < n as usize {
            v.push((sp.w - 1 - x, sp.h / 2));
        }
    }
    v
}

/// Cod teren: 0 liber, 1 zid, 2 adăpost, 3 înălțime.
fn tcode(sp: &Spec, x: i8, y: i8) -> u8 {
    if y < 2 || y >= sp.h - 2 {
        return 0;
    }
    let sp_cells =
        goals(sp).contains(&(x, y)) || (sp.obj == Obj::Loot && loot_cells(sp).contains(&(x, y)));
    if sp_cells {
        return 0;
    }
    let a = pat_code(sp, x, y);
    let b = pat_code(sp, sp.w - 1 - x, sp.h - 1 - y);
    if a == 1 || b == 1 {
        1
    } else if a != 0 {
        a
    } else {
        b
    }
}

fn los(sp: &Spec, a: P, b: P) -> bool {
    let n = (a.0 - b.0).abs().max((a.1 - b.1).abs()) as i32;
    if n <= 1 {
        return true;
    }
    let (dx, dy) = ((b.0 - a.0) as f32, (b.1 - a.1) as f32);
    for i in 1..n {
        let t = i as f32 / n as f32;
        let x = (a.0 as f32 + dx * t).round() as i8;
        let y = (a.1 as f32 + dy * t).round() as i8;
        if tcode(sp, x, y) == 1 {
            return false;
        }
    }
    true
}

#[derive(Clone)]
pub struct U {
    k: u8,
    side: u8,
    x: i8,
    y: i8,
    hp: i8,
    moved: bool,
    steps: u8,
    atk: bool,
    healed: bool,
    shock: u8,
    pin: bool,
    carry: bool,
    alive: bool,
}

#[derive(Clone)]
pub struct Skirm {
    sp: &'static Spec,
    u: Vec<U>,
    loot: Vec<P>,
    ball: Option<P>,
    carrier: Option<usize>,
    side: u8,
    ap: u8,
    round: u8,
    vp: [i32; 2],
    obj: [i32; 2],
    dice: Dice,
    over: Option<Outcome>,
    last: String,
}

impl Skirm {
    pub fn new(sp: &'static Spec) -> Self {
        Self::with_seed(sp, seed_now())
    }

    pub fn with_seed(sp: &'static Spec, seed: u64) -> Self {
        let mut u = Vec::new();
        let cols_n = (sp.w as usize).min(8);
        let mid = sp.w as i32 / 2;
        let mut order: Vec<i8> = Vec::new();
        for d in 0..sp.w as i32 {
            for c in [mid - d, mid + d] {
                if c >= 0 && c < sp.w as i32 && !order.contains(&(c as i8)) && order.len() < cols_n
                {
                    order.push(c as i8);
                }
            }
        }
        for s in 0..2u8 {
            for (i, &k) in sp.army[s as usize].iter().enumerate() {
                let (cx, r) = (order[i % order.len()], (i / order.len()) as i8);
                let (x, y) = if s == 0 {
                    (cx, r)
                } else {
                    (sp.w - 1 - cx, sp.h - 1 - r)
                };
                u.push(U {
                    k,
                    side: s,
                    x,
                    y,
                    hp: sp.kinds[k as usize].hp,
                    moved: false,
                    steps: 0,
                    atk: false,
                    healed: false,
                    shock: 0,
                    pin: false,
                    carry: false,
                    alive: true,
                });
            }
        }
        let mut g = Skirm {
            sp,
            u,
            loot: if sp.obj == Obj::Loot {
                loot_cells(sp)
            } else {
                Vec::new()
            },
            ball: if sp.obj == Obj::Ball {
                Some((sp.w / 2, sp.h / 2))
            } else {
                None
            },
            carrier: None,
            side: 0,
            ap: 0,
            round: 1,
            vp: [0; 2],
            obj: [0; 2],
            dice: Dice::new(seed),
            over: None,
            last: String::new(),
        };
        g.start_turn();
        g
    }

    fn kind(&self, i: usize) -> &'static Kind {
        &self.sp.kinds[self.u[i].k as usize]
    }

    fn at(&self, p: P) -> Option<usize> {
        self.u.iter().position(|u| u.alive && (u.x, u.y) == p)
    }

    fn pos(&self, i: usize) -> P {
        (self.u[i].x, self.u[i].y)
    }

    fn count(&self, s: u8) -> usize {
        self.u.iter().filter(|u| u.alive && u.side == s).count()
    }

    fn start_turn(&mut self) {
        let s = self.side;
        for u in self.u.iter_mut().filter(|u| u.side == s) {
            u.moved = false;
            u.steps = 0;
            u.atk = false;
            u.healed = false;
        }
        self.ap = match self.sp.ap {
            Ap::Fix(n) => n,
            Ap::Roll => 1 + self.dice.d6().div_ceil(2),
            Ap::Scale => ((self.count(s) / 2) as u8).clamp(2, 5),
            Ap::Cmd => {
                let l = self
                    .u
                    .iter()
                    .filter(|u| {
                        u.alive && u.side == s && self.sp.kinds[u.k as usize].fl & LEADER != 0
                    })
                    .count() as u8;
                (2 + l).min(5)
            }
        };
    }

    fn reach(&self, i: usize) -> Vec<(P, u8)> {
        let sp = self.sp;
        let k = self.kind(i);
        let fly = k.fl & FLY != 0;
        let mut mv = k.mv as i32;
        if self.u[i].pin {
            mv = (mv - 1).max(1);
        }
        let n = (sp.w as usize) * (sp.h as usize);
        let idx = |p: P| p.1 as usize * sp.w as usize + p.0 as usize;
        let mut seen = vec![-1i32; n];
        let start = self.pos(i);
        seen[idx(start)] = 0;
        let mut q = vec![start];
        let mut out = Vec::new();
        let mut head = 0;
        while head < q.len() {
            let c = q[head];
            head += 1;
            let d = seen[idx(c)];
            if c != start && self.at(c).is_none() {
                out.push((c, d as u8));
            }
            if d >= mv {
                continue;
            }
            if sp.mods & ZOC != 0 && !fly && c != start {
                let near = nbrs(sp, c.0, c.1).into_iter().any(|p| {
                    self.at(p)
                        .map(|j| self.u[j].side != self.u[i].side)
                        .unwrap_or(false)
                });
                if near {
                    continue;
                }
            }
            for p in nbrs(sp, c.0, c.1) {
                if seen[idx(p)] >= 0 {
                    continue;
                }
                if !fly && tcode(sp, p.0, p.1) == 1 {
                    continue;
                }
                if self.at(p).is_some() && !fly {
                    continue;
                }
                seen[idx(p)] = d + 1;
                q.push(p);
            }
        }
        out
    }

    fn can_hit(&self, a: usize, t: usize) -> bool {
        let (ua, ut) = (&self.u[a], &self.u[t]);
        if !ua.alive || !ut.alive || ua.side == ut.side || ua.atk {
            return false;
        }
        if self.sp.mods & MORALE != 0 && ua.shock >= 2 {
            return false;
        }
        let k = self.kind(a);
        let d = dist(self.sp, self.pos(a), self.pos(t));
        d >= 1 && d <= k.rng as i32 && los(self.sp, self.pos(a), self.pos(t))
    }

    fn att_dice(&self, a: usize, t: usize) -> i32 {
        let k = self.kind(a);
        let ua = &self.u[a];
        let mut n = k.dice as i32;
        if k.fl & DIAL != 0 {
            n -= ((k.hp - ua.hp) / 2) as i32;
        }
        if self.sp.mods & CHARGE != 0 && k.rng == 1 && ua.steps >= 2 {
            n += 1;
        }
        if self.sp.mods & FLANK != 0 {
            let tp = self.pos(t);
            let flank = self.u.iter().enumerate().any(|(j, o)| {
                j != a && o.alive && o.side == ua.side && dist(self.sp, (o.x, o.y), tp) == 1
            });
            if flank {
                n += 1;
            }
        }
        if k.rng > 1 && ua.moved && k.fl & MOBILE == 0 {
            n -= 1;
        }
        if self.sp.mods & MORALE != 0 && ua.shock >= 1 {
            n -= 1;
        }
        if tcode(self.sp, ua.x, ua.y) == 3 {
            n += 1;
        }
        n.clamp(1, 8)
    }

    /// Aruncă atacul: întoarce paguba.
    fn roll_hit(&mut self, kd: u8, n: i32, t: usize) -> i32 {
        let ka = &self.sp.kinds[kd as usize];
        let kt = self.kind(t);
        let mut tn = kt.tn as i32;
        if tcode(self.sp, self.u[t].x, self.u[t].y) == 2 {
            tn += 1;
        }
        let tn = tn.clamp(2, 6);
        let adm = ka.dmg as i32;
        let mut dmg = match self.sp.roll {
            Roll::Count => {
                let mut h = 0;
                for _ in 0..n {
                    if self.dice.d6() as i32 >= tn {
                        h += 1;
                    }
                }
                h * adm
            }
            Roll::Sum => {
                let tot = self.dice.d6() as i32 + self.dice.d6() as i32 + n;
                let need = 5 + tn;
                if tot >= need {
                    adm + if tot >= need + 4 { 1 } else { 0 }
                } else {
                    0
                }
            }
            Roll::Opp => {
                let mut best = 0;
                for _ in 0..n {
                    best = best.max(self.dice.d6() as i32);
                }
                let mut def = 0;
                for _ in 0..(tn - 1).max(1) {
                    def = def.max(self.dice.d6() as i32);
                }
                if best > def {
                    adm * (1 + if best - def >= 3 { 1 } else { 0 })
                } else {
                    0
                }
            }
        };
        if kt.fl & HEAVY != 0 && dmg > 1 {
            dmg -= 1;
        }
        dmg
    }

    fn kill(&mut self, t: usize, by: u8) {
        let k = self.kind(t);
        self.u[t].alive = false;
        self.vp[by as usize] += k.val as i32;
        let p = self.pos(t);
        if self.u[t].carry {
            self.u[t].carry = false;
            if self.sp.obj == Obj::Loot {
                self.loot.push(p);
            }
        }
        if self.carrier == Some(t) {
            self.carrier = None;
            self.ball = Some(p);
        }
        if k.fl & LEADER != 0 && self.sp.obj == Obj::Leader && self.over.is_none() {
            self.over = Some(Outcome::Win(if self.u[t].side == 0 {
                Player::Two
            } else {
                Player::One
            }));
        }
    }

    fn hurt(&mut self, t: usize, dmg: i32, by: u8) {
        if dmg <= 0 || !self.u[t].alive {
            return;
        }
        self.u[t].hp -= dmg as i8;
        if self.sp.mods & MORALE != 0 {
            self.u[t].shock = self.u[t].shock.saturating_add(1);
        }
        if self.carrier == Some(t) && self.u[t].hp > 0 {
            self.carrier = None;
            self.ball = Some(self.pos(t));
        }
        if self.u[t].hp <= 0 {
            self.kill(t, by);
        }
    }

    fn attack(&mut self, a: usize, t: usize) {
        let n = self.att_dice(a, t);
        let ka = self.kind(a);
        let side = self.u[a].side;
        let melee = ka.rng == 1;
        self.u[a].atk = true;
        let dmg = self.roll_hit(self.u[a].k, n, t);
        let (ap, tp) = (self.pos(a), self.pos(t));
        self.last = format!(
            "{} {} lovește {}: {} pagube",
            cell_name(ap.0 as i32, ap.1 as i32),
            ka.name,
            self.kind(t).name,
            dmg
        );
        self.hurt(t, dmg, side);
        if dmg > 0 && ka.fl & BLAST != 0 {
            for j in 0..self.u.len() {
                if j != a && j != t && self.u[j].alive && dist(self.sp, self.pos(j), tp) == 1 {
                    self.hurt(j, 1, side);
                }
            }
        }
        if self.u[t].alive {
            if self.sp.mods & SUPPRESS != 0 && !melee {
                self.u[t].pin = true;
            }
            if self.sp.mods & PUSH != 0 && melee && dmg >= 2 {
                let (dx, dy) = ((tp.0 - ap.0).signum(), (tp.1 - ap.1).signum());
                let np = (tp.0 + dx, tp.1 + dy);
                if np.0 >= 0
                    && np.1 >= 0
                    && np.0 < self.sp.w
                    && np.1 < self.sp.h
                    && tcode(self.sp, np.0, np.1) != 1
                    && self.at(np).is_none()
                {
                    self.u[t].x = np.0;
                    self.u[t].y = np.1;
                    if self.carrier == Some(t) {
                        self.carrier = None;
                    }
                }
            }
            if self.sp.mods & COUNTER != 0 && melee && self.kind(t).rng == 1 && self.u[a].alive {
                let n2 = ((self.kind(t).dice / 2) as i32).max(1);
                let d2 = self.roll_hit(self.u[t].k, n2, a);
                let ts = self.u[t].side;
                self.hurt(a, d2, ts);
            }
        }
        self.check_end();
    }

    fn after_move(&mut self, i: usize) {
        let p = self.pos(i);
        let s = self.u[i].side;
        match self.sp.obj {
            Obj::Loot => {
                if !self.u[i].carry {
                    if let Some(k) = self.loot.iter().position(|&c| c == p) {
                        self.loot.remove(k);
                        self.u[i].carry = true;
                    }
                }
                let home = if s == 0 {
                    p.1 <= 1
                } else {
                    p.1 >= self.sp.h - 2
                };
                if self.u[i].carry && home {
                    self.u[i].carry = false;
                    self.obj[s as usize] += 1;
                    self.vp[s as usize] += 2;
                }
            }
            Obj::Ball => {
                if self.carrier.is_none() && self.ball == Some(p) {
                    self.carrier = Some(i);
                }
                if self.carrier == Some(i) {
                    let goal_y = if s == 0 { self.sp.h - 1 } else { 0 };
                    if p.1 == goal_y {
                        self.obj[s as usize] += 1;
                        self.vp[s as usize] += 3;
                        self.carrier = None;
                        self.ball = Some((self.sp.w / 2, self.sp.h / 2));
                    }
                }
            }
            Obj::Breach if s == 0 && p.1 == self.sp.h - 1 => {
                self.u[i].alive = false;
                self.obj[0] += 1;
                self.vp[0] += 2;
            }
            _ => {}
        }
        self.check_end();
    }

    fn check_end(&mut self) {
        if self.over.is_some() {
            return;
        }
        let g = self.sp.goal as i32;
        if matches!(self.sp.obj, Obj::Loot | Obj::Ball | Obj::Breach | Obj::Hold) && g > 0 {
            let (a, b) = (self.obj[0] >= g, self.obj[1] >= g);
            if a || b {
                self.over = Some(match (a, b) {
                    (true, false) => Outcome::Win(Player::One),
                    (false, true) => Outcome::Win(Player::Two),
                    _ => by_score(self.vp[0], self.vp[1]),
                });
                return;
            }
        }
        let (c0, c1) = (self.count(0), self.count(1));
        if c0 == 0 && c1 == 0 {
            self.over = Some(Outcome::Draw);
        } else if c0 == 0 {
            self.over = Some(Outcome::Win(Player::Two));
        } else if c1 == 0 {
            self.over = Some(Outcome::Win(Player::One));
        }
    }

    fn end_turn(&mut self) {
        let s = self.side;
        for u in self.u.iter_mut().filter(|u| u.side == s) {
            u.pin = false;
        }
        if s == 1 {
            if self.sp.obj == Obj::Hold {
                for gcell in goals(self.sp) {
                    let mut c = [0i32; 2];
                    for u in self.u.iter().filter(|u| u.alive) {
                        if dist(self.sp, (u.x, u.y), gcell) <= 1 {
                            c[u.side as usize] += 1;
                        }
                    }
                    if c[0] > c[1] {
                        self.obj[0] += 1;
                        self.vp[0] += 1;
                    } else if c[1] > c[0] {
                        self.obj[1] += 1;
                        self.vp[1] += 1;
                    }
                }
                self.check_end();
            }
            self.round += 1;
            if self.over.is_none() && self.round > self.sp.rounds {
                let hp = |s: u8| -> i32 {
                    self.u
                        .iter()
                        .filter(|u| u.alive && u.side == s)
                        .map(|u| u.hp as i32)
                        .sum()
                };
                let sc = |s: usize| self.vp[s] * 100 + hp(s as u8);
                // în Breach apărătorul câștigă la egalitate de timp
                self.over = Some(if self.sp.obj == Obj::Breach {
                    Outcome::Win(Player::Two)
                } else {
                    by_score(sc(0), sc(1))
                });
            }
        }
        self.side = 1 - self.side;
        if self.over.is_none() {
            self.start_turn();
        }
    }

    fn spend(&mut self) {
        self.ap = self.ap.saturating_sub(1);
        if self.ap == 0 && self.over.is_none() {
            self.end_turn();
        }
    }
}

impl Mini for Skirm {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn moves(&self) -> Vec<String> {
        let mut v = Vec::new();
        let s = self.side;
        for i in 0..self.u.len() {
            let u = &self.u[i];
            if !u.alive || u.side != s {
                continue;
            }
            let k = self.kind(i);
            if !u.moved {
                for (p, _) in self.reach(i) {
                    v.push(format!("m u{} {}", i, cell_name(p.0 as i32, p.1 as i32)));
                }
            }
            for t in 0..self.u.len() {
                if self.can_hit(i, t) {
                    v.push(format!("a u{} u{}", i, t));
                }
            }
            if k.fl & HEAL != 0 && !u.healed {
                for j in 0..self.u.len() {
                    let o = &self.u[j];
                    if j != i
                        && o.alive
                        && o.side == s
                        && o.hp < self.kind(j).hp
                        && dist(self.sp, self.pos(i), self.pos(j)) == 1
                    {
                        v.push(format!("h u{} u{}", i, j));
                    }
                }
            }
            if self.sp.mods & MORALE != 0 && u.shock > 0 {
                v.push(format!("r u{}", i));
            }
            if self.carrier == Some(i) {
                for j in 0..self.u.len() {
                    let o = &self.u[j];
                    if j != i
                        && o.alive
                        && o.side == s
                        && dist(self.sp, self.pos(i), self.pos(j)) <= 4
                        && los(self.sp, self.pos(i), self.pos(j))
                    {
                        v.push(format!("p u{} u{}", i, j));
                    }
                }
            }
        }
        v.push("end".to_string());
        v
    }

    fn step(&mut self, m: &str) {
        self.last.clear();
        let parts: Vec<&str> = m.split_whitespace().collect();
        let uid = |s: &str| -> usize { s[1..].parse().unwrap() };
        match parts[0] {
            "end" => {
                self.end_turn();
                return;
            }
            "m" => {
                let i = uid(parts[1]);
                let (x, y) = parse_cell(parts[2], self.sp.w as i32, self.sp.h as i32).unwrap();
                let from = self.pos(i);
                let steps = dist(self.sp, from, (x as i8, y as i8)).min(9) as u8;
                self.u[i].x = x as i8;
                self.u[i].y = y as i8;
                self.u[i].moved = true;
                self.u[i].steps = steps;
                self.after_move(i);
            }
            "a" => {
                let (a, t) = (uid(parts[1]), uid(parts[2]));
                self.attack(a, t);
            }
            "h" => {
                let (a, t) = (uid(parts[1]), uid(parts[2]));
                self.u[a].healed = true;
                self.u[t].hp += 1;
            }
            "r" => {
                let i = uid(parts[1]);
                if self.dice.d6() >= 3 {
                    self.u[i].shock = 0;
                }
            }
            "p" => {
                let (a, b) = (uid(parts[1]), uid(parts[2]));
                if self.dice.d6() >= 3 {
                    self.carrier = Some(b);
                    self.ball = Some(self.pos(b));
                    self.after_move(b);
                } else {
                    self.carrier = None;
                    let p = self.pos(a);
                    let free: Vec<P> = nbrs(self.sp, p.0, p.1)
                        .into_iter()
                        .filter(|&c| tcode(self.sp, c.0, c.1) != 1)
                        .collect();
                    self.ball = Some(if free.is_empty() {
                        p
                    } else {
                        free[self.dice.below(free.len())]
                    });
                }
            }
            _ => {}
        }
        self.check_end();
        if self.over.is_none() {
            self.spend();
        }
    }

    fn result(&self) -> Option<Outcome> {
        self.over
    }

    fn eval(&self, p: Player) -> i32 {
        let me = p.idx();
        let sp = self.sp;
        let mut sc = 0i32;
        for (i, u) in self.u.iter().enumerate() {
            if !u.alive {
                continue;
            }
            let k = self.kind(i);
            let sgn = if u.side as usize == me { 1 } else { -1 };
            let w = (k.val as i32 + 1) * 10;
            sc += sgn * w * u.hp as i32;
            if u.side as usize != me {
                continue;
            }
            // aproprierea de țintă
            let here = self.pos(i);
            let mut best = 99;
            for (j, o) in self.u.iter().enumerate() {
                if o.alive && o.side as usize != me {
                    let d = dist(sp, here, self.pos(j));
                    if d < best {
                        best = d;
                    }
                }
            }
            let reach = k.rng as i32;
            if best < 99 {
                sc -= (best - reach).clamp(0, 12);
            }
            match sp.obj {
                Obj::Hold => {
                    let g = goals(sp)
                        .iter()
                        .map(|&c| dist(sp, here, c))
                        .min()
                        .unwrap_or(0);
                    sc -= 2 * g.min(10);
                }
                Obj::Loot => {
                    if u.carry {
                        let home = if me == 0 { 0 } else { sp.h as i32 - 1 };
                        sc += 20 - (here.1 as i32 - home).abs();
                    } else if let Some(d) = self.loot.iter().map(|&c| dist(sp, here, c)).min() {
                        sc -= 2 * d.min(10);
                    }
                }
                Obj::Ball => {
                    if self.carrier == Some(i) {
                        let gy = if me == 0 { sp.h as i32 - 1 } else { 0 };
                        sc += 25 - 3 * (here.1 as i32 - gy).abs();
                    } else if let Some(b) = self.ball {
                        if self.carrier.is_none() {
                            sc -= 2 * dist(sp, here, b).min(10);
                        }
                    }
                }
                Obj::Breach if me == 0 => sc += 3 * here.1 as i32,
                _ => {}
            }
        }
        sc += 30 * (self.vp[me] - self.vp[1 - me]);
        sc
    }

    fn show(&self) -> String {
        let sp = self.sp;
        let mut s = String::new();
        s.push_str(&format!(
            "Runda {}/{} · la mutare: {} · puncte de acțiune: {}\n",
            self.round.min(sp.rounds),
            sp.rounds,
            pname(self.side()),
            self.ap
        ));
        s.push_str(&format!(
            "Scor J1 {} (obiectiv {}) · J2 {} (obiectiv {}) · ținta obiectivului: {}\n",
            self.vp[0], self.obj[0], self.vp[1], self.obj[1], sp.goal
        ));
        s.push_str("   ");
        for x in 0..sp.w {
            s.push_str(&format!(" {}  ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        let gl = goals(sp);
        for y in 0..sp.h {
            s.push_str(&format!("{:>2} ", y + 1));
            if sp.grid == Grid::Hex && y & 1 == 1 {
                s.push_str("  ");
            }
            for x in 0..sp.w {
                let cell = if let Some(i) = self.at((x, y)) {
                    let k = self.kind(i);
                    let c = if self.u[i].side == 0 {
                        k.ch.to_ascii_uppercase()
                    } else {
                        k.ch.to_ascii_lowercase()
                    };
                    format!("{}{:x}", c, i)
                } else {
                    match tcode(sp, x, y) {
                        1 => "##".to_string(),
                        2 => "::".to_string(),
                        3 => "^^".to_string(),
                        _ => {
                            if self.loot.contains(&(x, y)) {
                                "$ ".to_string()
                            } else if self.ball == Some((x, y)) {
                                "o ".to_string()
                            } else if gl.contains(&(x, y)) {
                                "+ ".to_string()
                            } else {
                                ". ".to_string()
                            }
                        }
                    }
                };
                s.push_str(&format!(" {} ", cell));
            }
            s.push('\n');
        }
        s.push_str("Legendă: ## zid, :: adăpost (+1 la ținta de lovit), ^^ înălțime (+1 zar), $ pradă, o minge, + punct de control\n");
        for (i, u) in self.u.iter().enumerate() {
            if !u.alive {
                continue;
            }
            let k = self.kind(i);
            let mut fl = String::new();
            if u.moved {
                fl.push_str(" mutat");
            }
            if u.atk {
                fl.push_str(" a atacat");
            }
            if u.shock > 0 {
                fl.push_str(&format!(" șoc {}", u.shock));
            }
            if u.pin {
                fl.push_str(" imobilizat");
            }
            if u.carry {
                fl.push_str(" poartă prada");
            }
            if self.carrier == Some(i) {
                fl.push_str(" are mingea");
            }
            s.push_str(&format!(
                "u{} {} {} {} viață {}/{} mutare {} bătaie {} zaruri {} țintă {}{}\n",
                i,
                pname(if u.side == 0 {
                    Player::One
                } else {
                    Player::Two
                }),
                k.name,
                cell_name(u.x as i32, u.y as i32),
                u.hp,
                k.hp,
                k.mv,
                k.rng,
                k.dice,
                k.tn,
                fl
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        s.push_str("Mutări: m uN d4 (mută), a uN uM (atac), h uN uM (vindecă), r uN (reface moralul), p uN uM (pasă mingea), end\n");
        s
    }

    fn depth(&self) -> u32 {
        1
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    let sp = crate::t4_sdata::SPECS
        .iter()
        .find(|s| s.id == id)
        .expect("spec");
    Box::new(B(Skirm::new(sp)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::t4_sdata::SPECS;
    use polytabletop_core::rng::Rng;
    use polytabletop_core::GameObj;

    fn flood(sp: &Spec) -> bool {
        let mut seen = vec![false; sp.w as usize * sp.h as usize];
        let free: Vec<P> = (0..sp.h)
            .flat_map(|y| (0..sp.w).map(move |x| (x, y)))
            .filter(|&(x, y)| tcode(sp, x, y) != 1)
            .collect();
        let idx = |p: P| p.1 as usize * sp.w as usize + p.0 as usize;
        let mut q = vec![free[0]];
        seen[idx(free[0])] = true;
        let mut h = 0;
        while h < q.len() {
            let c = q[h];
            h += 1;
            for p in nbrs(sp, c.0, c.1) {
                if tcode(sp, p.0, p.1) != 1 && !seen[idx(p)] {
                    seen[idx(p)] = true;
                    q.push(p);
                }
            }
        }
        free.iter().all(|&p| seen[idx(p)])
    }

    #[test]
    fn boards_are_connected_and_armies_fit() {
        let bad: Vec<&str> = SPECS
            .iter()
            .filter(|sp| !flood(sp))
            .map(|sp| sp.id)
            .collect();
        assert!(bad.is_empty(), "table nelegate: {bad:?}");
        for sp in SPECS {
            for s in 0..2 {
                assert!(sp.army[s].len() <= 16, "{}", sp.id);
            }
            let g = Skirm::with_seed(sp, 1);
            let mut cells = std::collections::BTreeSet::new();
            for u in &g.u {
                assert!(
                    cells.insert((u.x, u.y)),
                    "{} suprapunere la desfășurare",
                    sp.id
                );
                assert_ne!(tcode(sp, u.x, u.y), 1, "{}", sp.id);
            }
        }
    }

    #[test]
    fn every_spec_plays_to_the_end_and_ai_is_legal() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        for sp in SPECS {
            for seed in 1..=2u64 {
                let mut g = B(Skirm::with_seed(sp, seed));
                let mut rng = Rng::new(seed * 31);
                let mut n = 0;
                while GameObj::outcome(&g).is_none() && n < 4000 {
                    let lm = GameObj::legal_moves(&g);
                    assert!(!lm.is_empty(), "{}", sp.id);
                    let m = if n % 11 == 0 && n < 400 {
                        let a = GameObj::ai_move(&g, rng.next_u64()).expect("AI");
                        assert!(lm.contains(&a), "{} AI ilegal {a}", sp.id);
                        a
                    } else {
                        lm[rng.below(lm.len())].clone()
                    };
                    GameObj::play(&mut g, &m).unwrap();
                    n += 1;
                }
                assert!(
                    GameObj::outcome(&g).is_some(),
                    "{} nu s-a terminat\n{}",
                    sp.id,
                    g.0.show()
                );
                assert!(!g.0.show().is_empty());
            }
        }
    }

    #[test]
    fn deterministic_and_clones_reseed() {
        for sp in SPECS.iter().take(6) {
            let run = |seed: u64| {
                let mut g = B(Skirm::with_seed(sp, seed));
                let mut rng = Rng::new(5);
                for _ in 0..40 {
                    if GameObj::outcome(&g).is_some() {
                        break;
                    }
                    let lm = GameObj::legal_moves(&g);
                    let m = lm[rng.below(lm.len())].clone();
                    GameObj::play(&mut g, &m).unwrap();
                }
                g.0.show()
            };
            assert_eq!(run(9), run(9), "{}", sp.id);
        }
    }
}
