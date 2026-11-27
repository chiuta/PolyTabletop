//! Motor comun „carte colecționabilă” (familia T6): 2 jucători, pachete fixe de 30 de cărți
//! construite de joc, mână ascunsă, parametrizat prin `Spec` (fiecare joc are tabele proprii).
//!
//! Reguli generice (scrise de noi): 10 unități (cost, atac, viață, cuvinte-cheie) și 5 vrăji
//! pe joc. Resurse: cărți-sursă, creștere automată, cărți puse drept sursă sau rezervă care se
//! păstrează. Victorie: viață, sigilii (cu breșă), glorie (misiuni) sau trei zone.
//! Mutări: `sursa i`, `joc i [țintă]`, `atac k țintă`, `miss k`, `fin`; ținte: `eN` (unitate
//! inamică), `mN` (unitate proprie), `f` (adversarul), `zN` (zonă).
//! Informație ascunsă: mâna și ordinea pachetului; `render` arată doar mâna privitorului
//! (HOTSEAT_SAFE). AI-ul are adâncime fixă 0, iar `eval` citește doar câmpul public, mâna proprie
//! și NUMĂRUL cărților din mâna adversarului.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub const HASTE: u8 = 1;
pub const GUARD: u8 = 2;
pub const FLY: u8 = 4;
pub const FIRST: u8 = 8;
pub const ARMOR: u8 = 16;
pub const VENOM: u8 = 32;
pub const LEECH: u8 = 64;
/// Cartea-sursă din pachet (indicele 15).
pub const LAND: u8 = 15;
const HANDMAX: usize = 8;

pub struct U {
    pub name: &'static str,
    pub cost: u8,
    pub atk: i8,
    pub hp: i8,
    pub kw: u8,
}
pub struct S {
    pub name: &'static str,
    pub cost: u8,
    pub fx: u8,
    pub n: i8,
}
pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub facs: [&'static str; 2],
    /// 0 sursă din pachet, 1 automat, 2 carte-sursă din mână, 3 rezervă care se păstrează
    pub res: u8,
    pub cap: u8,
    pub lands: u8,
    pub bank: u8,
    /// 0 viață, 1 sigilii, 2 glorie, 3 zone
    pub win: u8,
    pub wn: i8,
    pub big: i8,
    pub hand0: u8,
    pub maxt: u8,
    pub bmax: u8,
    /// b'-' fără, b'S' cicatrice, b'K' pradă, b'H' elan general, b'P' presiune
    pub twist: u8,
    pub units: [U; 10],
    pub spells: [S; 5],
    pub deck: [[u8; 15]; 2],
}

#[derive(Clone)]
struct Un {
    c: u8,
    atk: i8,
    hp: i8,
    dmg: i8,
    tired: bool,
    fresh: bool,
    frozen: bool,
}

#[derive(Clone)]
pub struct Tcg {
    sp: &'static Spec,
    viewer: Option<Player>,
    deck: [Vec<u8>; 2],
    hand: [Vec<u8>; 2],
    bd: [Vec<Un>; 2],
    src: [i32; 2],
    mana: [i32; 2],
    pts: [i32; 2],
    zones: [[i32; 3]; 2],
    sourced: bool,
    turn: usize,
    turns: [u32; 2],
    rng: Rng,
    done: Option<Outcome>,
}

impl Tcg {
    pub fn new(sp: &'static Spec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static Spec, seed: u64) -> Self {
        let mut t = Tcg {
            sp,
            viewer: Some(Player::One),
            deck: [Vec::new(), Vec::new()],
            hand: [Vec::new(), Vec::new()],
            bd: [Vec::new(), Vec::new()],
            src: [0; 2],
            mana: [0; 2],
            pts: [0; 2],
            zones: [[0; 3]; 2],
            sourced: false,
            turn: 0,
            turns: [0; 2],
            rng: Rng::new(seed),
            done: None,
        };
        for p in 0..2 {
            let mut d = Vec::new();
            for (c, &n) in sp.deck[p].iter().enumerate() {
                for _ in 0..n {
                    d.push(c as u8);
                }
            }
            if sp.res == 0 {
                d.extend(std::iter::repeat_n(LAND, sp.lands as usize));
            }
            shuf(&mut d, &mut t.rng);
            t.deck[p] = d;
            t.pts[p] = match sp.win {
                0 | 1 => sp.wn as i32,
                _ => 0,
            };
            if sp.win == 3 {
                t.zones[p] = [sp.wn as i32; 3];
            }
        }
        for p in 0..2 {
            for _ in 0..(sp.hand0 as usize + p) {
                t.draw(p);
            }
        }
        t.start_turn(0, false);
        t
    }

    fn draw(&mut self, p: usize) -> bool {
        match self.deck[p].pop() {
            Some(c) => {
                if self.hand[p].len() < HANDMAX {
                    self.hand[p].push(c);
                }
                true
            }
            None => false,
        }
    }

    fn start_turn(&mut self, p: usize, draw: bool) {
        self.turn = p;
        self.turns[p] += 1;
        for u in self.bd[p].iter_mut() {
            if u.frozen {
                u.frozen = false;
            } else {
                u.tired = false;
            }
            u.fresh = false;
        }
        let cap = self.sp.cap as i32;
        match self.sp.res {
            1 => {
                self.src[p] = (self.src[p] + 1).min(cap);
                self.mana[p] = self.src[p];
            }
            3 => self.mana[p] = (self.mana[p] + self.sp.bank as i32).min(cap),
            _ => self.mana[p] = self.src[p],
        }
        self.sourced = false;
        if draw && !self.draw(p) {
            self.done = Some(Outcome::Win(if p == 0 { Player::Two } else { Player::One }));
        }
    }

    fn side_of(i: usize) -> Player {
        if i == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn score(&self, p: usize) -> i32 {
        if self.sp.win == 3 {
            self.zones[p].iter().sum()
        } else {
            self.pts[p]
        }
    }

    fn uniq(&self, p: usize) -> Vec<usize> {
        let mut seen = [false; 16];
        let mut v = Vec::new();
        for (i, &c) in self.hand[p].iter().enumerate() {
            if !seen[c as usize] {
                seen[c as usize] = true;
                v.push(i);
            }
        }
        v
    }

    fn spell_targets(&self, p: usize, s: &S) -> Vec<String> {
        let o = 1 - p;
        let en = |f: &dyn Fn(&Un) -> bool| -> Vec<String> {
            self.bd[o]
                .iter()
                .enumerate()
                .filter(|(_, u)| f(u))
                .map(|(j, _)| format!("e{j}"))
                .collect()
        };
        let one = |ok: bool| -> Vec<String> {
            if ok {
                vec![String::new()]
            } else {
                Vec::new()
            }
        };
        match s.fx {
            0 => {
                let mut v = en(&|_| true);
                if self.sp.win <= 1 {
                    v.push("f".into());
                }
                if self.sp.win == 3 {
                    for z in 0..3 {
                        if self.zones[o][z] > 0 {
                            v.push(format!("z{z}"));
                        }
                    }
                }
                v
            }
            1 => one(!self.bd[o].is_empty()),
            2 => one(!self.deck[p].is_empty()),
            3 => one(!self.bd[p].is_empty()),
            4 => (0..self.bd[p].len()).map(|j| format!("m{j}")).collect(),
            5 => en(&|u| self.sp.units[u.c as usize].cost as i8 <= s.n),
            6 => en(&|_| true),
            7 => one(true),
            8 => one(self.bd[p].iter().any(|u| u.tired)),
            _ => en(&|u| !u.tired),
        }
    }

    fn hit(&mut self, p: usize, amt: i32, z: Option<usize>) {
        let o = 1 - p;
        let amt = amt + if self.sp.twist == b'P' { 1 } else { 0 };
        match self.sp.win {
            0 => {
                self.pts[o] -= amt;
                if self.pts[o] <= 0 {
                    self.done = Some(Outcome::Win(Self::side_of(p)));
                }
            }
            1 => {
                if self.pts[o] == 0 {
                    self.done = Some(Outcome::Win(Self::side_of(p)));
                } else {
                    let d = if amt >= self.sp.big as i32 { 2 } else { 1 };
                    self.pts[o] = (self.pts[o] - d).max(0);
                }
            }
            3 => {
                let z = z.unwrap_or(0);
                self.zones[o][z] = (self.zones[o][z] - amt).max(0);
                if self.zones[o].iter().filter(|&&h| h == 0).count() >= 2 {
                    self.done = Some(Outcome::Win(Self::side_of(p)));
                }
            }
            _ => {}
        }
    }

    fn gain(&mut self, p: usize, n: i32) {
        let wn = self.sp.wn as i32;
        match self.sp.win {
            0 => self.pts[p] = (self.pts[p] + 3 * n).min(wn),
            1 => self.pts[p] = (self.pts[p] + n).min(wn),
            2 => self.glory(p, n),
            _ => {
                for z in 0..3 {
                    if self.zones[p][z] > 0 {
                        self.zones[p][z] = (self.zones[p][z] + 2 * n).min(wn);
                    }
                }
            }
        }
    }

    fn glory(&mut self, p: usize, n: i32) {
        self.pts[p] += n;
        if self.pts[p] >= self.sp.wn as i32 && self.done.is_none() {
            self.done = Some(Outcome::Win(Self::side_of(p)));
        }
    }

    fn leech(&mut self, p: usize) {
        match self.sp.win {
            0 => self.pts[p] = (self.pts[p] + 1).min(self.sp.wn as i32),
            2 => self.glory(p, 1),
            _ => {}
        }
    }

    fn clean(&mut self) {
        for p in 0..2 {
            self.bd[p].retain(|u| u.dmg < u.hp);
        }
    }

    fn fight(&mut self, p: usize, k: usize, j: usize) {
        let o = 1 - p;
        let mut a = self.bd[p][k].clone();
        let mut d = self.bd[o][j].clone();
        let (ak, dk) = (
            self.sp.units[a.c as usize].kw,
            self.sp.units[d.c as usize].kw,
        );
        let da = if dk & ARMOR != 0 {
            (a.atk - 1).max(0)
        } else {
            a.atk
        };
        let dd = if ak & ARMOR != 0 {
            (d.atk - 1).max(0)
        } else {
            d.atk
        };
        let a_first = ak & FIRST != 0 && dk & FIRST == 0;
        let d_first = dk & FIRST != 0 && ak & FIRST == 0;
        let strike = |dmg: i8, venom: bool, t: &mut Un| -> bool {
            if dmg > 0 {
                t.dmg = if venom {
                    t.hp
                } else {
                    t.dmg.saturating_add(dmg)
                };
            }
            dmg > 0
        };
        let (mut a_hit, mut d_hit) = (false, false);
        if a_first {
            a_hit = strike(da, ak & VENOM != 0, &mut d);
            if d.dmg < d.hp {
                d_hit = strike(dd, dk & VENOM != 0, &mut a);
            }
        } else if d_first {
            d_hit = strike(dd, dk & VENOM != 0, &mut a);
            if a.dmg < a.hp {
                a_hit = strike(da, ak & VENOM != 0, &mut d);
            }
        } else {
            a_hit = strike(da, ak & VENOM != 0, &mut d);
            d_hit = strike(dd, dk & VENOM != 0, &mut a);
        }
        a.tired = true;
        let (a_dead, d_dead) = (a.dmg >= a.hp, d.dmg >= d.hp);
        self.bd[p][k] = a;
        self.bd[o][j] = d;
        if a_hit && ak & LEECH != 0 {
            self.leech(p);
        }
        if d_hit && dk & LEECH != 0 {
            self.leech(o);
        }
        self.clean();
        if self.sp.twist == b'K' {
            if d_dead {
                self.draw(p);
            }
            if a_dead {
                self.draw(o);
            }
        }
    }

    fn cast(&mut self, p: usize, si: usize, t: &str) {
        let o = 1 - p;
        let s = &self.sp.spells[si];
        let n = s.n;
        let idx: usize = t.get(1..).and_then(|x| x.parse().ok()).unwrap_or(0);
        match s.fx {
            0 => {
                if t == "f" {
                    self.hit(p, n as i32, None);
                } else if t.starts_with('z') {
                    self.hit(p, n as i32, Some(idx));
                } else {
                    self.bd[o][idx].dmg += n;
                    self.clean();
                }
            }
            1 => {
                for u in self.bd[o].iter_mut() {
                    u.dmg += n;
                }
                self.clean();
            }
            2 => {
                for _ in 0..n {
                    self.draw(p);
                }
            }
            3 => {
                for u in self.bd[p].iter_mut() {
                    u.hp += n;
                }
            }
            4 => self.bd[p][idx].atk += n,
            5 => {
                self.bd[o].remove(idx);
            }
            6 => {
                let u = self.bd[o].remove(idx);
                if self.hand[o].len() < HANDMAX {
                    self.hand[o].push(u.c);
                }
            }
            7 => self.gain(p, n as i32),
            8 => {
                for u in self.bd[p].iter_mut() {
                    u.tired = false;
                    u.frozen = false;
                }
            }
            _ => {
                self.bd[o][idx].tired = true;
                self.bd[o][idx].frozen = true;
            }
        }
    }

    fn board_val(&self, i: usize) -> i32 {
        self.bd[i]
            .iter()
            .map(|u| {
                let kw = self.sp.units[u.c as usize].kw;
                let b = [
                    (HASTE, 1),
                    (GUARD, 2),
                    (FLY, 3),
                    (FIRST, 3),
                    (ARMOR, 3),
                    (VENOM, 4),
                    (LEECH, 2),
                ];
                3 * u.atk as i32
                    + 2 * (u.hp - u.dmg).max(0) as i32
                    + b.iter()
                        .filter(|(f, _)| kw & f != 0)
                        .map(|(_, v)| v)
                        .sum::<i32>()
            })
            .sum()
    }

    fn prog(&self, i: usize) -> i32 {
        match self.sp.win {
            0 => 3 * self.pts[i],
            1 => 14 * self.pts[i] - if self.pts[i] == 0 { 40 } else { 0 },
            2 => 7 * self.pts[i],
            _ => {
                2 * self.zones[i].iter().sum::<i32>()
                    - 30 * self.zones[i].iter().filter(|&&h| h == 0).count() as i32
            }
        }
    }
}

impl Mini for Tcg {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        Self::side_of(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        let sp = self.sp;
        let p = self.turn;
        let o = 1 - p;
        let mut v = vec!["fin".to_string()];
        let mana = self.mana[p];
        let un = self.uniq(p);
        if !self.sourced && self.src[p] < sp.cap as i32 {
            for &i in &un {
                let c = self.hand[p][i];
                if (sp.res == 0 && c == LAND) || sp.res == 2 {
                    v.push(format!("sursa {i}"));
                }
            }
        }
        for &i in &un {
            let c = self.hand[p][i];
            if c == LAND {
                continue;
            }
            if c < 10 {
                let u = &sp.units[c as usize];
                if u.cost as i32 <= mana && self.bd[p].len() < sp.bmax as usize {
                    v.push(format!("joc {i}"));
                }
            } else {
                let s = &sp.spells[c as usize - 10];
                if s.cost as i32 <= mana {
                    for t in self.spell_targets(p, s) {
                        v.push(if t.is_empty() {
                            format!("joc {i}")
                        } else {
                            format!("joc {i} {t}")
                        });
                    }
                }
            }
        }
        for (k, u) in self.bd[p].iter().enumerate() {
            if u.tired || u.fresh {
                continue;
            }
            if sp.win == 2 {
                v.push(format!("miss {k}"));
            }
            if u.atk <= 0 {
                continue;
            }
            let fly = sp.units[u.c as usize].kw & FLY != 0;
            for (j, e) in self.bd[o].iter().enumerate() {
                if sp.units[e.c as usize].kw & FLY != 0 && !fly {
                    continue;
                }
                if sp.win == 2 && !e.tired {
                    continue;
                }
                v.push(format!("atac {k} e{j}"));
            }
            if sp.win != 2 {
                let guarded = !fly
                    && self.bd[o]
                        .iter()
                        .any(|e| !e.tired && sp.units[e.c as usize].kw & GUARD != 0);
                if !guarded {
                    if sp.win == 3 {
                        for z in 0..3 {
                            if self.zones[o][z] > 0 {
                                v.push(format!("atac {k} z{z}"));
                            }
                        }
                    } else {
                        v.push(format!("atac {k} f"));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let p = self.turn;
        let mut it = m.split(' ');
        let verb = it.next().unwrap_or("");
        let a: usize = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        let t = it.next().unwrap_or("");
        match verb {
            "fin" => {
                if p == 1 && self.turns[1] >= self.sp.maxt as u32 {
                    self.done = Some(by_score(self.score(0), self.score(1)));
                    return;
                }
                if self.sp.twist != b'S' {
                    for q in 0..2 {
                        for u in self.bd[q].iter_mut() {
                            u.dmg = 0;
                        }
                    }
                }
                self.start_turn(1 - p, true);
            }
            "sursa" => {
                self.hand[p].remove(a);
                self.src[p] += 1;
                self.mana[p] += 1;
                self.sourced = true;
            }
            "joc" => {
                let c = self.hand[p].remove(a);
                if c < 10 {
                    let u = &self.sp.units[c as usize];
                    self.mana[p] -= u.cost as i32;
                    let ready = u.kw & HASTE != 0 || self.sp.twist == b'H';
                    self.bd[p].push(Un {
                        c,
                        atk: u.atk,
                        hp: u.hp,
                        dmg: 0,
                        tired: false,
                        fresh: !ready,
                        frozen: false,
                    });
                } else {
                    let si = c as usize - 10;
                    self.mana[p] -= self.sp.spells[si].cost as i32;
                    self.cast(p, si, t);
                }
            }
            "miss" => {
                let cost = self.sp.units[self.bd[p][a].c as usize].cost as i32;
                self.bd[p][a].tired = true;
                self.glory(p, cost.max(1));
            }
            _ => {
                // atac k țintă
                let idx: usize = t.get(1..).and_then(|x| x.parse().ok()).unwrap_or(0);
                if t.starts_with('e') {
                    self.fight(p, a, idx);
                } else {
                    let atk = self.bd[p][a].atk as i32;
                    self.bd[p][a].tired = true;
                    self.hit(p, atk, if t == "f" { None } else { Some(idx) });
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.done
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        self.prog(i) - self.prog(o) + self.board_val(i) - self.board_val(o)
            + 2 * (self.hand[i].len() as i32 - self.hand[o].len() as i32)
            + 3 * (self.src[i] - self.src[o])
    }
    fn show(&self) -> String {
        let sp = self.sp;
        let me = self.viewer.map(pi).unwrap_or(self.turn);
        let mut s = format!(
            "{} — tura {}/{} | J1 {} contra J2 {}\n",
            sp.title, self.turns[self.turn], sp.maxt, sp.facs[0], sp.facs[1]
        );
        let what = ["viață", "sigilii", "glorie", "zone"][sp.win as usize];
        for q in 0..2 {
            let st = if sp.win == 3 {
                format!("{:?}", self.zones[q])
            } else {
                format!("{}", self.pts[q])
            };
            s.push_str(&format!(
                "J{}: {what} {st} | energie {}{} | pachet {} | mână {}\n",
                q + 1,
                self.mana[q],
                if sp.res == 3 {
                    format!("/{}", sp.cap)
                } else {
                    format!("/{}", self.src[q])
                },
                self.deck[q].len(),
                self.hand[q].len()
            ));
        }
        s.push_str(&format!("La mutare: J{}\n", self.turn + 1));
        const KN: [(u8, &str); 7] = [
            (HASTE, "elan"),
            (GUARD, "gardă"),
            (FLY, "zbor"),
            (FIRST, "primul"),
            (ARMOR, "armură"),
            (VENOM, "venin"),
            (LEECH, "sorbire"),
        ];
        for q in 0..2 {
            s.push_str(&format!("Câmp J{}:\n", q + 1));
            for (k, u) in self.bd[q].iter().enumerate() {
                let c = &sp.units[u.c as usize];
                let kws: Vec<&str> = KN
                    .iter()
                    .filter(|(f, _)| c.kw & f != 0)
                    .map(|x| x.1)
                    .collect();
                s.push_str(&format!(
                    "  {}{k}: {} {}/{} (daune {}){}{}{}\n",
                    if q == self.turn { "m" } else { "e" },
                    c.name,
                    u.atk,
                    u.hp,
                    u.dmg,
                    if kws.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", kws.join(", "))
                    },
                    if u.tired { " obosit" } else { "" },
                    if u.fresh { " nou" } else { "" }
                ));
            }
        }
        s.push_str(&format!("Mâna lui J{}:\n", me + 1));
        for (i, &c) in self.hand[me].iter().enumerate() {
            if c == LAND {
                s.push_str(&format!("  {i}: carte-sursă\n"));
            } else if c < 10 {
                let u = &sp.units[c as usize];
                s.push_str(&format!(
                    "  {i}: unitate {} cost {} {}/{}\n",
                    u.name, u.cost, u.atk, u.hp
                ));
            } else {
                let sc = &sp.spells[c as usize - 10];
                s.push_str(&format!(
                    "  {i}: vrajă {} cost {} (efect {} / {})\n",
                    sc.name, sc.cost, sc.fx, sc.n
                ));
            }
        }
        if self.done.is_some() {
            s.push_str("Joc încheiat.\n");
        }
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

/// Teste comune: AI-ul nu depinde de mâna/pachetul adversarului și `render` nu le scurge.
#[cfg(test)]
pub fn blind_check(sp: &'static Spec) {
    for seed in 1..=6u64 {
        let mut g = Tcg::with_seed(sp, seed);
        let mut rng = Rng::new(seed * 31);
        for ply in 0..80 {
            if g.done.is_some() {
                break;
            }
            let side = g.turn;
            let o = 1 - side;
            let mut h = g.clone();
            let mut pool: Vec<u8> = h.hand[o].iter().chain(h.deck[o].iter()).copied().collect();
            let hn = h.hand[o].len();
            let mut r2 = Rng::new(seed * 977 + ply as u64);
            shuf(&mut pool, &mut r2);
            h.hand[o] = pool[..hn].to_vec();
            h.deck[o] = pool[hn..].to_vec();
            if ply % 3 == 0 {
                assert_eq!(
                    ai_of(&g, 5),
                    ai_of(&h, 5),
                    "{} AI citește informația ascunsă",
                    sp.id
                );
            }
            g.view(Some(Tcg::side_of(side)));
            h.view(Some(Tcg::side_of(side)));
            assert_eq!(
                g.show(),
                h.show(),
                "{} render scurge mâna adversarului",
                sp.id
            );
            let lm = g.moves();
            let m = lm[rng.below(lm.len())].clone();
            g.step(&m);
        }
    }
}
