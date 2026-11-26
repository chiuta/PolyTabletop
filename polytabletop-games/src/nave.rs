//! „Escadrile din Rift”: manevre planificate simultan (id `nave`).
//!
//! Motor generic, original: fiecare jucător are 3 nave; la începutul rundei
//! fiecare navă primește în SECRET o manevră (cadranul), apoi manevrele se
//! dezvăluie și se execută forțat, în ordinea îndemânării pilotului; după mutare
//! se trage (cel mai priceput trage primul).
//!
//! Reguli (scrise de noi):
//! * Hartă 12×12, 4 orientări (N/E/S/V), 6 asteroizi simetrici.
//! * Manevre: `dN` drept N (1–4), `blN`/`brN` deviere stânga/dreapta (N înainte,
//!   1 lateral, aceeași orientare), `tlN`/`trN` viraj (N înainte, N lateral,
//!   orientarea se rotește 90°), `u` întoarcere (3 înainte, 180°), `s` oprire.
//!   Verzi: drept 1–2, deviere 1, viraj 1 (șterg stresul). Roșii: viraj 3, `u`, `s`
//!   (dau stres; o navă stresată nu poate planifica roșu). Bivolul (greu) nu
//!   are `u`, nici viraj 3, nici drept 4.
//! * Mutare: dacă pătratul final e ocupat, nava se oprește pe ultimul pătrat
//!   liber al traseului („ciocnire”) și nu trage; pe asteroid: 1 daună și nu trage.
//! * Foc: țintă în arcul din față (înainte ≥ 1, lateral ≤ înainte; Bivolul are
//!   turelă 360°) la distanță 1–3. Zaruri de atac = atac (+1 la distanță 1; Bivolul
//!   trage cu 2): 4–5 lovitură, 6 critic (2 daune); apărare = agilitate (+1 la
//!   distanță 3, +1 dacă un asteroid e pe linie): 5–6 evaziune; evaziunile anulează
//!   întâi loviturile, apoi criticele. Dauna scade întâi scutul, apoi coca.
//! * Final: o parte fără nave pierde; după 10 runde câștigă cine are mai mult
//!   (coca + scut + 3 pe navă); egal = remiză.
//!
//! Informație ascunsă: planurile. `render` arată doar planurile Jucătorului 1;
//! `evaluate` citește doar planurile PROPRII și pozițiile publice, deci AI-ul
//! nu trișează. Mutări: `A d2`, `A bl1`, `A tr2`, `A u`, `A s`, `revela`,
//! `A>X` (foc), `A pas`, `arunca`.

use crate::w7_util::{cell_name, line_between, Dice};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const N: i32 = 12;
const MAX_ROUNDS: u8 = 10;
const OBST: [(i32, i32); 6] = [(5, 5), (6, 6), (3, 8), (8, 3), (2, 5), (9, 6)];
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const ARROWS: [char; 4] = ['^', '>', 'v', '<'];

pub struct ShipTy {
    pub name: &'static str,
    pub atk: u8,
    pub agi: u8,
    pub hull: i8,
    pub shield: i8,
    pub skill: u8,
    pub heavy: bool,
}

pub const TYPES: [ShipTy; 3] = [
    ShipTy {
        name: "Bivol",
        atk: 2,
        agi: 1,
        hull: 6,
        shield: 3,
        skill: 1,
        heavy: true,
    },
    ShipTy {
        name: "Șoim",
        atk: 2,
        agi: 2,
        hull: 3,
        shield: 2,
        skill: 3,
        heavy: false,
    },
    ShipTy {
        name: "Vrabie",
        atk: 3,
        agi: 3,
        hull: 3,
        shield: 0,
        skill: 4,
        heavy: false,
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Man {
    Str(u8),
    BankL(u8),
    BankR(u8),
    TurnL(u8),
    TurnR(u8),
    UTurn,
    Stop,
}

impl Man {
    pub fn text(self) -> String {
        match self {
            Man::Str(s) => format!("d{s}"),
            Man::BankL(s) => format!("bl{s}"),
            Man::BankR(s) => format!("br{s}"),
            Man::TurnL(s) => format!("tl{s}"),
            Man::TurnR(s) => format!("tr{s}"),
            Man::UTurn => "u".into(),
            Man::Stop => "s".into(),
        }
    }
    pub fn red(self) -> bool {
        matches!(self, Man::TurnL(3) | Man::TurnR(3) | Man::UTurn | Man::Stop)
    }
    pub fn green(self) -> bool {
        matches!(
            self,
            Man::Str(1)
                | Man::Str(2)
                | Man::BankL(1)
                | Man::BankR(1)
                | Man::TurnL(1)
                | Man::TurnR(1)
        )
    }
}

fn dial(heavy: bool) -> Vec<Man> {
    let mut v = Vec::new();
    for s in 1..=(if heavy { 3 } else { 4 }) {
        v.push(Man::Str(s));
    }
    for s in 1..=3 {
        v.push(Man::BankL(s));
        v.push(Man::BankR(s));
    }
    for s in 1..=(if heavy { 2 } else { 3 }) {
        v.push(Man::TurnL(s));
        v.push(Man::TurnR(s));
    }
    if !heavy {
        v.push(Man::UTurn);
    }
    v.push(Man::Stop);
    v
}

/// Traseul (pătratele parcurse, fără start) și orientarea finală.
pub fn path_of(x: i32, y: i32, f: u8, m: Man) -> (Vec<(i32, i32)>, u8) {
    let fw = DIRS[f as usize];
    let lf = (f + 3) % 4;
    let rf = (f + 1) % 4;
    let mut cur = (x, y);
    let mut v = Vec::new();
    let go = |dir: (i32, i32), n: u8, cur: &mut (i32, i32), v: &mut Vec<(i32, i32)>| {
        for _ in 0..n {
            cur.0 += dir.0;
            cur.1 += dir.1;
            v.push(*cur);
        }
    };
    let mut nf = f;
    match m {
        Man::Str(s) => go(fw, s, &mut cur, &mut v),
        Man::BankL(s) => {
            go(fw, s, &mut cur, &mut v);
            go(DIRS[lf as usize], 1, &mut cur, &mut v);
        }
        Man::BankR(s) => {
            go(fw, s, &mut cur, &mut v);
            go(DIRS[rf as usize], 1, &mut cur, &mut v);
        }
        Man::TurnL(s) => {
            go(fw, s, &mut cur, &mut v);
            go(DIRS[lf as usize], s, &mut cur, &mut v);
            nf = lf;
        }
        Man::TurnR(s) => {
            go(fw, s, &mut cur, &mut v);
            go(DIRS[rf as usize], s, &mut cur, &mut v);
            nf = rf;
        }
        Man::UTurn => {
            go(fw, 3, &mut cur, &mut v);
            nf = (f + 2) % 4;
        }
        Man::Stop => {}
    }
    (v, nf)
}

fn inside(c: (i32, i32)) -> bool {
    c.0 >= 0 && c.0 < N && c.1 >= 0 && c.1 < N
}

#[derive(Clone, Debug)]
pub struct Ship {
    pub ty: u8,
    pub side: u8,
    pub x: i32,
    pub y: i32,
    pub f: u8,
    pub hull: i8,
    pub shield: i8,
    pub stress: u8,
    pub plan: Option<Man>,
    pub nofire: bool,
    pub alive: bool,
}

impl Ship {
    fn t(&self) -> &'static ShipTy {
        &TYPES[self.ty as usize]
    }
    fn id(&self, i: usize) -> char {
        ship_id(i)
    }
    /// Distanța la ținta (tx,ty) dacă e în arcul de foc, altfel None.
    pub fn range_to(&self, tx: i32, ty: i32) -> Option<i32> {
        let dx = tx - self.x;
        let dy = ty - self.y;
        let d = dx.abs().max(dy.abs());
        if !(1..=3).contains(&d) {
            return None;
        }
        if self.t().heavy {
            return Some(d);
        }
        let fw = DIRS[self.f as usize];
        let rt = DIRS[((self.f + 1) % 4) as usize];
        let fwd = dx * fw.0 + dy * fw.1;
        let side = dx * rt.0 + dy * rt.1;
        if fwd >= 1 && side.abs() <= fwd {
            Some(d)
        } else {
            None
        }
    }
}

fn ship_id(i: usize) -> char {
    ['A', 'B', 'C', 'X', 'Y', 'Z'][i]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Plan,
    Reveal,
    Combat,
    Roll,
    End,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Plan(u8, Man),
    Reveal,
    Fire(u8, u8),
    Hold(u8),
    Roll,
}

#[derive(Clone)]
pub struct Nave {
    pub ships: Vec<Ship>,
    pub round: u8,
    phase: Phase,
    side: Player,
    order: Vec<u8>,
    pos: usize,
    pending: Option<(u8, u8)>,
    dice: Dice,
    over: Option<Outcome>,
    log: Vec<String>,
    /// `Some(p)` = render shows `p`'s secret plans; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Nave {
    fn default() -> Self {
        Self::new()
    }
}

impl Nave {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut ships = Vec::new();
        let starts = [(5, 1, 0u8), (3, 0, 1), (8, 0, 2)];
        for side in 0..2u8 {
            for &(x, y, ty) in &starts {
                let (x, y, f) = if side == 0 {
                    (x, y, 0)
                } else {
                    (N - 1 - x, N - 1 - y, 2)
                };
                let t = &TYPES[ty as usize];
                ships.push(Ship {
                    ty,
                    side,
                    x,
                    y,
                    f,
                    hull: t.hull,
                    shield: t.shield,
                    stress: 0,
                    plan: None,
                    nofire: false,
                    alive: true,
                });
            }
        }
        Nave {
            ships,
            round: 1,
            phase: Phase::Plan,
            side: Player::One,
            order: Vec::new(),
            pos: 0,
            pending: None,
            dice: Dice::new(seed),
            over: None,
            log: Vec::new(),
            viewer: Some(Player::One),
        }
    }

    fn occupied(&self, c: (i32, i32), except: usize) -> bool {
        self.ships
            .iter()
            .enumerate()
            .any(|(i, s)| i != except && s.alive && (s.x, s.y) == c)
    }

    fn valid_dial(&self, i: usize) -> Vec<Man> {
        let s = &self.ships[i];
        let all: Vec<Man> = dial(s.t().heavy)
            .into_iter()
            .filter(|&m| !(s.stress > 0 && m.red()))
            .filter(|&m| {
                let (p, _) = path_of(s.x, s.y, s.f, m);
                p.iter().all(|&c| inside(c))
            })
            .collect();
        if all.is_empty() {
            vec![Man::Stop]
        } else {
            all
        }
    }

    fn next_to_plan(&self, side: u8) -> Option<usize> {
        self.ships
            .iter()
            .position(|s| s.alive && s.side == side && s.plan.is_none())
    }

    fn targets(&self, i: usize) -> Vec<usize> {
        let s = &self.ships[i];
        if !s.alive || s.nofire {
            return Vec::new();
        }
        (0..self.ships.len())
            .filter(|&j| {
                let t = &self.ships[j];
                t.alive && t.side != s.side && s.range_to(t.x, t.y).is_some()
            })
            .collect()
    }

    fn dice_counts(&self, a: usize, d: usize) -> (u8, u8) {
        let s = &self.ships[a];
        let t = &self.ships[d];
        let r = s.range_to(t.x, t.y).unwrap_or(3);
        let mut atk = s.t().atk + if r == 1 { 1 } else { 0 };
        if s.t().heavy {
            atk = s.t().atk + if r == 1 { 1 } else { 0 };
        }
        let obstructed = line_between((s.x, s.y), (t.x, t.y))
            .iter()
            .any(|c| OBST.contains(c));
        let def = t.t().agi + if r == 3 { 1 } else { 0 } + if obstructed { 1 } else { 0 };
        (atk, def)
    }

    /// Daună așteptată (în puncte de scut+coca) pentru un atac a atac-zaruri
    /// contra d zaruri de apărare. Calcul exact, fără zaruri.
    pub fn expected_damage(a: u8, d: u8) -> f32 {
        let mut total = 0.0f32;
        let na = 3usize.pow(a as u32);
        let nd = 1usize << d;
        for ai in 0..na {
            let mut x = ai;
            let mut h = 0i32;
            let mut c = 0i32;
            let mut pa = 1.0f32;
            for _ in 0..a {
                match x % 3 {
                    0 => pa *= 0.5,
                    1 => {
                        pa *= 1.0 / 3.0;
                        h += 1;
                    }
                    _ => {
                        pa *= 1.0 / 6.0;
                        c += 1;
                    }
                }
                x /= 3;
            }
            for di in 0..nd {
                let e = (di as u32).count_ones() as i32;
                let pd = (1.0f32 / 3.0).powi(e) * (2.0f32 / 3.0).powi(d as i32 - e);
                let hl = (h - e).max(0);
                let cl = (c - (e - h).max(0)).max(0);
                total += pa * pd * (hl + 2 * cl) as f32;
            }
        }
        total
    }

    fn damage(&mut self, i: usize, mut n: i32) {
        let s = &mut self.ships[i];
        while n > 0 && s.alive {
            if s.shield > 0 {
                s.shield -= 1;
            } else {
                s.hull -= 1;
                if s.hull <= 0 {
                    s.alive = false;
                }
            }
            n -= 1;
        }
    }

    fn side_alive(&self, side: u8) -> bool {
        self.ships.iter().any(|s| s.alive && s.side == side)
    }

    fn score(&self, side: u8) -> i32 {
        self.ships
            .iter()
            .filter(|s| s.alive && s.side == side)
            .map(|s| s.hull as i32 + s.shield as i32 + 3)
            .sum()
    }

    fn check_over(&mut self) {
        let a = self.side_alive(0);
        let b = self.side_alive(1);
        if !a || !b {
            self.over = Some(match (a, b) {
                (true, false) => Outcome::Win(Player::One),
                (false, true) => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
            self.phase = Phase::End;
        }
    }

    fn move_order(&self) -> Vec<u8> {
        let mut v: Vec<u8> = (0..6u8).filter(|&i| self.ships[i as usize].alive).collect();
        let r = self.round % 2;
        v.sort_by_key(|&i| {
            let s = &self.ships[i as usize];
            let tie = if s.side == r { 0 } else { 1 };
            (s.t().skill, tie, i)
        });
        v
    }

    fn do_reveal(&mut self) {
        let order = self.move_order();
        for &iu in &order {
            let i = iu as usize;
            if !self.ships[i].alive {
                continue;
            }
            let m = self.ships[i].plan.take().unwrap_or(Man::Stop);
            let (x, y, f) = (self.ships[i].x, self.ships[i].y, self.ships[i].f);
            let (p, nf) = path_of(x, y, f, m);
            let mut cells = vec![(x, y)];
            cells.extend(p.iter().copied().filter(|&c| inside(c)));
            let mut j = cells.len() - 1;
            while j > 0 && self.occupied(cells[j], i) {
                j -= 1;
            }
            let bumped = j != cells.len() - 1;
            let fin = cells[j];
            {
                let s = &mut self.ships[i];
                s.x = fin.0;
                s.y = fin.1;
                s.f = nf;
                s.nofire = bumped;
                if m.red() {
                    s.stress = 1;
                } else if m.green() {
                    s.stress = 0;
                }
            }
            if bumped {
                self.log.push(format!("{} se ciocnește", ship_id(i)));
            }
            if OBST.contains(&fin) {
                self.ships[i].nofire = true;
                self.damage(i, 1);
                self.log.push(format!("{} lovește un asteroid", ship_id(i)));
            }
        }
        // ordinea focului: îndemânare descrescătoare
        let mut fo = order;
        fo.reverse();
        self.order = fo;
        self.pos = usize::MAX;
        self.check_over();
        if self.over.is_none() {
            self.advance_combat();
        }
    }

    /// Trece la următoarea navă care poate trage; la sfârșit încheie runda.
    fn advance_combat(&mut self) {
        let mut p = if self.pos == usize::MAX {
            0
        } else {
            self.pos + 1
        };
        while p < self.order.len() {
            let i = self.order[p] as usize;
            if self.ships[i].alive && !self.targets(i).is_empty() {
                self.pos = p;
                self.phase = Phase::Combat;
                self.side = if self.ships[i].side == 0 {
                    Player::One
                } else {
                    Player::Two
                };
                return;
            }
            p += 1;
        }
        self.end_round();
    }

    fn end_round(&mut self) {
        for s in &mut self.ships {
            s.nofire = false;
        }
        if self.round >= MAX_ROUNDS {
            let a = self.score(0);
            let b = self.score(1);
            self.over = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
            self.phase = Phase::End;
            return;
        }
        self.round += 1;
        self.phase = Phase::Plan;
        self.side = Player::One;
        self.order.clear();
        self.pos = 0;
    }

    fn roll_attack(&mut self, a: usize, d: usize) {
        let (na, nd) = self.dice_counts(a, d);
        let mut h = 0i32;
        let mut c = 0i32;
        for _ in 0..na {
            match self.dice.d6() {
                4 | 5 => h += 1,
                6 => c += 1,
                _ => {}
            }
        }
        let mut e = 0i32;
        for _ in 0..nd {
            if self.dice.d6() >= 5 {
                e += 1;
            }
        }
        let hl = (h - e).max(0);
        let cl = (c - (e - h).max(0)).max(0);
        let dmg = hl + 2 * cl;
        self.log.push(format!(
            "{}>{}: {h} lov + {c} crit, {e} evaziuni, {dmg} daune",
            ship_id(a),
            ship_id(d)
        ));
        self.damage(d, dmg);
    }

    /// Poziția proiectată a navei i după planul ei (fără ciocniri).
    fn projected(&self, i: usize) -> Option<(i32, i32, u8, Man)> {
        let s = &self.ships[i];
        let m = s.plan?;
        let (p, nf) = path_of(s.x, s.y, s.f, m);
        let last = p.last().copied().unwrap_or((s.x, s.y));
        Some((last.0.clamp(0, N - 1), last.1.clamp(0, N - 1), nf, m))
    }

    fn side_of(p: Player) -> u8 {
        p.idx() as u8
    }
}

impl Game for Nave {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::End => Vec::new(),
            Phase::Roll => vec![Move::Roll],
            Phase::Reveal => vec![Move::Reveal],
            Phase::Plan => {
                let side = Self::side_of(self.side);
                match self.next_to_plan(side) {
                    Some(i) => self
                        .valid_dial(i)
                        .into_iter()
                        .map(|m| Move::Plan(i as u8, m))
                        .collect(),
                    None => Vec::new(),
                }
            }
            Phase::Combat => {
                let i = self.order[self.pos] as usize;
                let mut v: Vec<Move> = self
                    .targets(i)
                    .into_iter()
                    .map(|t| Move::Fire(i as u8, t as u8))
                    .collect();
                v.push(Move::Hold(i as u8));
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Plan(i, man) => {
                self.ships[i as usize].plan = Some(man);
                let side = Self::side_of(self.side);
                if self.next_to_plan(side).is_none() {
                    if side == 0 {
                        self.side = Player::Two;
                        if self.next_to_plan(1).is_none() {
                            self.phase = Phase::Reveal;
                            self.side = Player::One;
                        }
                    } else {
                        self.phase = Phase::Reveal;
                        self.side = Player::One;
                    }
                }
            }
            Move::Reveal => {
                self.log.clear();
                self.do_reveal();
            }
            Move::Fire(a, d) => {
                self.pending = Some((a, d));
                self.phase = Phase::Roll;
            }
            Move::Hold(_) => {
                self.advance_combat();
            }
            Move::Roll => {
                if let Some((a, d)) = self.pending.take() {
                    self.roll_attack(a as usize, d as usize);
                }
                self.check_over();
                if self.over.is_none() {
                    self.advance_combat();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8;
        let mut v = (self.score(me) - self.score(1 - me)) * 10;
        // atac declarat: daună așteptată
        if let Some((a, d)) = self.pending {
            let (na, nd) = self.dice_counts(a as usize, d as usize);
            let e = (Self::expected_damage(na, nd) * 10.0) as i32;
            if self.ships[a as usize].side == me {
                v += e;
            } else {
                v -= e;
            }
        }
        // poziționare: doar planurile PROPRII (informație ascunsă)
        for (i, s) in self.ships.iter().enumerate() {
            if !s.alive || s.side != me {
                continue;
            }
            if let Some((x, y, f, m)) = self.projected(i) {
                let ghost = Ship {
                    x,
                    y,
                    f,
                    ..s.clone()
                };
                let mut atk = 0;
                let mut exposed = 0;
                for e in self.ships.iter().filter(|e| e.alive && e.side != me) {
                    if ghost.range_to(e.x, e.y).is_some() {
                        atk += 1;
                    }
                    if e.range_to(x, y).is_some() {
                        exposed += 1;
                    }
                }
                v += atk * 14 - exposed * 6;
                if m.red() {
                    v -= 8;
                }
                let cx = (x - 5).abs().min((x - 6).abs());
                let cy = (y - 5).abs().min((y - 6).abs());
                v -= cx + cy;
                if OBST.contains(&(x, y)) {
                    v -= 25;
                }
                // mai aproape de inamic când nu are țintă
                if atk == 0 {
                    let d = self
                        .ships
                        .iter()
                        .filter(|e| e.alive && e.side != me)
                        .map(|e| (e.x - x).abs().max((e.y - y).abs()))
                        .min()
                        .unwrap_or(0);
                    v -= d;
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Escadrile din Rift — runda {}/{}  ({})\n",
            self.round,
            MAX_ROUNDS,
            match self.phase {
                Phase::Plan => "planificare secretă",
                Phase::Reveal => "dezvăluire",
                Phase::Combat => "foc",
                Phase::Roll => "zaruri",
                Phase::End => "sfârșit",
            }
        ));
        for y in (0..N).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..N {
                let mut cell = if OBST.contains(&(x, y)) {
                    " * ".to_string()
                } else {
                    " . ".to_string()
                };
                for (i, sh) in self.ships.iter().enumerate() {
                    if sh.alive && sh.x == x && sh.y == y {
                        cell = format!("{}{} ", sh.id(i), ARROWS[sh.f as usize]);
                    }
                }
                s.push_str(&cell);
            }
            s.push('\n');
        }
        s.push_str("   ");
        for x in 0..N {
            s.push_str(&format!(" {} ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        let v = self.viewer.unwrap_or(self.side);
        for (i, sh) in self.ships.iter().enumerate() {
            let plan = if sh.side == v.idx() as u8 {
                sh.plan
                    .map(|m| format!(" plan {}", m.text()))
                    .unwrap_or_default()
            } else {
                String::new()
            };
            s.push_str(&format!(
                "{} {:<6} {} {}  coca {} scut {} stres {}{}{}\n",
                ship_id(i),
                sh.t().name,
                if sh.alive { "      " } else { "[dist]" },
                cell_name(sh.x, sh.y),
                sh.hull.max(0),
                sh.shield,
                sh.stress,
                plan,
                if sh.nofire && sh.alive {
                    " (nu trage)"
                } else {
                    ""
                }
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
            Move::Plan(i, man) => format!("{} {}", ship_id(*i as usize), man.text()),
            Move::Reveal => "revela".into(),
            Move::Fire(a, d) => format!("{}>{}", ship_id(*a as usize), ship_id(*d as usize)),
            Move::Hold(i) => format!("{} pas", ship_id(*i as usize)),
            Move::Roll => "arunca".into(),
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

    fn total_health(g: &Nave) -> i32 {
        g.ships
            .iter()
            .filter(|s| s.alive)
            .map(|s| s.hull as i32 + s.shield as i32)
            .sum()
    }

    #[test]
    fn paths_geometry() {
        // spre N din (5,5)
        assert_eq!(path_of(5, 5, 0, Man::Str(2)).0, vec![(5, 6), (5, 7)]);
        let (p, f) = path_of(5, 5, 0, Man::BankR(2));
        assert_eq!(*p.last().unwrap(), (6, 7));
        assert_eq!(f, 0);
        let (p, f) = path_of(5, 5, 0, Man::TurnL(2));
        assert_eq!(*p.last().unwrap(), (3, 7));
        assert_eq!(f, 3);
        let (p, f) = path_of(5, 5, 1, Man::TurnR(1));
        assert_eq!(*p.last().unwrap(), (6, 4));
        assert_eq!(f, 2);
        let (p, f) = path_of(5, 5, 2, Man::UTurn);
        assert_eq!(*p.last().unwrap(), (5, 2));
        assert_eq!(f, 0);
        assert!(path_of(5, 5, 3, Man::Stop).0.is_empty());
    }

    #[test]
    fn arcs_and_range() {
        let s = Nave::with_seed(1).ships[2].clone(); // Vrabie la (8,0) spre N
        assert_eq!(s.range_to(8, 1), Some(1));
        assert_eq!(s.range_to(9, 2), Some(2));
        assert_eq!(s.range_to(10, 2), Some(2)); // lateral == înainte: încă în arc
        assert_eq!(s.range_to(11, 2), None);
        assert_eq!(s.range_to(8, 0), None);
        assert_eq!(s.range_to(8, 4), None); // prea departe
        assert_eq!(s.range_to(8, 0 - 1), None);
        let heavy = Nave::with_seed(1).ships[0].clone();
        assert!(heavy.range_to(4, 0).is_some()); // turelă
    }

    #[test]
    fn stress_rules() {
        let mut g = Nave::with_seed(2);
        g.ships[0].stress = 1;
        let d = g.valid_dial(0);
        assert!(d.iter().all(|m| !m.red()));
        assert!(!d.is_empty());
        // Bivolul nu are întoarcere
        let g = Nave::with_seed(2);
        assert!(!g.valid_dial(0).contains(&Man::UTurn));
        assert!(g.valid_dial(1).contains(&Man::UTurn));
    }

    #[test]
    fn expected_damage_sane() {
        assert!(Nave::expected_damage(3, 0) > Nave::expected_damage(3, 2));
        assert!(Nave::expected_damage(4, 1) > Nave::expected_damage(2, 1));
        assert!(Nave::expected_damage(0, 3).abs() < 1e-6);
        // 1 zar de atac, fără apărare: 1/3*1 + 1/6*2 = 2/3
        assert!((Nave::expected_damage(1, 0) - 2.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn damage_shields_first() {
        let mut g = Nave::with_seed(3);
        g.damage(1, 3); // Șoim: scut 2, coca 3
        assert_eq!(g.ships[1].shield, 0);
        assert_eq!(g.ships[1].hull, 2);
        g.damage(1, 5);
        assert!(!g.ships[1].alive);
    }

    #[test]
    fn bump_stops_ship_and_blocks_fire() {
        let mut g = Nave::with_seed(4);
        // Vrabie A2 (8,0) N; pune o navă inamică pe (8,2)
        g.ships[5] = Ship {
            x: 8,
            y: 2,
            f: 2,
            ..g.ships[5].clone()
        };
        for i in 0..6 {
            g.ships[i].plan = Some(Man::Stop);
        }
        g.ships[2].plan = Some(Man::Str(2));
        g.do_reveal();
        assert_eq!((g.ships[2].x, g.ships[2].y), (8, 1));
        assert!(g.ships[2].nofire);
    }

    #[test]
    fn asteroid_hurts() {
        let mut g = Nave::with_seed(4);
        g.ships[2].x = 3;
        g.ships[2].y = 6;
        for i in 0..6 {
            g.ships[i].plan = Some(Man::Stop);
        }
        g.ships[2].plan = Some(Man::Str(2)); // ajunge pe (3,8), asteroid
        g.do_reveal();
        assert_eq!(g.ships[2].hull, 2);
        assert!(g.ships[2].nofire);
    }

    #[test]
    fn plan_flow_and_hidden_plans() {
        let mut g = Nave::with_seed(7);
        assert_eq!(g.to_move(), Player::One);
        for _ in 0..3 {
            let m = Game::legal_moves(&g)[0].clone();
            g.apply(&m);
        }
        assert_eq!(g.to_move(), Player::Two);
        // planurile lui P1 apar în render (P1 e omul)
        assert!(Game::render(&g).contains("plan "));
        for _ in 0..3 {
            let m = Game::legal_moves(&g)[0].clone();
            g.apply(&m);
        }
        assert_eq!(Game::legal_moves(&g), vec![Move::Reveal]);
        // planurile lui P2 nu se afișează niciodată
        let r = Game::render(&g);
        assert_eq!(r.matches("plan ").count(), 3);
    }

    #[test]
    fn ai_ignores_opponent_plan() {
        // două stări identice, în afara planului lui P1: AI-ul (P2) alege la fel
        let mut a = Nave::with_seed(9);
        let mut b = a.clone();
        let la = Game::legal_moves(&a);
        let lb = Game::legal_moves(&b);
        a.apply(&la[0]);
        b.apply(&lb[lb.len() - 1]);
        for g in [&mut a, &mut b] {
            for _ in 0..2 {
                let m = Game::legal_moves(g)[0].clone();
                g.apply(&m);
            }
        }
        assert_eq!(a.to_move(), Player::Two);
        for seed in 1..6 {
            let ma = polytabletop_core::GameObj::ai_move(&a, seed);
            let mb = polytabletop_core::GameObj::ai_move(&b, seed);
            assert_eq!(ma, mb);
        }
    }

    #[test]
    fn random_games_end_and_health_never_rises() {
        for seed in 1..=12u64 {
            let mut g = Nave::with_seed(seed);
            let mut rng = Rng::new(seed * 31);
            let mut last = total_health(&g);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = Game::legal_moves(&g);
                assert!(!mv.is_empty(), "{}", Game::render(&g));
                let m = mv[rng.below(mv.len())].clone();
                g.apply(&m);
                let h = total_health(&g);
                assert!(h <= last);
                last = h;
                n += 1;
                assert!(n < 3000);
            }
            assert!(g.round <= MAX_ROUNDS);
        }
    }

    #[test]
    fn ai_beats_random_mostly_and_is_fast() {
        let mut wins = 0;
        let mut worst = 0u128;
        for seed in 1..=10u64 {
            let mut g = Nave::with_seed(seed + 100);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = if g.to_move() == Player::Two {
                    let t = std::time::Instant::now();
                    let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                    worst = worst.max(t.elapsed().as_millis());
                    m
                } else {
                    let mv = Game::legal_moves(&g);
                    mv[rng.below(mv.len())].clone()
                };
                g.apply(&m);
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 7, "AI a câștigat doar {wins}/10");
        assert!(worst < 1000, "AI prea lent: {worst} ms");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn viewer_sees_only_own_plans() {
        let mut g = Nave::with_seed(7);
        for _ in 0..3 {
            let m = Game::legal_moves(&g)[0].clone();
            g.apply(&m);
        }
        assert_eq!(g.to_move(), Player::Two);
        Game::set_viewer(&mut g, Some(Player::Two));
        assert_eq!(Game::render(&g).matches("plan ").count(), 0);
        Game::set_viewer(&mut g, None); // hot-seat: mută J2
        assert_eq!(Game::render(&g).matches("plan ").count(), 0);
        Game::set_viewer(&mut g, Some(Player::One));
        assert_eq!(Game::render(&g).matches("plan ").count(), 3);
    }
}
