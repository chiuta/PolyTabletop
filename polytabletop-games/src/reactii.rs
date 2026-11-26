//! „Ordine și reacții” — skirmish cu reacții în tura adversarului (id `reactii`).
//!
//! Motor generic, original: câte 4 soldați pe parte (Lider, Pușcaș, Lunetist,
//! Asalt) pe o hartă 10×10 cu ziduri, acoperiri și 2 obiective. Zaruri d20.
//!
//! Reguli (scrise de noi):
//! * Tura: partea activă are atâtea ORDINE câți soldați vii are (+1 dacă Liderul
//!   trăiește). Un ordin merge unui soldat (oricare, de mai multe ori): mută ≤ mv
//!   și/sau trage într-o țintă vizibilă: `A>d4`, `A xV`, `A>d4 xV`. `fin` încheie tura.
//! * REACȚII în tura adversarului: după ce soldatul activ a ajuns pe pătratul
//!   final, fiecare soldat inamic cu linie de vizibilitate liberă spre el decide pe
//!   rând: `V pas`, `V tira` (tragere de reacție) sau, dacă e ținta declarată, `V
//!   esiva`. (Simplificare: vizibilitatea se verifică doar pe pătratul final.)
//! * Zaruri: tragerea reușește la d20 ≤ îndemânarea + modificatorul benzii de
//!   distanță a armei (≤3 scurtă, ≤6 medie, mai mult lungă); un rezultat egal cu
//!   ținta e CRITIC (nu se poate salva). Dacă ținta răspunde tragând sau esivând,
//!   e față în față: câștigă cel mai mare rezultat reușit (critic bate simplu,
//!   egal = nimic; la esivare egalul îl avantajează pe apărător). O lovitură
//!   necritică e salvată dacă d20 ≤ armura țintei (+4 în acoperire); altfel
//!   soldatul pierde răni egale cu dauna armei.
//! * Tragerile de reacție ale celorlalți inamici sunt aruncări simple spre soldatul
//!   activ. Toate se rezolvă la mutarea forțată `arunca`.
//! * Rundă = tura ambelor părți (J1 apoi J2). La sfârșitul rundei, obiectivul e al
//!   celui cu mai mulți soldați la ≤ 1 pătrat: 1 punct. 8 runde; apoi puncte, apoi
//!   răni; eliminarea adversarului câștigă imediat.
//!
//! Informație completă; AI = AlphaBeta(1) cu valori așteptate (calcul exact al
//! rezultatelor față în față, fără zaruri). Mutări: `A>d4 xV`, `A xV`, `A>d4`,
//! `fin`, `V pas`, `V tira`, `V esiva`, `arunca`.

use crate::w7_util::{bfs8, cell_name, cheb, Dice, GridMap};
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::Arc;

pub const W: i32 = 10;
pub const H: i32 = 10;
const MAX_ROUNDS: u8 = 8;
const PER: usize = 4;
const OBJ: [(i32, i32); 2] = [(1, 4), (8, 5)];
const WALLS: [(i32, i32); 5] = [(3, 2), (4, 2), (6, 3), (2, 5), (4, 4)];
const COVERS: [(i32, i32); 3] = [(1, 3), (5, 3), (8, 1)];
const START: [(i32, i32); PER] = [(4, 0), (2, 1), (7, 1), (5, 1)];

pub struct Wp {
    pub name: &'static str,
    pub mods: [i32; 3],
    pub dmg: i8,
}

pub const WEAPONS: [Wp; 3] = [
    Wp {
        name: "Carabină",
        mods: [0, 3, -3],
        dmg: 1,
    },
    Wp {
        name: "Lunetă",
        mods: [-6, 0, 3],
        dmg: 2,
    },
    Wp {
        name: "Pompă",
        mods: [3, -3, -9],
        dmg: 2,
    },
];

pub struct STy {
    pub name: &'static str,
    pub w: i8,
    pub mv: i32,
    pub bs: i32,
    pub ph: i32,
    pub arm: i32,
    pub wp: usize,
}

pub const TYPES: [STy; 4] = [
    STy {
        name: "Lider",
        w: 2,
        mv: 4,
        bs: 12,
        ph: 11,
        arm: 9,
        wp: 0,
    },
    STy {
        name: "Pușcaș",
        w: 1,
        mv: 4,
        bs: 12,
        ph: 11,
        arm: 9,
        wp: 0,
    },
    STy {
        name: "Lunetist",
        w: 1,
        mv: 3,
        bs: 13,
        ph: 10,
        arm: 8,
        wp: 1,
    },
    STy {
        name: "Asalt",
        w: 1,
        mv: 5,
        bs: 11,
        ph: 13,
        arm: 10,
        wp: 2,
    },
];

fn sid(i: usize) -> char {
    ['A', 'B', 'C', 'D', 'V', 'W', 'X', 'Y'][i]
}

#[derive(Clone, Debug)]
pub struct Sold {
    pub ty: u8,
    pub side: u8,
    pub pos: (i32, i32),
    pub w: i8,
}

impl Sold {
    fn t(&self) -> &'static STy {
        &TYPES[self.ty as usize]
    }
    pub fn alive(&self) -> bool {
        self.w > 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rx {
    Shoot,
    Dodge,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Order(u8, Option<(u8, u8)>, Option<u8>),
    End,
    React(u8, Option<Rx>),
    Roll,
}

#[derive(Clone, Debug)]
struct Decl {
    actor: u8,
    target: Option<u8>,
    queue: Vec<u8>,
    reacts: Vec<(u8, Rx)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Orders,
    React,
    Roll,
}

/// Rezultatul unei aruncări: 0 = ratat, 1 = reușit, 2 = critic; plus valoarea zarului.
fn grade(roll: i32, target: i32) -> (u8, i32) {
    if roll == target {
        (2, roll)
    } else if roll < target {
        (1, roll)
    } else {
        (0, roll)
    }
}

/// Față în față: 1 = câștigă a, −1 = câștigă b, 0 = nimic.
fn f2f_winner(ga: u8, va: i32, gb: u8, vb: i32) -> i8 {
    match (ga, gb) {
        (0, 0) => 0,
        (_, 0) => 1,
        (0, _) => -1,
        _ => {
            if ga != gb {
                if ga > gb {
                    1
                } else {
                    -1
                }
            } else if va > vb {
                1
            } else if vb > va {
                -1
            } else {
                0
            }
        }
    }
}

#[derive(Clone)]
pub struct Reactii {
    map: Arc<GridMap>,
    pub s: Vec<Sold>,
    pub round: u8,
    pub vp: [u8; 2],
    side: Player,
    orders: u8,
    phase: Phase,
    decl: Option<Decl>,
    dice: Dice,
    over: Option<Outcome>,
    log: Vec<String>,
}

impl Default for Reactii {
    fn default() -> Self {
        Self::new()
    }
}

impl Reactii {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut s = Vec::new();
        for side in 0..2u8 {
            for (k, &(x, y)) in START.iter().enumerate() {
                let pos = if side == 0 {
                    (x, y)
                } else {
                    (W - 1 - x, H - 1 - y)
                };
                s.push(Sold {
                    ty: k as u8,
                    side,
                    pos,
                    w: TYPES[k].w,
                });
            }
        }
        let mut g = Reactii {
            map: Arc::new(GridMap::build(W, H, &WALLS, &COVERS)),
            s,
            round: 1,
            vp: [0, 0],
            side: Player::One,
            orders: 0,
            phase: Phase::Orders,
            decl: None,
            dice: Dice::new(seed),
            over: None,
            log: Vec::new(),
        };
        g.orders = g.order_pool(0);
        g
    }

    fn order_pool(&self, side: u8) -> u8 {
        let n = self
            .s
            .iter()
            .filter(|m| m.alive() && m.side == side)
            .count() as u8;
        let leader = self
            .s
            .iter()
            .any(|m| m.alive() && m.side == side && m.ty == 0);
        n + if leader { 1 } else { 0 }
    }

    fn alive_count(&self, side: u8) -> usize {
        self.s
            .iter()
            .filter(|m| m.alive() && m.side == side)
            .count()
    }

    fn wounds(&self, side: u8) -> i32 {
        self.s
            .iter()
            .filter(|m| m.alive() && m.side == side)
            .map(|m| m.w as i32)
            .sum()
    }

    fn occupied(&self, c: (i32, i32)) -> bool {
        self.s.iter().any(|m| m.alive() && m.pos == c)
    }

    fn band(d: i32) -> usize {
        if d <= 3 {
            0
        } else if d <= 6 {
            1
        } else {
            2
        }
    }

    /// Ținta de tragere pentru tragătorul a din pătratul `from` spre b.
    fn shoot_target(&self, a: usize, from: (i32, i32), b: usize) -> i32 {
        let sa = &self.s[a];
        let d = cheb(from, self.s[b].pos);
        (sa.t().bs + WEAPONS[sa.t().wp].mods[Self::band(d)]).clamp(1, 20)
    }

    fn save_target(&self, b: usize) -> i32 {
        let m = &self.s[b];
        (m.t().arm + if self.map.is_cover(m.pos) { 4 } else { 0 }).clamp(1, 19)
    }

    fn can_see(&self, _a: usize, from: (i32, i32), b: usize) -> bool {
        let t = &self.s[b];
        t.alive() && cheb(from, t.pos) >= 1 && self.map.see(from, t.pos)
    }

    fn hurt(&mut self, i: usize, n: i32) {
        let m = &mut self.s[i];
        m.w = (m.w as i32 - n).max(0) as i8;
    }

    /// Daună așteptată (în răni) a unei lovituri de tip `grade`.
    fn hit_value(&self, victim: usize, dmg: i8, crit: bool) -> f32 {
        let save = if crit {
            0.0
        } else {
            self.save_target(victim) as f32 / 20.0
        };
        let d = dmg as f32 * (1.0 - save);
        d.min(self.s[victim].w as f32)
    }

    /// Șanse (și daune așteptate) pentru o aruncare simplă a lui a asupra lui b.
    fn plain_expected(&self, a: usize, from: (i32, i32), b: usize) -> f32 {
        let tn = self.shoot_target(a, from, b);
        let dmg = WEAPONS[self.s[a].t().wp].dmg;
        let ps = (tn - 1).max(0) as f32 / 20.0;
        let pc = 1.0 / 20.0;
        ps * self.hit_value(b, dmg, false) + pc * self.hit_value(b, dmg, true)
    }

    /// Față în față: întoarce (daună așteptată pentru b, daună așteptată pentru a).
    fn f2f_expected(&self, a: usize, from: (i32, i32), b: usize) -> (f32, f32) {
        let ta = self.shoot_target(a, from, b);
        let tb = self.shoot_target(b, self.s[b].pos, a);
        let da = WEAPONS[self.s[a].t().wp].dmg;
        let db = WEAPONS[self.s[b].t().wp].dmg;
        let (mut ea, mut eb) = (0.0f32, 0.0f32);
        for ra in 1..=20 {
            for rb in 1..=20 {
                let (ga, va) = grade(ra, ta);
                let (gb, vb) = grade(rb, tb);
                let w = f2f_winner(ga, va, gb, vb);
                if w == 1 {
                    ea += self.hit_value(b, da, ga == 2) / 400.0;
                } else if w == -1 {
                    eb += self.hit_value(a, db, gb == 2) / 400.0;
                }
            }
        }
        (ea, eb)
    }

    fn dodge_expected(&self, a: usize, from: (i32, i32), b: usize) -> f32 {
        let ta = self.shoot_target(a, from, b);
        let tb = self.s[b].t().ph;
        let da = WEAPONS[self.s[a].t().wp].dmg;
        let mut e = 0.0;
        for ra in 1..=20 {
            for rb in 1..=20 {
                let (ga, va) = grade(ra, ta);
                if ga == 0 {
                    continue;
                }
                let (gb, vb) = grade(rb, tb);
                let dodged = gb > 0 && (gb > ga || (gb == ga && vb >= va));
                if !dodged {
                    e += self.hit_value(b, da, ga == 2) / 400.0;
                }
            }
        }
        e
    }

    fn attack_roll(&mut self, a: usize, from: (i32, i32), b: usize, f2f: Option<usize>) {
        // `f2f`: 0 = față în față (b trage înapoi), 1 = esivare, None = simplu
        let ta = self.shoot_target(a, from, b);
        let da = WEAPONS[self.s[a].t().wp].dmg as i32;
        let ra = self.dice.d20() as i32;
        let (ga, va) = grade(ra, ta);
        match f2f {
            None => {
                if ga > 0 {
                    self.land(a, b, da, ga == 2);
                } else {
                    self.log.push(format!("{} ratează {}", sid(a), sid(b)));
                }
            }
            Some(0) => {
                let tb = self.shoot_target(b, self.s[b].pos, a);
                let db = WEAPONS[self.s[b].t().wp].dmg as i32;
                let rb = self.dice.d20() as i32;
                let (gb, vb) = grade(rb, tb);
                match f2f_winner(ga, va, gb, vb) {
                    1 => self.land(a, b, da, ga == 2),
                    -1 => self.land(b, a, db, gb == 2),
                    _ => self.log.push(format!("{}↔{}: nimic", sid(a), sid(b))),
                }
            }
            Some(_) => {
                let tb = self.s[b].t().ph;
                let rb = self.dice.d20() as i32;
                let (gb, vb) = grade(rb, tb);
                let dodged = ga == 0 || (gb > 0 && (gb > ga || (gb == ga && vb >= va)));
                if ga > 0 && !dodged {
                    self.land(a, b, da, ga == 2);
                } else {
                    self.log.push(format!("{} esivează {}", sid(b), sid(a)));
                }
            }
        }
    }

    fn land(&mut self, a: usize, b: usize, dmg: i32, crit: bool) {
        let saved = !crit && (self.dice.d20() as i32) <= self.save_target(b);
        if saved {
            self.log.push(format!("{}→{}: salvat", sid(a), sid(b)));
        } else {
            self.log.push(format!(
                "{}→{}: {}{} răni",
                sid(a),
                sid(b),
                if crit { "critic, " } else { "" },
                dmg
            ));
            self.hurt(b, dmg);
        }
    }

    fn resolve(&mut self, d: Decl) {
        let a = d.actor as usize;
        let from = self.s[a].pos;
        if let Some(t) = d.target {
            let t = t as usize;
            if self.s[a].alive() && self.s[t].alive() {
                let rx = d.reacts.iter().find(|(r, _)| *r as usize == t).map(|x| x.1);
                match rx {
                    Some(Rx::Shoot) => self.attack_roll(a, from, t, Some(0)),
                    Some(Rx::Dodge) => self.attack_roll(a, from, t, Some(1)),
                    None => self.attack_roll(a, from, t, None),
                }
            }
        }
        for (r, rx) in &d.reacts {
            if Some(*r) == d.target {
                continue;
            }
            let r = *r as usize;
            if *rx == Rx::Shoot && self.s[r].alive() && self.s[a].alive() {
                let from_r = self.s[r].pos;
                // reactorul trage în a: arunc simplu cu pozițiile curente
                let ta = self.shoot_target(r, from_r, a);
                let dr = WEAPONS[self.s[r].t().wp].dmg as i32;
                let (g, _) = grade(self.dice.d20() as i32, ta);
                if g > 0 {
                    self.land(r, a, dr, g == 2);
                } else {
                    self.log.push(format!("{} ratează {}", sid(r), sid(a)));
                }
            }
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

    fn end_turn(&mut self) {
        self.check_over();
        if self.over.is_some() {
            return;
        }
        if self.side == Player::One {
            self.side = Player::Two;
        } else {
            // sfârșit de rundă
            for o in OBJ {
                let mut c = [0i32; 2];
                for m in self.s.iter().filter(|m| m.alive()) {
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
            self.side = Player::One;
        }
        self.orders = self.order_pool(self.side.idx() as u8);
        self.phase = Phase::Orders;
    }

    fn after_order(&mut self) {
        self.check_over();
        if self.over.is_some() {
            return;
        }
        self.phase = Phase::Orders;
        if self.orders == 0 {
            self.end_turn();
        }
    }

    fn reachable(&self, i: usize) -> Vec<(i32, i32)> {
        let m = &self.s[i];
        bfs8(&self.map, m.pos, m.t().mv, &|c| self.occupied(c), &|_| {
            false
        })
        .into_iter()
        .map(|x| x.0)
        .collect()
    }

    fn owner(i: usize) -> Player {
        if i < PER {
            Player::One
        } else {
            Player::Two
        }
    }
}

impl Game for Reactii {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.phase == Phase::React {
            if let Some(d) = &self.decl {
                if let Some(&r) = d.queue.first() {
                    return Self::owner(r as usize);
                }
            }
        }
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Roll => vec![Move::Roll],
            Phase::React => {
                let d = self.decl.as_ref().unwrap();
                let r = d.queue[0];
                let mut v = vec![Move::React(r, None), Move::React(r, Some(Rx::Shoot))];
                if d.target == Some(r) {
                    v.push(Move::React(r, Some(Rx::Dodge)));
                }
                v
            }
            Phase::Orders => {
                let side = self.side.idx() as u8;
                let mut v = vec![Move::End];
                for (i, m) in self.s.iter().enumerate() {
                    if !m.alive() || m.side != side {
                        continue;
                    }
                    let mut spots = vec![m.pos];
                    spots.extend(self.reachable(i));
                    for c in spots {
                        let moved = c != m.pos;
                        let to = if moved {
                            Some((c.0 as u8, c.1 as u8))
                        } else {
                            None
                        };
                        if moved {
                            v.push(Move::Order(i as u8, to, None));
                        }
                        for (t, e) in self.s.iter().enumerate() {
                            if e.alive() && e.side != side && self.can_see(i, c, t) {
                                v.push(Move::Order(i as u8, to, Some(t as u8)));
                            }
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::End => self.end_turn(),
            Move::Order(i, to, target) => {
                let iu = i as usize;
                if let Some(c) = to {
                    self.s[iu].pos = (c.0 as i32, c.1 as i32);
                }
                self.orders = self.orders.saturating_sub(1);
                let pos = self.s[iu].pos;
                let side = self.s[iu].side;
                let mut queue: Vec<u8> = Vec::new();
                if let Some(t) = target {
                    queue.push(t);
                }
                for (j, e) in self.s.iter().enumerate() {
                    if e.alive()
                        && e.side != side
                        && Some(j as u8) != target
                        && self.map.see(pos, e.pos)
                    {
                        queue.push(j as u8);
                    }
                }
                if let Some(t) = target {
                    // ținta e deja în queue; ea decide prima
                    let _ = t;
                }
                if queue.is_empty() {
                    self.after_order();
                } else {
                    self.decl = Some(Decl {
                        actor: i,
                        target,
                        queue,
                        reacts: Vec::new(),
                    });
                    self.phase = Phase::React;
                }
            }
            Move::React(r, rx) => {
                if let Some(d) = self.decl.as_mut() {
                    d.queue.remove(0);
                    if let Some(x) = rx {
                        d.reacts.push((r, x));
                    }
                    if d.queue.is_empty() {
                        self.phase = Phase::Roll;
                    }
                }
            }
            Move::Roll => {
                if let Some(d) = self.decl.take() {
                    self.resolve(d);
                }
                self.after_order();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8;
        let en = 1 - me;
        let mut v = (self.wounds(me) - self.wounds(en)) * 25;
        v += (self.alive_count(me) as i32 - self.alive_count(en) as i32) * 30;
        v += (self.vp[me as usize] as i32 - self.vp[en as usize] as i32) * 40;
        // declarația în curs: daune așteptate
        if let Some(d) = &self.decl {
            let a = d.actor as usize;
            let from = self.s[a].pos;
            let mut dmg_to_actor = 0.0f32;
            let mut dmg_to_target = 0.0f32;
            if let Some(t) = d.target {
                let t = t as usize;
                match d.reacts.iter().find(|(r, _)| *r as usize == t).map(|x| x.1) {
                    Some(Rx::Shoot) => {
                        let (x, y) = self.f2f_expected(a, from, t);
                        dmg_to_target += x;
                        dmg_to_actor += y;
                    }
                    Some(Rx::Dodge) => dmg_to_target += self.dodge_expected(a, from, t),
                    None => {
                        // dacă ținta încă nu a decis, presupunem tragere simplă
                        dmg_to_target += self.plain_expected(a, from, t)
                    }
                }
            }
            for (r, rx) in &d.reacts {
                if *rx == Rx::Shoot && Some(*r) != d.target {
                    dmg_to_actor += self.plain_expected(*r as usize, self.s[*r as usize].pos, a);
                }
            }
            let (gain_actor_side, loss_actor_side) = (dmg_to_target, dmg_to_actor);
            let sgn = if self.s[a].side == me { 1.0 } else { -1.0 };
            v += (sgn * (gain_actor_side - loss_actor_side) * 25.0) as i32;
        }
        // poziționare și expunere
        for (i, m) in self.s.iter().enumerate().filter(|(_, m)| m.alive()) {
            let sgn = if m.side == me { 1 } else { -1 };
            let mut s = 0i32;
            let dmin = OBJ.iter().map(|&o| cheb(m.pos, o)).min().unwrap_or(0);
            s -= dmin;
            if dmin <= 1 {
                s += 8;
            }
            if self.map.is_cover(m.pos) {
                s += 4;
            }
            for (j, e) in self
                .s
                .iter()
                .enumerate()
                .filter(|(_, e)| e.alive() && e.side != m.side)
            {
                if self.map.see(m.pos, e.pos) {
                    // cine ar trage mai bine în celălalt
                    let ex = self.plain_expected(j, e.pos, i);
                    let op = self.plain_expected(i, m.pos, j);
                    s += ((op - ex) * 12.0) as i32;
                }
            }
            v += sgn * s;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ordine și reacții — runda {}/{}  puncte {}–{}  ordine rămase: {}\n",
            self.round, MAX_ROUNDS, self.vp[0], self.vp[1], self.orders
        );
        match (self.phase, &self.decl) {
            (Phase::React, Some(d)) => {
                s.push_str(&format!(
                    "{} acționează; reacție cerută de {}\n",
                    sid(d.actor as usize),
                    sid(d.queue[0] as usize)
                ));
            }
            (Phase::Roll, _) => s.push_str("zaruri de aruncat\n"),
            _ => {}
        }
        for y in (0..H).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..W {
                let c = (x, y);
                let mut cell = if self.map.is_wall(c) {
                    "##".to_string()
                } else if OBJ.contains(&c) {
                    "()".to_string()
                } else if self.map.is_cover(c) {
                    "::".to_string()
                } else {
                    " .".to_string()
                };
                for (i, m) in self.s.iter().enumerate() {
                    if m.alive() && m.pos == c {
                        cell = format!("{} ", sid(i));
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
        for (i, m) in self.s.iter().enumerate() {
            s.push_str(&format!(
                "{} {:<8} {:>3} răni {}/{} {}\n",
                sid(i),
                m.t().name,
                cell_name(m.pos.0, m.pos.1),
                m.w.max(0),
                m.t().w,
                WEAPONS[m.t().wp].name
            ));
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::End => "fin".into(),
            Move::Roll => "arunca".into(),
            Move::Order(i, to, t) => {
                let mut s = sid(*i as usize).to_string();
                if let Some(c) = to {
                    s.push_str(&format!(">{}", cell_name(c.0 as i32, c.1 as i32)));
                }
                if let Some(t) = t {
                    s.push_str(&format!(" x{}", sid(*t as usize)));
                }
                s
            }
            Move::React(r, rx) => format!(
                "{} {}",
                sid(*r as usize),
                match rx {
                    None => "pas",
                    Some(Rx::Shoot) => "tira",
                    Some(Rx::Dodge) => "esiva",
                }
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn legal(g: &Reactii) -> Vec<Move> {
        Game::legal_moves(g)
    }

    fn texts(g: &Reactii) -> Vec<String> {
        legal(g).iter().map(|m| g.format_move(m)).collect()
    }

    #[test]
    fn setup_and_orders_pool() {
        let g = Reactii::with_seed(1);
        assert_eq!(g.order_pool(0), 5);
        assert_eq!(g.order_pool(1), 5);
        assert_eq!(g.orders, 5);
        for m in &g.s {
            assert!(!g.map.is_wall(m.pos));
        }
        for o in OBJ {
            assert!(!g.map.is_wall(o));
        }
        let mut t = texts(&g);
        let n = t.len();
        t.sort();
        t.dedup();
        assert_eq!(t.len(), n);
    }

    #[test]
    fn grade_logic() {
        assert_eq!(grade(5, 10), (1, 5));
        assert_eq!(grade(10, 10), (2, 10));
        assert_eq!(grade(11, 10), (0, 11));
    }

    #[test]
    fn f2f_expectation_is_symmetric_for_equal_shooters() {
        let mut g = Reactii::with_seed(1);
        // două Carabine egale față în față pe câmp deschis
        g.s[1].pos = (5, 6);
        g.s[5].pos = (5, 9);
        g.s[5].ty = 1;
        let (ea, eb) = g.f2f_expected(1, g.s[1].pos, 5);
        assert!((ea - eb).abs() < 1e-4, "{ea} vs {eb}");
        assert!(ea > 0.0);
    }

    #[test]
    fn dodge_beats_plain_when_agile() {
        let mut g = Reactii::with_seed(1);
        g.s[0].pos = (0, 0);
        g.s[7].pos = (0, 3);
        g.s[7].ty = 3; // Asalt, esivare 13
        let plain = g.plain_expected(0, g.s[0].pos, 7);
        let dodge = g.dodge_expected(0, g.s[0].pos, 7);
        assert!(dodge < plain);
    }

    #[test]
    fn reactions_trigger_on_visible_move_and_flow() {
        let mut g = Reactii::with_seed(2);
        // pune un Pușcaș P1 în fața unui inamic vizibil
        g.s[1].pos = (0, 0);
        g.s[5].pos = (0, 5);
        let mv = Move::Order(1, None, Some(5));
        assert!(legal(&g).contains(&mv));
        g.apply(&mv);
        assert_eq!(g.to_move(), Player::Two);
        // ținta e prima și are și opțiunea esiva
        let t = texts(&g);
        assert!(t.contains(&"W esiva".to_string()), "{t:?}");
        assert!(t.contains(&"W tira".to_string()));
        // fiecare reactor decide pe rând până la arunca
        let mut guard = 0;
        while legal(&g) != vec![Move::Roll] {
            let m = legal(&g)[0];
            g.apply(&m);
            guard += 1;
            assert!(guard < 10);
        }
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Roll);
        assert_eq!(g.orders, 4);
    }

    #[test]
    fn no_reaction_without_line_of_sight() {
        let mut g = Reactii::with_seed(2);
        g.s[1].pos = (3, 1); // în spatele zidului (3,2)
        g.s[5].pos = (3, 3);
        assert!(!g.map.see((3, 1), (3, 3)));
        let before = g.orders;
        // mutare fără țintă, care nu vede pe nimeni
        g.apply(&Move::Order(1, Some((2, 1)), None));
        let _ = before;
        // dacă nimeni nu vede pozițiile, nu apare faza de reacție
        if g.phase == Phase::React {
            let d = g.decl.as_ref().unwrap();
            assert!(d
                .queue
                .iter()
                .all(|&r| g.map.see(g.s[1].pos, g.s[r as usize].pos)));
        }
    }

    #[test]
    fn turn_passes_after_orders_run_out_and_round_scores() {
        let mut g = Reactii::with_seed(3);
        g.apply(&Move::End);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.round, 1);
        g.s[4].pos = OBJ[0];
        g.apply(&Move::End);
        assert_eq!(g.round, 2);
        assert_eq!(g.vp, [0, 1]);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn crit_ignores_save() {
        // mulți aruncați: un critic lovește mereu, deci cu ținta 20 nu avem salvări la critic
        let mut g = Reactii::with_seed(4);
        g.s[1].pos = (0, 0);
        g.s[5].pos = (0, 2);
        g.s[5].w = 127;
        let mut hits = 0;
        for k in 0..100 {
            g.dice = Dice::new(k + 1);
            let before = g.s[5].w;
            g.land(1, 5, 1, true);
            if g.s[5].w < before {
                hits += 1;
            }
        }
        assert_eq!(hits, 100);
    }

    #[test]
    fn random_games_finish_and_wounds_never_rise() {
        for seed in 1..=20u64 {
            let mut g = Reactii::with_seed(seed);
            let mut rng = Rng::new(seed * 7);
            let mut last = g.wounds(0) + g.wounds(1);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = legal(&g);
                assert!(!mv.is_empty(), "{}", Game::render(&g));
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                let w = g.wounds(0) + g.wounds(1);
                assert!(w <= last);
                last = w;
                n += 1;
                assert!(n < 5000);
            }
            let live: Vec<_> = g.s.iter().filter(|m| m.alive()).map(|m| m.pos).collect();
            for (i, a) in live.iter().enumerate() {
                assert!(!g.map.is_wall(*a));
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
            let mut g = Reactii::with_seed(seed + 100);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let m = if Game::to_move(&g) == Player::Two {
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
}
