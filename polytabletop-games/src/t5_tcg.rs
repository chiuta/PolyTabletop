//! Motor comun T5 pentru jocurile de cărți de tip „pachete fixe, mână ascunsă” (variante originale,
//! clean-room: reguli proprii, scrise de noi din mecanica generală a genului; fără nume, text sau
//! cărți ale vreunui produs comercial).
//!
//! Fiecare joc are un `Spec` propriu (tabele de unități și vrăji, două pachete fixe construite de joc,
//! mod de resurse, condiție de victorie, regula de atac, limite și „răsuciri”).
//!
//! * Resurse: `Land` (cărți-sursă în pachet, una pe tură), `Auto` (energie care crește singură cu
//!   fiecare tură) sau `Ink` (orice carte din mână poate fi pusă o dată pe tură ca sursă).
//! * Victorie: `Life` (viață), `Shield` (scuturi: fiecare lovitură reușită rupe scuturi și le
//!   dă în mână adversarului; lovitura fără scuturi câștigă), `Points` (misiuni: unitățile obosesc
//!   pentru puncte, iar atacurile lovesc doar unități obosite) sau `Zones` (trei zone cu viață
//!   proprie; două zone distruse câștigă).
//! * Unități: cost, atac, rezistență, cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul,
//!   Armură, Venin); vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere,
//!   întoarcere în mână, vindecare.
//! * Informație ascunsă: mâinile, pachetele și scuturile. `render` arată doar mâna privitorului
//!   (`set_viewer`; fără privitor = mâna celui la mutare). `eval` citește doar câmpurile publice,
//!   mâna proprie și NUMĂRUL cărților adverse; AI-ul are adâncime 0 (o singură mutare), deci nu
//!   cheamă niciodată informația ascunsă a adversarului.
#![allow(clippy::needless_range_loop)]

use crate::brand_util::{by_score, Mini};
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub const HASTE: u8 = 1;
pub const GUARD: u8 = 2;
pub const FLY: u8 = 4;
pub const FIRST: u8 = 8;
pub const ARMOR: u8 = 16;
pub const VENOM: u8 = 32;

/// Unitățile se vindecă la începutul turei proprii.
pub const T_REGEN: u8 = 1;
/// Cine ucide o unitate în luptă trage o carte.
pub const T_TROPHY: u8 = 2;
/// Nicio unitate nu are „boală de invocare”.
pub const T_RUSH: u8 = 4;
/// Loviturile directe sunt cu 1 mai tari (prag de dublă ruptură cu 1 mai jos).
pub const T_BLEED: u8 = 8;

const SRC: u8 = 250;
const HANDMAX: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fx {
    Draw(u8),
    Bolt(i16),
    Sweep(i16),
    Boon(i16),
    Buff(i16),
    Slay,
    Bounce,
    Mend(i16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Res {
    Land,
    Auto,
    Ink,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Win {
    Life(i16),
    Shield(u8),
    Points(i16),
    Zones(i16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    Free,
    Cols,
}

pub struct Unit {
    pub name: &'static str,
    pub cost: u8,
    pub atk: i16,
    pub hp: i16,
    pub kw: u8,
    pub val: i16,
}

pub struct Spell {
    pub name: &'static str,
    pub cost: u8,
    pub fx: Fx,
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub fac: [&'static str; 2],
    pub src: &'static str,
    pub units: &'static [Unit],
    pub spells: &'static [Spell],
    pub deck: [&'static [(u8, u8)]; 2],
    pub res: Res,
    pub win: Win,
    pub face: Face,
    pub hand0: u8,
    pub cap: u8,
    pub board_max: u8,
    pub lands: u8,
    pub big: i16,
    pub max_turns: u16,
    pub twist: u8,
}

#[derive(Clone, Debug)]
struct U {
    card: u8,
    dmg: i16,
    bonus: i16,
    ready: bool,
    fresh: bool,
}

#[derive(Clone, Debug, Default)]
struct Side {
    deck: Vec<u8>,
    hand: Vec<u8>,
    shield: Vec<u8>,
    board: Vec<U>,
    grave: u16,
    life: i16,
    zones: [i16; 3],
    pts: i16,
    emax: u8,
    spent: u8,
    res_used: bool,
    tn: u16,
    /// Cărți trase în pasul curent printr-o vrajă (la sfârșitul mâinii): evaluarea nu le citește,
    /// ca AI-ul să nu vadă vârful propriului pachet.
    blind: u8,
}

#[derive(Clone)]
pub struct Game5 {
    spec: &'static Spec,
    s: [Side; 2],
    turn: Player,
    over: Option<Outcome>,
    viewer: Option<Player>,
    last: String,
}

fn kw_text(kw: u8) -> String {
    let mut v = Vec::new();
    for (f, n) in [
        (HASTE, "Elan"),
        (GUARD, "Gardian"),
        (FLY, "Zburător"),
        (FIRST, "Lovește primul"),
        (ARMOR, "Armură"),
        (VENOM, "Venin"),
    ] {
        if kw & f != 0 {
            v.push(n);
        }
    }
    v.join(", ")
}

fn fx_text(f: Fx) -> String {
    match f {
        Fx::Draw(n) => format!("pescuiești {n}"),
        Fx::Bolt(n) => format!("{n} daune unei unități adverse sau adversarului"),
        Fx::Sweep(n) => format!("{n} daune fiecărei unități adverse"),
        Fx::Boon(n) => format!("binecuvântare {n}"),
        Fx::Buff(n) => format!("unitățile tale +{n} atac până la sfârșitul turei"),
        Fx::Slay => "distruge o unitate adversă".into(),
        Fx::Bounce => "o unitate adversă se întoarce în mâna adversarului".into(),
        Fx::Mend(n) => format!("vindeci {n} daune unei unități a tale"),
    }
}

fn shuf(v: &mut [u8], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

impl Game5 {
    pub fn new(spec: &'static Spec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut s: [Side; 2] = [Side::default(), Side::default()];
        for p in 0..2 {
            let mut d: Vec<u8> = Vec::new();
            for &(c, n) in spec.deck[p] {
                debug_assert!((c as usize) < spec.units.len() + spec.spells.len());
                d.extend(std::iter::repeat_n(c, n as usize));
            }
            if spec.res == Res::Land {
                d.extend(std::iter::repeat_n(SRC, spec.lands as usize));
            }
            shuf(&mut d, &mut rng);
            let sd = &mut s[p];
            if let Win::Shield(n) = spec.win {
                for _ in 0..n {
                    sd.shield.push(d.pop().unwrap());
                }
            }
            let h = spec.hand0 as usize + p;
            for _ in 0..h {
                sd.hand.push(d.pop().unwrap());
            }
            if spec.res == Res::Land && !sd.hand.contains(&SRC) {
                if let Some(k) = d.iter().rposition(|&c| c == SRC) {
                    let c = sd.hand[0];
                    sd.hand[0] = SRC;
                    d[k] = c;
                }
            }
            sd.deck = d;
            match spec.win {
                Win::Life(l) => sd.life = l,
                Win::Zones(h) => sd.zones = [h; 3],
                _ => {}
            }
        }
        let mut g = Game5 {
            spec,
            s,
            turn: Player::One,
            over: None,
            viewer: Some(Player::One),
            last: String::new(),
        };
        g.begin_turn(0);
        g
    }

    /// Pentru teste (audit AI): amestecă ce nu vede jucătorul `1 - hide` (mâna, pachetul și scuturile
    /// lui `hide`) și ordinea propriului pachet al celui la mutare.
    #[cfg(test)]
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let (h, sh) = (self.s[hide].hand.len(), self.s[hide].shield.len());
        let mut pool: Vec<u8> = self.s[hide].hand.clone();
        pool.extend(self.s[hide].shield.iter().copied());
        pool.extend(self.s[hide].deck.iter().copied());
        crate::p3_comun::shuffle_with(&mut pool, seed);
        self.s[hide].hand = pool.drain(..h).collect();
        self.s[hide].shield = pool.drain(..sh).collect();
        self.s[hide].deck = pool;
        crate::p3_comun::shuffle_with(&mut self.s[1 - hide].deck, seed ^ 0x55);
    }

    fn avail(&self, i: usize) -> u8 {
        self.s[i].emax.saturating_sub(self.s[i].spent)
    }

    fn unit(&self, u: &U) -> &'static Unit {
        &self.spec.units[u.card as usize]
    }

    fn draw1(&mut self, i: usize) -> bool {
        match self.s[i].deck.pop() {
            Some(c) => {
                if self.s[i].hand.len() >= HANDMAX {
                    self.s[i].grave += 1;
                } else {
                    self.s[i].hand.push(c);
                }
                true
            }
            None => false,
        }
    }

    fn begin_turn(&mut self, i: usize) {
        let regen = self.spec.twist & T_REGEN != 0;
        {
            let sd = &mut self.s[i];
            sd.tn += 1;
            sd.spent = 0;
            sd.res_used = false;
            for u in sd.board.iter_mut() {
                u.ready = true;
                u.fresh = false;
                u.bonus = 0;
                if regen {
                    u.dmg = 0;
                }
            }
            if self.spec.res == Res::Auto {
                sd.emax = (sd.tn.min(self.spec.cap as u16)) as u8;
            }
        }
        if !(i == 0 && self.s[i].tn == 1) && !self.draw1(i) && self.over.is_none() {
            self.over = Some(Outcome::Win(if i == 0 { Player::Two } else { Player::One }));
        }
    }

    fn zones_alive(&self, i: usize) -> Vec<usize> {
        (0..3).filter(|&z| self.s[i].zones[z] > 0).collect()
    }

    fn can_hit(&self, att_fly: bool, tgt: &U) -> bool {
        let ud = self.unit(tgt);
        if ud.kw & FLY != 0 && !att_fly {
            return false;
        }
        if matches!(self.spec.win, Win::Points(_)) && tgt.ready {
            return false;
        }
        true
    }

    fn can_act(&self, u: &U) -> bool {
        let ud = self.unit(u);
        u.ready && (!u.fresh || ud.kw & HASTE != 0 || self.spec.twist & T_RUSH != 0)
    }

    fn spell_targets(&self, fx: Fx, me: usize) -> Vec<String> {
        let foe = 1 - me;
        let mut v = Vec::new();
        match fx {
            Fx::Bolt(_) => {
                for j in 0..self.s[foe].board.len() {
                    v.push(format!("e{}", j + 1));
                }
                match self.spec.win {
                    Win::Life(_) => v.push("f".into()),
                    Win::Shield(_) => {
                        if !self.s[foe].shield.is_empty() {
                            v.push("f".into())
                        }
                    }
                    Win::Zones(_) => {
                        for z in self.zones_alive(foe) {
                            v.push(format!("z{}", z + 1));
                        }
                    }
                    Win::Points(_) => {}
                }
            }
            Fx::Slay | Fx::Bounce => {
                for j in 0..self.s[foe].board.len() {
                    v.push(format!("e{}", j + 1));
                }
            }
            Fx::Mend(_) => {
                for (j, u) in self.s[me].board.iter().enumerate() {
                    if u.dmg > 0 {
                        v.push(format!("m{}", j + 1));
                    }
                }
            }
            Fx::Sweep(_) => {
                if !self.s[foe].board.is_empty() {
                    v.push(String::new());
                }
            }
            Fx::Buff(_) => {
                if !self.s[me].board.is_empty() {
                    v.push(String::new());
                }
            }
            Fx::Draw(_) => {
                if !self.s[me].deck.is_empty() {
                    v.push(String::new());
                }
            }
            Fx::Boon(_) => v.push(String::new()),
        }
        v
    }

    /// Ținte de atac pentru unitatea `k`: `e<j>`, `f`, `z<n>`.
    fn attack_targets(&self, me: usize, k: usize) -> Vec<String> {
        let foe = 1 - me;
        let sp = self.spec;
        let a = &self.s[me].board[k];
        let fly = self.unit(a).kw & FLY != 0;
        let points = matches!(sp.win, Win::Points(_));
        let cands: Vec<usize> = (0..self.s[foe].board.len())
            .filter(|&j| self.can_hit(fly, &self.s[foe].board[j]))
            .collect();
        let mut t: Vec<String> = Vec::new();
        let mut face_ok = !points;
        if sp.face == Face::Cols && !fly && !points {
            if cands.contains(&k) {
                t.push(format!("e{}", k + 1));
                face_ok = false;
            }
        } else {
            let guards: Vec<usize> = cands
                .iter()
                .copied()
                .filter(|&j| self.unit(&self.s[foe].board[j]).kw & GUARD != 0)
                .collect();
            if !guards.is_empty() && !fly {
                for j in guards {
                    t.push(format!("e{}", j + 1));
                }
                face_ok = false;
            } else {
                for j in cands {
                    t.push(format!("e{}", j + 1));
                }
            }
        }
        if face_ok {
            match sp.win {
                Win::Zones(_) => {
                    for z in self.zones_alive(foe) {
                        t.push(format!("z{}", z + 1));
                    }
                }
                _ => t.push("f".into()),
            }
        }
        t
    }

    fn kill_dead(&mut self, i: usize) -> usize {
        let sp = self.spec;
        let before = self.s[i].board.len();
        self.s[i]
            .board
            .retain(|u| u.dmg < sp.units[u.card as usize].hp);
        let n = before - self.s[i].board.len();
        self.s[i].grave += n as u16;
        n
    }

    fn win_for(&mut self, i: usize) {
        if self.over.is_none() {
            self.over = Some(Outcome::Win(if i == 0 { Player::One } else { Player::Two }));
        }
    }

    /// Lovitură directă a lui `me` asupra adversarului. `z`: zona țintită. `spell`: din vrajă.
    fn face_hit(&mut self, me: usize, e: i16, z: usize, spell: bool) {
        let foe = 1 - me;
        let sp = self.spec;
        let bleed = !spell && sp.twist & T_BLEED != 0;
        match sp.win {
            Win::Life(_) => {
                self.s[foe].life -= e + bleed as i16;
                if self.s[foe].life <= 0 {
                    self.win_for(me);
                }
            }
            Win::Zones(_) => {
                self.s[foe].zones[z] -= e + bleed as i16;
                if self.zones_alive(foe).len() <= 1 {
                    self.win_for(me);
                }
            }
            Win::Shield(_) => {
                let big = sp.big - if bleed { 1 } else { 0 };
                let n = if e >= big { 2 } else { 1 };
                if self.s[foe].shield.is_empty() {
                    if !spell {
                        self.win_for(me);
                    }
                    return;
                }
                for _ in 0..n {
                    if let Some(c) = self.s[foe].shield.pop() {
                        if self.s[foe].hand.len() >= HANDMAX {
                            self.s[foe].grave += 1;
                        } else {
                            self.s[foe].hand.push(c);
                        }
                    }
                }
            }
            Win::Points(_) => {}
        }
    }

    fn combat(&mut self, me: usize, a: usize, d: usize) {
        let foe = 1 - me;
        let (ua, ud) = (&self.s[me].board[a], &self.s[foe].board[d]);
        let (ca, cd) = (self.unit(ua), self.unit(ud));
        let (atk_a, atk_d) = (ca.atk + ua.bonus, cd.atk + ud.bonus);
        let hit_d = (atk_a - if cd.kw & ARMOR != 0 { 1 } else { 0 }).max(0);
        let hit_a = (atk_d - if ca.kw & ARMOR != 0 { 1 } else { 0 }).max(0);
        let (fa, fd) = (ca.kw & FIRST != 0, cd.kw & FIRST != 0);
        let (va, vd) = (ca.kw & VENOM != 0, cd.kw & VENOM != 0);
        let kill_d = |dm: i16, v: bool, h: i16| h > 0 && (v || dm >= cd.hp);
        let kill_a = |dm: i16, v: bool, h: i16| h > 0 && (v || dm >= ca.hp);
        let mut a_hurt = true;
        let mut d_hurt = true;
        if fa && !fd {
            if kill_d(ud.dmg + hit_d, va, hit_d) {
                a_hurt = false;
            }
        } else if fd && !fa && kill_a(ua.dmg + hit_a, vd, hit_a) {
            d_hurt = false;
        }
        let (mut da, mut dd) = (0i16, 0i16);
        let (mut a_dies, mut d_dies) = (false, false);
        if d_hurt {
            dd = hit_d;
            d_dies = kill_d(ud.dmg + hit_d, va, hit_d);
        }
        if a_hurt {
            da = hit_a;
            a_dies = kill_a(ua.dmg + hit_a, vd, hit_a);
        }
        self.s[foe].board[d].dmg += dd;
        self.s[me].board[a].dmg += da;
        if d_dies {
            self.s[foe].board[d].dmg = 99;
        }
        if a_dies {
            self.s[me].board[a].dmg = 99;
        }
        self.s[me].board[a].ready = false;
        let killed = self.kill_dead(foe);
        self.kill_dead(me);
        if killed > 0 && self.spec.twist & T_TROPHY != 0 {
            let before = self.s[me].hand.len();
            self.draw1(me);
            let got = self.s[me].hand.len().saturating_sub(before) as u8;
            self.s[me].blind = self.s[me].blind.saturating_add(got);
        }
    }

    fn cast(&mut self, me: usize, fx: Fx, tgt: &str) {
        let foe = 1 - me;
        let idx = |s: &str| s[1..].parse::<usize>().unwrap() - 1;
        match fx {
            Fx::Draw(n) => {
                let before = self.s[me].hand.len();
                for _ in 0..n {
                    self.draw1(me);
                }
                let got = self.s[me].hand.len().saturating_sub(before) as u8;
                self.s[me].blind = self.s[me].blind.saturating_add(got);
            }
            Fx::Bolt(n) => {
                if tgt.starts_with('e') {
                    let j = idx(tgt);
                    self.s[foe].board[j].dmg += n;
                    self.kill_dead(foe);
                } else if tgt == "f" {
                    self.face_hit(me, n, 0, true);
                } else {
                    self.face_hit(me, n, idx(tgt), true);
                }
            }
            Fx::Sweep(n) => {
                for u in self.s[foe].board.iter_mut() {
                    u.dmg += n;
                }
                self.kill_dead(foe);
            }
            Fx::Boon(n) => match self.spec.win {
                Win::Life(_) => self.s[me].life += n,
                Win::Shield(_) => {
                    for _ in 0..(n / 2).max(1) {
                        if let Some(c) = self.s[me].deck.pop() {
                            self.s[me].shield.push(c);
                        }
                    }
                }
                Win::Zones(h) => {
                    let mut best: Option<usize> = None;
                    for z in 0..3 {
                        if self.s[me].zones[z] > 0
                            && best.is_none_or(|b| self.s[me].zones[z] < self.s[me].zones[b])
                        {
                            best = Some(z);
                        }
                    }
                    if let Some(z) = best {
                        self.s[me].zones[z] = (self.s[me].zones[z] + n).min(h);
                    }
                }
                Win::Points(_) => self.s[me].pts += (n / 2).max(1),
            },
            Fx::Buff(n) => {
                for u in self.s[me].board.iter_mut() {
                    u.bonus += n;
                }
            }
            Fx::Slay => {
                let j = idx(tgt);
                self.s[foe].board[j].dmg = 99;
                self.kill_dead(foe);
            }
            Fx::Bounce => {
                let j = idx(tgt);
                let u = self.s[foe].board.remove(j);
                if self.s[foe].hand.len() >= HANDMAX {
                    self.s[foe].grave += 1;
                } else {
                    self.s[foe].hand.push(u.card);
                }
            }
            Fx::Mend(n) => {
                let j = idx(tgt);
                let u = &mut self.s[me].board[j];
                u.dmg = (u.dmg - n).max(0);
            }
        }
        if let Win::Points(n) = self.spec.win {
            if self.s[me].pts >= n {
                self.win_for(me);
            }
        }
    }

    fn end_turn(&mut self) {
        let me = self.turn.idx();
        for u in self.s[me].board.iter_mut() {
            u.bonus = 0;
        }
        if me == 1 && self.s[1].tn >= self.spec.max_turns {
            let (a, b) = (self.score(0), self.score(1));
            self.over = Some(by_score(a, b));
            return;
        }
        self.turn = self.turn.other();
        self.begin_turn(1 - me);
    }

    fn score(&self, i: usize) -> i32 {
        let s = &self.s[i];
        let main = match self.spec.win {
            Win::Life(_) => s.life as i32,
            Win::Shield(_) => s.shield.len() as i32,
            Win::Points(_) => s.pts as i32,
            Win::Zones(_) => s.zones.iter().map(|&z| z.max(0) as i32).sum(),
        };
        let board: i32 = s
            .board
            .iter()
            .map(|u| {
                let d = self.unit(u);
                (d.atk + d.hp) as i32
            })
            .sum();
        main * 1000 + board
    }

    fn card_txt(&self, c: u8) -> String {
        let sp = self.spec;
        if c == SRC {
            return format!("{} (sursă)", sp.src);
        }
        if (c as usize) < sp.units.len() {
            let u = &sp.units[c as usize];
            let k = kw_text(u.kw);
            let v = if matches!(sp.win, Win::Points(_)) {
                format!(", misiune {}", u.val)
            } else {
                String::new()
            };
            format!(
                "{} (unitate cost {}, {}/{}{}{})",
                u.name,
                u.cost,
                u.atk,
                u.hp,
                v,
                if k.is_empty() {
                    String::new()
                } else {
                    format!(", {k}")
                }
            )
        } else {
            let s = &sp.spells[c as usize - sp.units.len()];
            format!("{} (vrajă cost {}: {})", s.name, s.cost, fx_text(s.fx))
        }
    }

    fn unit_sc(&self, u: &U) -> i32 {
        let d = self.unit(u);
        let hp = (d.hp - u.dmg).max(0) as i32;
        let mut v = (d.atk + u.bonus) as i32 * 3 + hp * 2 + d.kw.count_ones() as i32 * 2;
        if matches!(self.spec.win, Win::Points(_)) {
            v += d.val as i32 * 3;
        }
        v
    }

    fn sc(&self, i: usize, detail: bool) -> i32 {
        let s = &self.s[i];
        let mut v: i32 = s.board.iter().map(|u| self.unit_sc(u)).sum();
        if detail {
            // cărțile trase chiar acum printr-o vrajă nu se citesc (valoare medie fixă)
            let seen = s.hand.len().saturating_sub(s.blind as usize);
            v += 5 * (s.hand.len() - seen) as i32;
            for &c in &s.hand[..seen] {
                v += if c == SRC {
                    3
                } else if (c as usize) < self.spec.units.len() {
                    2 + self.spec.units[c as usize].cost as i32
                } else {
                    2 + self.spec.spells[c as usize - self.spec.units.len()].cost as i32
                };
            }
        } else {
            v += 4 * s.hand.len() as i32;
        }
        v += match self.spec.win {
            Win::Life(_) => s.life as i32 * 3,
            Win::Shield(_) => s.shield.len() as i32 * 14,
            Win::Points(_) => s.pts as i32 * 20,
            Win::Zones(_) => {
                s.zones.iter().map(|&z| z.max(0) as i32).sum::<i32>() * 3
                    + s.zones.iter().filter(|&&z| z <= 0).count() as i32 * -30
            }
        };
        if self.spec.res != Res::Auto {
            v += s.emax as i32 * 5;
        }
        v
    }
}

impl Mini for Game5 {
    fn side(&self) -> Player {
        self.turn
    }

    fn moves(&self) -> Vec<String> {
        let me = self.turn.idx();
        let sd = &self.s[me];
        let sp = self.spec;
        let av = self.avail(me);
        let mut v: Vec<String> = Vec::new();
        let mut seen: Vec<u8> = Vec::new();
        for (i, &c) in sd.hand.iter().enumerate() {
            if seen.contains(&c) {
                continue;
            }
            seen.push(c);
            let can_res = !sd.res_used && sd.emax < sp.cap;
            if c == SRC {
                if sp.res == Res::Land && can_res {
                    v.push(format!("sursa {}", i + 1));
                }
                continue;
            }
            if sp.res == Res::Ink && can_res {
                v.push(format!("sursa {}", i + 1));
            }
            if (c as usize) < sp.units.len() {
                let u = &sp.units[c as usize];
                if u.cost <= av && sd.board.len() < sp.board_max as usize {
                    v.push(format!("joc {}", i + 1));
                }
            } else {
                let s = &sp.spells[c as usize - sp.units.len()];
                if s.cost <= av {
                    for t in self.spell_targets(s.fx, me) {
                        if t.is_empty() {
                            v.push(format!("joc {}", i + 1));
                        } else {
                            v.push(format!("joc {} {}", i + 1, t));
                        }
                    }
                }
            }
        }
        for k in 0..sd.board.len() {
            if !self.can_act(&sd.board[k]) {
                continue;
            }
            if matches!(sp.win, Win::Points(_)) {
                v.push(format!("miss {}", k + 1));
            }
            for t in self.attack_targets(me, k) {
                if t == "f" {
                    v.push(format!("atac {}", k + 1));
                } else {
                    v.push(format!("atac {} {}", k + 1, t));
                }
            }
        }
        v.push("fin".into());
        v
    }

    fn step(&mut self, m: &str) {
        let me = self.turn.idx();
        let sp = self.spec;
        self.s[0].blind = 0;
        self.s[1].blind = 0;
        let parts: Vec<&str> = m.split_whitespace().collect();
        match parts[0] {
            "fin" => self.end_turn(),
            "sursa" => {
                let i: usize = parts[1].parse::<usize>().unwrap() - 1;
                self.s[me].hand.remove(i);
                self.s[me].emax += 1;
                self.s[me].res_used = true;
            }
            "joc" => {
                let i: usize = parts[1].parse::<usize>().unwrap() - 1;
                let c = self.s[me].hand.remove(i);
                if (c as usize) < sp.units.len() {
                    let u = &sp.units[c as usize];
                    self.s[me].spent += u.cost;
                    let fresh = !(u.kw & HASTE != 0 || sp.twist & T_RUSH != 0);
                    self.s[me].board.push(U {
                        card: c,
                        dmg: 0,
                        bonus: 0,
                        ready: true,
                        fresh,
                    });
                } else {
                    let s = &sp.spells[c as usize - sp.units.len()];
                    self.s[me].spent += s.cost;
                    self.s[me].grave += 1;
                    let t = parts.get(2).copied().unwrap_or("");
                    self.cast(me, s.fx, t);
                }
            }
            "miss" => {
                let k: usize = parts[1].parse::<usize>().unwrap() - 1;
                let val = self.unit(&self.s[me].board[k]).val;
                self.s[me].board[k].ready = false;
                self.s[me].pts += val;
                if let Win::Points(n) = sp.win {
                    if self.s[me].pts >= n {
                        self.win_for(me);
                    }
                }
            }
            "atac" => {
                let k: usize = parts[1].parse::<usize>().unwrap() - 1;
                match parts.get(2) {
                    Some(t) if t.starts_with('e') => {
                        let j: usize = t[1..].parse::<usize>().unwrap() - 1;
                        self.combat(me, k, j);
                    }
                    other => {
                        let z = other
                            .map(|t| t[1..].parse::<usize>().unwrap() - 1)
                            .unwrap_or(0);
                        let a = &self.s[me].board[k];
                        let e = self.unit(a).atk + a.bonus;
                        self.s[me].board[k].ready = false;
                        self.face_hit(me, e, z, false);
                    }
                }
            }
            _ => {}
        }
        self.last = format!("J{}: {}", me + 1, m);
    }

    fn result(&self) -> Option<Outcome> {
        self.over
    }

    fn eval(&self, p: Player) -> i32 {
        let i = p.idx();
        self.sc(i, true) - self.sc(1 - i, false)
    }

    fn show(&self) -> String {
        let sp = self.spec;
        let v = self.viewer.unwrap_or(self.turn);
        let mut o = format!(
            "{} — tura {}/{} — la mutare: J{}\n",
            sp.title,
            self.s[self.turn.idx()].tn,
            sp.max_turns,
            self.turn.idx() + 1
        );
        for &p in &[1usize, 0] {
            let s = &self.s[p];
            let bars = match sp.win {
                Win::Life(_) => format!("viață {}", s.life),
                Win::Shield(_) => format!("scuturi {}", s.shield.len()),
                Win::Points(n) => format!("puncte {}/{}", s.pts, n),
                Win::Zones(_) => format!("zone {}/{}/{}", s.zones[0], s.zones[1], s.zones[2]),
            };
            let en = if sp.res == Res::Auto {
                "energie"
            } else {
                sp.src
            };
            o.push_str(&format!(
                "J{} ({}): {} | {} {}/{} | mână {} | pachet {} | cimitir {}\n",
                p + 1,
                sp.fac[p],
                bars,
                en,
                self.avail(p),
                s.emax,
                s.hand.len(),
                s.deck.len(),
                s.grave
            ));
            for (k, u) in s.board.iter().enumerate() {
                let d = self.unit(u);
                let kw = kw_text(d.kw);
                o.push_str(&format!(
                    "  {}) {} {}/{}{}{}{}{}\n",
                    k + 1,
                    d.name,
                    d.atk + u.bonus,
                    d.hp,
                    if u.dmg > 0 {
                        format!(" (daune {})", u.dmg)
                    } else {
                        String::new()
                    },
                    if kw.is_empty() {
                        String::new()
                    } else {
                        format!(" [{kw}]")
                    },
                    if u.ready { " gata" } else { " obosit" },
                    if u.fresh && u.ready { " (nou)" } else { "" }
                ));
            }
        }
        if !self.last.is_empty() {
            o.push_str(&format!("Ultima mutare: {}\n", self.last));
        }
        let h = &self.s[v.idx()].hand;
        let txt: Vec<String> = h
            .iter()
            .enumerate()
            .map(|(i, &c)| format!("{}) {}", i + 1, self.card_txt(c)))
            .collect();
        o.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            if txt.is_empty() {
                "—".to_string()
            } else {
                txt.join("; ")
            }
        ));
        o.push_str("Mutări: joc i [țintă] | sursa i | atac k [eN|zN] | miss k | fin\n");
        o
    }

    fn depth(&self) -> u32 {
        0
    }

    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
pub static TEST_SPEC: Spec = Spec {
    id: "t5_test",
    title: "Test",
    fac: ["A", "B"],
    src: "cristal",
    units: &[
        Unit {
            name: "u0",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "u1",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: HASTE,
            val: 1,
        },
        Unit {
            name: "u2",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "u3",
            cost: 4,
            atk: 3,
            hp: 4,
            kw: FLY,
            val: 2,
        },
    ],
    spells: &[
        Spell {
            name: "s0",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "s1",
            cost: 2,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[(0, 6), (1, 5), (2, 5), (3, 4), (4, 4), (5, 4)],
        &[(0, 5), (1, 5), (2, 6), (3, 5), (4, 5), (5, 4)],
    ],
    res: Res::Land,
    win: Win::Life(20),
    face: Face::Free,
    hand0: 5,
    cap: 7,
    board_max: 5,
    lands: 10,
    big: 4,
    max_turns: 16,
    twist: 0,
};

#[cfg(test)]
mod t {
    use super::*;
    use crate::brand_util::B;
    use polytabletop_core::{GameObj, Player};

    fn all_modes() -> Vec<&'static Spec> {
        fn leak(f: impl Fn(&mut Spec)) -> &'static Spec {
            let mut s = Spec {
                ..clone_spec(&TEST_SPEC)
            };
            f(&mut s);
            Box::leak(Box::new(s))
        }
        vec![
            &TEST_SPEC,
            leak(|s| {
                s.res = Res::Auto;
                s.win = Win::Shield(5);
                s.face = Face::Cols;
                s.twist = T_TROPHY | T_BLEED;
            }),
            leak(|s| {
                s.res = Res::Ink;
                s.win = Win::Points(8);
                s.twist = T_REGEN;
            }),
            leak(|s| {
                s.res = Res::Auto;
                s.win = Win::Zones(8);
                s.twist = T_RUSH;
            }),
        ]
    }

    fn clone_spec(s: &Spec) -> Spec {
        Spec {
            id: s.id,
            title: s.title,
            fac: s.fac,
            src: s.src,
            units: s.units,
            spells: s.spells,
            deck: s.deck,
            res: s.res,
            win: s.win,
            face: s.face,
            hand0: s.hand0,
            cap: s.cap,
            board_max: s.board_max,
            lands: s.lands,
            big: s.big,
            max_turns: s.max_turns,
            twist: s.twist,
        }
    }

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        for spec in all_modes() {
            crate::p3_comun::ai_blind_check(
                spec.id,
                |s| B(Game5::new(spec, s)),
                |g, me, s| g.0.scramble(1 - me.idx(), s),
                8,
            );
        }
    }

    #[test]
    fn engine_all_modes_play_and_ai_does_not_peek() {
        for spec in all_modes() {
            let mk = |s: u64| Game5::new(spec, s);
            for seed in 1..=4u64 {
                let mut g = B(mk(seed));
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while GameObj::outcome(&g).is_none() {
                    let lm = GameObj::legal_moves(&g);
                    assert!(!lm.is_empty());
                    let m = lm[rng.below(lm.len())].clone();
                    GameObj::play(&mut g, &m).unwrap();
                    n += 1;
                    assert!(n < 6000, "partida nu se termină");
                }
            }
            // AI: aceeași alegere indiferent de mâna/pachetul/scuturile ascunse ale adversarului
            let mut g = B(mk(3));
            let mut rng = Rng::new(11);
            for _ in 0..30 {
                if GameObj::outcome(&g).is_some() {
                    break;
                }
                let lm = GameObj::legal_moves(&g);
                GameObj::play(&mut g, &lm[rng.below(lm.len())].clone()).unwrap();
            }
            if GameObj::outcome(&g).is_none() {
                let mut h = g.clone();
                let foe = 1 - h.0.turn.idx();
                h.0.s[foe].hand.reverse();
                h.0.s[foe].deck.reverse();
                h.0.s[foe].shield.reverse();
                let a = GameObj::ai_move(&g, 5).unwrap();
                let b = GameObj::ai_move(&h, 5).unwrap();
                assert_eq!(a, b);
                let _ = Player::One;
            }
        }
    }
}
