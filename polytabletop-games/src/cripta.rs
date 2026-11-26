//! Cripta Clopotelor — aventură cooperativă tactică cu cărți de acțiune (variantă originală, partener AI).
//!
//! Doi eroi coboară în cripta unui far părăsit: Paznicul de Far (J1, 10 viață) și Vânătoarea de
//! Umbre (J2, AI, 8 viață). Mecanica „fiecare tură alegi 2 cărți din mână, una dă inițiativa, apoi joci
//! partea de sus a uneia și partea de jos a celeilalte; monștrii își trag cărți de comportament; odihna
//! recuperează cărțile jucate, cu prețul uneia pierdute; pachet de modificatori la atac” este
//! generică; eroii, cărțile, monștrii, hărțile și scenariile sunt ale noastre.
//!
//! Trei scenarii originale (alese la început; în joc la întâmplare):
//! 1. „Subsolul Clopotarului” — omorâți toți monștrii.
//! 2. „Coridorul Oglinzilor” — ajungeți amândoi la ieșire (E; pe ea încap amândoi).
//! 3. „Tronul Roșu” — omorâți Regele Clopot și garda lui.
//!
//! * Runda: fiecare erou joacă o mutare `joc 3 5` (cartea 3 conduce: inițiativa ei; a doua e 5) sau
//!   `odihna` (inițiativă 99: +2 viață, cărțile jucate revin în mână, dar una la întâmplare se pierde;
//!   cere cel puțin 2 cărți jucate). Apoi `revela` (forțat): monștrii trag cărți de comportament și
//!   ordinea se stabilește după inițiativă (la egalitate, eroii înaintea monștrilor).
//! * Tura eroului: `sus 3 jos 5` (partea de sus a cărții 3 și cea de jos a cărții 5) sau `sus 5 jos 3`;
//!   apoi fiecare acțiune: `mers c4` (până la câte pași spune cartea), `atac m2` / `atac toti`,
//!   `aplic` (vindecare/scut) sau `sari`. Atacul își trage modificatorul într-o mutare forțată (`trage`).
//! * Monștrii (`monstri`, forțat) se mișcă spre cel mai apropiat erou și lovesc; scutul reduce fiecare
//!   lovitură primită în runda aceea.
//! * Un erou rămâne fără viață sau fără cărți (mai puțin de 2 în mână și 2 jucate) = epuizat.
//!
//! Rezultat: obiectivul scenariului îndeplinit = victorie (J1 „câștigă”); ambii eroi epuizați sau 25 de runde
//! = înfrângere = REMIZĂ (documentat: nu există adversar). Încheierea e o mutare forțată (`sfârșit`).
//!
//! Simplificări: hartă pe pătrate (fără hexagoane), linia de vedere aproximată (fără ocol de ziduri),
//! un singur pachet de modificatori pentru echipă, fără monede/obiecte/niveluri.
#![allow(clippy::needless_range_loop)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::Arc;

pub const ROUND_CAP: u32 = 25;
const HAND0: u8 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    Atk { dmg: i8, range: u8, all: bool },
    Mv(u8),
    Heal(u8),
    Shield(u8),
}

pub struct HCard {
    pub name: &'static str,
    pub init: u8,
    pub top: Act,
    pub bottom: Act,
}

const fn a(dmg: i8, range: u8) -> Act {
    Act::Atk {
        dmg,
        range,
        all: false,
    }
}

const fn hc(name: &'static str, init: u8, top: Act, bottom: Act) -> HCard {
    HCard {
        name,
        init,
        top,
        bottom,
    }
}

pub const CARDS: [[HCard; 8]; 2] = [
    [
        hc(
            "Rotire de Lanternă",
            62,
            Act::Atk {
                dmg: 2,
                range: 1,
                all: true,
            },
            Act::Mv(2),
        ),
        hc("Lovitură de Ancoră", 24, a(4, 1), Act::Mv(2)),
        hc("Zid de Scut", 71, Act::Shield(2), a(2, 1)),
        hc("Pas Greoi", 40, Act::Mv(3), Act::Heal(2)),
        hc("Ecoul Clopotului", 18, a(3, 2), Act::Shield(1)),
        hc("Cârpeli", 80, Act::Heal(3), Act::Mv(2)),
        hc("Atac în Salt", 35, a(3, 1), Act::Mv(4)),
        hc("Zdrobire", 55, a(5, 1), Act::Shield(1)),
    ],
    [
        hc("Salvă de Ace", 20, a(2, 3), Act::Mv(3)),
        hc("Țintă Perfectă", 45, a(4, 4), Act::Heal(1)),
        hc("Dans printre Lespezi", 12, Act::Mv(4), a(2, 2)),
        hc("Capcană de Sârmă", 60, Act::Shield(2), a(3, 3)),
        hc("Foc Rapid", 30, a(2, 2), a(2, 2)),
        hc("Pansament de Mușchi", 75, Act::Heal(3), Act::Mv(2)),
        hc("Săgeată Lungă", 50, a(3, 5), Act::Mv(2)),
        hc("Pas de Ceață", 25, Act::Mv(3), Act::Shield(1)),
    ],
];

pub const HERO_NAMES: [&str; 2] = ["Paznicul de Far", "Vânătoarea de Umbre"];
const HERO_HP: [i32; 2] = [12, 10];

#[derive(Clone, Copy)]
pub struct MKind {
    pub name: &'static str,
    pub hp: i32,
    pub elite_hp: i32,
    pub mv: i32,
    pub atk: i32,
    pub range: i32,
}

pub const KINDS: [MKind; 4] = [
    MKind {
        name: "Schelet",
        hp: 4,
        elite_hp: 7,
        mv: 2,
        atk: 1,
        range: 1,
    },
    MKind {
        name: "Arbaletist",
        hp: 3,
        elite_hp: 5,
        mv: 1,
        atk: 2,
        range: 3,
    },
    MKind {
        name: "Gărgui",
        hp: 6,
        elite_hp: 9,
        mv: 1,
        atk: 2,
        range: 1,
    },
    MKind {
        name: "Regele Clopot",
        hp: 14,
        elite_hp: 14,
        mv: 1,
        atk: 3,
        range: 1,
    },
];

/// (inițiativă, bonus mișcare, bonus atac)
const MCARDS: [(u8, i32, i32); 6] = [
    (30, 0, 0),
    (45, 1, 0),
    (60, 0, 1),
    (15, -1, 1),
    (75, 1, -1),
    (50, 0, 0),
];

pub const SCENARIO_NAMES: [&str; 3] = [
    "Subsolul Clopotarului",
    "Coridorul Oglinzilor",
    "Tronul Roșu",
];

const MAPS: [&[&str]; 3] = [
    &[
        "#########",
        "#.......#",
        "#.##.##.#",
        "#.......#",
        "#.##.##.#",
        "#..H.H..#",
        "#########",
    ],
    &[
        "###########",
        "#H...#...E#",
        "#.##...##.#",
        "#H...#....#",
        "###########",
    ],
    &[
        "#########",
        "#.......#",
        "#.......#",
        "#..#.#..#",
        "#.......#",
        "#..#.#..#",
        "#.......#",
        "#..H.H..#",
        "#########",
    ],
];

/// (tip, elită, x, y)
const MONS: [&[(u8, bool, usize, usize)]; 3] = [
    &[
        (0, false, 1, 1),
        (0, false, 7, 1),
        (0, true, 4, 3),
        (1, false, 1, 3),
        (2, false, 7, 3),
    ],
    &[
        (0, false, 6, 1),
        (1, false, 8, 1),
        (2, false, 5, 2),
        (0, true, 7, 3),
    ],
    &[
        (3, false, 4, 1),
        (0, true, 2, 2),
        (0, false, 6, 2),
        (1, false, 1, 1),
    ],
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mod {
    Add(i8),
    Double,
    Miss,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Actor {
    Hero(u8),
    Mons(u8),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pend {
    hero: u8,
    dmg: i32,
    targets: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Phase {
    Select,
    Reveal,
    Combo,
    Step,
    RestTurn,
    Roll(Pend),
    MonTurn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8, u8),
    Rest,
    Reveal,
    Combo(u8, u8),
    Walk(u8),
    Attack(u8),
    AttackAll,
    Apply,
    Skip,
    Roll,
    Monsters,
    RestNow,
    Finish,
}

#[derive(Clone, Debug)]
pub struct Hero {
    pub pos: usize,
    pub hp: i32,
    pub hand: Vec<u8>,
    pub discard: Vec<u8>,
    pub lost: Vec<u8>,
    chosen: Option<(u8, u8)>,
    resting: bool,
    shield: i32,
    pub exhausted: bool,
}

#[derive(Clone, Debug)]
pub struct Mon {
    pub kind: u8,
    pub elite: bool,
    pub pos: usize,
    pub hp: i32,
}

#[derive(Clone)]
pub struct Cripta {
    scen: usize,
    grid: Vec<Vec<u8>>,
    w: usize,
    h: usize,
    dist: Arc<Vec<u8>>,
    heroes: [Hero; 2],
    mons: Vec<Mon>,
    mdecks: [Vec<u8>; 4],
    mcur: [Option<u8>; 4],
    mods: Vec<Mod>,
    mod_disc: Vec<Mod>,
    phase: Phase,
    side: Player,
    order: Vec<Actor>,
    oi: usize,
    plan: Vec<(u8, bool)>,
    step: usize,
    round: u32,
    pending: Option<bool>,
    over: bool,
    won: bool,
    rng: Rng,
    last: String,
}

impl Default for Cripta {
    fn default() -> Self {
        Self::new()
    }
}

fn cell_name(w: usize, z: usize) -> String {
    format!("{}{}", (b'a' + (z % w) as u8) as char, z / w + 1)
}

impl Cripta {
    pub fn new() -> Self {
        let mut r = Rng::from_clock();
        let sc = r.below(3);
        Self::with_scenario(sc, r.next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::with_scenario((seed % 3) as usize, seed)
    }

    pub fn with_scenario(scen: usize, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let rows = MAPS[scen];
        let h = rows.len();
        let w = rows[0].len();
        let grid: Vec<Vec<u8>> = rows.iter().map(|r| r.bytes().collect()).collect();
        let n = w * h;
        let mut dist = vec![99u8; n * n];
        for s in 0..n {
            if grid[s / w][s % w] == b'#' {
                continue;
            }
            dist[s * n + s] = 0;
            let mut q = std::collections::VecDeque::from([s]);
            while let Some(x) = q.pop_front() {
                for (dx, dy) in [(0isize, -1isize), (0, 1), (1, 0), (-1, 0)] {
                    let nx = (x % w) as isize + dx;
                    let ny = (x / w) as isize + dy;
                    if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                        continue;
                    }
                    let y = ny as usize * w + nx as usize;
                    if grid[ny as usize][nx as usize] == b'#' || dist[s * n + y] != 99 {
                        continue;
                    }
                    dist[s * n + y] = dist[s * n + x] + 1;
                    q.push_back(y);
                }
            }
        }
        let mut starts = Vec::new();
        for y in 0..h {
            for x in 0..w {
                if grid[y][x] == b'H' {
                    starts.push(y * w + x);
                }
            }
        }
        let mk_hero = |i: usize, pos: usize| Hero {
            pos,
            hp: HERO_HP[i],
            hand: (0..HAND0).collect(),
            discard: Vec::new(),
            lost: Vec::new(),
            chosen: None,
            resting: false,
            shield: 0,
            exhausted: false,
        };
        let mons: Vec<Mon> = MONS[scen]
            .iter()
            .map(|&(k, e, x, y)| Mon {
                kind: k,
                elite: e,
                pos: y * w + x,
                hp: if e {
                    KINDS[k as usize].elite_hp
                } else {
                    KINDS[k as usize].hp
                },
            })
            .collect();
        let mut mods: Vec<Mod> = Vec::new();
        for (m, c) in [
            (Mod::Add(0), 6),
            (Mod::Add(1), 5),
            (Mod::Add(-1), 5),
            (Mod::Add(2), 1),
            (Mod::Add(-2), 1),
            (Mod::Double, 1),
            (Mod::Miss, 1),
        ] {
            for _ in 0..c {
                mods.push(m);
            }
        }
        shuffle(&mut mods, &mut rng);
        let mut mdecks: [Vec<u8>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for d in mdecks.iter_mut() {
            *d = (0..MCARDS.len() as u8).collect();
            shuffle(d, &mut rng);
        }
        Cripta {
            scen,
            grid,
            w,
            h,
            dist: Arc::new(dist),
            heroes: [mk_hero(0, starts[0]), mk_hero(1, starts[1])],
            mons,
            mdecks,
            mcur: [None; 4],
            mods,
            mod_disc: Vec::new(),
            phase: Phase::Select,
            side: Player::One,
            order: Vec::new(),
            oi: 0,
            plan: Vec::new(),
            step: 0,
            round: 1,
            pending: None,
            over: false,
            won: false,
            rng,
            last: String::new(),
        }
    }

    pub fn scenario(&self) -> usize {
        self.scen
    }

    pub fn is_won(&self) -> bool {
        self.over && self.won
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn monsters_alive(&self) -> usize {
        self.mons.len()
    }

    fn n(&self) -> usize {
        self.w * self.h
    }

    fn d(&self, a: usize, b: usize) -> u8 {
        self.dist[a * self.n() + b]
    }

    fn manh(&self, a: usize, b: usize) -> i32 {
        let (ax, ay) = ((a % self.w) as i32, (a / self.w) as i32);
        let (bx, by) = ((b % self.w) as i32, (b / self.w) as i32);
        (ax - bx).abs() + (ay - by).abs()
    }

    fn in_range(&self, a: usize, b: usize, range: i32) -> bool {
        let m = self.manh(a, b);
        m <= range && i32::from(self.d(a, b)) == m && m >= 1
    }

    fn exit_zone(&self) -> Option<usize> {
        (0..self.n()).find(|&z| self.grid[z / self.w][z % self.w] == b'E')
    }

    fn occupied(&self, z: usize) -> bool {
        self.mons.iter().any(|m| m.pos == z)
            || self.heroes.iter().any(|h| !h.exhausted && h.pos == z)
    }

    fn set_end(&mut self, won: bool) {
        if self.pending.is_none() {
            self.pending = Some(won);
        }
    }

    fn check_end(&mut self) {
        if self.heroes.iter().all(|h| h.exhausted) {
            self.set_end(false);
            return;
        }
        let ok = match self.scen {
            0 => self.mons.is_empty(),
            1 => {
                let e = self.exit_zone();
                e.is_some() && self.heroes.iter().all(|h| !h.exhausted && Some(h.pos) == e)
            }
            _ => !self.mons.iter().any(|m| m.kind == 3),
        };
        if ok {
            self.set_end(true);
        }
    }

    /// Celule atinse cu cel mult `steps` pași, prin celule libere (aliații pot fi traversați).
    fn reach(&self, from: usize, steps: i32, hero: bool) -> Vec<(usize, i32)> {
        let n = self.n();
        let mut seen = vec![-1i32; n];
        seen[from] = 0;
        let mut q = std::collections::VecDeque::from([from]);
        let mut out = Vec::new();
        while let Some(x) = q.pop_front() {
            let shared_exit =
                hero && Some(x) == self.exit_zone() && !self.mons.iter().any(|m| m.pos == x);
            if x != from && (!self.occupied(x) || shared_exit) {
                out.push((x, seen[x]));
            }
            if seen[x] >= steps {
                continue;
            }
            for (dx, dy) in [(0isize, -1isize), (0, 1), (1, 0), (-1, 0)] {
                let nx = (x % self.w) as isize + dx;
                let ny = (x / self.w) as isize + dy;
                if nx < 0 || ny < 0 || nx >= self.w as isize || ny >= self.h as isize {
                    continue;
                }
                let y = ny as usize * self.w + nx as usize;
                if self.grid[ny as usize][nx as usize] == b'#' || seen[y] >= 0 {
                    continue;
                }
                // monștrii blochează eroii; eroii blochează monștrii; aliații se traversează
                let blocked_by_mons = self.mons.iter().any(|m| m.pos == y);
                let blocked_by_hero = self.heroes.iter().any(|h| !h.exhausted && h.pos == y);
                if hero && blocked_by_mons {
                    continue;
                }
                if !hero && (blocked_by_mons || blocked_by_hero) {
                    continue;
                }
                seen[y] = seen[x] + 1;
                q.push_back(y);
            }
        }
        out
    }

    fn act_of(&self, hero: usize, card: u8, top: bool) -> Act {
        let c = &CARDS[hero][card as usize];
        if top {
            c.top
        } else {
            c.bottom
        }
    }

    fn current_hero(&self) -> usize {
        match self.order.get(self.oi) {
            Some(Actor::Hero(h)) => *h as usize,
            _ => self.side.idx(),
        }
    }

    fn atk_targets(&self, hero: usize, range: u8) -> Vec<u8> {
        let pos = self.heroes[hero].pos;
        (0..self.mons.len())
            .filter(|&i| self.in_range(pos, self.mons[i].pos, i32::from(range)))
            .map(|i| i as u8)
            .collect()
    }

    /// Prima etapă a rundei: eroii care mai pot juca.
    fn start_select(&mut self) {
        for i in 0..2 {
            let h = &mut self.heroes[i];
            h.chosen = None;
            h.resting = false;
            if !h.exhausted && (h.hand.len() < 2 && h.discard.len() < 2 || h.hp <= 0) {
                h.exhausted = true;
            }
        }
        self.check_end();
        self.phase = Phase::Select;
        self.side = if !self.heroes[0].exhausted {
            Player::One
        } else {
            Player::Two
        };
    }

    fn advance_select(&mut self) {
        if self.side == Player::One && !self.heroes[1].exhausted {
            self.side = Player::Two;
        } else {
            self.phase = Phase::Reveal;
        }
    }

    /// Trece la următorul actor din ordine, sau încheie runda.
    fn advance(&mut self) {
        self.check_end();
        if self.pending.is_some() {
            return;
        }
        loop {
            if self.oi >= self.order.len() {
                self.end_round();
                return;
            }
            match self.order[self.oi] {
                Actor::Hero(h) => {
                    let hh = &self.heroes[h as usize];
                    if hh.exhausted {
                        self.oi += 1;
                        continue;
                    }
                    self.side = if h == 0 { Player::One } else { Player::Two };
                    if hh.resting {
                        self.phase = Phase::RestTurn;
                    } else {
                        self.phase = Phase::Combo;
                    }
                    self.plan.clear();
                    self.step = 0;
                    return;
                }
                Actor::Mons(k) => {
                    if !self.mons.iter().any(|m| m.kind == k) {
                        self.oi += 1;
                        continue;
                    }
                    self.phase = Phase::MonTurn;
                    self.side = Player::One;
                    return;
                }
            }
        }
    }

    fn end_round(&mut self) {
        for h in self.heroes.iter_mut() {
            h.shield = 0;
        }
        self.mcur = [None; 4];
        self.round += 1;
        if self.round > ROUND_CAP {
            self.set_end(false);
            return;
        }
        self.start_select();
    }

    fn finish_hero_turn(&mut self) {
        let h = self.current_hero();
        let (a, b) = self.heroes[h].chosen.expect("cărți alese");
        let hh = &mut self.heroes[h];
        for c in [a, b] {
            if let Some(i) = hh.hand.iter().position(|x| *x == c) {
                hh.hand.remove(i);
                hh.discard.push(c);
            }
        }
        self.oi += 1;
        self.plan.clear();
        self.step = 0;
        self.advance();
    }

    fn next_step(&mut self) {
        self.check_end();
        if self.pending.is_some() {
            return;
        }
        self.step += 1;
        if self.step >= 2 {
            self.finish_hero_turn();
        } else {
            self.phase = Phase::Step;
        }
    }

    fn draw_mod(&mut self) -> Mod {
        if self.mods.is_empty() {
            let mut d = std::mem::take(&mut self.mod_disc);
            shuffle(&mut d, &mut self.rng);
            self.mods = d;
        }
        let m = self.mods.pop().unwrap_or(Mod::Add(0));
        self.mod_disc.push(m);
        m
    }

    fn kill_dead(&mut self) {
        self.mons.retain(|m| m.hp > 0);
    }

    fn value_act(&self, hero: usize, act: Act) -> i32 {
        match act {
            Act::Atk { dmg, all, .. } => i32::from(dmg) * if all { 9 } else { 6 },
            Act::Mv(n) => 3 * i32::from(n),
            Act::Heal(n) => {
                let h = &self.heroes[hero];
                let deficit = (HERO_HP[hero] - h.hp).max(0);
                10 * i32::from(n).min(deficit)
            }
            Act::Shield(n) => 5 * i32::from(n),
        }
    }

    fn team_value(&self) -> i32 {
        if let Some(w) = self.pending {
            return if w { 100_000 } else { -100_000 };
        }
        let mut s = 0;
        for m in &self.mons {
            s -= 9 * m.hp + 30;
            if m.kind == 3 {
                s -= 30 * m.hp;
            }
        }
        for (i, h) in self.heroes.iter().enumerate() {
            if h.exhausted {
                s -= 2000;
                continue;
            }
            s += 13 * h.hp;
            s += 4 * h.shield;
            s += 5 * h.hand.len() as i32;
            if self.scen == 1 {
                if let Some(e) = self.exit_zone() {
                    s -= 25 * i32::from(self.d(h.pos, e).min(40));
                }
            } else if !self.mons.is_empty() {
                let dmin = self
                    .mons
                    .iter()
                    .map(|m| i32::from(self.d(h.pos, m.pos)))
                    .min()
                    .unwrap_or(0);
                let pref = if i == 0 { 1 } else { 3 };
                s -= 4 * (dmin - pref).abs().min(20);
            }
            if let Some((a, b)) = h.chosen {
                let (ca, cb) = (&CARDS[i][a as usize], &CARDS[i][b as usize]);
                let p1 = self.value_act(i, ca.top) + self.value_act(i, cb.bottom);
                let p2 = self.value_act(i, cb.top) + self.value_act(i, ca.bottom);
                s += p1.max(p2);
                s += (100 - i32::from(ca.init)) / 8;
            }
            if h.resting {
                s += match h.hand.len() {
                    0..=2 => 120,
                    3 => 25,
                    _ => -60,
                };
            }
        }
        if let Phase::Roll(p) = &self.phase {
            for &t in &p.targets {
                if let Some(m) = self.mons.get(t as usize) {
                    let d = p.dmg.min(m.hp);
                    s += 9 * d;
                    if p.dmg >= m.hp {
                        s += 30;
                    }
                }
            }
        }
        s
    }
}

const BEAM: usize = 5;
const SAMPLES: usize = 6;

impl Cripta {
    /// Cea mai bună valoare de echipă atinsă prin cel mult `depth` mutări deterministe.
    fn look(&self, depth: u32) -> i32 {
        if depth == 0
            || self.over
            || self.pending.is_some()
            || !matches!(self.phase, Phase::Combo | Phase::Step)
        {
            return self.team_value();
        }
        let mut kids: Vec<(i32, Cripta)> = Game::legal_moves(self)
            .iter()
            .map(|m| {
                let mut c = self.clone();
                c.apply(m);
                (c.team_value(), c)
            })
            .collect();
        kids.sort_by_key(|k| std::cmp::Reverse(k.0));
        kids.truncate(BEAM);
        kids.iter()
            .map(|(_, c)| c.look(depth - 1))
            .max()
            .unwrap_or_else(|| self.team_value())
    }

    /// Mutare greedy (cea mai bună valoare la un pas) pentru simulări.
    fn greedy(&self) -> Option<Move> {
        let mv = Game::legal_moves(self);
        if mv.len() <= 1 {
            return mv.into_iter().next();
        }
        let mut best: Option<(i32, Move)> = None;
        for m in mv {
            let mut c = self.clone();
            c.apply(&m);
            let v = if matches!(c.phase, Phase::Combo | Phase::Step) {
                c.look(1)
            } else {
                c.team_value()
            };
            if best.is_none_or(|(b, _)| v > b) {
                best = Some((v, m));
            }
        }
        best.map(|b| b.1)
    }

    /// Media valorilor la finalul rundei, cu extrageri (monștri, modificatori) reeșantionate.
    fn sample_round(&self) -> i32 {
        let mut seed: u64 = 0xC4197A ^ (self.round as u64) << 8 ^ self.mons.len() as u64;
        for h in &self.heroes {
            seed = seed.rotate_left(7) ^ (h.pos as u64 * 31 + h.hp as u64 + 1);
            seed = seed.rotate_left(5) ^ h.hand.len() as u64 ^ (h.discard.len() as u64) << 4;
        }
        let mut rng = Rng::new(seed);
        let mut total = 0i64;
        for _ in 0..SAMPLES {
            let mut g = self.clone();
            g.rng = Rng::new(rng.next_u64());
            // ordine canonică înainte de amestec: rezultatul depinde doar de multimea cărților nevăzute
            for d in g.mdecks.iter_mut() {
                d.sort_unstable();
                shuffle(d, &mut rng);
            }
            g.mods.sort_by_key(|m| match m {
                Mod::Add(x) => i32::from(*x),
                Mod::Double => 100,
                Mod::Miss => -100,
            });
            shuffle(&mut g.mods, &mut rng);
            let r0 = g.round;
            let mut guard = 0;
            while !g.over && g.pending.is_none() && g.round == r0 && guard < 60 {
                guard += 1;
                match g.greedy() {
                    Some(m) => g.apply(&m),
                    None => break,
                }
            }
            total += i64::from(g.team_value());
        }
        (total / SAMPLES as i64) as i32
    }
}

impl Game for Cripta {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Finish];
        }
        match &self.phase {
            Phase::Select => {
                let h = &self.heroes[self.side.idx()];
                let mut v = Vec::new();
                for &x in &h.hand {
                    for &y in &h.hand {
                        if x != y {
                            v.push(Move::Play(x, y));
                        }
                    }
                }
                if h.discard.len() >= 2 {
                    v.push(Move::Rest);
                }
                v
            }
            Phase::Reveal => vec![Move::Reveal],
            Phase::RestTurn => vec![Move::RestNow],
            Phase::MonTurn => vec![Move::Monsters],
            Phase::Roll(_) => vec![Move::Roll],
            Phase::Combo => {
                let h = self.current_hero();
                let (a, b) = self.heroes[h].chosen.expect("cărți alese");
                vec![Move::Combo(a, b), Move::Combo(b, a)]
            }
            Phase::Step => {
                let h = self.current_hero();
                let (card, top) = self.plan[self.step];
                let act = self.act_of(h, card, top);
                let mut v = Vec::new();
                match act {
                    Act::Mv(n) => {
                        let mut cells: Vec<usize> = self
                            .reach(self.heroes[h].pos, i32::from(n), true)
                            .into_iter()
                            .map(|(z, _)| z)
                            .collect();
                        cells.sort_unstable();
                        for z in cells {
                            v.push(Move::Walk(z as u8));
                        }
                    }
                    Act::Atk { range, all, .. } => {
                        let t = self.atk_targets(h, range);
                        if all {
                            if !t.is_empty() {
                                v.push(Move::AttackAll);
                            }
                        } else {
                            for i in t {
                                v.push(Move::Attack(i));
                            }
                        }
                    }
                    Act::Heal(_) | Act::Shield(_) => v.push(Move::Apply),
                }
                v.push(Move::Skip);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Play(a, b) => {
                let p = self.side.idx();
                self.heroes[p].chosen = Some((a, b));
                self.last = format!("{} a ales două cărți.", HERO_NAMES[p]);
                self.advance_select();
            }
            Move::Rest => {
                let p = self.side.idx();
                self.heroes[p].resting = true;
                self.heroes[p].chosen = None;
                self.last = format!("{} se pregătește de odihnă.", HERO_NAMES[p]);
                self.advance_select();
            }
            Move::Reveal => {
                self.order.clear();
                self.oi = 0;
                for i in 0..2 {
                    let h = &self.heroes[i];
                    if h.exhausted {
                        continue;
                    }
                    let init = if h.resting {
                        99
                    } else if let Some((a, _)) = h.chosen {
                        CARDS[i][a as usize].init
                    } else {
                        99
                    };
                    self.order.push(Actor::Hero(i as u8));
                    let _ = init;
                }
                let mut keyed: Vec<(u8, u8, Actor)> = Vec::new();
                for &ac in &self.order.clone() {
                    if let Actor::Hero(i) = ac {
                        let h = &self.heroes[i as usize];
                        let init = if h.resting {
                            99
                        } else {
                            CARDS[i as usize][h.chosen.expect("alese").0 as usize].init
                        };
                        keyed.push((init, 0, ac));
                    }
                }
                let kinds: Vec<u8> = (0..4u8)
                    .filter(|k| self.mons.iter().any(|m| m.kind == *k))
                    .collect();
                for k in kinds {
                    let ku = k as usize;
                    if self.mdecks[ku].is_empty() {
                        self.mdecks[ku] = (0..MCARDS.len() as u8).collect();
                        let mut d = std::mem::take(&mut self.mdecks[ku]);
                        shuffle(&mut d, &mut self.rng);
                        self.mdecks[ku] = d;
                    }
                    let c = self.mdecks[ku].pop().unwrap();
                    self.mcur[ku] = Some(c);
                    keyed.push((MCARDS[c as usize].0, 1, Actor::Mons(k)));
                }
                keyed.sort_by_key(|(i, t, a)| {
                    (
                        *i,
                        *t,
                        match a {
                            Actor::Hero(h) => *h,
                            Actor::Mons(k) => *k,
                        },
                    )
                });
                self.order = keyed.into_iter().map(|(_, _, a)| a).collect();
                self.last = "Ordinea rundei stabilită.".into();
                self.advance();
            }
            Move::Combo(top_card, bottom_card) => {
                self.plan = vec![(top_card, true), (bottom_card, false)];
                self.step = 0;
                self.phase = Phase::Step;
            }
            Move::Walk(z) => {
                let h = self.current_hero();
                self.heroes[h].pos = z as usize;
                self.last = format!(
                    "{} merge în {}.",
                    HERO_NAMES[h],
                    cell_name(self.w, z as usize)
                );
                self.next_step();
            }
            Move::Attack(i) => {
                let h = self.current_hero();
                let (card, top) = self.plan[self.step];
                if let Act::Atk { dmg, .. } = self.act_of(h, card, top) {
                    self.phase = Phase::Roll(Pend {
                        hero: h as u8,
                        dmg: i32::from(dmg),
                        targets: vec![i],
                    });
                    self.last = format!("{} atacă pe m{}.", HERO_NAMES[h], i + 1);
                }
            }
            Move::AttackAll => {
                let h = self.current_hero();
                let (card, top) = self.plan[self.step];
                if let Act::Atk { dmg, range, .. } = self.act_of(h, card, top) {
                    let t = self.atk_targets(h, range);
                    self.phase = Phase::Roll(Pend {
                        hero: h as u8,
                        dmg: i32::from(dmg),
                        targets: t,
                    });
                    self.last = format!("{} lovește în jur.", HERO_NAMES[h]);
                }
            }
            Move::Apply => {
                let h = self.current_hero();
                let (card, top) = self.plan[self.step];
                match self.act_of(h, card, top) {
                    Act::Heal(n) => {
                        self.heroes[h].hp = (self.heroes[h].hp + i32::from(n)).min(HERO_HP[h]);
                        self.last = format!("{} se vindecă.", HERO_NAMES[h]);
                    }
                    Act::Shield(n) => {
                        self.heroes[h].shield += i32::from(n);
                        self.last = format!("{} ridică scutul ({}).", HERO_NAMES[h], n);
                    }
                    _ => {}
                }
                self.next_step();
            }
            Move::Skip => {
                self.next_step();
            }
            Move::Roll => {
                let Phase::Roll(p) = self.phase.clone() else {
                    return;
                };
                let md = self.draw_mod();
                let mut log = format!("{} — modificator {:?}:", HERO_NAMES[p.hero as usize], md);
                let dmg = match md {
                    Mod::Miss => 0,
                    Mod::Double => p.dmg * 2,
                    Mod::Add(k) => (p.dmg + i32::from(k)).max(0),
                };
                for &t in &p.targets {
                    if let Some(mo) = self.mons.get_mut(t as usize) {
                        mo.hp -= dmg;
                        log.push_str(&format!(" {} −{}", KINDS[mo.kind as usize].name, dmg));
                    }
                }
                self.last = log;
                self.kill_dead();
                self.phase = Phase::Step;
                self.next_step();
            }
            Move::Monsters => {
                let Some(Actor::Mons(k)) = self.order.get(self.oi).copied() else {
                    return;
                };
                let (_, mvb, atkb) = MCARDS[self.mcur[k as usize].unwrap_or(0) as usize];
                let kind = KINDS[k as usize];
                let mut log = format!("{}:", kind.name);
                let ids: Vec<usize> = (0..self.mons.len())
                    .filter(|&i| self.mons[i].kind == k)
                    .collect();
                for i in ids {
                    let alive: Vec<usize> = (0..2).filter(|&h| !self.heroes[h].exhausted).collect();
                    if alive.is_empty() {
                        break;
                    }
                    let pos = self.mons[i].pos;
                    let foc = *alive
                        .iter()
                        .min_by_key(|&&h| (self.d(pos, self.heroes[h].pos), h))
                        .unwrap();
                    let tpos = self.heroes[foc].pos;
                    let range = kind.range;
                    let mv = (kind.mv + mvb).max(0);
                    let atk = (kind.atk + i32::from(self.mons[i].elite) + atkb).max(0);
                    if !self.in_range(pos, tpos, range) {
                        let mut cells = self.reach(pos, mv, false);
                        cells.push((pos, 0));
                        let best = cells
                            .into_iter()
                            .min_by_key(|&(z, st)| {
                                (
                                    u8::from(!self.in_range(z, tpos, range)),
                                    self.d(z, tpos),
                                    st,
                                    z,
                                )
                            })
                            .map(|c| c.0)
                            .unwrap_or(pos);
                        self.mons[i].pos = best;
                    }
                    let pos = self.mons[i].pos;
                    if self.in_range(pos, tpos, range) {
                        let dmg = (atk - self.heroes[foc].shield).max(0);
                        self.heroes[foc].hp -= dmg;
                        log.push_str(&format!(" lovește pe {} (−{dmg})", HERO_NAMES[foc]));
                        if self.heroes[foc].hp <= 0 {
                            self.heroes[foc].exhausted = true;
                            self.heroes[foc].hp = 0;
                        }
                    }
                }
                self.last = log;
                self.oi += 1;
                self.advance();
            }
            Move::RestNow => {
                let h = self.current_hero();
                let hero = &mut self.heroes[h];
                if !hero.discard.is_empty() {
                    let i = self.rng.below(hero.discard.len());
                    let lost = hero.discard.remove(i);
                    hero.lost.push(lost);
                }
                let d = std::mem::take(&mut hero.discard);
                hero.hand.extend(d);
                hero.hand.sort_unstable();
                hero.hp = (hero.hp + 2).min(HERO_HP[h]);
                self.last = format!(
                    "{} se odihnește (+2 viață, o carte pierdută).",
                    HERO_NAMES[h]
                );
                self.oi += 1;
                self.plan.clear();
                self.advance();
            }
            Move::Finish => {
                self.over = true;
                self.won = self.pending == Some(true);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            None
        } else if self.won {
            Some(Outcome::Win(Player::One))
        } else {
            Some(Outcome::Draw)
        }
    }

    /// Cooperativ. În fazele `Combo` / `Step` acțiunile sunt deterministe și publice: căutare
    /// în lățime limitată pe restul planului eroului. În `Select` / `Reveal` (înainte să se
    /// tragă cărțile de comportament ale monștrilor) se simulează runda cu extrageri
    /// independente (generatorul și pachetele ascunse se re-amestecă; nu se citește viitorul
    /// real), alegând greedy cu `team_value`, și se mediază.
    fn evaluate(&self, _p: Player) -> i32 {
        if self.over || self.pending.is_some() {
            return self.team_value();
        }
        match self.phase {
            Phase::Combo | Phase::Step => self.look(3),
            Phase::Select | Phase::Reveal => self.sample_round(),
            _ => self.team_value(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Cripta Clopotelor — scenariul {}: {} | runda {}/{}\n",
            self.scen + 1,
            SCENARIO_NAMES[self.scen],
            self.round,
            ROUND_CAP
        );
        s.push_str("   ");
        for x in 0..self.w {
            s.push_str(&format!(" {} ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for y in 0..self.h {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..self.w {
                let z = y * self.w + x;
                let c = self.grid[y][x];
                let mut t = if c == b'#' {
                    "###".to_string()
                } else if c == b'E' {
                    " E ".to_string()
                } else {
                    " . ".to_string()
                };
                if let Some(i) =
                    (0..2).find(|&i| !self.heroes[i].exhausted && self.heroes[i].pos == z)
                {
                    t = format!("<{}>", i + 1);
                }
                if let Some(i) = self.mons.iter().position(|m| m.pos == z) {
                    let m = &self.mons[i];
                    let ch = KINDS[m.kind as usize].name.chars().next().unwrap_or('m');
                    t = format!("{}{}{}", if m.elite { '*' } else { ' ' }, ch, i + 1);
                }
                s.push_str(&t);
            }
            s.push('\n');
        }
        s.push_str("Monștri: ");
        let ml: Vec<String> = self
            .mons
            .iter()
            .enumerate()
            .map(|(i, m)| {
                format!(
                    "m{} {}{} viață {} la {}",
                    i + 1,
                    if m.elite { "elită " } else { "" },
                    KINDS[m.kind as usize].name,
                    m.hp,
                    cell_name(self.w, m.pos)
                )
            })
            .collect();
        s.push_str(&if ml.is_empty() {
            "—".into()
        } else {
            ml.join("; ")
        });
        s.push('\n');
        for i in 0..2 {
            let h = &self.heroes[i];
            s.push_str(&format!(
                "J{} {} — viață {}/{}{}{} la {}\n",
                i + 1,
                HERO_NAMES[i],
                h.hp,
                HERO_HP[i],
                if h.shield > 0 {
                    format!(" scut {}", h.shield)
                } else {
                    String::new()
                },
                if h.exhausted { " EPUIZAT" } else { "" },
                cell_name(self.w, h.pos)
            ));
            if !h.exhausted {
                let list: Vec<String> = h
                    .hand
                    .iter()
                    .map(|&c| {
                        let cd = &CARDS[i][c as usize];
                        format!(
                            "{}. {} (init {}; sus {}; jos {})",
                            c + 1,
                            cd.name,
                            cd.init,
                            act_text(cd.top),
                            act_text(cd.bottom)
                        )
                    })
                    .collect();
                s.push_str(&format!("   în mână: {}\n", list.join(" | ")));
                s.push_str(&format!(
                    "   jucate: {}  pierdute: {}\n",
                    h.discard.len(),
                    h.lost.len()
                ));
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        if self.over {
            if self.won {
                s.push_str("VICTORIE: obiectivul scenariului e îndeplinit.\n");
            } else {
                s.push_str("ÎNFRÂNGERE (remiză): cripta v-a înghițit.\n");
            }
        } else if self.pending.is_some() {
            s.push_str("Jocul se încheie: sfârșit.\n");
        } else {
            match &self.phase {
                Phase::Select => s.push_str(&format!(
                    "La rând J{}: alege cărțile — joc A B (A conduce) sau odihna.\n",
                    self.side.idx() + 1
                )),
                Phase::Reveal => s.push_str("Se dezvăluie ordinea: revela.\n"),
                Phase::Combo => s.push_str("Alege: sus A jos B.\n"),
                Phase::Step => {
                    let h = self.current_hero();
                    let (c, t) = self.plan[self.step];
                    s.push_str(&format!(
                        "Acțiunea {} ({}): {} — mers cell / atac mN / atac toti / aplic / sari.\n",
                        self.step + 1,
                        if t { "sus" } else { "jos" },
                        act_text(self.act_of(h, c, t))
                    ));
                }
                Phase::RestTurn => s.push_str("Odihnă: odihneste.\n"),
                Phase::Roll(_) => s.push_str("Se trage modificatorul: trage.\n"),
                Phase::MonTurn => s.push_str("Monștrii acționează: monstri.\n"),
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(a, b) => format!("joc {} {}", a + 1, b + 1),
            Move::Rest => "odihna".into(),
            Move::Reveal => "revela".into(),
            Move::Combo(a, b) => format!("sus {} jos {}", a + 1, b + 1),
            Move::Walk(z) => format!("mers {}", cell_name(self.w, *z as usize)),
            Move::Attack(i) => format!("atac m{}", i + 1),
            Move::AttackAll => "atac toti".into(),
            Move::Apply => "aplic".into(),
            Move::Skip => "sari".into(),
            Move::Roll => "trage".into(),
            Move::Monsters => "monstri".into(),
            Move::RestNow => "odihneste".into(),
            Move::Finish => "sfârșit".into(),
        }
    }
}

fn act_text(a: Act) -> String {
    match a {
        Act::Atk { dmg, range, all } => {
            if all {
                format!("atac {dmg} în jur")
            } else {
                format!("atac {dmg} raza {range}")
            }
        }
        Act::Mv(n) => format!("mers {n}"),
        Act::Heal(n) => format!("vindecare {n}"),
        Act::Shield(n) => format!("scut {n}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ai_game(scen: usize, seed: u64) -> Cripta {
        let mut g = Cripta::with_scenario(scen, seed);
        let mut rng = Rng::new(seed ^ 21);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            n += 1;
            assert!(n < 6000, "{}", g.render());
        }
        g
    }

    #[test]
    fn maps_are_valid() {
        for sc in 0..3 {
            let g = Cripta::with_scenario(sc, 1);
            let a = g.heroes[0].pos;
            let b = g.heroes[1].pos;
            assert!(g.d(a, b) < 99);
            for m in &g.mons {
                assert!(g.d(a, m.pos) < 99, "scen {sc}");
                assert_ne!(m.pos, a);
            }
            if sc == 1 {
                assert!(g.d(a, g.exit_zone().unwrap()) < 99);
            }
        }
    }

    #[test]
    fn select_offers_ordered_pairs_and_rest_needs_discard() {
        let mut g = Cripta::with_scenario(0, 2);
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 8 * 7);
        assert!(!mv.contains(&Move::Rest));
        g.heroes[0].discard = vec![0, 1];
        g.heroes[0].hand = (2..8).collect();
        assert!(g.legal_moves().contains(&Move::Rest));
    }

    #[test]
    fn full_round_flows_and_cards_move_to_discard() {
        let mut g = Cripta::with_scenario(0, 3);
        g.apply(&Move::Play(1, 6));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Play(0, 4));
        assert_eq!(g.legal_moves(), vec![Move::Reveal]);
        g.apply(&Move::Reveal);
        // orice ar urma, la sfârșitul rundei cărțile alese sunt în discard
        let mut guard = 0;
        while g.round == 1 && g.outcome().is_none() && g.pending.is_none() {
            let mv = g.legal_moves();
            let m = *mv.iter().find(|m| **m == Move::Skip).unwrap_or(&mv[0]);
            g.apply(&m);
            guard += 1;
            assert!(guard < 200);
        }
        if g.pending.is_none() {
            assert_eq!(g.round, 2);
            assert_eq!(g.heroes[0].discard.len(), 2);
            assert_eq!(g.heroes[0].hand.len(), 6);
            assert_eq!(g.heroes[1].hand.len() + g.heroes[1].discard.len(), 8);
        }
    }

    #[test]
    fn initiative_orders_actors() {
        let mut g = Cripta::with_scenario(0, 4);
        g.apply(&Move::Play(4, 0)); // init 18
        g.apply(&Move::Play(5, 0)); // init 75
        g.apply(&Move::Reveal);
        // eroul 1 (18) trebuie să fie înaintea eroului 2 (75)
        let p1 = g.order.iter().position(|a| *a == Actor::Hero(0)).unwrap();
        let p2 = g.order.iter().position(|a| *a == Actor::Hero(1)).unwrap();
        assert!(p1 < p2);
    }

    #[test]
    fn attack_uses_forced_modifier_draw() {
        let mut g = Cripta::with_scenario(0, 5);
        // aducem un monstru lângă erou
        let hp = g.heroes[0].pos;
        g.mons[0].pos = hp - 1;
        let hp0 = g.mons[0].hp;
        g.heroes[0].chosen = Some((1, 6));
        g.order = vec![Actor::Hero(0)];
        g.oi = 0;
        g.phase = Phase::Combo;
        g.apply(&Move::Combo(1, 6));
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Attack(0)));
        g.apply(&Move::Attack(0));
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        assert_eq!(g.mons[0].hp, hp0);
        g.apply(&Move::Roll);
        assert!(g
            .mons
            .iter()
            .find(|m| m.pos == hp - 1)
            .is_none_or(|m| m.hp <= hp0));
    }

    #[test]
    fn rest_restores_cards_but_loses_one() {
        let mut g = Cripta::with_scenario(0, 6);
        g.heroes[0].hand = vec![0, 1, 2, 3];
        g.heroes[0].discard = vec![4, 5, 6, 7];
        g.heroes[0].hp = 5;
        g.heroes[0].resting = true;
        g.order = vec![Actor::Hero(0)];
        g.oi = 0;
        g.phase = Phase::RestTurn;
        g.apply(&Move::RestNow);
        assert_eq!(g.heroes[0].hand.len(), 7);
        assert_eq!(g.heroes[0].lost.len(), 1);
        assert_eq!(g.heroes[0].hp, 7);
        assert_eq!(
            g.heroes[0].hand.len() + g.heroes[0].discard.len() + g.heroes[0].lost.len(),
            8
        );
    }

    #[test]
    fn shield_reduces_damage() {
        let mut g = Cripta::with_scenario(0, 7);
        let hp = g.heroes[0].pos;
        g.mons[0].pos = hp - 1;
        g.mons[0].kind = 0;
        g.mons[0].elite = false;
        g.heroes[0].shield = 5;
        g.order = vec![Actor::Mons(0)];
        g.oi = 0;
        g.mcur[0] = Some(0);
        g.phase = Phase::MonTurn;
        let before = g.heroes[0].hp;
        g.apply(&Move::Monsters);
        assert_eq!(g.heroes[0].hp, before);
    }

    #[test]
    fn scenario_objectives() {
        let mut g = Cripta::with_scenario(0, 8);
        g.mons.clear();
        g.check_end();
        assert_eq!(g.pending, Some(true));
        let mut g = Cripta::with_scenario(2, 8);
        g.mons.retain(|m| m.kind != 3);
        g.check_end();
        assert_eq!(g.pending, Some(true));
        let mut g = Cripta::with_scenario(1, 8);
        let e = g.exit_zone().unwrap();
        g.heroes[0].pos = e;
        g.check_end();
        assert!(g.pending.is_none());
        g.heroes[1].pos = e;
        g.check_end();
        assert_eq!(g.pending, Some(true));
    }

    #[test]
    fn both_heroes_fit_on_the_exit() {
        let mut g = Cripta::with_scenario(1, 10);
        g.mons.clear();
        let e = g.exit_zone().unwrap();
        g.heroes[0].pos = e;
        let cells = g.reach(g.heroes[1].pos, 30, true);
        assert!(cells.iter().any(|(z, _)| *z == e));
        // un monstru pe ieșire o blochează
        g.mons.push(Mon {
            kind: 0,
            elite: false,
            pos: e,
            hp: 4,
        });
        let cells = g.reach(g.heroes[1].pos, 30, true);
        assert!(!cells.iter().any(|(z, _)| *z == e));
    }

    #[test]
    fn both_exhausted_is_draw() {
        let mut g = Cripta::with_scenario(0, 9);
        g.heroes[0].exhausted = true;
        g.heroes[1].exhausted = true;
        g.check_end();
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn random_games_end_in_all_scenarios() {
        for sc in 0..3 {
            for seed in 1..=15u64 {
                let mut g = Cripta::with_scenario(sc, seed);
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty(), "{}", g.render());
                    let m = mv[rng.below(mv.len())];
                    g.apply(&m);
                    n += 1;
                    assert!(n < 20000);
                }
            }
        }
    }

    #[test]
    fn ai_team_wins_sometimes() {
        let mut wins = [0; 3];
        for sc in 0..3 {
            for seed in 1..=15u64 {
                if ai_game(sc, seed).is_won() {
                    wins[sc] += 1;
                }
            }
        }
        eprintln!("cripta: victorii pe scenarii {wins:?}/15");
        assert!(wins.iter().sum::<i32>() >= 6, "{wins:?}");
    }

    #[test]
    fn evaluate_ignores_the_hidden_deck_order() {
        let g = Cripta::with_seed(3);
        let mut h = g.clone();
        for d in h.mdecks.iter_mut() {
            d.reverse();
        }
        h.mods.reverse();
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }
}
