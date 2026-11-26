//! „Patrula” — skirmish cu echipă mică de miniaturi individuale (id `patrula`).
//!
//! Motor generic, original: fiecare jucător are o patrulă de 5 miniaturi cu
//! nume de tip (Lider, 2 Pușcași, Greu, Asalt), fiecare cu răni proprii, armă de
//! foc și armă de luptă. Hartă 10×10 cu ziduri (`##`, blochează mersul și vederea),
//! acoperiri (`::`, +1 la salvare) și 4 obiective (`()`).
//!
//! Reguli (scrise de noi):
//! * Runda: fiecare miniatură se activează o dată; părțile alternează câte o
//!   activare (J1 începe rundele impare). La activare se alege un ORDIN:
//!   - `B>d4` Avans: mută ≤ mv; opțional `B>d4 xK` trage după (−1 la lovire; Greul
//!     nu poate trage după ce a mutat); nu ai voie dacă ești lipit de un inamic.
//!   - `B xK` Țintă: stai pe loc; dacă K e lipit = luptă, altfel tragi cu +1 la lovire.
//!   - `B!d4 xK` Asalt: mută ≤ mv+2 lângă K și luptă (+1 zar de luptă).
//!   - `B<d4` Retragere: doar dacă ești lipit; mută ≤ mv într-un pătrat liber de inamici.
//!   - `B garda`: +1 la salvare până la următoarea activare.
//!     (Intrarea într-un pătrat vecin cu un inamic oprește mutarea.)
//! * Foc: linie de vizibilitate liberă (ziduri) și distanță ≤ rază. Zaruri = zaruri
//!   de armă; lovire la d6 ≥ ținta armei (+1 dacă ai mutat sau ești rănit, −1 la
//!   Țintă). Fiecare lovitură e salvată de un d6 ≥ salvarea apărătorului (−1 în
//!   acoperire, −1 în gardă). O lovitură nesalvată = daună. Rănit = la sau sub
//!   jumătate din răni: mv −1.
//! * Luptă: atacatorul lovește, supraviețuitorul răspunde cu aceleași reguli.
//! * Obiective: la sfârșitul rundei, cine are mai multe miniaturi la ≤ 1 pătrat
//!   ia 1 punct. 6 runde; câștigă cine are mai multe puncte, apoi mai multe răni
//!   rămase; eliminarea patrulei adverse câștigă imediat.
//!
//! Zarurile se aruncă la mutarea forțată `arunca` (informație completă, zaruri cu
//! sămânță care se schimbă la `clone`). AI = AlphaBeta(1) cu valori așteptate.
//! Mutări: `A>d4`, `A>d4 xV`, `A xV`, `A!d4 xV`, `A<d4`, `A garda`, `arunca`.

use crate::w7_util::{cell_name, cheb, line_between, parse_cell, Dice};
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::Arc;

pub const W: i32 = 10;
pub const H: i32 = 10;
const MAX_ROUNDS: u8 = 6;
const PER_SIDE: usize = 5;

pub struct MTy {
    pub name: &'static str,
    pub w: i8,
    pub mv: i32,
    pub save: u8,
    pub range: i32,
    pub rdice: u8,
    pub rhit: u8,
    pub rdmg: i8,
    pub mdice: u8,
    pub mhit: u8,
    pub mdmg: i8,
    pub heavy: bool,
}

pub const TYPES: [MTy; 4] = [
    MTy {
        name: "Lider",
        w: 10,
        mv: 5,
        save: 4,
        range: 8,
        rdice: 2,
        rhit: 3,
        rdmg: 2,
        mdice: 3,
        mhit: 3,
        mdmg: 2,
        heavy: false,
    },
    MTy {
        name: "Pușcaș",
        w: 8,
        mv: 5,
        save: 4,
        range: 12,
        rdice: 2,
        rhit: 4,
        rdmg: 2,
        mdice: 2,
        mhit: 4,
        mdmg: 1,
        heavy: false,
    },
    MTy {
        name: "Greu",
        w: 10,
        mv: 4,
        save: 3,
        range: 12,
        rdice: 4,
        rhit: 4,
        rdmg: 2,
        mdice: 2,
        mhit: 4,
        mdmg: 2,
        heavy: true,
    },
    MTy {
        name: "Asalt",
        w: 8,
        mv: 6,
        save: 5,
        range: 6,
        rdice: 1,
        rhit: 4,
        rdmg: 1,
        mdice: 4,
        mhit: 3,
        mdmg: 2,
        heavy: false,
    },
];
/// Tipul miniaturii pe indice în patrulă.
const ROSTER: [usize; PER_SIDE] = [0, 1, 1, 2, 3];
const START: [(i32, i32); PER_SIDE] = [(4, 0), (2, 1), (6, 1), (5, 1), (8, 0)];

const WALLS: [(i32, i32); 6] = [(2, 4), (3, 4), (7, 2), (5, 2), (4, 6), (6, 7)];
const COVER: [(i32, i32); 8] = [
    (6, 4),
    (1, 3),
    (4, 3),
    (8, 2),
    (3, 1),
    (6, 1),
    (5, 6),
    (3, 8),
];
pub const OBJ: [(i32, i32); 4] = [(2, 6), (7, 3), (4, 5), (5, 4)];

fn is_wall(c: (i32, i32)) -> bool {
    WALLS.contains(&c) || WALLS.contains(&(W - 1 - c.0, H - 1 - c.1))
}
fn is_cover(c: (i32, i32)) -> bool {
    COVER.contains(&c) || COVER.contains(&(W - 1 - c.0, H - 1 - c.1))
}

struct Map {
    wall: Vec<bool>,
    cover: Vec<bool>,
    los: Vec<bool>,
}

fn ix(c: (i32, i32)) -> usize {
    (c.1 * W + c.0) as usize
}

impl Map {
    fn build() -> Map {
        let n = (W * H) as usize;
        let mut wall = vec![false; n];
        let mut cover = vec![false; n];
        for y in 0..H {
            for x in 0..W {
                wall[ix((x, y))] = is_wall((x, y));
                cover[ix((x, y))] = is_cover((x, y)) && !is_wall((x, y));
            }
        }
        let mut los = vec![false; n * n];
        for a in 0..n {
            for b in 0..n {
                let ca = ((a as i32) % W, (a as i32) / W);
                let cb = ((b as i32) % W, (b as i32) / W);
                if wall[a] || wall[b] {
                    continue;
                }
                los[a * n + b] = line_between(ca, cb).iter().all(|&c| !wall[ix(c)]);
            }
        }
        Map { wall, cover, los }
    }
    fn see(&self, a: (i32, i32), b: (i32, i32)) -> bool {
        self.los[ix(a) * (W * H) as usize + ix(b)]
    }
}

#[derive(Clone, Debug)]
pub struct Mini {
    pub ty: u8,
    pub side: u8,
    pub pos: (i32, i32),
    pub w: i8,
    pub guard: bool,
    pub done: bool,
}

impl Mini {
    fn t(&self) -> &'static MTy {
        &TYPES[self.ty as usize]
    }
    pub fn alive(&self) -> bool {
        self.w > 0
    }
    pub fn wounded(&self) -> bool {
        self.w * 2 <= self.t().w
    }
    fn mv(&self) -> i32 {
        (self.t().mv - if self.wounded() { 1 } else { 0 }).max(1)
    }
}

fn mid(i: usize) -> char {
    ['A', 'B', 'C', 'D', 'E', 'V', 'W', 'X', 'Y', 'Z'][i]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Advance(u8, (u8, u8), Option<u8>),
    Aim(u8, u8),
    Charge(u8, (u8, u8), u8),
    Retreat(u8, (u8, u8)),
    Guard(u8),
    Roll,
}

#[derive(Clone, Copy, Debug)]
struct Pend {
    a: u8,
    t: u8,
    melee: bool,
    aim: bool,
    moved: bool,
    charge: bool,
}

#[derive(Clone)]
pub struct Patrula {
    map: Arc<Map>,
    pub minis: Vec<Mini>,
    pub round: u8,
    side: Player,
    pub vp: [u8; 2],
    pub kills: [u8; 2],
    pending: Option<Pend>,
    dice: Dice,
    over: Option<Outcome>,
    log: Vec<String>,
}

impl Default for Patrula {
    fn default() -> Self {
        Self::new()
    }
}

impl Patrula {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut minis = Vec::new();
        for side in 0..2u8 {
            for k in 0..PER_SIDE {
                let (x, y) = START[k];
                let pos = if side == 0 {
                    (x, y)
                } else {
                    (W - 1 - x, H - 1 - y)
                };
                minis.push(Mini {
                    ty: ROSTER[k] as u8,
                    side,
                    pos,
                    w: TYPES[ROSTER[k]].w,
                    guard: false,
                    done: false,
                });
            }
        }
        Patrula {
            map: Arc::new(Map::build()),
            minis,
            round: 1,
            side: Player::One,
            vp: [0, 0],
            kills: [0, 0],
            pending: None,
            dice: Dice::new(seed),
            over: None,
            log: Vec::new(),
        }
    }

    fn occupied(&self, c: (i32, i32)) -> bool {
        self.minis.iter().any(|m| m.alive() && m.pos == c)
    }

    fn enemy_adj(&self, side: u8, c: (i32, i32)) -> bool {
        self.minis
            .iter()
            .any(|m| m.alive() && m.side != side && cheb(m.pos, c) == 1)
    }

    /// Pătrate atinse cu cost ≤ `maxd` (BFS pe 8 vecini; intrarea lângă un inamic
    /// oprește mutarea). Fără start.
    pub fn reach(&self, i: usize, maxd: i32) -> Vec<((i32, i32), i32)> {
        let m = &self.minis[i];
        let mut dist = vec![-1i32; (W * H) as usize];
        let mut q = std::collections::VecDeque::new();
        dist[ix(m.pos)] = 0;
        q.push_back(m.pos);
        let mut out = Vec::new();
        while let Some(c) = q.pop_front() {
            let d = dist[ix(c)];
            if d >= maxd {
                continue;
            }
            if c != m.pos && self.enemy_adj(m.side, c) {
                continue;
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let n = (c.0 + dx, c.1 + dy);
                    if n.0 < 0 || n.1 < 0 || n.0 >= W || n.1 >= H {
                        continue;
                    }
                    if self.map.wall[ix(n)] || self.occupied(n) || dist[ix(n)] >= 0 {
                        continue;
                    }
                    dist[ix(n)] = d + 1;
                    out.push((n, d + 1));
                    q.push_back(n);
                }
            }
        }
        out
    }

    fn can_shoot(&self, from: (i32, i32), range: i32, t: &Mini) -> bool {
        t.alive() && {
            let d = cheb(from, t.pos);
            d >= 2 && d <= range && self.map.see(from, t.pos)
        }
    }

    fn remaining(&self, side: u8) -> usize {
        self.minis
            .iter()
            .filter(|m| m.alive() && m.side == side && !m.done)
            .count()
    }

    fn alive_count(&self, side: u8) -> usize {
        self.minis
            .iter()
            .filter(|m| m.alive() && m.side == side)
            .count()
    }

    fn wounds(&self, side: u8) -> i32 {
        self.minis
            .iter()
            .filter(|m| m.side == side && m.alive())
            .map(|m| m.w as i32)
            .sum()
    }

    fn initiative(&self) -> Player {
        if self.round % 2 == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn check_over(&mut self) {
        let a = self.alive_count(0);
        let b = self.alive_count(1);
        if a == 0 || b == 0 {
            self.over = Some(match (a, b) {
                (0, 0) => Outcome::Draw,
                (0, _) => Outcome::Win(Player::Two),
                _ => Outcome::Win(Player::One),
            });
        }
    }

    /// După o activare încheiată: următoarea parte / sfârșit de rundă.
    fn next_activation(&mut self) {
        self.check_over();
        if self.over.is_some() {
            return;
        }
        let r0 = self.remaining(0);
        let r1 = self.remaining(1);
        if r0 + r1 == 0 {
            self.end_round();
            return;
        }
        let cur = self.side.idx();
        let other = 1 - cur;
        let rem = [r0, r1];
        self.side = if rem[other] > 0 {
            if other == 0 {
                Player::One
            } else {
                Player::Two
            }
        } else {
            self.side
        };
    }

    fn end_round(&mut self) {
        for o in OBJ {
            let mut c = [0i32; 2];
            for m in self.minis.iter().filter(|m| m.alive()) {
                if cheb(m.pos, o) <= 1 {
                    c[m.side as usize] += 1;
                }
            }
            if c[0] > c[1] {
                self.vp[0] += 1;
            } else if c[1] > c[0] {
                self.vp[1] += 1;
            }
        }
        if self.round >= MAX_ROUNDS {
            self.over = Some(
                match (self.vp[0], self.wounds(0)).cmp(&(self.vp[1], self.wounds(1))) {
                    std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                    std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                    _ => Outcome::Draw,
                },
            );
            return;
        }
        self.round += 1;
        for m in &mut self.minis {
            m.done = false;
        }
        self.side = self.initiative();
        // dacă partea cu inițiativă nu mai are miniaturi, trece cealaltă
        if self.remaining(self.side.idx() as u8) == 0 {
            self.side = self.side.other();
        }
    }

    // ---- zaruri ----

    fn p_hit(target: i32) -> f32 {
        let t = target.clamp(2, 7);
        ((7 - t) as f32 / 6.0).max(0.0)
    }

    /// (zaruri, ținta de lovire, ținta salvării, daună) pentru atacul p.
    fn attack_profile(&self, p: &Pend, strike_back: bool) -> (i32, i32, i32, i32) {
        let (a, t) = if strike_back {
            (p.t as usize, p.a as usize)
        } else {
            (p.a as usize, p.t as usize)
        };
        let am = &self.minis[a];
        let tm = &self.minis[t];
        let ty = am.t();
        let (dice, mut hit, dmg) = if p.melee {
            (ty.mdice as i32, ty.mhit as i32, ty.mdmg as i32)
        } else {
            (ty.rdice as i32, ty.rhit as i32, ty.rdmg as i32)
        };
        let mut dice = dice;
        if am.wounded() {
            hit += 1;
        }
        if !p.melee {
            if p.aim {
                hit -= 1;
            }
            if p.moved {
                hit += 1;
            }
        } else if p.charge && !strike_back {
            dice += 1;
        }
        let mut save = tm.t().save as i32;
        if self.map.cover[ix(tm.pos)] {
            save -= 1;
        }
        if tm.guard {
            save -= 1;
        }
        (dice, hit.clamp(2, 7), save.clamp(2, 7), dmg)
    }

    /// Daună așteptată (plafonată la rănile țintei) pentru un profil.
    fn expected(&self, prof: (i32, i32, i32, i32), target_w: i32) -> f32 {
        let (dice, hit, save, dmg) = prof;
        let e = dice as f32 * Self::p_hit(hit) * (1.0 - Self::p_hit(save)) * dmg as f32;
        e.min(target_w as f32)
    }

    fn strike(&mut self, prof: (i32, i32, i32, i32), target: usize) -> i32 {
        let (dice, hit, save, dmg) = prof;
        let mut total = 0;
        for _ in 0..dice {
            if (self.dice.d6() as i32) >= hit && hit < 7 {
                let s = self.dice.d6() as i32;
                if save >= 7 || s < save {
                    total += dmg;
                }
            }
        }
        self.hurt(target, total);
        total
    }

    fn hurt(&mut self, t: usize, n: i32) {
        let side = self.minis[t].side as usize;
        let m = &mut self.minis[t];
        let was = m.alive();
        m.w = (m.w as i32 - n).max(0) as i8;
        if was && !m.alive() {
            self.kills[1 - side] += 1;
        }
    }

    fn resolve(&mut self, p: Pend) {
        let prof = self.attack_profile(&p, false);
        let t = p.t as usize;
        let n = self.strike(prof, t);
        self.log.push(format!(
            "{}{}{}: {} daune",
            mid(p.a as usize),
            if p.melee { " luptă " } else { " trage în " },
            mid(t),
            n
        ));
        if p.melee && self.minis[t].alive() && self.minis[p.a as usize].alive() {
            let prof = self.attack_profile(&p, true);
            let n = self.strike(prof, p.a as usize);
            self.log.push(format!("{} ripostează: {} daune", mid(t), n));
        }
    }

    #[cfg(test)]
    fn wounds_left_all(&self) -> i32 {
        self.minis
            .iter()
            .filter(|m| m.alive())
            .map(|m| m.w as i32)
            .sum()
    }
}

fn pos8(c: (i32, i32)) -> (u8, u8) {
    (c.0 as u8, c.1 as u8)
}
fn pos32(c: (u8, u8)) -> (i32, i32) {
    (c.0 as i32, c.1 as i32)
}

impl Game for Patrula {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Roll];
        }
        let side = self.side.idx() as u8;
        let mut v = Vec::new();
        for (i, m) in self.minis.iter().enumerate() {
            if !m.alive() || m.side != side || m.done {
                continue;
            }
            let iu = i as u8;
            let engaged = self.enemy_adj(side, m.pos);
            v.push(Move::Guard(iu));
            if engaged {
                for (t, e) in self.minis.iter().enumerate() {
                    if e.alive() && e.side != side && cheb(e.pos, m.pos) == 1 {
                        v.push(Move::Aim(iu, t as u8));
                    }
                }
                for (c, _) in self.reach(i, m.mv()) {
                    if !self.enemy_adj(side, c) {
                        v.push(Move::Retreat(iu, pos8(c)));
                    }
                }
                continue;
            }
            // Țintă (foc pe loc)
            for (t, e) in self.minis.iter().enumerate() {
                if e.side != side && self.can_shoot(m.pos, m.t().range, e) {
                    v.push(Move::Aim(iu, t as u8));
                }
            }
            // Avans (+ foc)
            for (c, _) in self.reach(i, m.mv()) {
                v.push(Move::Advance(iu, pos8(c), None));
                if !m.t().heavy && !self.enemy_adj(side, c) {
                    for (t, e) in self.minis.iter().enumerate() {
                        if e.side != side && self.can_shoot(c, m.t().range, e) {
                            v.push(Move::Advance(iu, pos8(c), Some(t as u8)));
                        }
                    }
                }
            }
            // Asalt
            for (c, _) in self.reach(i, m.mv() + 2) {
                for (t, e) in self.minis.iter().enumerate() {
                    if e.alive() && e.side != side && cheb(e.pos, c) == 1 {
                        v.push(Move::Charge(iu, pos8(c), t as u8));
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Roll => {
                if let Some(p) = self.pending.take() {
                    self.resolve(p);
                }
                self.next_activation();
            }
            Move::Guard(i) => {
                let mi = &mut self.minis[i as usize];
                mi.guard = true;
                mi.done = true;
                self.next_activation();
            }
            Move::Retreat(i, to) => {
                let mi = &mut self.minis[i as usize];
                mi.pos = pos32(to);
                mi.guard = false;
                mi.done = true;
                self.next_activation();
            }
            Move::Advance(i, to, t) => {
                let mi = &mut self.minis[i as usize];
                mi.pos = pos32(to);
                mi.guard = false;
                mi.done = true;
                match t {
                    Some(t) => {
                        self.pending = Some(Pend {
                            a: i,
                            t,
                            melee: false,
                            aim: false,
                            moved: true,
                            charge: false,
                        })
                    }
                    None => self.next_activation(),
                }
            }
            Move::Aim(i, t) => {
                let melee = {
                    let a = &self.minis[i as usize];
                    cheb(a.pos, self.minis[t as usize].pos) == 1
                };
                let mi = &mut self.minis[i as usize];
                mi.guard = false;
                mi.done = true;
                self.pending = Some(Pend {
                    a: i,
                    t,
                    melee,
                    aim: !melee,
                    moved: false,
                    charge: false,
                });
            }
            Move::Charge(i, to, t) => {
                let mi = &mut self.minis[i as usize];
                mi.pos = pos32(to);
                mi.guard = false;
                mi.done = true;
                self.pending = Some(Pend {
                    a: i,
                    t,
                    melee: true,
                    aim: false,
                    moved: true,
                    charge: true,
                });
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8;
        let en = 1 - me;
        let mut v = 0i32;
        v += (self.wounds(me) - self.wounds(en)) * 6;
        v += (self.alive_count(me) as i32 - self.alive_count(en) as i32) * 30;
        v += (self.vp[me as usize] as i32 - self.vp[en as usize] as i32) * 40;
        if let Some(pd) = self.pending {
            let prof = self.attack_profile(&pd, false);
            let tw = self.minis[pd.t as usize].w as i32;
            let mut e = self.expected(prof, tw);
            if pd.melee {
                let back = self.attack_profile(&pd, true);
                let aw = self.minis[pd.a as usize].w as i32;
                let eb = self.expected(back, aw) * (1.0 - e / tw.max(1) as f32).max(0.0);
                let from_a = self.minis[pd.a as usize].side;
                let sgn = if from_a == me { 1.0 } else { -1.0 };
                v += (sgn * (e * 6.0 - eb * 6.0)) as i32;
                e = 0.0;
            }
            let from_a = self.minis[pd.a as usize].side;
            let sgn = if from_a == me { 1.0 } else { -1.0 };
            v += (sgn * e * 6.0) as i32;
        }
        // poziționare
        for m in self.minis.iter().filter(|m| m.alive()) {
            let sgn = if m.side == me { 1 } else { -1 };
            let mut s = 0i32;
            let dmin = OBJ.iter().map(|&o| cheb(m.pos, o)).min().unwrap_or(0);
            s -= dmin;
            if dmin <= 1 {
                s += 8;
            }
            if self.map.cover[ix(m.pos)] {
                s += 4;
            }
            if m.guard {
                s += 2;
            }
            // expunere: inamici cu linie de vizibilitate în raza lor
            let mut exp = 0;
            let mut opp = 0;
            for e in self.minis.iter().filter(|e| e.alive() && e.side != m.side) {
                let d = cheb(m.pos, e.pos);
                if d >= 2 && self.map.see(m.pos, e.pos) {
                    if d <= e.t().range {
                        exp += e.t().rdice as i32 * e.t().rdmg as i32;
                    }
                    if d <= m.t().range {
                        opp += m.t().rdice as i32 * m.t().rdmg as i32;
                    }
                }
            }
            s += opp / 2 - exp / 2;
            v += sgn * s;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Patrula — runda {}/{}  puncte {}–{}  doborâri {}–{}\n",
            self.round, MAX_ROUNDS, self.vp[0], self.vp[1], self.kills[0], self.kills[1]
        );
        for y in (0..H).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..W {
                let c = (x, y);
                let mut cell = if self.map.wall[ix(c)] {
                    "##".to_string()
                } else if OBJ.contains(&c) {
                    "()".to_string()
                } else if self.map.cover[ix(c)] {
                    "::".to_string()
                } else {
                    " .".to_string()
                };
                for (i, m) in self.minis.iter().enumerate() {
                    if m.alive() && m.pos == c {
                        cell = format!("{} ", mid(i));
                    }
                }
                s.push_str(&cell);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   ");
        for x in 0..W {
            s.push_str(&format!("{}  ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for (i, m) in self.minis.iter().enumerate() {
            s.push_str(&format!(
                "{} {:<7} {:>3}  răni {:>2}/{:<2}{}{}{}\n",
                mid(i),
                m.t().name,
                cell_name(m.pos.0, m.pos.1),
                m.w.max(0),
                m.t().w,
                if m.wounded() && m.alive() {
                    " rănit"
                } else {
                    ""
                },
                if m.guard { " gardă" } else { "" },
                if m.done && m.alive() {
                    " (a acționat)"
                } else {
                    ""
                },
            ));
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let c = |p: &(u8, u8)| cell_name(p.0 as i32, p.1 as i32);
        match m {
            Move::Advance(i, to, None) => format!("{}>{}", mid(*i as usize), c(to)),
            Move::Advance(i, to, Some(t)) => {
                format!("{}>{} x{}", mid(*i as usize), c(to), mid(*t as usize))
            }
            Move::Aim(i, t) => format!("{} x{}", mid(*i as usize), mid(*t as usize)),
            Move::Charge(i, to, t) => {
                format!("{}!{} x{}", mid(*i as usize), c(to), mid(*t as usize))
            }
            Move::Retreat(i, to) => format!("{}<{}", mid(*i as usize), c(to)),
            Move::Guard(i) => format!("{} garda", mid(*i as usize)),
            Move::Roll => "arunca".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

/// Pentru teste: parsează un pătrat.
pub fn cell_from_str(s: &str) -> Option<(i32, i32)> {
    parse_cell(s, W, H)
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn legal(g: &Patrula) -> Vec<Move> {
        Game::legal_moves(g)
    }

    #[test]
    fn map_is_symmetric_and_los_symmetric() {
        for y in 0..H {
            for x in 0..W {
                assert_eq!(is_wall((x, y)), is_wall((W - 1 - x, H - 1 - y)));
                assert_eq!(is_cover((x, y)), is_cover((W - 1 - x, H - 1 - y)));
            }
        }
        let g = Patrula::with_seed(1);
        for a in 0..(W * H) {
            for b in 0..(W * H) {
                let ca = (a % W, a / W);
                let cb = (b % W, b / W);
                assert_eq!(g.map.see(ca, cb), g.map.see(cb, ca));
            }
        }
        for o in OBJ {
            assert!(!is_wall(o), "obiectiv pe zid {o:?}");
        }
        for st in START {
            assert!(!is_wall(st));
            assert!(!is_wall((W - 1 - st.0, H - 1 - st.1)));
        }
    }

    #[test]
    fn walls_block_sight_and_movement() {
        let g = Patrula::with_seed(1);
        // zidurile (2,4),(3,4) blochează linia c3—d7? verificăm direct (2,3)->(2,5)
        assert!(!g.map.see((2, 3), (2, 5)));
        assert!(g.map.see((0, 0), (0, 5)));
        let m = g.reach(0, 12);
        assert!(m.iter().all(|(c, _)| !g.map.wall[ix(*c)]));
    }

    #[test]
    fn reach_stops_next_to_enemy() {
        let mut g = Patrula::with_seed(1);
        g.minis[5].pos = (4, 4); // inamic în centru
        let r = g.reach(0, 12);
        // (4,3) e vecin: se poate intra, dar nu se trece mai departe prin el
        assert!(r.iter().any(|(c, _)| *c == (4, 3)));
        for (c, _) in &r {
            assert_ne!(*c, (4, 4));
        }
    }

    #[test]
    fn engaged_models_cannot_advance_or_shoot() {
        let mut g = Patrula::with_seed(1);
        g.minis[0].pos = (4, 4);
        g.minis[5].pos = (4, 5);
        // A lipit de V
        for m in legal(&g) {
            match m {
                Move::Advance(0, _, _) | Move::Charge(0, _, _) => panic!("A e lipit"),
                _ => {}
            }
        }
        assert!(legal(&g).contains(&Move::Aim(0, 5)));
        assert!(legal(&g).iter().any(|m| matches!(m, Move::Retreat(0, _))));
    }

    #[test]
    fn heavy_cannot_shoot_after_moving() {
        let mut g = Patrula::with_seed(1);
        g.minis[3].pos = (4, 2);
        g.minis[5].pos = (4, 7);
        for m in legal(&g) {
            if let Move::Advance(3, _, Some(_)) = m {
                panic!("Greul nu trage după mutare");
            }
        }
    }

    #[test]
    fn round_flow_each_model_once_and_alternates() {
        let mut g = Patrula::with_seed(2);
        let mut rng = Rng::new(3);
        let mut acts = [0usize; 10];
        let r0 = g.round;
        let mut last_side: Option<Player> = None;
        let mut alternations = 0;
        while g.round == r0 && g.outcome().is_none() {
            let mv = legal(&g);
            let m = mv[rng.below(mv.len())];
            if m != Move::Roll {
                let i = match m {
                    Move::Advance(i, ..)
                    | Move::Aim(i, _)
                    | Move::Charge(i, ..)
                    | Move::Retreat(i, _)
                    | Move::Guard(i) => i,
                    Move::Roll => unreachable!(),
                };
                acts[i as usize] += 1;
                if last_side.is_some() && last_side != Some(g.to_move()) {
                    alternations += 1;
                }
                last_side = Some(g.to_move());
            }
            g.apply(&m);
        }
        if g.outcome().is_none() {
            assert!(acts.iter().all(|&a| a == 1), "{acts:?}");
            assert!(alternations >= 5);
        }
    }

    #[test]
    fn combat_statistics_match_expected() {
        // Greul A(3) trage în Pușcașul V(6) pe loc: media zarurilor ≈ valoarea așteptată
        let mut total = 0i32;
        let n = 4000;
        let mut exp = 0.0f32;
        for s in 0..n {
            let mut g = Patrula::with_seed(s as u64 + 1);
            g.minis[3].pos = (4, 2);
            g.minis[6].pos = (4, 7);
            g.minis[6].w = 100;
            let p = Pend {
                a: 3,
                t: 6,
                melee: false,
                aim: true,
                moved: false,
                charge: false,
            };
            let prof = g.attack_profile(&p, false);
            exp = g.expected(prof, 100);
            g.pending = Some(p);
            g.apply(&Move::Roll);
            total += 100 - g.minis[6].w as i32;
        }
        let mean = total as f32 / n as f32;
        assert!((mean - exp).abs() < 0.25, "medie {mean} vs {exp}");
    }

    #[test]
    fn cover_and_guard_reduce_damage() {
        let mut g = Patrula::with_seed(1);
        g.minis[3].pos = (4, 2);
        g.minis[6].pos = (6, 4); // acoperire
        let p = Pend {
            a: 3,
            t: 6,
            melee: false,
            aim: false,
            moved: false,
            charge: false,
        };
        let (_, _, save_cover, _) = g.attack_profile(&p, false);
        g.minis[6].guard = true;
        let (_, _, save_both, _) = g.attack_profile(&p, false);
        g.minis[6].pos = (5, 5);
        g.minis[6].guard = false;
        let (_, _, save_open, _) = g.attack_profile(&p, false);
        assert_eq!(save_open - save_cover, 1);
        assert_eq!(save_cover - save_both, 1);
    }

    #[test]
    fn kills_and_win_by_elimination() {
        let mut g = Patrula::with_seed(1);
        for i in 5..10 {
            g.minis[i].w = 0;
        }
        g.check_over();
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn objectives_score_at_round_end() {
        let mut g = Patrula::with_seed(1);
        g.minis[0].pos = OBJ[2];
        g.end_round();
        assert_eq!(g.vp, [2, 0]);
        assert_eq!(g.round, 2);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn random_games_finish_and_wounds_never_rise() {
        for seed in 1..=20u64 {
            let mut g = Patrula::with_seed(seed);
            let mut rng = Rng::new(seed * 17);
            let mut last = g.wounds_left_all();
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = legal(&g);
                assert!(!mv.is_empty(), "{}", Game::render(&g));
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                let w = g.wounds_left_all();
                assert!(w <= last);
                last = w;
                n += 1;
                assert!(n < 2000);
            }
            // nicio miniatură pe zid sau suprapusă
            let live: Vec<_> = g
                .minis
                .iter()
                .filter(|m| m.alive())
                .map(|m| m.pos)
                .collect();
            for (i, a) in live.iter().enumerate() {
                assert!(!g.map.wall[ix(*a)]);
                for b in &live[i + 1..] {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly_and_is_fast() {
        let mut wins = 0;
        let mut worst = 0u128;
        for seed in 1..=10u64 {
            let mut g = Patrula::with_seed(seed + 100);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = if g.to_move() == Player::Two {
                    let t = std::time::Instant::now();
                    let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                    worst = worst.max(t.elapsed().as_millis());
                    m
                } else {
                    let mv = legal(&g);
                    mv[rng.below(mv.len())]
                };
                g.apply(&m);
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 8, "AI a câștigat doar {wins}/10");
        assert!(worst < 1000, "AI prea lent: {worst} ms");
    }

    #[test]
    fn move_texts_are_unique_and_parse() {
        let g = Patrula::with_seed(5);
        let mv: Vec<String> = legal(&g).iter().map(|m| g.format_move(m)).collect();
        let mut s = mv.clone();
        s.sort();
        s.dedup();
        assert_eq!(s.len(), mv.len());
        assert!(cell_from_str("d4").is_some());
    }
}
