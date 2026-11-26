//! Motor generic de miniaturi pe hărți hexagonale, 2 jucători, cu unități
//! ORIGINALE ("Ținutul Cenușiu": 6 tipuri) și un scenariu cu obiective.
//!
//! Reguli (scrise de noi, din mecanici generice):
//! * Hartă hexagonală 9 rânduri (rândurile pare 11 hexuri, cele impare 10), cu
//!   teren: `#` stâncă (nu se trece, blochează linia de vizibilitate), `f` pădure
//!   (mișcare 2, acoperire: salvarea apărătorului +1), `O` obiectiv.
//! * Armatele (7 unități fiecare, simetrice): Lăncieri (L)×2, Arcași (A),
//!   Călăreți (C), Colos (G), Catapulte (T, grele: mută SAU trage), Comandant (K).
//!   Unitățile au tag-uri A–G (J1) și H–N (J2).
//! * Runda: jucătorii activează pe rând câte o unitate (J1 începe rundele
//!   impare, J2 pe cele pare); fiecare unitate se activează o dată pe rundă.
//!   O activare = mutare opțională (cost de mișcare ≤ mv; nu treci prin alte
//!   unități; intrarea lângă un inamic oprește mișcarea) + o acțiune opțională:
//!   foc (distanță ≤ rază, linie de vizibilitate liberă, nu cât timp ești lipit
//!   de un inamic) sau luptă corp la corp cu un inamic alăturat.
//! * Zaruri: ataci cu (figuri × zaruri/figură) zaruri d6; lovește ≥ ținta de
//!   lovire (foc la peste jumătate din rază: +1; Călăreți care au mutat ≥ 3: −1
//!   la corp la corp; Lăncieri contra Călăreți: −1, Călăreți contra Lăncieri:
//!   +1). Fiecare lovitură e apoi salvată de un d6 ≥ salvarea apărătorului
//!   (acoperire: −1). Fiecare lovitură nesalvată elimină o figură. Apărătorul
//!   supraviețuitor lovește înapoi în corp la corp.
//! * Moral: o unitate care pierde figuri dă un d6; cu zarul + pierderile (+1 sub
//!   jumătate de efectiv, −1 dacă un Comandant prieten e la ≤ 2 hexuri) depășește
//!   moral plus 1, unitatea devine Șovăitoare (nu poate ataca, nu contează la obiective); a doua
//!   cădere de moral o pune pe fugă (distrusă). Șovăitoarele își revin la
//!   sfârșitul rundei dacă nu au inamic alăturat.
//! * Obiective: la sfârșitul rundei, obiectivul e al celui cu mai multe figuri
//!   nesovăitoare la ≤ 1 hex: 1 punct de victorie. 6 runde; câștigă cine are mai
//!   multe puncte, apoi mai mult efectiv rămas; distrugerea întregii armate
//!   adverse câștigă imediat.
//!
//! Zarurile și AI-ul: atacul este o mutare în două trepte. Declari (mutare +
//! țintă), apoi mutarea forțată `arunca` aruncă zarurile (Rng propriu al stării;
//! la `clone()` zarurile își schimbă sămânța, deci o căutare nu poate „ghici”
//! aruncarea reală). Informație completă; AI = AlphaBeta(1), iar `evaluate`
//! folosește valoarea AȘTEPTATĂ a atacului declarat (calcul exact, fără zaruri).
//!
//! Mutări: `B pas`, `B>d4`, `B>d4 xK`, `B xK`, `arunca`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::cell::Cell;

pub const W: usize = 11;
pub const H: usize = 9;
const MAX_ROUNDS: u8 = 6;

pub struct UType {
    pub name: &'static str,
    pub glyph: char,
    pub figs: i8,
    pub mv: u8,
    pub range: u8,
    pub sdice: u8,
    pub shit: u8,
    pub mdice: u8,
    pub mhit: u8,
    pub save: u8,
    pub morale: u8,
    pub cost: u16,
    pub heavy: bool,
}

pub const LANC: u8 = 0;
pub const ARC: u8 = 1;
pub const CAL: u8 = 2;
pub const COLOS: u8 = 3;
pub const CAT: u8 = 4;
pub const COMAND: u8 = 5;

pub const TYPES: [UType; 6] = [
    UType {
        name: "Lăncieri",
        glyph: 'L',
        figs: 5,
        mv: 3,
        range: 0,
        sdice: 0,
        shit: 7,
        mdice: 1,
        mhit: 4,
        save: 5,
        morale: 4,
        cost: 50,
        heavy: false,
    },
    UType {
        name: "Arcași",
        glyph: 'A',
        figs: 4,
        mv: 3,
        range: 7,
        sdice: 1,
        shit: 4,
        mdice: 1,
        mhit: 5,
        save: 6,
        morale: 3,
        cost: 55,
        heavy: false,
    },
    UType {
        name: "Călăreți",
        glyph: 'C',
        figs: 3,
        mv: 6,
        range: 0,
        sdice: 0,
        shit: 7,
        mdice: 2,
        mhit: 4,
        save: 5,
        morale: 4,
        cost: 70,
        heavy: false,
    },
    UType {
        name: "Colos",
        glyph: 'G',
        figs: 4,
        mv: 2,
        range: 0,
        sdice: 0,
        shit: 7,
        mdice: 2,
        mhit: 4,
        save: 4,
        morale: 5,
        cost: 100,
        heavy: false,
    },
    UType {
        name: "Catapulte",
        glyph: 'T',
        figs: 2,
        mv: 2,
        range: 10,
        sdice: 3,
        shit: 5,
        mdice: 1,
        mhit: 5,
        save: 6,
        morale: 2,
        cost: 70,
        heavy: true,
    },
    UType {
        name: "Comandant",
        glyph: 'K',
        figs: 2,
        mv: 4,
        range: 4,
        sdice: 1,
        shit: 4,
        mdice: 2,
        mhit: 4,
        save: 4,
        morale: 5,
        cost: 60,
        heavy: false,
    },
];

/// (tip, col, rând) pentru J1; J2 primește poziția în oglindă.
const DEPLOY: [(u8, u8, u8); 7] = [
    (LANC, 3, 1),
    (LANC, 6, 1),
    (ARC, 5, 0),
    (CAL, 8, 2),
    (COLOS, 4, 2),
    (CAT, 3, 0),
    (COMAND, 4, 1),
];

const TERRAIN_TOP: [&str; 5] = [
    ".....f.....",
    "..f......f",
    "..#...f..f.",
    ".f...#..f.",
    "f..O.O.O..f",
];

pub fn row_width(r: usize) -> usize {
    if r.is_multiple_of(2) {
        W
    } else {
        W - 1
    }
}

pub fn valid(c: i32, r: i32) -> bool {
    r >= 0 && r < H as i32 && c >= 0 && c < row_width(r as usize) as i32
}

/// Oglindirea prin centrul hărții (rotație de 180°).
pub fn mirror(c: usize, r: usize) -> (usize, usize) {
    if r.is_multiple_of(2) {
        (W - 1 - c, H - 1 - r)
    } else {
        (W - 2 - c, H - 1 - r)
    }
}

fn to_cube(c: i32, r: i32) -> (i32, i32, i32) {
    let x = c - (r - (r & 1)) / 2;
    (x, -x - r, r)
}

fn from_cube(x: i32, z: i32) -> (i32, i32) {
    (x + (z - (z & 1)) / 2, z)
}

pub fn hex_dist(a: (u8, u8), b: (u8, u8)) -> i32 {
    let (ax, ay, az) = to_cube(a.0 as i32, a.1 as i32);
    let (bx, by, bz) = to_cube(b.0 as i32, b.1 as i32);
    (ax - bx).abs().max((ay - by).abs()).max((az - bz).abs())
}

pub fn neighbors(p: (u8, u8)) -> Vec<(u8, u8)> {
    const D: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
    let (x, _, z) = to_cube(p.0 as i32, p.1 as i32);
    let mut v = Vec::with_capacity(6);
    for (dx, dz) in D {
        let (c, r) = from_cube(x + dx, z + dz);
        if valid(c, r) {
            v.push((c as u8, r as u8));
        }
    }
    v
}

/// Hexurile de pe linia dreaptă a→b (inclusiv capetele).
pub fn line(a: (u8, u8), b: (u8, u8)) -> Vec<(i32, i32)> {
    let n = hex_dist(a, b);
    let (ax, ay, az) = to_cube(a.0 as i32, a.1 as i32);
    let (bx, by, bz) = to_cube(b.0 as i32, b.1 as i32);
    let mut out = Vec::new();
    for i in 0..=n {
        let t = if n == 0 { 0.0 } else { i as f64 / n as f64 };
        let fx = ax as f64 + 1e-6 + (bx - ax) as f64 * t;
        let fy = ay as f64 + 2e-6 + (by - ay) as f64 * t;
        let fz = az as f64 - 3e-6 + (bz - az) as f64 * t;
        let (mut rx, mut ry, mut rz) = (fx.round(), fy.round(), fz.round());
        let (dx, dy, dz) = ((rx - fx).abs(), (ry - fy).abs(), (rz - fz).abs());
        if dx > dy && dx > dz {
            rx = -ry - rz;
        } else if dy > dz {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }
        let _ = ry;
        let (c, r) = from_cube(rx as i32, rz as i32);
        out.push((c, r));
    }
    out
}

pub fn hex_name(p: (u8, u8)) -> String {
    format!("{}{}", (b'a' + p.0) as char, p.1 + 1)
}

fn parse_hex(s: &str) -> Option<(u8, u8)> {
    let mut ch = s.chars();
    let c = ch.next()?;
    if !c.is_ascii_lowercase() {
        return None;
    }
    let r: u8 = ch.as_str().parse().ok()?;
    Some((c as u8 - b'a', r.checked_sub(1)?))
}

/// Zarurile stării. La `clone()` sămânța se schimbă, ca o căutare AI să nu poată
/// „prevedea” aruncarea reală.
pub struct Dice {
    rng: Rng,
    ctr: Cell<u64>,
}

impl Clone for Dice {
    fn clone(&self) -> Self {
        let k = self.ctr.get().wrapping_add(1);
        self.ctr.set(k);
        let mut r = self.rng.clone();
        Dice {
            rng: Rng::new(r.next_u64() ^ k.wrapping_mul(0x9E37_79B9_7F4A_7C15)),
            ctr: Cell::new(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub ut: u8,
    pub side: u8,
    pub pos: (u8, u8),
    pub figs: i8,
    pub shaken: bool,
    pub acted: bool,
}

impl Unit {
    pub fn alive(&self) -> bool {
        self.figs > 0
    }
    pub fn ty(&self) -> &'static UType {
        &TYPES[self.ut as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pending {
    pub att: u8,
    pub def: u8,
    pub melee: bool,
    pub charge: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Act {
        u: u8,
        to: Option<(u8, u8)>,
        target: Option<u8>,
    },
    Roll,
}

#[derive(Clone)]
pub struct Miniaturi {
    pub units: Vec<Unit>,
    terrain: [[u8; W]; H],
    pub objectives: Vec<(u8, u8)>,
    pub side: Player,
    pub first: Player,
    pub round: u8,
    pub vp: [u8; 2],
    pub pending: Option<Pending>,
    dice: Dice,
    over: Option<Outcome>,
}

impl Default for Miniaturi {
    fn default() -> Self {
        Self::new()
    }
}

pub fn tag(u: usize) -> char {
    (b'A' + u as u8) as char
}

fn prob(t: i32) -> f32 {
    if t >= 7 {
        0.0
    } else {
        (7 - t.max(2)) as f32 / 6.0
    }
}

/// Profilul unui atac: (zaruri, p(lovitură), p(nesalvată)).
struct Profile {
    n: i32,
    hit: i32,
    save: i32,
}

impl Profile {
    fn expected(&self) -> f32 {
        self.n as f32 * prob(self.hit) * (1.0 - prob(self.save))
    }
}

impl Miniaturi {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut terrain = [[b'.'; W]; H];
        for (r, row) in TERRAIN_TOP.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                terrain[r][c] = ch;
            }
        }
        for r in 5..H {
            for c in 0..row_width(r) {
                let (mc, mr) = mirror(c, r);
                terrain[r][c] = terrain[mr][mc];
            }
        }
        let mut objectives = Vec::new();
        for (r, row) in terrain.iter().enumerate() {
            for (c, t) in row.iter().enumerate() {
                if *t == b'O' {
                    objectives.push((c as u8, r as u8));
                }
            }
        }
        let mut units = Vec::new();
        for side in 0..2u8 {
            for (ut, c, r) in DEPLOY {
                let pos = if side == 0 {
                    (c, r)
                } else {
                    let (mc, mr) = mirror(c as usize, r as usize);
                    (mc as u8, mr as u8)
                };
                units.push(Unit {
                    ut,
                    side,
                    pos,
                    figs: TYPES[ut as usize].figs,
                    shaken: false,
                    acted: false,
                });
            }
        }
        Miniaturi {
            units,
            terrain,
            objectives,
            side: Player::One,
            first: Player::One,
            round: 1,
            vp: [0, 0],
            pending: None,
            dice: Dice {
                rng: Rng::new(seed),
                ctr: Cell::new(0),
            },
            over: None,
        }
    }

    pub fn terrain_at(&self, p: (u8, u8)) -> u8 {
        self.terrain[p.1 as usize][p.0 as usize]
    }

    fn occupant(&self, p: (u8, u8)) -> Option<usize> {
        self.units.iter().position(|u| u.alive() && u.pos == p)
    }

    fn adjacent_enemy(&self, side: u8, p: (u8, u8)) -> bool {
        self.units
            .iter()
            .any(|u| u.alive() && u.side != side && hex_dist(u.pos, p) == 1)
    }

    /// Linie de vizibilitate: nicio stâncă între capete.
    pub fn los(&self, a: (u8, u8), b: (u8, u8)) -> bool {
        let l = line(a, b);
        l.iter()
            .skip(1)
            .take(l.len().saturating_sub(2))
            .all(|p| !valid(p.0, p.1) || self.terrain[p.1 as usize][p.0 as usize] != b'#')
    }

    /// Hexurile accesibile unității `ui` (fără cel de start) și costul lor.
    pub fn reach(&self, ui: usize) -> Vec<((u8, u8), u8)> {
        let u = &self.units[ui];
        let mv = u.ty().mv;
        let mut cost = [[u8::MAX; W]; H];
        cost[u.pos.1 as usize][u.pos.0 as usize] = 0;
        let mut frontier = vec![u.pos];
        let mut out = Vec::new();
        while !frontier.is_empty() {
            // Dijkstra simplu: cel mai ieftin hex primul.
            let (bi, _) = frontier
                .iter()
                .enumerate()
                .min_by_key(|(_, p)| cost[p.1 as usize][p.0 as usize])
                .unwrap();
            let cur = frontier.swap_remove(bi);
            let cc = cost[cur.1 as usize][cur.0 as usize];
            if cur != u.pos && self.adjacent_enemy(u.side, cur) {
                continue; // contactul oprește mișcarea
            }
            for n in neighbors(cur) {
                let t = self.terrain_at(n);
                if t == b'#' || self.occupant(n).is_some() {
                    continue;
                }
                let step = if t == b'f' { 2 } else { 1 };
                let nc = cc + step;
                if nc > mv {
                    continue;
                }
                if nc < cost[n.1 as usize][n.0 as usize] {
                    if !frontier.contains(&n) {
                        frontier.push(n);
                    }
                    cost[n.1 as usize][n.0 as usize] = nc;
                }
            }
        }
        for (r, row) in cost.iter().enumerate() {
            for (c, k) in row.iter().enumerate() {
                if *k != u8::MAX && (c as u8, r as u8) != u.pos {
                    out.push(((c as u8, r as u8), *k));
                }
            }
        }
        out
    }

    fn profile(&self, a: usize, d: usize, melee: bool, charge: bool, dist: i32) -> Profile {
        let au = &self.units[a];
        let du = &self.units[d];
        let at = au.ty();
        let dt = du.ty();
        let (n, mut hit) = if melee {
            (au.figs as i32 * at.mdice as i32, at.mhit as i32)
        } else {
            let mut h = at.shit as i32;
            if dist * 2 > at.range as i32 {
                h += 1;
            }
            (au.figs as i32 * at.sdice as i32, h)
        };
        if melee {
            if charge && au.ut == CAL {
                hit -= 1;
            }
            if au.ut == CAL && du.ut == LANC {
                hit += 1;
            }
            if au.ut == LANC && du.ut == CAL {
                hit -= 1;
            }
        }
        let mut save = dt.save as i32;
        if self.terrain_at(du.pos) == b'f' {
            save = if save >= 7 { 6 } else { (save - 1).max(2) };
        }
        Profile { n, hit, save }
    }

    fn roll_wounds(&mut self, p: &Profile, figs: i8) -> i8 {
        let mut unsaved = 0i32;
        for _ in 0..p.n {
            if self.dice.rng.d6() as i32 >= p.hit.max(2) && p.hit < 7 {
                let saved = p.save < 7 && self.dice.rng.d6() as i32 >= p.save.max(2);
                if !saved {
                    unsaved += 1;
                }
            }
        }
        unsaved.min(figs as i32) as i8
    }

    fn morale(&mut self, ui: usize, wounds: i8) {
        let t = self.units[ui].ty();
        let side = self.units[ui].side;
        let low = (self.units[ui].figs * 2 <= t.figs) as i32;
        let aura = self.units.iter().enumerate().any(|(i, o)| {
            i != ui
                && o.alive()
                && o.side == side
                && o.ut == COMAND
                && hex_dist(o.pos, self.units[ui].pos) <= 2
        }) as i32;
        let r = self.dice.rng.d6() as i32;
        if r + wounds as i32 + low - aura > t.morale as i32 + 1 {
            if self.units[ui].shaken {
                self.units[ui].figs = 0;
            } else {
                self.units[ui].shaken = true;
            }
        }
    }

    fn has_unacted(&self, side: Player) -> bool {
        self.units
            .iter()
            .any(|u| u.alive() && u.side == side.idx() as u8 && !u.acted)
    }

    fn army_cost(&self, side: usize) -> f32 {
        self.units
            .iter()
            .filter(|u| u.alive() && u.side as usize == side)
            .map(|u| u.ty().cost as f32 * u.figs as f32 / u.ty().figs as f32)
            .sum()
    }

    fn check_elimination(&mut self) {
        let a0 = self.units.iter().any(|u| u.alive() && u.side == 0);
        let a1 = self.units.iter().any(|u| u.alive() && u.side == 1);
        self.over = match (a0, a1) {
            (false, false) => Some(Outcome::Draw),
            (false, true) => Some(Outcome::Win(Player::Two)),
            (true, false) => Some(Outcome::Win(Player::One)),
            _ => None,
        };
    }

    /// Controlul unui obiectiv: figurile nesovăitoare la ≤ 1 hex.
    pub fn control(&self, obj: (u8, u8)) -> Option<Player> {
        let mut f = [0i32; 2];
        for u in &self.units {
            if u.alive() && !u.shaken && hex_dist(u.pos, obj) <= 1 {
                f[u.side as usize] += u.figs as i32;
            }
        }
        match f[0].cmp(&f[1]) {
            std::cmp::Ordering::Greater => Some(Player::One),
            std::cmp::Ordering::Less => Some(Player::Two),
            _ => None,
        }
    }

    fn end_round(&mut self) {
        for o in self.objectives.clone() {
            if let Some(p) = self.control(o) {
                self.vp[p.idx()] += 1;
            }
        }
        // revenirea celor șovăitoare
        let snap = self.units.clone();
        for u in &mut self.units {
            if u.alive() && u.shaken {
                let adj = snap
                    .iter()
                    .any(|e| e.alive() && e.side != u.side && hex_dist(e.pos, u.pos) == 1);
                if !adj {
                    u.shaken = false;
                }
            }
            u.acted = false;
        }
        if self.round >= MAX_ROUNDS {
            let (c0, c1) = (self.army_cost(0), self.army_cost(1));
            self.over = Some(if self.vp[0] != self.vp[1] {
                Outcome::Win(if self.vp[0] > self.vp[1] {
                    Player::One
                } else {
                    Player::Two
                })
            } else if (c0 - c1).abs() > 0.5 {
                Outcome::Win(if c0 > c1 { Player::One } else { Player::Two })
            } else {
                Outcome::Draw
            });
            return;
        }
        self.round += 1;
        self.first = self.first.other();
        self.side = self.first;
    }

    fn advance(&mut self) {
        self.check_elimination();
        if self.over.is_some() {
            return;
        }
        let o = self.side.other();
        if self.has_unacted(o) {
            self.side = o;
        } else if !self.has_unacted(self.side) {
            self.end_round();
        }
    }

    fn resolve(&mut self) {
        let Some(p) = self.pending.take() else {
            return;
        };
        let (a, d) = (p.att as usize, p.def as usize);
        let dist = hex_dist(self.units[a].pos, self.units[d].pos);
        let prof = self.profile(a, d, p.melee, p.charge, dist);
        let dfigs = self.units[d].figs;
        let w = self.roll_wounds(&prof, dfigs);
        self.units[d].figs -= w;
        let mut wounded = Vec::new();
        if w > 0 {
            wounded.push((d, w));
        }
        if p.melee && self.units[d].figs > 0 {
            let prof2 = self.profile(d, a, true, false, 1);
            let afigs = self.units[a].figs;
            let w2 = self.roll_wounds(&prof2, afigs);
            self.units[a].figs -= w2;
            if w2 > 0 {
                wounded.push((a, w2));
            }
        }
        for (u, w) in wounded {
            if self.units[u].alive() {
                self.morale(u, w);
            }
        }
        self.advance();
    }

    /// Valoarea așteptată (în puncte de efectiv) a atacului declarat, din
    /// punctul de vedere al atacatorului: (daune pricinuite, daune primite).
    fn pending_expect(&self) -> Option<(f32, f32)> {
        let p = self.pending?;
        let (a, d) = (p.att as usize, p.def as usize);
        let dist = hex_dist(self.units[a].pos, self.units[d].pos);
        let pr = self.profile(a, d, p.melee, p.charge, dist);
        let ew = pr.expected().min(self.units[d].figs as f32);
        let mut cw = 0.0;
        if p.melee {
            let left = (self.units[d].figs as f32 - ew).max(0.0);
            let pr2 = self.profile(d, a, true, false, 1);
            let scale = if self.units[d].figs > 0 {
                left / self.units[d].figs as f32
            } else {
                0.0
            };
            cw = (pr2.expected() * scale).min(self.units[a].figs as f32);
        }
        let cpf = |u: &Unit| u.ty().cost as f32 / u.ty().figs as f32;
        Some((ew * cpf(&self.units[d]), cw * cpf(&self.units[a])))
    }

    fn threat(&self, s: &Unit, t: usize) -> f32 {
        let tu = &self.units[t];
        let d = hex_dist(s.pos, tu.pos);
        let st = s.ty();
        let tcpf = tu.ty().cost as f32 / tu.ty().figs as f32;
        let mut best = 0.0f32;
        if st.sdice > 0 && d <= st.range as i32 + if st.heavy { 0 } else { st.mv as i32 } {
            let h = st.shit as i32 + (d * 2 > st.range as i32) as i32;
            let mut save = tu.ty().save as i32;
            if self.terrain_at(tu.pos) == b'f' {
                save = if save >= 7 { 6 } else { (save - 1).max(2) };
            }
            let ew = s.figs as f32 * st.sdice as f32 * prob(h) * (1.0 - prob(save));
            best = best.max(ew * tcpf);
        }
        if st.mdice > 0 && d <= st.mv as i32 + 1 {
            let mut save = tu.ty().save as i32;
            if self.terrain_at(tu.pos) == b'f' {
                save = if save >= 7 { 6 } else { (save - 1).max(2) };
            }
            let ew = s.figs as f32 * st.mdice as f32 * prob(st.mhit as i32) * (1.0 - prob(save));
            best = best.max(ew * tcpf * 0.8);
        }
        best
    }
}

impl Game for Miniaturi {
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
        for (ui, u) in self.units.iter().enumerate() {
            if !u.alive() || u.side != side || u.acted {
                continue;
            }
            let t = u.ty();
            v.push(Move::Act {
                u: ui as u8,
                to: None,
                target: None,
            });
            let mut spots: Vec<((u8, u8), u8)> = vec![(u.pos, 0)];
            spots.extend(self.reach(ui));
            for (pos, _) in spots {
                let moved = pos != u.pos;
                let to = if moved { Some(pos) } else { None };
                if moved {
                    v.push(Move::Act {
                        u: ui as u8,
                        to,
                        target: None,
                    });
                }
                if u.shaken {
                    continue;
                }
                let engaged = self.adjacent_enemy_from(ui, pos);
                for (ti, e) in self.units.iter().enumerate() {
                    if !e.alive() || e.side == side {
                        continue;
                    }
                    let d = hex_dist(pos, e.pos);
                    let melee_ok = d == 1 && t.mdice > 0;
                    let shoot_ok = !engaged
                        && t.sdice > 0
                        && d <= t.range as i32
                        && d >= 1
                        && !(t.heavy && moved)
                        && self.los(pos, e.pos);
                    if melee_ok || shoot_ok {
                        v.push(Move::Act {
                            u: ui as u8,
                            to,
                            target: Some(ti as u8),
                        });
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        if self.over.is_some() {
            return;
        }
        match *m {
            Move::Roll => self.resolve(),
            Move::Act { u, to, target } => {
                let ui = u as usize;
                let cost = match to {
                    Some(p) => self
                        .reach(ui)
                        .iter()
                        .find(|(q, _)| *q == p)
                        .map(|(_, c)| *c)
                        .unwrap_or(0),
                    None => 0,
                };
                if let Some(p) = to {
                    self.units[ui].pos = p;
                }
                self.units[ui].acted = true;
                match target {
                    Some(t) => {
                        // Alăturat = corp la corp (lipit de inamic nu se trage la distanță).
                        let melee = hex_dist(self.units[ui].pos, self.units[t as usize].pos) == 1;
                        self.pending = Some(Pending {
                            att: u,
                            def: t,
                            melee,
                            charge: self.units[ui].ut == CAL && cost >= 3,
                        });
                    }
                    None => self.advance(),
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let foe = 1 - me;
        let mut v = 60.0 * (self.vp[me] as f32 - self.vp[foe] as f32);
        let mut mine = 0.0f32;
        let mut theirs = 0.0f32;
        for (i, u) in self.units.iter().enumerate() {
            if !u.alive() {
                continue;
            }
            let t = u.ty();
            let mut val = t.cost as f32 * u.figs as f32 / t.figs as f32;
            if u.shaken {
                val *= 0.7;
            }
            let mut pull = 0.0;
            if !u.shaken {
                let dmin = self
                    .objectives
                    .iter()
                    .filter(|o| {
                        self.control(**o)
                            != Some(if u.side == 0 {
                                Player::One
                            } else {
                                Player::Two
                            })
                    })
                    .map(|o| hex_dist(u.pos, *o))
                    .min()
                    .unwrap_or(0) as f32;
                let w = if t.heavy { 0.5 } else { 1.2 };
                pull = -w * dmin;
            }
            let mut thr = 0.0;
            for (j, e) in self.units.iter().enumerate() {
                if e.alive() && e.side != u.side && !e.shaken {
                    thr += self.threat(e, i);
                    let _ = j;
                }
            }
            let net = val + pull - 0.4 * thr;
            if u.side as usize == me {
                mine += net;
            } else {
                theirs += net;
            }
        }
        v += mine - theirs;
        for o in &self.objectives {
            match self.control(*o) {
                Some(c) if c == p => v += 25.0,
                Some(_) => v -= 25.0,
                None => {}
            }
        }
        if let Some((gain, loss)) = self.pending_expect() {
            // atacatorul este jucătorul la mutare
            if self.side == p {
                v += gain - loss;
            } else {
                v -= gain - loss;
            }
        }
        (v * 10.0) as i32
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Miniaturi — Trecătoarea Cenușie — runda {}/{} — puncte: J1 {}, J2 {}\n",
            self.round, MAX_ROUNDS, self.vp[0], self.vp[1]
        ));
        s.push_str("    ");
        for c in 0..W {
            s.push_str(&format!(" {}  ", (b'a' + c as u8) as char));
        }
        s.push('\n');
        for r in 0..H {
            s.push_str(&format!("{:>2}  ", r + 1));
            if r % 2 == 1 {
                s.push_str("  ");
            }
            for c in 0..row_width(r) {
                let t = self.terrain[r][c];
                let cell = match self.occupant((c as u8, r as u8)) {
                    Some(i) => {
                        let u = &self.units[i];
                        format!(
                            "{}{}{}",
                            if t == b'.' { ' ' } else { t as char },
                            tag(i),
                            u.figs
                        )
                    }
                    None => match t {
                        b'.' => " . ".to_string(),
                        b'#' => "###".to_string(),
                        x => format!(" {} ", x as char),
                    },
                };
                s.push_str(&cell);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("Legendă: # stâncă, f pădure (acoperire), O obiectiv, TagFigurine (ex. A5).\n");
        for (i, u) in self.units.iter().enumerate() {
            if !u.alive() {
                continue;
            }
            s.push_str(&format!(
                "  {} J{} {} {}/{} @{}{}{}\n",
                tag(i),
                u.side + 1,
                u.ty().name,
                u.figs,
                u.ty().figs,
                hex_name(u.pos),
                if u.shaken { " [șovăie]" } else { "" },
                if u.acted { " *" } else { "" }
            ));
        }
        if self.over.is_some() {
            s.push_str("Joc încheiat.\n");
        } else if self.pending.is_some() {
            s.push_str("Atac declarat: «arunca» pentru zaruri.\n");
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunca".into(),
            Move::Act { u, to, target } => {
                let mut s = format!("{}", tag(*u as usize));
                match to {
                    Some(p) => s.push_str(&format!(">{}", hex_name(*p))),
                    None => {
                        if target.is_none() {
                            s.push_str(" pas");
                        }
                    }
                }
                if let Some(t) = target {
                    s.push_str(&format!(" x{}", tag(*t as usize)));
                }
                s
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn playout_cap(&self) -> usize {
        400
    }
}

impl Miniaturi {
    /// Ar fi unitatea `ui` lipită de un inamic dacă ar sta pe `pos`?
    fn adjacent_enemy_from(&self, ui: usize, pos: (u8, u8)) -> bool {
        let side = self.units[ui].side;
        self.units
            .iter()
            .any(|e| e.alive() && e.side != side && hex_dist(e.pos, pos) == 1)
    }
}

/// Folosit de teste: parsează un hex.
pub fn hex_from_str(s: &str) -> Option<(u8, u8)> {
    parse_hex(s).filter(|p| valid(p.0 as i32, p.1 as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear(g: &mut Miniaturi) {
        for u in &mut g.units {
            u.figs = 0;
        }
    }

    fn put(g: &mut Miniaturi, i: usize, pos: (u8, u8)) {
        g.units[i].pos = pos;
        g.units[i].figs = g.units[i].ty().figs;
        g.units[i].acted = false;
        g.units[i].shaken = false;
    }

    // indici: J1 0..7 (L,L,A,C,G,T,K), J2 7..14
    const L1: usize = 0;
    const A1: usize = 2;
    const C1: usize = 3;
    const T1: usize = 5;
    const K1: usize = 6;
    const L2: usize = 7;
    const A2: usize = 9;
    const C2: usize = 10;

    #[test]
    fn map_is_symmetric_and_has_three_objectives() {
        let g = Miniaturi::with_seed(1);
        assert_eq!(g.objectives.len(), 3);
        for r in 0..H {
            for c in 0..row_width(r) {
                let (mc, mr) = mirror(c, r);
                assert!(valid(mc as i32, mr as i32));
                assert_eq!(g.terrain[r][c], g.terrain[mr][mc], "{c},{r}");
            }
        }
        for i in 0..7 {
            let a = &g.units[i];
            let b = &g.units[i + 7];
            let (mc, mr) = mirror(a.pos.0 as usize, a.pos.1 as usize);
            assert_eq!(b.pos, (mc as u8, mr as u8));
            assert_ne!(g.terrain_at(a.pos), b'#');
        }
    }

    #[test]
    fn hex_geometry() {
        assert_eq!(hex_dist((0, 0), (0, 0)), 0);
        assert_eq!(neighbors((5, 4)).len(), 6);
        for n in neighbors((5, 4)) {
            assert_eq!(hex_dist((5, 4), n), 1);
        }
        assert!(neighbors((0, 0)).len() >= 2);
        // distanța e simetrică și respectă inegalitatea triunghiului
        for a in [(0u8, 0u8), (3, 2), (10, 4), (4, 7)] {
            for b in [(1u8, 1u8), (9, 8), (5, 4)] {
                assert_eq!(hex_dist(a, b), hex_dist(b, a));
                for c in [(2u8, 6u8), (8, 0)] {
                    assert!(hex_dist(a, b) <= hex_dist(a, c) + hex_dist(c, b));
                }
            }
        }
        let l = line((0, 0), (6, 0));
        assert_eq!(l.len(), 7);
        assert!(l.iter().all(|p| p.1 == 0));
        assert_eq!(hex_name((3, 3)), "d4");
        assert_eq!(hex_from_str("d4"), Some((3, 3)));
        assert_eq!(hex_from_str("z9"), None);
    }

    #[test]
    fn rocks_block_line_of_sight_and_forest_does_not() {
        let mut g = Miniaturi::with_seed(2);
        g.terrain = [[b'.'; W]; H];
        g.terrain[3][5] = b'#';
        assert!(!g.los((3, 3), (7, 3)));
        assert!(g.los((3, 2), (3, 4)));
        g.terrain[3][5] = b'f';
        assert!(g.los((3, 3), (7, 3)));
    }

    #[test]
    fn movement_costs_forest_double_and_rock_blocks() {
        let mut g = Miniaturi::with_seed(3);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, L1, (2, 4)); // mv 3
        g.terrain[4][3] = b'f';
        let r = g.reach(L1);
        let cost_of = |p: (u8, u8)| r.iter().find(|(q, _)| *q == p).map(|(_, c)| *c);
        assert_eq!(cost_of((3, 4)), Some(2));
        assert_eq!(cost_of((1, 4)), Some(1));
        assert!(r.iter().all(|(p, c)| *c <= 3 && *p != (2, 4)));
        g.terrain[4][1] = b'#';
        assert_eq!(g.reach(L1).iter().find(|(q, _)| *q == (1, 4)), None);
    }

    #[test]
    fn contact_stops_movement_and_units_block() {
        let mut g = Miniaturi::with_seed(4);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, C1, (1, 4)); // mv 6
        put(&mut g, L2, (5, 4));
        let r = g.reach(C1);
        // hexul (4,4) e lângă L2: se poate intra, dar nu se trece mai departe
        assert!(r.iter().any(|(p, _)| *p == (4, 4)));
        assert!(!r.iter().any(|(p, _)| *p == (6, 4)));
        assert!(!r.iter().any(|(p, _)| *p == (5, 4)));
    }

    #[test]
    fn shooting_needs_range_los_and_no_contact() {
        let mut g = Miniaturi::with_seed(5);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, A1, (1, 4));
        put(&mut g, L2, (7, 4)); // distanță 6 ≤ 7
        let shoots = |g: &Miniaturi| {
            g.legal_moves().iter().any(|m| {
                matches!(m, Move::Act { u, to: None, target: Some(t) } if *u as usize == A1 && *t as usize == L2)
            })
        };
        assert!(shoots(&g));
        g.terrain[4][4] = b'#';
        assert!(!shoots(&g), "stânca blochează");
        g.terrain[4][4] = b'.';
        g.units[L2].pos = (10, 4); // prea departe
        assert!(!shoots(&g));
        // lipit de inamic: nu trage la distanță (dar poate lovi în corp la corp)
        g.units[L2].pos = (2, 4);
        put(&mut g, C2, (6, 4));
        assert!(!g.legal_moves().iter().any(|m| matches!(
            m,
            Move::Act { u, to: None, target: Some(t) } if *u as usize == A1 && *t as usize == C2
        )));
    }

    #[test]
    fn catapult_cannot_move_and_shoot() {
        let mut g = Miniaturi::with_seed(6);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, T1, (1, 4));
        put(&mut g, L2, (6, 4));
        let mv = g.legal_moves();
        assert!(mv
            .iter()
            .any(|m| matches!(m, Move::Act { u, to: None, target: Some(_) } if *u as usize == T1)));
        assert!(!mv.iter().any(
            |m| matches!(m, Move::Act { u, to: Some(_), target: Some(_) } if *u as usize == T1)
        ));
    }

    #[test]
    fn declaring_an_attack_forces_a_separate_roll() {
        let mut g = Miniaturi::with_seed(7);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, A1, (1, 4));
        put(&mut g, L2, (5, 4));
        let m = Move::Act {
            u: A1 as u8,
            to: None,
            target: Some(L2 as u8),
        };
        g.apply(&m);
        assert!(g.pending.is_some());
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        assert_eq!(g.units[L2].figs, 5, "încă nimic aruncat");
        assert_eq!(g.side, Player::One);
        // J2 mai are unități? doar L2 + (toate ucise) -> să punem alta
        g.apply(&Move::Roll);
        assert!(g.pending.is_none());
        assert!(g.units[L2].figs <= 5);
    }

    #[test]
    fn dice_statistics_match_expectation() {
        // 4 arcași la distanță 3 (<= 7/2): lovesc la 4+, apărătorul salvare 5+.
        let mut total = 0.0;
        let n = 4000;
        for seed in 0..n {
            let mut g = Miniaturi::with_seed(seed + 1);
            clear(&mut g);
            g.terrain = [[b'.'; W]; H];
            put(&mut g, A1, (1, 4));
            put(&mut g, L2, (4, 4));
            put(&mut g, K1, (0, 0)); // martor ca să nu se termine
            g.units[L2].figs = 5;
            g.apply(&Move::Act {
                u: A1 as u8,
                to: None,
                target: Some(L2 as u8),
            });
            let exp = g.pending_expect().unwrap().0;
            g.apply(&Move::Roll);
            total += (5 - g.units[L2].figs) as f32 * (50.0 / 5.0);
            if seed == 0 {
                assert!((exp - 4.0 * 0.5 * (4.0 / 6.0) * 10.0).abs() < 1e-3, "{exp}");
            }
        }
        let mean = total / n as f32;
        let expected = 4.0 * 0.5 * (4.0 / 6.0) * 10.0;
        assert!((mean - expected).abs() < 0.8, "{mean} vs {expected}");
    }

    #[test]
    fn dice_clone_differs_from_original() {
        let g = Miniaturi::with_seed(9);
        let mut a = g.clone();
        let mut b = g.clone();
        let sa: Vec<u8> = (0..16).map(|_| a.dice.rng.d6()).collect();
        let sb: Vec<u8> = (0..16).map(|_| b.dice.rng.d6()).collect();
        assert_ne!(sa, sb);
    }

    #[test]
    fn cavalry_charge_and_lance_wall_modifiers() {
        let mut g = Miniaturi::with_seed(10);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, C1, (2, 4));
        put(&mut g, L2, (3, 4));
        put(&mut g, A2, (3, 3));
        let vs_l = g.profile(C1, L2, true, true, 1);
        assert_eq!(vs_l.hit, 4); // 4 -1 (încărcare) +1 (lănci)
        let vs_a = g.profile(C1, A2, true, true, 1);
        assert_eq!(vs_a.hit, 3);
        let back = g.profile(L2, C1, true, false, 1);
        assert_eq!(back.hit, 3); // lăncierii contra călăreților
        assert_eq!(vs_l.n, 6);
    }

    #[test]
    fn forest_gives_cover() {
        let mut g = Miniaturi::with_seed(11);
        clear(&mut g);
        g.terrain = [[b'.'; W]; H];
        put(&mut g, A1, (1, 4));
        put(&mut g, L2, (3, 4));
        assert_eq!(g.profile(A1, L2, false, false, 2).save, 5);
        g.terrain[4][3] = b'f';
        assert_eq!(g.profile(A1, L2, false, false, 2).save, 4);
    }

    #[test]
    fn morale_shakes_then_routs() {
        let mut g = Miniaturi::with_seed(12);
        clear(&mut g);
        put(&mut g, A2, (5, 4));
        put(&mut g, K1, (0, 0));
        g.units[A2].figs = 1; // jumătate sau mai puțin, moral 3
        let mut shaken = 0;
        let mut routed = 0;
        for _ in 0..200 {
            let mut h = g.clone();
            h.morale(A2, 3);
            if !h.units[A2].alive() {
                routed += 1;
            }
            if h.units[A2].shaken {
                shaken += 1;
            }
            h.units[A2].shaken = true;
            h.units[A2].figs = 1;
            let mut k = h.clone();
            k.morale(A2, 3);
            if !k.units[A2].alive() {
                routed += 1;
            }
        }
        assert!(shaken > 150, "{shaken}");
        assert!(routed > 100, "{routed}");
        // niciodată distrusă din prima cădere
        let mut h = g.clone();
        for _ in 0..100 {
            h.units[A2].shaken = false;
            h.units[A2].figs = 1;
            h.morale(A2, 3);
            assert!(h.units[A2].alive());
        }
    }

    #[test]
    fn commander_aura_helps_morale() {
        let mut g = Miniaturi::with_seed(13);
        clear(&mut g);
        put(&mut g, L2, (5, 4));
        put(&mut g, K1, (0, 0));
        let fails = |g: &Miniaturi| {
            let mut n = 0;
            let mut h = g.clone();
            for _ in 0..600 {
                h.units[L2].shaken = false;
                h.morale(L2, 2);
                if h.units[L2].shaken {
                    n += 1;
                }
            }
            n
        };
        let base = fails(&g);
        put(&mut g, 13, (5, 5)); // Comandantul J2 alături
        let with = fails(&g);
        assert!(with < base, "{with} !< {base}");
    }

    #[test]
    fn objectives_score_at_round_end() {
        let mut g = Miniaturi::with_seed(14);
        clear(&mut g);
        let ob = g.objectives[1];
        put(&mut g, L1, ob);
        put(&mut g, L2, (10, 0));
        g.units[L2].acted = true;
        g.units[L1].acted = true;
        g.end_round();
        assert_eq!(g.vp, [1, 0]);
        assert_eq!(g.round, 2);
        assert_eq!(g.side, Player::Two);
        // șovăitoarea nu contează
        g.units[L1].shaken = true;
        g.end_round();
        assert_eq!(g.vp, [1, 0]);
    }

    #[test]
    fn elimination_wins_immediately() {
        let mut g = Miniaturi::with_seed(15);
        clear(&mut g);
        put(&mut g, C1, (1, 4));
        put(&mut g, A2, (2, 4));
        g.units[A2].figs = 1;
        g.apply(&Move::Act {
            u: C1 as u8,
            to: None,
            target: Some(A2 as u8),
        });
        let mut won = false;
        for s in 0..40 {
            let mut h = g.clone();
            h.dice = Dice {
                rng: Rng::new(s),
                ctr: Cell::new(0),
            };
            h.apply(&Move::Roll);
            if h.outcome() == Some(Outcome::Win(Player::One)) {
                won = true;
            }
        }
        assert!(won);
    }

    #[test]
    fn units_never_revive_and_games_finish() {
        for seed in 1..=8u64 {
            let mut g = Miniaturi::with_seed(seed);
            let mut rng = Rng::new(seed * 7 + 3);
            let mut n = 0;
            let mut prev: Vec<i8> = g.units.iter().map(|u| u.figs).collect();
            while g.outcome().is_none() && n < 2000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                for (i, u) in g.units.iter().enumerate() {
                    assert!(u.figs <= prev[i]);
                    assert!(u.figs >= 0);
                    prev[i] = u.figs;
                }
                // fără suprapuneri
                for (i, a) in g.units.iter().enumerate() {
                    for b in g.units.iter().skip(i + 1) {
                        assert!(!(a.alive() && b.alive() && a.pos == b.pos));
                    }
                }
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }

    #[test]
    fn render_shows_map_and_roster() {
        let g = Miniaturi::with_seed(1);
        let r = g.render();
        assert!(r.contains("runda 1/6"));
        assert!(r.contains("Lăncieri"));
        assert!(r.contains('#') && r.contains('O'));
    }

    #[test]
    fn move_texts_are_unique_and_parse_back() {
        let g = Miniaturi::with_seed(3);
        let mv = g.legal_moves();
        let mut t: Vec<String> = mv.iter().map(|m| g.format_move(m).to_lowercase()).collect();
        t.sort();
        let n = t.len();
        t.dedup();
        assert_eq!(n, t.len());
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        let games = 8;
        for seed in 0..games {
            let mut g = Miniaturi::with_seed(500 + seed);
            let mut rng = Rng::new(seed + 9);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let m = if g.to_move() == Player::One {
                    if g.pending.is_some() {
                        Move::Roll
                    } else {
                        polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                    }
                } else {
                    let mv = g.legal_moves();
                    mv[rng.below(mv.len())]
                };
                g.apply(&m);
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 7, "AI a câștigat doar {wins}/{games}");
    }

    #[test]
    fn ai_move_is_fast() {
        let mut g = Miniaturi::with_seed(77);
        let mut rng = Rng::new(1);
        let mut worst = 0.0f64;
        let mut n = 0;
        while g.outcome().is_none() && n < 60 {
            let t = std::time::Instant::now();
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            worst = worst.max(t.elapsed().as_secs_f64());
            g.apply(&m);
            n += 1;
        }
        assert!(worst < 1.0, "{worst}");
    }

    #[test]
    fn k1_is_commander() {
        let g = Miniaturi::with_seed(1);
        assert_eq!(g.units[K1].ut, COMAND);
    }
}
