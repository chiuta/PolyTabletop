//! Noapte Cenușie — joc cooperativ de supraviețuire pe grilă (variantă originală, partener AI).
//!
//! O noapte lungă într-un cartier cu 42 de zone (7 coloane a–g, 6 rânduri 1–6). Doi supraviețuitori
//! — Ilinca (J1) și Radu (J2, AI) — trebuie să strângă 3 baterii de pe hartă (B) și să ajungă
//! amândoi la ieșire (E), în timp ce „umbrele” (cenușiii) apar din cele două zone de apariție (S).
//! Mecanica „supraviețuitori cu 3 acțiuni, arme cu zaruri, căutare de echipament, activarea
//! zombilor și apariții în creștere” este generică; harta, personajele, armele și textele sunt ale
//! noastre.
//!
//! * Tura ta: 3 acțiuni. `mers n|s|e|v` (1 acțiune; 2 dacă pleci dintr-o zonă cu umbre),
//!   `atac 1 c3` (arma din mâna 1 asupra zonei c3: arme de corp la corp doar în zona ta; cele
//!   de la distanță în linie dreaptă, fără ziduri, până la raza lor), `cauta` (o dată pe tură, într-o
//!   zonă fără umbre), `ia` (o baterie din zonă), `pansez` (vindeci o rană), `gata`.
//! * Zarurile și extragerea echipamentului sunt mutări forțate separate (`zaruri`, `extrage`),
//!   ca AI-ul să nu „vadă” rezultatul înainte de a alege.
//! * Umbre: rătăcit (viață 1, 1 pas), alergător (viață 1, activează de două ori), colos (viață 2,
//!   lovește cu 2; o armă cu forță < 2 nu-l poate omorî). După fiecare tură: `umbre` (cele din zona unui
//!   supraviețuitor lovesc, celelalte merg spre cel mai apropiat; alergătorii încă o dată), apoi
//!   `aparitie` (într-una din cele două zone S, pe rând; zarurile hotărăsc tipurile; mai multe și mai periculoase pe măsură ce trece noaptea).
//! * Rănile: 4 rane = mort. Armele: Ciomag, Topor, Praștie, Arbaletă, Pistol; un obiect găsit se pune
//!   în mâna liberă (altfel se pierde). Pansamentele se păstrează.
//!
//! Rezultat: 3 baterii și ambii supraviețuitori vii la ieșire = victorie (J1 „câștigă”). Moartea unuia,
//! 24 de umbre pe hartă sau 40 de ture = înfrângere = REMIZĂ (documentat: nu există adversar).
//! Încheierea e o mutare forțată (`sfârșit`).
//!
//! Simplificări: fără uși/clădiri, fără experiență și abilități, fără foc prietenesc, alegerea țintelor
//! de către armă e fixă (cele mai slabe umbre primele).
#![allow(clippy::needless_range_loop)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::Arc;

pub const W: usize = 7;
pub const H: usize = 6;
pub const NZ: usize = W * H;
pub const MAX_WOUNDS: u8 = 4;
pub const TURN_CAP: u32 = 40;
const SWARM: u32 = 24;
const MAP: [&str; H] = [
    "S..E..S", ".#...#.", "...B...", ".#...#.", "..B.B..", "...P...",
];

#[derive(Clone, Copy, Debug)]
pub struct Weapon {
    pub name: &'static str,
    pub ranged: bool,
    pub range: u8,
    pub dice: u8,
    pub acc: u8,
    pub dmg: u8,
}

pub const WEAPONS: [Weapon; 5] = [
    Weapon {
        name: "Ciomag",
        ranged: false,
        range: 0,
        dice: 1,
        acc: 4,
        dmg: 1,
    },
    Weapon {
        name: "Topor",
        ranged: false,
        range: 0,
        dice: 1,
        acc: 3,
        dmg: 2,
    },
    Weapon {
        name: "Praștie",
        ranged: true,
        range: 1,
        dice: 1,
        acc: 4,
        dmg: 1,
    },
    Weapon {
        name: "Arbaletă",
        ranged: true,
        range: 2,
        dice: 1,
        acc: 3,
        dmg: 2,
    },
    Weapon {
        name: "Pistol",
        ranged: true,
        range: 1,
        dice: 2,
        acc: 4,
        dmg: 1,
    },
];

const HERO_NAMES: [&str; 2] = ["Ilinca", "Radu"];
const KIND_NAMES: [&str; 3] = ["rătăciți", "alergători", "colosi"];
const KIND_VAL: [i32; 3] = [6, 10, 22];

/// Echipament: 0..5 = armă, 5 = pansament, 6 = gol.
const ITEM_BANDAGE: u8 = 5;
const ITEM_EMPTY: u8 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pending {
    Attack { weapon: u8, zone: u8 },
    Search,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Act,
    Roll(Pending),
    Zombies,
    Spawn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Walk(u8),
    Attack(u8, u8),
    Search,
    Take,
    Bandage,
    End,
    Roll,
    Zombies,
    Spawn,
    Finish,
}

#[derive(Clone, Debug)]
pub struct Hero {
    pub pos: usize,
    pub wounds: u8,
    pub hands: [Option<u8>; 2],
    pub bandages: u8,
}

#[derive(Clone)]
pub struct Cenusii {
    grid: Vec<Vec<u8>>,
    dist: Arc<Vec<u8>>,
    zomb: [[u8; 3]; NZ],
    batt: [bool; NZ],
    got: u8,
    heroes: [Hero; 2],
    deck: Vec<u8>,
    side: Player,
    actions: u8,
    searched: bool,
    phase: Phase,
    turns: u32,
    pending: Option<bool>,
    over: bool,
    won: bool,
    rng: Rng,
    last: String,
}

impl Default for Cenusii {
    fn default() -> Self {
        Self::new()
    }
}

fn zone_name(z: usize) -> String {
    format!("{}{}", (b'a' + (z % W) as u8) as char, z / W + 1)
}

#[cfg(test)]
fn parse_zone(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    if b.len() != 2 {
        return None;
    }
    let x = (b[0].to_ascii_lowercase() as isize) - 97;
    let y = (b[1] as isize) - 49;
    if (0..W as isize).contains(&x) && (0..H as isize).contains(&y) {
        Some(y as usize * W + x as usize)
    } else {
        None
    }
}

const DIRS: [(isize, isize, &str); 4] = [(0, -1, "n"), (0, 1, "s"), (1, 0, "e"), (-1, 0, "v")];

impl Cenusii {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let grid: Vec<Vec<u8>> = MAP.iter().map(|r| r.bytes().collect()).collect();
        let mut dist = vec![99u8; NZ * NZ];
        for s in 0..NZ {
            if grid[s / W][s % W] == b'#' {
                continue;
            }
            dist[s * NZ + s] = 0;
            let mut q = std::collections::VecDeque::from([s]);
            while let Some(x) = q.pop_front() {
                for (dx, dy, _) in DIRS {
                    let nx = (x % W) as isize + dx;
                    let ny = (x / W) as isize + dy;
                    if nx < 0 || ny < 0 || nx >= W as isize || ny >= H as isize {
                        continue;
                    }
                    let y = ny as usize * W + nx as usize;
                    if grid[ny as usize][nx as usize] == b'#' || dist[s * NZ + y] != 99 {
                        continue;
                    }
                    dist[s * NZ + y] = dist[s * NZ + x] + 1;
                    q.push_back(y);
                }
            }
        }
        let mut batt = [false; NZ];
        let mut start = 0;
        for y in 0..H {
            for x in 0..W {
                match grid[y][x] {
                    b'B' => batt[y * W + x] = true,
                    b'P' => start = y * W + x,
                    _ => {}
                }
            }
        }
        let mut deck: Vec<u8> = Vec::new();
        for (item, n) in [
            (0u8, 2),
            (1, 2),
            (2, 3),
            (3, 2),
            (4, 2),
            (ITEM_BANDAGE, 4),
            (ITEM_EMPTY, 5),
        ] {
            for _ in 0..n {
                deck.push(item);
            }
        }
        shuffle(&mut deck, &mut rng);
        Cenusii {
            grid,
            dist: Arc::new(dist),
            zomb: [[0; 3]; NZ],
            batt,
            got: 0,
            heroes: [
                Hero {
                    pos: start,
                    wounds: 0,
                    hands: [Some(0), Some(2)],
                    bandages: 0,
                },
                Hero {
                    pos: start,
                    wounds: 0,
                    hands: [Some(1), Some(4)],
                    bandages: 0,
                },
            ],
            deck,
            side: Player::One,
            actions: 3,
            searched: false,
            phase: Phase::Act,
            turns: 0,
            pending: None,
            over: false,
            won: false,
            rng,
            last: String::new(),
        }
    }

    pub fn batteries(&self) -> u8 {
        self.got
    }

    pub fn is_won(&self) -> bool {
        self.over && self.won
    }

    pub fn total_zombies(&self) -> u32 {
        self.zomb
            .iter()
            .map(|z| z.iter().map(|x| u32::from(*x)).sum::<u32>())
            .sum()
    }

    pub fn total_items(&self) -> usize {
        self.deck.len()
    }

    fn d(&self, a: usize, b: usize) -> u8 {
        self.dist[a * NZ + b]
    }

    fn spawn_zones(&self) -> Vec<usize> {
        (0..NZ)
            .filter(|&z| self.grid[z / W][z % W] == b'S')
            .collect()
    }

    fn exit_zone(&self) -> usize {
        (0..NZ)
            .find(|&z| self.grid[z / W][z % W] == b'E')
            .expect("ieșire")
    }

    fn zn(&self, z: usize) -> u32 {
        self.zomb[z].iter().map(|x| u32::from(*x)).sum()
    }

    fn neighbors(&self, z: usize) -> Vec<(u8, usize)> {
        let mut v = Vec::new();
        for (i, (dx, dy, _)) in DIRS.iter().enumerate() {
            let nx = (z % W) as isize + dx;
            let ny = (z / W) as isize + dy;
            if nx < 0 || ny < 0 || nx >= W as isize || ny >= H as isize {
                continue;
            }
            if self.grid[ny as usize][nx as usize] == b'#' {
                continue;
            }
            v.push((i as u8, ny as usize * W + nx as usize));
        }
        v
    }

    /// Zonele ținta posibile ale unei arme (cu umbre, în rază și cu linie liberă).
    fn targets(&self, from: usize, w: Weapon) -> Vec<usize> {
        let mut v = Vec::new();
        if self.zn(from) > 0 {
            v.push(from);
        }
        if !w.ranged {
            return v;
        }
        for (dx, dy, _) in DIRS {
            let (mut x, mut y) = ((from % W) as isize, (from / W) as isize);
            for _ in 0..w.range {
                x += dx;
                y += dy;
                if x < 0 || y < 0 || x >= W as isize || y >= H as isize {
                    break;
                }
                if self.grid[y as usize][x as usize] == b'#' {
                    break;
                }
                let z = y as usize * W + x as usize;
                if self.zn(z) > 0 {
                    v.push(z);
                }
            }
        }
        v
    }

    fn set_end(&mut self, won: bool) {
        if self.pending.is_none() {
            self.pending = Some(won);
        }
    }

    fn check_win(&mut self) {
        if self.heroes.iter().any(|h| h.wounds >= MAX_WOUNDS) {
            self.set_end(false);
            return;
        }
        let e = self.exit_zone();
        if self.got >= 3 && self.heroes.iter().all(|h| h.pos == e) {
            self.set_end(true);
        }
    }

    fn after_action(&mut self) {
        self.actions = self.actions.saturating_sub(1);
        self.check_win();
        if self.pending.is_none() && self.actions == 0 {
            self.phase = Phase::Zombies;
        }
    }

    fn dangerous(&self) -> usize {
        ((self.turns / 8) as usize).min(3)
    }

    /// Valoare euristică a stării (echipă).
    fn team_value(&self) -> i32 {
        if let Some(w) = self.pending {
            return if w { 100_000 } else { -100_000 };
        }
        let mut s = 400 * i32::from(self.got);
        for h in &self.heroes {
            let w = i32::from(h.wounds);
            s -= 150 * w * w;
            s += 12 * i32::from(h.bandages);
            for &sl in &h.hands {
                if let Some(wi) = sl {
                    let wp = WEAPONS[wi as usize];
                    s += 10 * i32::from(wp.dice) * i32::from(wp.dmg) * (7 - i32::from(wp.acc)) / 3;
                }
            }
        }
        let alive_pos: Vec<usize> = self.heroes.iter().map(|h| h.pos).collect();
        for z in 0..NZ {
            if self.zn(z) == 0 {
                continue;
            }
            let near = alive_pos.iter().map(|&p| self.d(p, z)).min().unwrap_or(9);
            for k in 0..3 {
                let n = i32::from(self.zomb[z][k]);
                if n == 0 {
                    continue;
                }
                let mut v = KIND_VAL[k] * n;
                if near == 0 {
                    v *= 5;
                } else if near == 1 {
                    v *= 3;
                } else if near == 2 {
                    v *= 2;
                }
                s -= v;
            }
        }
        let e = self.exit_zone();
        for h in &self.heroes {
            let target = if self.got >= 3 {
                e
            } else {
                (0..NZ)
                    .filter(|&z| self.batt[z])
                    .min_by_key(|&z| self.d(h.pos, z))
                    .unwrap_or(e)
            };
            s -= 14 * i32::from(self.d(h.pos, target).min(30));
            if self.got >= 3 && h.pos == e {
                s += 120;
            }
        }
        if let Phase::Roll(p) = self.phase {
            match p {
                Pending::Attack { weapon, zone } => {
                    let w = WEAPONS[weapon as usize];
                    let p_hit = (7.0 - f64::from(w.acc)) / 6.0;
                    let hits = f64::from(w.dice) * p_hit;
                    let z = zone as usize;
                    let mut gain = 0.0;
                    let mut left = hits;
                    for k in 0..3 {
                        if k == 2 && w.dmg < 2 {
                            continue;
                        }
                        let n = f64::from(self.zomb[z][k]).min(left);
                        gain += n * f64::from(KIND_VAL[k]) * 2.5;
                        left -= n;
                    }
                    s += gain as i32;
                }
                Pending::Search => s += if self.deck.is_empty() { 0 } else { 40 },
            }
        }
        s
    }
}

impl Cenusii {
    /// Cea mai bună valoare de echipă atinsă prin cel mult `depth` acțiuni rămase în tură.
    fn look(&self, depth: u32) -> i32 {
        if depth == 0
            || self.over
            || self.pending.is_some()
            || self.phase != Phase::Act
            || self.actions == 0
        {
            return self.team_value();
        }
        let mut kids: Vec<(i32, Cenusii)> = Game::legal_moves(self)
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
}

const BEAM: usize = 5;

impl Game for Cenusii {
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
        match self.phase {
            Phase::Roll(_) => vec![Move::Roll],
            Phase::Zombies => vec![Move::Zombies],
            Phase::Spawn => vec![Move::Spawn],
            Phase::Act => {
                let p = self.side.idx();
                let h = &self.heroes[p];
                let mut v = Vec::new();
                let cost = if self.zn(h.pos) > 0 { 2 } else { 1 };
                if self.actions >= cost {
                    for (i, _) in self.neighbors(h.pos) {
                        v.push(Move::Walk(i));
                    }
                }
                if self.actions >= 1 {
                    for (si, sl) in h.hands.iter().enumerate() {
                        if let Some(wi) = sl {
                            for z in self.targets(h.pos, WEAPONS[*wi as usize]) {
                                v.push(Move::Attack(si as u8, z as u8));
                            }
                        }
                    }
                    if !self.searched && self.zn(h.pos) == 0 && !self.deck.is_empty() {
                        v.push(Move::Search);
                    }
                    if self.batt[h.pos] {
                        v.push(Move::Take);
                    }
                    if h.bandages > 0 && h.wounds > 0 {
                        v.push(Move::Bandage);
                    }
                }
                v.push(Move::End);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let who = HERO_NAMES[p];
        match *m {
            Move::Walk(dir) => {
                let cost = if self.zn(self.heroes[p].pos) > 0 {
                    2
                } else {
                    1
                };
                let (dx, dy, name) = DIRS[dir as usize];
                let pos = self.heroes[p].pos;
                let nx = ((pos % W) as isize + dx) as usize;
                let ny = ((pos / W) as isize + dy) as usize;
                self.heroes[p].pos = ny * W + nx;
                self.last = format!("{who} merge {name} spre {}.", zone_name(ny * W + nx));
                self.actions = self.actions.saturating_sub(cost - 1);
                self.after_action();
            }
            Move::Attack(slot, z) => {
                let wi = self.heroes[p].hands[slot as usize].expect("armă în mână");
                self.last = format!(
                    "{who} atacă {} cu {}.",
                    zone_name(z as usize),
                    WEAPONS[wi as usize].name
                );
                self.phase = Phase::Roll(Pending::Attack {
                    weapon: wi,
                    zone: z,
                });
            }
            Move::Search => {
                self.searched = true;
                self.last = format!("{who} caută echipament.");
                self.phase = Phase::Roll(Pending::Search);
            }
            Move::Take => {
                let pos = self.heroes[p].pos;
                self.batt[pos] = false;
                self.got += 1;
                self.last = format!("{who} a luat o baterie ({}/3).", self.got);
                self.after_action();
            }
            Move::Bandage => {
                self.heroes[p].bandages -= 1;
                self.heroes[p].wounds -= 1;
                self.last = format!("{who} își pansează o rană.");
                self.after_action();
            }
            Move::End => {
                self.actions = 0;
                self.phase = Phase::Zombies;
                self.last = format!("{who} își încheie tura.");
            }
            Move::Roll => {
                let Phase::Roll(pd) = self.phase else {
                    return;
                };
                self.phase = Phase::Act;
                match pd {
                    Pending::Attack { weapon, zone } => {
                        let w = WEAPONS[weapon as usize];
                        let rolls: Vec<u8> = (0..w.dice).map(|_| self.rng.d6()).collect();
                        let hits = rolls.iter().filter(|r| **r >= w.acc).count();
                        let z = zone as usize;
                        let mut killed = [0u8; 3];
                        for _ in 0..hits {
                            if self.zomb[z][0] > 0 {
                                self.zomb[z][0] -= 1;
                                killed[0] += 1;
                            } else if self.zomb[z][1] > 0 {
                                self.zomb[z][1] -= 1;
                                killed[1] += 1;
                            } else if self.zomb[z][2] > 0 && w.dmg >= 2 {
                                self.zomb[z][2] -= 1;
                                killed[2] += 1;
                            }
                        }
                        self.last = format!(
                            "{who}: zaruri {:?} — {hits} lovituri, uciși {}/{}/{}.",
                            rolls, killed[0], killed[1], killed[2]
                        );
                    }
                    Pending::Search => {
                        let item = self.deck.pop().unwrap_or(ITEM_EMPTY);
                        match item {
                            ITEM_EMPTY => self.last = format!("{who} n-a găsit nimic."),
                            ITEM_BANDAGE => {
                                self.heroes[p].bandages += 1;
                                self.last = format!("{who} a găsit un pansament.");
                            }
                            wi => {
                                let h = &mut self.heroes[p];
                                if let Some(slot) = h.hands.iter().position(|x| x.is_none()) {
                                    h.hands[slot] = Some(wi);
                                    self.last = format!(
                                        "{who} a găsit {} (mâna {}).",
                                        WEAPONS[wi as usize].name,
                                        slot + 1
                                    );
                                } else {
                                    self.last = format!(
                                        "{who} a găsit {} dar nu mai are loc.",
                                        WEAPONS[wi as usize].name
                                    );
                                }
                            }
                        }
                    }
                }
                self.after_action();
            }
            Move::Zombies => {
                self.last = String::from("Umbrele se activează.");
                let hp: Vec<usize> = self.heroes.iter().map(|h| h.pos).collect();
                for pass in 0..2 {
                    let mut next = [[0u8; 3]; NZ];
                    for z in 0..NZ {
                        for k in 0..3 {
                            let n = self.zomb[z][k];
                            if n == 0 || (pass == 1 && k != 1) {
                                continue;
                            }
                            let here: Vec<usize> = (0..2).filter(|&i| hp[i] == z).collect();
                            if !here.is_empty() {
                                let dmg = if k == 2 { 2 } else { 1 };
                                for _ in 0..n {
                                    let t = *here
                                        .iter()
                                        .min_by_key(|&&i| (self.heroes[i].wounds, i))
                                        .unwrap();
                                    self.heroes[t].wounds =
                                        (self.heroes[t].wounds + dmg).min(MAX_WOUNDS);
                                }
                                next[z][k] += n;
                            } else {
                                let best = self
                                    .neighbors(z)
                                    .into_iter()
                                    .map(|(_, y)| y)
                                    .min_by_key(|&y| {
                                        (hp.iter().map(|&q| self.d(y, q)).min().unwrap_or(99), y)
                                    })
                                    .unwrap_or(z);
                                next[best][k] += n;
                            }
                        }
                    }
                    if pass == 0 {
                        // alergătorii nu s-au copiat în pasul 2: păstrează non-alergătorii mutați deja
                        self.zomb = next;
                    } else {
                        // pasul 2: doar alergătorii: ceilalți rămân; înlocuim alergătorii
                        let mut merged = self.zomb;
                        for z in 0..NZ {
                            merged[z][1] = 0;
                        }
                        for z in 0..NZ {
                            merged[z][1] += next[z][1];
                        }
                        self.zomb = merged;
                    }
                }
                self.check_win();
                if self.pending.is_none() {
                    self.phase = Phase::Spawn;
                }
            }
            Move::Spawn => {
                let lvl = self.dangerous();
                let per = if lvl >= 2 { 2 } else { 1 };
                let zones = self.spawn_zones();
                let pick = zones[(self.turns as usize) % zones.len()];
                for z in [pick] {
                    for _ in 0..per {
                        let r = self.rng.d6();
                        let k = match lvl {
                            0 => 0,
                            1 => usize::from(r >= 5),
                            2 => match r {
                                1..=3 => 0,
                                4 | 5 => 1,
                                _ => 2,
                            },
                            _ => match r {
                                1 | 2 => 0,
                                3 | 4 => 1,
                                _ => 2,
                            },
                        };
                        self.zomb[z][k] += 1;
                    }
                }
                self.turns += 1;
                self.last = format!("Apar umbre noi (nivel {}).", lvl + 1);
                if self.total_zombies() >= SWARM || self.turns >= TURN_CAP {
                    self.set_end(false);
                    return;
                }
                self.side = self.side.other();
                self.actions = 3;
                self.searched = false;
                self.phase = Phase::Act;
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

    /// Cooperativ: acțiunile din faza `Act` sunt deterministe; zarurile și extragerile sunt
    /// mutări forțate separate (frunze). Căutăm în lățime limitată restul acțiunilor din
    /// tură și evaluăm cu `team_value`; nu citim pachetul ascuns.
    fn evaluate(&self, _p: Player) -> i32 {
        self.look(u32::from(self.actions).min(3))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Noapte Cenușie — tura {}/{} | baterii {}/3 | umbre pe hartă {}\n",
            self.turns + 1,
            TURN_CAP,
            self.got,
            self.total_zombies()
        );
        s.push_str("    a      b      c      d      e      f      g\n");
        for y in 0..H {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..W {
                let z = y * W + x;
                let c = self.grid[y][x];
                let mut cell = String::new();
                if c == b'#' {
                    cell.push_str("[#####]");
                } else {
                    let h1 = if self.heroes[0].pos == z { '1' } else { ' ' };
                    let h2 = if self.heroes[1].pos == z { '2' } else { ' ' };
                    let zt = self.zn(z);
                    let f = if self.batt[z] {
                        'B'
                    } else if c == b'E' {
                        'E'
                    } else if c == b'S' {
                        'S'
                    } else {
                        '.'
                    };
                    cell.push_str(&format!(
                        "[{h1}{h2}{}{f}]",
                        if zt > 0 {
                            format!("z{zt:<2}")
                        } else {
                            "   ".into()
                        }
                    ));
                    // lățime 7 ("[" + 2 + 3 + 1 + "]")
                }
                s.push_str(&cell);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("Legendă: 1 = Ilinca (tu), 2 = Radu (AI), zN = umbre, B baterie, E ieșire, S apariție, # zid.\n");
        for z in 0..NZ {
            if self.zn(z) > 0 {
                let parts: Vec<String> = (0..3)
                    .filter(|&k| self.zomb[z][k] > 0)
                    .map(|k| format!("{} {}", self.zomb[z][k], KIND_NAMES[k]))
                    .collect();
                s.push_str(&format!("  {}: {}\n", zone_name(z), parts.join(", ")));
            }
        }
        for (i, h) in self.heroes.iter().enumerate() {
            let hands: Vec<String> = h
                .hands
                .iter()
                .enumerate()
                .map(|(k, x)| match x {
                    Some(w) => format!("{}={}", k + 1, WEAPONS[*w as usize].name),
                    None => format!("{}=—", k + 1),
                })
                .collect();
            s.push_str(&format!(
                "J{} {} la {} — răni {}/{} — {} — pansamente {}\n",
                i + 1,
                HERO_NAMES[i],
                zone_name(h.pos),
                h.wounds,
                MAX_WOUNDS,
                hands.join(" "),
                h.bandages
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        if self.over {
            if self.won {
                s.push_str("VICTORIE: ați scăpat din cartier.\n");
            } else {
                s.push_str("ÎNFRÂNGERE (remiză): noaptea v-a prins.\n");
            }
        } else if self.pending.is_some() {
            s.push_str("Jocul se încheie: sfârșit.\n");
        } else {
            match self.phase {
                Phase::Act => s.push_str(&format!(
                    "La rând J{} ({}) — acțiuni {}. Mutări: mers n|s|e|v, atac 1 c3, cauta, ia, pansez, gata.\n",
                    self.side.idx() + 1,
                    HERO_NAMES[self.side.idx()],
                    self.actions
                )),
                Phase::Roll(Pending::Attack { .. }) => s.push_str("Se aruncă zarurile: zaruri.\n"),
                Phase::Roll(Pending::Search) => s.push_str("Se extrage echipament: extrage.\n"),
                Phase::Zombies => s.push_str("Umbrele se activează: umbre.\n"),
                Phase::Spawn => s.push_str("Apariții: aparitie.\n"),
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Walk(d) => format!("mers {}", DIRS[*d as usize].2),
            Move::Attack(s, z) => format!("atac {} {}", s + 1, zone_name(*z as usize)),
            Move::Search => "cauta".into(),
            Move::Take => "ia".into(),
            Move::Bandage => "pansez".into(),
            Move::End => "gata".into(),
            Move::Roll => match self.phase {
                Phase::Roll(Pending::Search) => "extrage".into(),
                _ => "zaruri".into(),
            },
            Move::Zombies => "umbre".into(),
            Move::Spawn => "aparitie".into(),
            Move::Finish => "sfârșit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ai_game(seed: u64) -> Cenusii {
        let mut g = Cenusii::with_seed(seed);
        let mut rng = Rng::new(seed ^ 11);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            n += 1;
            assert!(n < 5000);
        }
        g
    }

    fn empty(g: &mut Cenusii) {
        g.zomb = [[0; 3]; NZ];
    }

    #[test]
    fn zone_names_roundtrip() {
        for z in 0..NZ {
            assert_eq!(parse_zone(&zone_name(z)), Some(z));
        }
        assert_eq!(parse_zone("z9"), None);
    }

    #[test]
    fn map_is_connected_from_start() {
        let g = Cenusii::with_seed(1);
        let start = g.heroes[0].pos;
        for z in 0..NZ {
            if g.grid[z / W][z % W] != b'#' {
                assert!(g.d(start, z) < 99, "{}", zone_name(z));
            }
        }
        assert_eq!(g.batt.iter().filter(|x| **x).count(), 3);
        assert_eq!(g.deck.len(), 20);
    }

    #[test]
    fn walking_costs_extra_when_engaged() {
        let mut g = Cenusii::with_seed(2);
        let pos = g.heroes[0].pos;
        g.zomb[pos][0] = 1;
        g.actions = 1;
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Walk(_))));
        g.actions = 2;
        let w = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Walk(_)))
            .unwrap();
        g.apply(&w);
        assert_eq!(g.actions, 0);
        assert_eq!(g.legal_moves(), vec![Move::Zombies]);
    }

    #[test]
    fn ranged_line_and_walls() {
        let mut g = Cenusii::with_seed(3);
        empty(&mut g);
        let c = parse_zone("d6").unwrap();
        g.zomb[parse_zone("d4").unwrap()][0] = 1;
        let pistol = WEAPONS[3]; // arbaleta, raza 2
        assert!(g.targets(c, pistol).contains(&parse_zone("d4").unwrap()));
        let sling = WEAPONS[2]; // raza 1
        assert!(!g.targets(c, sling).contains(&parse_zone("d4").unwrap()));
        // zid: b2 e zid; din b3 spre nord nu se vede b1
        g.zomb[parse_zone("b1").unwrap()][0] = 1;
        assert!(!g
            .targets(parse_zone("b3").unwrap(), pistol)
            .contains(&parse_zone("b1").unwrap()));
        // corp la corp doar în zona proprie
        g.zomb[c][0] = 1;
        assert_eq!(g.targets(c, WEAPONS[0]), vec![c]);
    }

    #[test]
    fn attack_resolves_in_forced_step() {
        let mut g = Cenusii::with_seed(4);
        empty(&mut g);
        let pos = g.heroes[0].pos;
        g.zomb[pos][0] = 3;
        g.apply(&Move::Attack(0, pos as u8));
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        assert_eq!(g.zn(pos), 3);
        let before = g.actions;
        g.apply(&Move::Roll);
        assert_eq!(g.actions, before - 1);
        assert!(g.zn(pos) <= 3);
        assert_eq!(g.format_move(&Move::Roll), "zaruri");
    }

    #[test]
    fn brute_needs_strong_weapon() {
        let mut g = Cenusii::with_seed(5);
        empty(&mut g);
        let pos = g.heroes[0].pos;
        g.zomb[pos][2] = 1;
        // ciomag (dmg 1): nu poate omorî un colos, oricâte zaruri ar nimeri
        for seed in 0..20 {
            let mut h = g.clone();
            h.rng = Rng::new(seed);
            h.apply(&Move::Attack(0, pos as u8));
            h.apply(&Move::Roll);
            assert_eq!(h.zomb[pos][2], 1);
        }
        let mut killed = false;
        for seed in 0..40 {
            let mut h = g.clone();
            h.heroes[0].hands[0] = Some(1);
            h.rng = Rng::new(seed);
            h.apply(&Move::Attack(0, pos as u8));
            h.apply(&Move::Roll);
            if h.zomb[pos][2] == 0 {
                killed = true;
            }
        }
        assert!(killed);
    }

    #[test]
    fn search_gives_item_or_nothing() {
        let mut g = Cenusii::with_seed(6);
        empty(&mut g);
        g.heroes[0].hands = [None, None];
        let n = g.deck.len();
        g.apply(&Move::Search);
        assert_eq!(g.format_move(&Move::Roll), "extrage");
        g.apply(&Move::Roll);
        assert_eq!(g.deck.len(), n - 1);
        assert!(!g.legal_moves().contains(&Move::Search));
    }

    #[test]
    fn zombies_attack_and_chase() {
        let mut g = Cenusii::with_seed(7);
        empty(&mut g);
        let pos = g.heroes[0].pos;
        g.heroes[1].pos = pos;
        g.zomb[pos][0] = 2;
        g.zomb[parse_zone("d1").unwrap()][0] = 1;
        g.phase = Phase::Zombies;
        g.apply(&Move::Zombies);
        assert_eq!(g.heroes[0].wounds + g.heroes[1].wounds, 2);
        let far = parse_zone("d1").unwrap();
        assert_eq!(g.zomb[far][0], 0);
        assert_eq!(g.total_zombies(), 3);
    }

    #[test]
    fn runner_moves_twice() {
        let mut g = Cenusii::with_seed(8);
        empty(&mut g);
        let pos = g.heroes[0].pos;
        let start = parse_zone("d3").unwrap();
        g.zomb[start][1] = 1;
        g.phase = Phase::Zombies;
        g.apply(&Move::Zombies);
        let z = (0..NZ).find(|&z| g.zomb[z][1] > 0).unwrap();
        assert_eq!(g.d(z, pos), g.d(start, pos) - 2);
    }

    #[test]
    fn spawn_adds_and_passes_turn() {
        let mut g = Cenusii::with_seed(9);
        empty(&mut g);
        g.phase = Phase::Spawn;
        g.apply(&Move::Spawn);
        assert_eq!(g.total_zombies(), 1);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.actions, 3);
    }

    #[test]
    fn win_requires_batteries_and_both_at_exit() {
        let mut g = Cenusii::with_seed(10);
        empty(&mut g);
        let e = g.exit_zone();
        g.heroes[0].pos = e;
        g.heroes[1].pos = e;
        g.got = 2;
        g.check_win();
        assert!(g.pending.is_none());
        g.got = 3;
        g.check_win();
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn death_is_a_draw() {
        let mut g = Cenusii::with_seed(11);
        g.heroes[1].wounds = MAX_WOUNDS;
        g.check_win();
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=40u64 {
            let mut g = Cenusii::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                n += 1;
                assert!(n < 5000);
            }
        }
    }

    #[test]
    fn ai_team_survives_and_sometimes_wins() {
        let mut wins = 0;
        let mut batt = 0;
        for seed in 1..=30u64 {
            let g = ai_game(seed);
            if g.is_won() {
                wins += 1;
            }
            batt += u32::from(g.batteries());
        }
        eprintln!(
            "cenusii: victorii {wins}/30, baterii medii {}",
            f64::from(batt) / 30.0
        );
        assert!(wins >= 5 && batt >= 30, "AI: {wins}/30");
    }

    #[test]
    fn evaluate_ignores_the_hidden_deck_order() {
        let g = Cenusii::with_seed(3);
        let mut h = g.clone();
        h.deck.reverse();
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }
}
