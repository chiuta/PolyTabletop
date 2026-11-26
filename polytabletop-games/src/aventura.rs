//! Aventura — motor generic de aventură cooperativă de rol (1–2 jucători contra „Maestrului-AI”).
//!
//! Un singur motor, șaisprezece „pachete de conținut” proprii (vezi `aventura_pachete.rs`): fiecare are
//! lume, nume, arhetipuri, monștri, tabele de evenimente și resursă specifică, și folosește
//! **mecanica de test** a unui stil de rol cunoscut (d20 peste prag, d100 sub abilitate, grămezi de d10/d6,
//! cel mai mare d6, 3d20 față de trei atribute, 2d6 față de țintă, d10 exploziv). Mecanicile sunt generice
//! (zaruri + prag), nu regulile vreunui editor; tot conținutul este original.
//!
//! * Aventura = 7–8 scene într-un graf mic orientat (explorare, test, luptă, odihnă, boss final).
//! * Fiecare jucător conduce un erou (J1 = eroul 1, J2 = eroul 2; partenerul AI joacă J2). Mai întâi se
//!   alege arhetipul (`alege N`), apoi eroii se alternează câte o acțiune.
//! * **Maestrul-AI nu trișează**: dușmanii urmează reguli fixe (ținte alese după regulă, nu după zaruri
//!   viitoare), zarurile sunt aruncate în mutări forțate publice (`zaruri`, `eveniment`, `maestrul`) și
//!   afișate în jurnal. AI-ul eroilor alege acțiunea fără să vadă zarurile care urmează.
//! * Rezultat: boss învins = „Câștigă Jucătorul 1” (victorie comună); eroi doborâți / timp epuizat = remiză
//!   (nu există adversar uman). Încheierea este mutarea forțată `sfârșit`.
//!
//! Informație: totul este public (zaruri, dușmani, resurse), deci nu există informație ascunsă.
#![allow(clippy::needless_range_loop)]

use crate::aventura_pachete::{pack_by_id, PACKS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::cell::RefCell;
use std::collections::HashMap;

/// Mecanica de test (stilul de zaruri) a unui pachet.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Mech {
    /// d20 + modificator ≥ prag; 20 / 1 naturale critice.
    D20,
    /// ca `D20`, cu grade de succes (±10 = critic).
    D20Deg,
    /// d100 sub abilitate, trepte de dificultate (normal / greu / extrem).
    D100,
    /// d100 sub caracteristică, grade de succes = zecile de marjă.
    D100Deg,
    /// d20 sub abilitate.
    D20Under,
    /// grămadă de d10: 6+ succes, 1 anulează.
    Pool10,
    /// grămadă de d6: 5–6 lovituri, jumătate de 1 = complicație.
    Pool6,
    /// cel mai mare d6 dintr-o grămadă: 6 curat, 4–5 cu complicație, 1–3 eșec.
    Highest6,
    /// 3d20 față de trei atribute, diferențele se plătesc din punctele de abilitate.
    Triple20,
    /// 2d6 + rating față de țintă.
    TwoD6,
    /// d10 exploziv + rating față de prag.
    D10Exp,
}

impl Mech {
    pub fn desc(self) -> &'static str {
        match self {
            Mech::D20 => "d20 + modificator ≥ prag (20 natural = critic, 1 = ratare gravă)",
            Mech::D20Deg => "d20 + modificator ≥ prag, cu grade: +10 critic, −10 ratare gravă",
            Mech::D100 => "d100 sub abilitate; treptele greu/extrem înjumătățesc/împart la cinci",
            Mech::D100Deg => "d100 sub caracteristică; gradele de succes = zecile de marjă",
            Mech::D20Under => "d20 sub valoarea abilității (1 = critic, 20 = ratare gravă)",
            Mech::Pool10 => "grămadă de d10: 6+ = succes, fiecare 1 anulează un succes",
            Mech::Pool6 => "grămadă de d6: 5–6 = lovitură; jumătate de 1 = complicație",
            Mech::Highest6 => "cel mai mare d6: 6 curat, 4–5 cu preț, 1–3 eșec; două 6 = critic",
            Mech::Triple20 => {
                "3d20 față de trei atribute; excesul se plătește din punctele de abilitate"
            }
            Mech::TwoD6 => "2d6 + rating ≥ țintă (12 natural = critic, 2 = ratare gravă)",
            Mech::D10Exp => "d10 exploziv (10 adaugă, 1 scade) + rating ≥ prag",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Grade {
    CritFail,
    Fail,
    Partial,
    Success,
    Crit,
}

pub struct Roll {
    pub grade: Grade,
    pub text: String,
}

fn d(rng: &mut Rng, n: usize) -> i32 {
    rng.below(n) as i32 + 1
}

/// Un test: `lvl` = nivel abstract 1..=6 (rating-ul se scalează după mecanică), `diff` 0..=3
/// (ușor / normal / greu / extrem).
pub fn check(mech: Mech, lvl: u8, diff: u8, rng: &mut Rng) -> Roll {
    let l = i32::from(lvl);
    let df = usize::from(diff.min(3));
    let (grade, text) = match mech {
        Mech::D20 | Mech::D20Deg => {
            let dc = [8, 11, 14, 18][df];
            let nat = d(rng, 20);
            let tot = nat + l + 1;
            let m = tot - dc;
            let deg = mech == Mech::D20Deg;
            let g = if nat == 20 || (deg && m >= 10) {
                Grade::Crit
            } else if nat == 1 || (deg && m <= -10) {
                Grade::CritFail
            } else if m >= 0 {
                Grade::Success
            } else {
                Grade::Fail
            };
            (g, format!("d20 {nat} + {} = {tot} față de CD {dc}", l + 1))
        }
        Mech::D100 => {
            let r = 30 + 10 * l;
            let t = [(r + 20).min(95), r, r / 2, r / 5][df];
            let n = d(rng, 100);
            let g = if n == 1 || (n <= t && n <= r / 5) {
                Grade::Crit
            } else if n >= 96 && n > t {
                Grade::CritFail
            } else if n <= t {
                Grade::Success
            } else {
                Grade::Fail
            };
            (g, format!("d100 {n} față de {t}"))
        }
        Mech::D100Deg => {
            let r = 30 + 10 * l;
            let t = (r + [20, 0, -20, -40][df]).clamp(5, 95);
            let n = d(rng, 100);
            let deg = ((t / 10) - (n / 10)).abs();
            let g = if n <= t {
                if deg >= 3 {
                    Grade::Crit
                } else if deg == 0 {
                    Grade::Partial
                } else {
                    Grade::Success
                }
            } else if deg >= 3 {
                Grade::CritFail
            } else {
                Grade::Fail
            };
            (g, format!("d100 {n} față de {t} ({deg} grade)"))
        }
        Mech::D20Under => {
            let r = 6 + 2 * l;
            let t = [r + 4, r, r / 2, r / 4][df];
            let n = d(rng, 20);
            let g = if n == 1 {
                Grade::Crit
            } else if n == 20 {
                Grade::CritFail
            } else if n <= t {
                Grade::Success
            } else {
                Grade::Fail
            };
            (g, format!("d20 {n} sub {t}"))
        }
        Mech::Pool10 => {
            let n = (l + 2) as usize;
            let need = df as i32 + 1;
            let rolls: Vec<i32> = (0..n).map(|_| d(rng, 10)).collect();
            let succ = rolls.iter().filter(|x| **x >= 6).count() as i32;
            let ones = rolls.iter().filter(|x| **x == 1).count() as i32;
            let net = (succ - ones).max(0);
            let g = if net >= need + 2 {
                Grade::Crit
            } else if net >= need {
                Grade::Success
            } else if net > 0 {
                Grade::Partial
            } else if succ == 0 && ones > 0 {
                Grade::CritFail
            } else {
                Grade::Fail
            };
            (
                g,
                format!("{n}d10 {rolls:?}: {net} succese net (nevoie {need})"),
            )
        }
        Mech::Pool6 => {
            let n = (l + 3) as usize;
            let need = df as i32 + 1;
            let rolls: Vec<i32> = (0..n).map(|_| d(rng, 6)).collect();
            let hits = rolls.iter().filter(|x| **x >= 5).count() as i32;
            let ones = rolls.iter().filter(|x| **x == 1).count();
            let glitch = ones * 2 >= n;
            let g = if hits >= need + 2 && !glitch {
                Grade::Crit
            } else if hits >= need {
                if glitch {
                    Grade::Partial
                } else {
                    Grade::Success
                }
            } else if hits == 0 && glitch {
                Grade::CritFail
            } else {
                Grade::Fail
            };
            (
                g,
                format!("{n}d6 {rolls:?}: {hits} lovituri (nevoie {need})"),
            )
        }
        Mech::Highest6 => {
            let n = ((l + 3) / 2 + [1, 0, -1, -2][df]).max(0);
            let (best, rolls): (i32, Vec<i32>) = if n == 0 {
                let r = vec![d(rng, 6), d(rng, 6)];
                (*r.iter().min().unwrap(), r)
            } else {
                let r: Vec<i32> = (0..n).map(|_| d(rng, 6)).collect();
                (*r.iter().max().unwrap(), r)
            };
            let sixes = rolls.iter().filter(|x| **x == 6).count();
            let g = if n > 0 && sixes >= 2 {
                Grade::Crit
            } else if best == 6 {
                Grade::Success
            } else if best >= 4 {
                Grade::Partial
            } else if best == 1 && n == 0 {
                Grade::CritFail
            } else {
                Grade::Fail
            };
            (g, format!("{n} zaruri d6 {rolls:?}: cel mai bun {best}"))
        }
        Mech::Triple20 => {
            let fp = l + 3;
            let attr = 10 + l / 2 + [3, 0, -3, -6][df];
            let r = [d(rng, 20), d(rng, 20), d(rng, 20)];
            let short: i32 = r.iter().map(|x| (x - attr).max(0)).sum();
            let ones = r.iter().filter(|x| **x == 1).count();
            let twenties = r.iter().filter(|x| **x == 20).count();
            let g = if ones >= 2 {
                Grade::Crit
            } else if twenties >= 2 {
                Grade::CritFail
            } else if short > fp {
                Grade::Fail
            } else if short == fp && short > 0 {
                Grade::Partial
            } else if fp - short >= 6 {
                Grade::Crit
            } else {
                Grade::Success
            };
            (
                g,
                format!("3d20 {r:?} față de {attr} (puncte {fp}, exces {short})"),
            )
        }
        Mech::TwoD6 => {
            let target = [8, 10, 12, 14][df];
            let (a, b) = (d(rng, 6), d(rng, 6));
            let tot = a + b + l;
            let g = if a + b == 12 {
                Grade::Crit
            } else if a + b == 2 {
                Grade::CritFail
            } else if tot >= target {
                Grade::Success
            } else {
                Grade::Fail
            };
            (g, format!("2d6 {a}+{b} + {l} = {tot} față de {target}"))
        }
        Mech::D10Exp => {
            let dc = [9, 12, 14, 16][df];
            let first = d(rng, 10);
            let mut roll = first;
            if first == 10 {
                roll += d(rng, 10);
            } else if first == 1 {
                roll -= d(rng, 10);
            }
            let tot = roll + l + 5;
            let g = if tot >= dc {
                if first == 10 {
                    Grade::Crit
                } else {
                    Grade::Success
                }
            } else if first == 1 {
                Grade::CritFail
            } else {
                Grade::Fail
            };
            (
                g,
                format!("d10 {first} → {roll} + {} = {tot} față de {dc}", l + 5),
            )
        }
    };
    Roll { grade, text }
}

type ProbCache = HashMap<(Mech, u8, u8), (i32, i32, i32)>;

thread_local! {
    static PCACHE: RefCell<ProbCache> = RefCell::new(HashMap::new());
}

/// Probabilități (în miimi) pentru un test: (succes sau mai bine, parțial, critic).
/// Calculate cu un generator propriu, fix — nu ating zarurile jocului.
pub fn probs(mech: Mech, lvl: u8, diff: u8) -> (i32, i32, i32) {
    let key = (mech, lvl, diff);
    if let Some(v) = PCACHE.with(|c| c.borrow().get(&key).copied()) {
        return v;
    }
    let mut rng = Rng::new(0x5EED ^ (mech as u64 * 977 + u64::from(lvl) * 31 + u64::from(diff)));
    let n = 600;
    let (mut s, mut p, mut c) = (0, 0, 0);
    for _ in 0..n {
        match check(mech, lvl, diff, &mut rng).grade {
            Grade::Crit => {
                s += 1;
                c += 1;
            }
            Grade::Success => s += 1,
            Grade::Partial => p += 1,
            _ => {}
        }
    }
    let v = (s * 1000 / n, p * 1000 / n, c * 1000 / n);
    PCACHE.with(|cc| cc.borrow_mut().insert(key, v));
    v
}

// ───────────────────────── conținut ─────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResMode {
    /// Rezervă care se cheltuiește pentru a reface un zar ratat.
    Pool,
    /// Presiune care crește (refacerea costă o unitate); la maxim eroul se frânge.
    Strain,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sp {
    /// Lovitură dublă (o țintă, test cu o treaptă mai greu).
    Strike,
    /// Vindecă 4 PV unui aliat.
    Heal,
    /// Toți dușmanii lovesc cu o treaptă mai greu în runda asta.
    Ward,
}

pub struct Arch {
    pub name: &'static str,
    pub hp: i32,
    pub skills: [u8; 3],
    pub dmg: i32,
    pub def: u8,
    pub sp: Sp,
    pub sp_name: &'static str,
}

pub const TERROR: u8 = 1;
pub const BOSS: u8 = 2;
pub const TOUGH: u8 = 4;

pub struct Foe {
    pub name: &'static str,
    pub hp: i32,
    pub lvl: u8,
    pub dmg: i32,
    pub def: u8,
    pub flags: u8,
}

pub enum SK {
    Explore,
    /// abilitate (0..3), dificultate, succese necesare, daune la ratare.
    Test(u8, u8, u8, i32),
    Fight(&'static [u8]),
    Rest,
}

pub struct Scene {
    pub name: &'static str,
    pub text: &'static str,
    pub kind: SK,
    pub exits: &'static [u8],
}

pub struct Pack {
    pub id: &'static str,
    pub title: &'static str,
    pub style: &'static str,
    pub intro: &'static str,
    pub mech: Mech,
    pub res_name: &'static str,
    pub res_mode: ResMode,
    pub res_max: i32,
    pub skills: [&'static str; 3],
    pub potion: &'static str,
    pub boss_move: &'static str,
    pub arch: [Arch; 3],
    pub foes: &'static [Foe],
    pub scenes: &'static [Scene],
    /// tabel d6: (text, cod efect): 0 nimic, 1 poțiune, 2 vindecă 3, 3 cursă, 4 presiune, 5 ambuscadă, 6 răsfrângere.
    pub events: [(&'static str, u8); 6],
}

// ───────────────────────── stare ─────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Go(u8),
    Attack(u8),
    Guard,
    Special(u8),
    Potion(u8),
    Search,
    Try,
    Rest,
    Pass,
    Roll,
    Accept,
    Reroll,
    Event,
    Foes,
    Finish,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Act,
    Roll,
    Reroll,
    Event,
    Foes,
    Finish,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PK {
    Attack(usize, bool),
    Try,
    Search,
}

#[derive(Clone, Copy, Debug)]
struct Pend {
    kind: PK,
    who: usize,
    rerolled: bool,
    grade: Grade,
}

#[derive(Clone, Debug)]
struct Hero {
    arch: usize,
    hp: i32,
    max: i32,
    res: i32,
    guard: bool,
    used: bool,
}

const TICK_CAP: u32 = 1500;
const ROUND_CAP: u32 = 24;
const POTION_HEAL: i32 = 5;
const REST_HEAL: i32 = 6;
const MAX_ATTEMPTS: u8 = 4;

#[derive(Clone)]
pub struct Aventura {
    pack: &'static Pack,
    rng: Rng,
    heroes: [Hero; 2],
    scene: usize,
    visited: Vec<bool>,
    done: bool,
    foes: Vec<(u8, i32)>,
    progress: u8,
    attempts: u8,
    searched: bool,
    rested: bool,
    potions: u8,
    side: usize,
    acted: [bool; 2],
    passed: bool,
    ward: bool,
    round: u32,
    ticks: u32,
    phase: Phase,
    pend: Option<Pend>,
    pending_end: Option<bool>,
    over: bool,
    won: bool,
    picks: usize,
    log: Vec<String>,
}

impl Default for Aventura {
    fn default() -> Self {
        Self::new(PACKS[0].id)
    }
}

impl Aventura {
    pub fn new(id: &str) -> Self {
        Self::with_seed(id, Rng::from_clock().next_u64())
    }

    pub fn with_seed(id: &str, seed: u64) -> Self {
        let pack = pack_by_id(id).unwrap_or(&PACKS[0]);
        let hero = Hero {
            arch: 0,
            hp: 1,
            max: 1,
            res: 0,
            guard: false,
            used: false,
        };
        let mut visited = vec![false; pack.scenes.len()];
        visited[0] = true;
        Aventura {
            pack,
            rng: Rng::new(seed),
            heroes: [hero.clone(), hero],
            scene: 0,
            visited,
            done: true,
            foes: Vec::new(),
            progress: 0,
            attempts: 0,
            searched: false,
            rested: false,
            potions: 1,
            side: 0,
            acted: [false; 2],
            passed: false,
            ward: false,
            round: 0,
            ticks: 0,
            phase: Phase::Pick,
            pend: None,
            pending_end: None,
            over: false,
            won: false,
            picks: 0,
            log: vec![format!("{}", pack.intro)],
        }
    }

    /// Indicele scenei curente (pentru teste și statistici).
    pub fn scene_index(&self) -> usize {
        self.scene
    }

    pub fn pack_id(&self) -> &'static str {
        self.pack.id
    }

    fn note(&mut self, s: String) {
        self.log.push(s);
        if self.log.len() > 8 {
            self.log.remove(0);
        }
    }

    fn arch(&self, h: usize) -> &'static Arch {
        &self.pack.arch[self.heroes[h].arch]
    }

    fn hero_name(&self, h: usize) -> String {
        format!("{} (J{})", self.arch(h).name, h + 1)
    }

    fn fight_active(&self) -> bool {
        !self.foes.is_empty()
    }

    fn kind(&self) -> &'static SK {
        &self.pack.scenes[self.scene].kind
    }

    fn is_final(&self) -> bool {
        self.scene + 1 == self.pack.scenes.len()
    }

    fn can_reroll(&self, h: usize) -> bool {
        let he = &self.heroes[h];
        match self.pack.res_mode {
            ResMode::Pool => he.res > 0,
            ResMode::Strain => he.res + 1 < self.pack.res_max,
        }
    }

    fn damage(&mut self, h: usize, n: i32) {
        if n <= 0 {
            return;
        }
        let was = self.heroes[h].hp;
        self.heroes[h].hp = (was - n).max(0);
        if was > 0 && self.heroes[h].hp == 0 {
            let nm = self.hero_name(h);
            self.note(format!("{nm} se prăbușește!"));
        }
        if self.heroes.iter().all(|x| x.hp == 0) && self.pending_end.is_none() {
            self.pending_end = Some(false);
            self.note("Ultimul erou a căzut.".into());
            self.phase = Phase::Finish;
        }
    }

    fn strain(&mut self, h: usize, n: i32) {
        match self.pack.res_mode {
            ResMode::Pool => {
                self.heroes[h].res = (self.heroes[h].res - n).max(0);
            }
            ResMode::Strain => {
                self.heroes[h].res += n;
                if self.heroes[h].res >= self.pack.res_max {
                    self.heroes[h].res = self.pack.res_max / 2;
                    let nm = self.hero_name(h);
                    self.note(format!(
                        "{nm} se frânge sub {} și pierde 3 PV.",
                        self.pack.res_name.to_lowercase()
                    ));
                    self.damage(h, 3);
                }
            }
        }
    }

    fn gain_res(&mut self, h: usize) {
        let r = &mut self.heroes[h].res;
        match self.pack.res_mode {
            ResMode::Pool => *r = (*r + 1).min(self.pack.res_max),
            ResMode::Strain => *r = (*r - 1).max(0),
        }
    }

    fn flip_side(&mut self) {
        let o = 1 - self.side;
        if self.heroes[o].hp > 0 {
            self.side = o;
        }
    }

    /// Eroul curent a terminat o acțiune.
    fn after_action(&mut self) {
        if self.pending_end.is_some() {
            self.phase = Phase::Finish;
            return;
        }
        if self.fight_active() {
            self.acted[self.side] = true;
            let o = 1 - self.side;
            if self.heroes[o].hp > 0 && !self.acted[o] {
                self.side = o;
                self.phase = Phase::Act;
            } else {
                self.phase = Phase::Foes;
            }
        } else {
            self.acted = [false; 2];
            self.flip_side();
            self.phase = Phase::Act;
        }
    }

    fn start_scene(&mut self, s: usize) {
        self.scene = s;
        self.visited[s] = true;
        self.progress = 0;
        self.attempts = 0;
        self.searched = false;
        self.rested = false;
        self.acted = [false; 2];
        self.ward = false;
        self.round = 0;
        for h in self.heroes.iter_mut() {
            h.used = false;
            h.guard = false;
        }
        let sc = &self.pack.scenes[s];
        self.foes.clear();
        self.done = true;
        match &sc.kind {
            SK::Fight(ids) => {
                for &i in ids.iter() {
                    self.foes.push((i, self.pack.foes[i as usize].hp));
                }
                self.done = false;
            }
            SK::Test(..) => self.done = false,
            _ => {}
        }
        let txt = format!("» {} — {}", sc.name, sc.text);
        self.note(txt);
    }

    fn end_fight(&mut self) {
        self.foes.clear();
        self.done = true;
        self.acted = [false; 2];
        self.ward = false;
        for h in self.heroes.iter_mut() {
            h.guard = false;
        }
        if self.is_final() {
            self.pending_end = Some(true);
            self.note("Stăpânul aventurii a fost învins!".into());
            self.phase = Phase::Finish;
        } else {
            self.note("Dușmanii au căzut.".into());
        }
    }

    fn pend_params(&self, p: &Pend) -> (u8, u8) {
        let a = self.arch(p.who);
        match p.kind {
            PK::Attack(fi, power) => {
                let f = &self.pack.foes[self.foes[fi].0 as usize];
                (a.skills[0], (f.def + u8::from(power)).min(3))
            }
            PK::Try => match self.kind() {
                SK::Test(sk, df, _, _) => (a.skills[*sk as usize % 3], *df),
                _ => (a.skills[1], 1),
            },
            PK::Search => (a.skills[1], 1),
        }
    }

    fn resolve(&mut self) {
        let Some(p) = self.pend.take() else {
            return;
        };
        let who = p.who;
        let nm = self.hero_name(who);
        let g = p.grade;
        if g == Grade::Crit {
            self.gain_res(who);
        }
        match p.kind {
            PK::Attack(fi, power) => {
                let (id, _) = self.foes[fi];
                let f = &self.pack.foes[id as usize];
                let base = self.arch(who).dmg;
                let mut dmg = match g {
                    Grade::Crit => base * if power { 3 } else { 2 },
                    Grade::Success | Grade::Partial => base * if power { 2 } else { 1 },
                    _ => 0,
                };
                if dmg > 0 && f.flags & TOUGH != 0 {
                    dmg = (dmg - 1).max(1);
                }
                if dmg > 0 {
                    self.foes[fi].1 -= dmg;
                    let left = self.foes[fi].1;
                    self.note(format!("{nm} lovește {} pentru {dmg}.", f.name));
                    if left <= 0 {
                        self.foes.remove(fi);
                        self.note(format!("{} cade.", f.name));
                    }
                } else {
                    self.note(format!("{nm} nu nimerește {}.", f.name));
                }
                if g == Grade::Partial {
                    self.note(format!("{nm} plătește un preț: 1 PV."));
                    self.damage(who, 1);
                } else if g == Grade::CritFail {
                    self.note(format!("{nm} se dezechilibrează: 1 PV."));
                    self.damage(who, 1);
                }
                if self.pending_end.is_none() && self.foes.is_empty() {
                    self.end_fight();
                }
            }
            PK::Try => {
                let (need, dmg) = match self.kind() {
                    SK::Test(_, _, n, dm) => (*n, *dm),
                    _ => (1, 1),
                };
                self.attempts += 1;
                match g {
                    Grade::Crit => {
                        self.progress += 2;
                        self.note(format!("{nm} reușește strălucit."));
                    }
                    Grade::Success => {
                        self.progress += 1;
                        self.note(format!("{nm} reușește."));
                    }
                    Grade::Partial => {
                        self.progress += 1;
                        self.note(format!("{nm} reușește cu un preț: 1 PV."));
                        self.damage(who, 1);
                    }
                    Grade::Fail => {
                        self.note(format!("{nm} dă greș și pățește {dmg} PV."));
                        self.damage(who, dmg);
                    }
                    Grade::CritFail => {
                        self.note(format!("{nm} greșește rău: {} PV și presiune.", dmg + 1));
                        self.damage(who, dmg + 1);
                        self.strain(who, 1);
                    }
                }
                if self.pending_end.is_none() && !self.done {
                    if self.progress >= need {
                        self.done = true;
                        self.note("Obstacolul e depășit.".into());
                    } else if self.attempts >= MAX_ATTEMPTS {
                        self.done = true;
                        self.note("Treceți cu forța, șifonați: fiecare pierde 1 PV.".into());
                        for h in 0..2 {
                            self.damage(h, 1);
                        }
                    }
                }
            }
            PK::Search => {
                self.searched = true;
                match g {
                    Grade::Crit => {
                        self.potions += 1;
                        self.gain_res(who);
                        self.note(format!("{nm} găsește {} și un răgaz.", self.pack.potion));
                    }
                    Grade::Success => {
                        self.potions += 1;
                        self.note(format!("{nm} găsește {}.", self.pack.potion));
                    }
                    Grade::Partial => {
                        self.potions += 1;
                        self.note(format!(
                            "{nm} găsește {}, dar pățește 1 PV.",
                            self.pack.potion
                        ));
                        self.damage(who, 1);
                    }
                    Grade::Fail => self.note(format!("{nm} nu găsește nimic.")),
                    Grade::CritFail => {
                        self.note(format!("{nm} declanșează o cursă: 2 PV."));
                        self.damage(who, 2);
                    }
                }
            }
        }
        self.after_action();
    }

    fn pick_target(&self, fi: usize) -> usize {
        let alive: Vec<usize> = (0..2).filter(|&h| self.heroes[h].hp > 0).collect();
        if alive.len() == 1 {
            return alive[0];
        }
        if fi.is_multiple_of(2) {
            // cel mai slab (PV mai puține; egalitate: eroul 1)
            *alive
                .iter()
                .min_by_key(|&&h| (self.heroes[h].hp, h))
                .unwrap()
        } else {
            (self.round as usize) % 2
        }
    }

    fn do_foes(&mut self) {
        let mut msgs: Vec<String> = Vec::new();
        let foes: Vec<(u8, i32)> = self.foes.clone();
        for (fi, (id, _)) in foes.iter().enumerate() {
            if self.pending_end.is_some() {
                break;
            }
            let f = &self.pack.foes[*id as usize];
            let t = self.pick_target(fi);
            if self.heroes[t].hp == 0 {
                continue;
            }
            let df =
                (self.arch(t).def + u8::from(self.heroes[t].guard) + u8::from(self.ward)).min(3);
            let r = check(self.pack.mech, f.lvl, df, &mut self.rng);
            let dmg = match r.grade {
                Grade::Crit => f.dmg + 1,
                Grade::Success => f.dmg,
                Grade::Partial => (f.dmg + 1) / 2,
                _ => 0,
            };
            let tn = self.hero_name(t);
            if dmg > 0 {
                msgs.push(format!(
                    "{} lovește {tn} pentru {dmg} [{}].",
                    f.name, r.text
                ));
                self.damage(t, dmg);
                if f.flags & TERROR != 0 && self.heroes[t].hp > 0 {
                    self.strain(t, 1);
                    msgs.push(format!("Groaza îl atinge pe {tn}."));
                }
            } else {
                msgs.push(format!("{} ratează {tn} [{}].", f.name, r.text));
            }
        }
        // mișcarea specială a stăpânului la fiecare a treia rundă
        if self.pending_end.is_none() && self.round % 3 == 2 {
            if let Some(b) = self
                .foes
                .iter()
                .find(|(id, _)| self.pack.foes[*id as usize].flags & BOSS != 0)
            {
                let bf = &self.pack.foes[b.0 as usize];
                msgs.push(format!("{} dezlănțuie „{}”!", bf.name, self.pack.boss_move));
                for h in 0..2 {
                    if self.heroes[h].hp > 0 {
                        self.damage(h, (bf.dmg / 2).max(1));
                        if self.heroes[h].hp > 0 {
                            self.strain(h, 1);
                        }
                    }
                }
            }
        }
        for m in msgs {
            self.note(m);
        }
        self.round += 1;
        self.acted = [false; 2];
        self.ward = false;
        for h in self.heroes.iter_mut() {
            h.guard = false;
        }
        if self.pending_end.is_some() {
            self.phase = Phase::Finish;
            return;
        }
        if self.round >= ROUND_CAP {
            self.pending_end = Some(false);
            self.note("Lupta s-a prelungit prea mult: eroii sunt puși pe fugă.".into());
            self.phase = Phase::Finish;
            return;
        }
        let first = (self.round as usize) % 2;
        self.side = if self.heroes[first].hp > 0 {
            first
        } else {
            1 - first
        };
        self.phase = Phase::Act;
    }

    fn do_event(&mut self) {
        let r = d(&mut self.rng, 6) as usize;
        let (txt, code) = self.pack.events[r - 1];
        let who = self.side;
        let mut s = format!("Eveniment ({r}): {txt}");
        match code {
            1 => {
                self.potions += 1;
                s.push_str(&format!(" +1 {}.", self.pack.potion));
            }
            2 => {
                for h in self.heroes.iter_mut() {
                    if h.hp > 0 {
                        h.hp = (h.hp + 3).min(h.max);
                    }
                }
                s.push_str(" Eroii prind putere (+3 PV).");
            }
            3 => {
                s.push_str(" Cursă: 2 PV.");
                self.note(s.clone());
                self.damage(who, 2);
                s = String::new();
            }
            4 => {
                s.push_str(&format!(" {} scade pentru amândoi.", self.pack.res_name));
                self.note(s.clone());
                for h in 0..2 {
                    if self.heroes[h].hp > 0 {
                        self.strain(h, 1);
                    }
                }
                s = String::new();
            }
            5 => {
                self.foes.push((0, self.pack.foes[0].hp));
                self.done = false;
                s.push_str(&format!(" Apare {}!", self.pack.foes[0].name));
            }
            6 => {
                for h in 0..2 {
                    if self.heroes[h].hp > 0 {
                        self.gain_res(h);
                    }
                }
                s.push_str(&format!(" {} se întoarce.", self.pack.res_name));
            }
            _ => {}
        }
        if !s.is_empty() {
            self.note(s);
        }
        if self.pending_end.is_some() {
            self.phase = Phase::Finish;
            return;
        }
        self.acted = [false; 2];
        self.flip_side();
        self.phase = Phase::Act;
    }

    fn dists(&self, goal: &[bool]) -> Vec<i32> {
        let n = self.pack.scenes.len();
        let mut dist = vec![50i32; n];
        for i in (0..n).rev() {
            if goal[i] {
                dist[i] = 0;
            } else {
                let m = self.pack.scenes[i]
                    .exits
                    .iter()
                    .map(|&e| dist[e as usize])
                    .min()
                    .unwrap_or(50);
                dist[i] = (m + 1).min(50);
            }
        }
        dist
    }

    fn goal_distance(&self) -> i32 {
        let n = self.pack.scenes.len();
        let mut boss = vec![false; n];
        boss[n - 1] = true;
        let tot: i32 = self.heroes.iter().map(|h| h.hp).sum();
        let mx: i32 = self.heroes.iter().map(|h| h.max).sum();
        if tot * 100 < mx * 65 {
            let rest: Vec<bool> = (0..n)
                .map(|i| matches!(self.pack.scenes[i].kind, SK::Rest) && !self.visited[i])
                .collect();
            let dr = self.dists(&rest)[self.scene];
            if dr < 50 {
                return dr;
            }
        }
        self.dists(&boss)[self.scene]
    }

    fn arch_value(&self, a: usize) -> i32 {
        let ar = &self.pack.arch[a];
        4 * ar.hp
            + 20 * ar.skills.iter().map(|&x| i32::from(x)).sum::<i32>()
            + 30 * ar.dmg
            + 25 * i32::from(ar.def)
    }

    /// Valoarea euristică a stării pentru echipă (doar informație publică).
    fn team_value(&self) -> i32 {
        if let Some(w) = self.pending_end {
            return if w { 200_000 } else { -200_000 };
        }
        let mut s = 0;
        if self.phase == Phase::Pick {
            for h in 0..self.picks.min(2) {
                s += self.arch_value(self.heroes[h].arch);
            }
            if self.picks == 2 && self.arch(0).sp != self.arch(1).sp {
                s += 80;
            }
            return s;
        }
        for h in &self.heroes {
            s += 40 * h.hp;
            if h.hp == 0 {
                s -= 250;
            }
            match self.pack.res_mode {
                ResMode::Pool => s += 18 * h.res,
                ResMode::Strain => {
                    s -= 22 * h.res;
                    if h.res + 1 >= self.pack.res_max {
                        s -= 60;
                    }
                }
            }
            if h.guard && self.fight_active() {
                s += 30;
            }
        }
        s += 110 * i32::from(self.potions);
        for (id, hp) in &self.foes {
            let f = &self.pack.foes[*id as usize];
            s -= 28 * hp + 10 * (f.dmg + i32::from(f.lvl));
        }
        if self.ward && self.fight_active() {
            s += 90;
        }
        if let SK::Test(sk, _, _, _) = self.kind() {
            if !self.done {
                s += 160 * i32::from(self.progress);
                s += 45 * i32::from(self.arch(self.side).skills[*sk as usize % 3]);
            }
        }
        s -= 450 * self.goal_distance();
        if self.phase == Phase::Roll || self.phase == Phase::Reroll {
            if let Some(p) = self.pend {
                if self.phase == Phase::Roll {
                    s += self.expected_gain(&p);
                }
            }
        }
        s
    }

    fn expected_gain(&self, p: &Pend) -> i32 {
        let (lvl, diff) = self.pend_params(p);
        let (ok, part, crit) = probs(self.pack.mech, lvl, diff);
        match p.kind {
            PK::Attack(fi, power) => {
                let f = &self.pack.foes[self.foes[fi].0 as usize];
                let base = self.arch(p.who).dmg * if power { 2 } else { 1 };
                let dm = if f.flags & TOUGH != 0 {
                    (base - 1).max(1)
                } else {
                    base
                };
                let dm = dm.min(self.foes[fi].1);
                let mut g = (ok * dm * 28 + crit * dm * 14 + part * (dm * 28 - 40)) / 1000;
                if dm >= self.foes[fi].1 {
                    let last = self.foes.len() == 1;
                    g += ok * if last && self.is_final() { 3000 } else { 150 } / 1000;
                }
                g
            }
            PK::Try => (ok * 160 + part * 90 - (1000 - ok - part) * 40) / 1000,
            PK::Search => (ok * 110 + part * 60 - (1000 - ok - part) * 30) / 1000,
        }
    }
}

impl Game for Aventura {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        match self.phase {
            Phase::Finish => vec![Move::Finish],
            Phase::Roll => vec![Move::Roll],
            Phase::Reroll => vec![Move::Accept, Move::Reroll],
            Phase::Event => vec![Move::Event],
            Phase::Foes => vec![Move::Foes],
            Phase::Pick => {
                let taken = if self.picks == 1 {
                    Some(self.heroes[0].arch)
                } else {
                    None
                };
                (0..3u8)
                    .filter(|&a| taken != Some(usize::from(a)))
                    .map(Move::Pick)
                    .collect()
            }
            Phase::Act => {
                let me = &self.heroes[self.side];
                let mut v = Vec::new();
                let hurt = |h: &Hero| h.hp < h.max;
                if self.fight_active() {
                    for fi in 0..self.foes.len() {
                        v.push(Move::Attack(fi as u8));
                    }
                    v.push(Move::Guard);
                } else {
                    if self.done {
                        for &e in self.pack.scenes[self.scene].exits {
                            v.push(Move::Go(e));
                        }
                    } else {
                        v.push(Move::Try);
                    }
                    if !self.searched && matches!(self.kind(), SK::Explore | SK::Rest) {
                        v.push(Move::Search);
                    }
                    if !self.rested && matches!(self.kind(), SK::Rest) {
                        v.push(Move::Rest);
                    }
                    if !self.passed && self.heroes[1 - self.side].hp > 0 {
                        v.push(Move::Pass);
                    }
                }
                if !me.used {
                    match self.arch(self.side).sp {
                        Sp::Strike if self.fight_active() => {
                            for fi in 0..self.foes.len() {
                                v.push(Move::Special(fi as u8));
                            }
                        }
                        Sp::Heal => {
                            for h in 0..2 {
                                if hurt(&self.heroes[h]) {
                                    v.push(Move::Special(h as u8));
                                }
                            }
                        }
                        Sp::Ward if self.fight_active() && !self.ward => v.push(Move::Special(0)),
                        _ => {}
                    }
                }
                if self.potions > 0 {
                    for h in 0..2 {
                        if hurt(&self.heroes[h]) {
                            v.push(Move::Potion(h as u8));
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.ticks += 1;
        if *m != Move::Pass {
            self.passed = false;
        }
        let who = self.side;
        let nm = if self.phase == Phase::Pick {
            String::new()
        } else {
            self.hero_name(who)
        };
        match *m {
            Move::Pick(a) => {
                let ar = &self.pack.arch[usize::from(a)];
                let h = &mut self.heroes[self.picks];
                h.arch = usize::from(a);
                h.hp = ar.hp;
                h.max = ar.hp;
                h.res = match self.pack.res_mode {
                    ResMode::Pool => self.pack.res_max,
                    ResMode::Strain => 0,
                };
                let line = format!("J{} alege {}.", self.picks + 1, ar.name);
                self.picks += 1;
                self.note(line);
                if self.picks == 2 {
                    self.side = 0;
                    self.phase = Phase::Act;
                    self.note(format!("Aventura începe: {}.", self.pack.scenes[0].name));
                } else {
                    self.side = 1;
                }
            }
            Move::Go(e) => {
                self.start_scene(usize::from(e));
                self.acted = [false; 2];
                self.flip_side();
                if matches!(self.kind(), SK::Explore) {
                    self.phase = Phase::Event;
                } else {
                    self.phase = Phase::Act;
                }
            }
            Move::Attack(fi) => {
                self.pend = Some(Pend {
                    kind: PK::Attack(usize::from(fi), false),
                    who,
                    rerolled: false,
                    grade: Grade::Fail,
                });
                self.phase = Phase::Roll;
            }
            Move::Special(t) => {
                self.heroes[who].used = true;
                match self.arch(who).sp {
                    Sp::Strike => {
                        let nmf = self.pack.foes[self.foes[usize::from(t)].0 as usize].name;
                        let sp = self.arch(who).sp_name;
                        self.note(format!("{nm} folosește „{sp}” împotriva {nmf}."));
                        self.pend = Some(Pend {
                            kind: PK::Attack(usize::from(t), true),
                            who,
                            rerolled: false,
                            grade: Grade::Fail,
                        });
                        self.phase = Phase::Roll;
                    }
                    Sp::Heal => {
                        let tgt = usize::from(t);
                        let h = &mut self.heroes[tgt];
                        h.hp = (h.hp.max(0) + 4).min(h.max);
                        let sp = self.arch(who).sp_name;
                        let tn = self.hero_name(tgt);
                        self.note(format!("{nm} folosește „{sp}”: {tn} se vindecă (+4 PV)."));
                        self.after_action();
                    }
                    Sp::Ward => {
                        self.ward = true;
                        let sp = self.arch(who).sp_name;
                        self.note(format!(
                            "{nm} folosește „{sp}”: dușmanii lovesc mai greu runda asta."
                        ));
                        self.after_action();
                    }
                }
            }
            Move::Guard => {
                self.heroes[who].guard = true;
                self.note(format!("{nm} se pune în gardă."));
                self.after_action();
            }
            Move::Potion(t) => {
                self.potions -= 1;
                let tgt = usize::from(t);
                let h = &mut self.heroes[tgt];
                h.hp = (h.hp + POTION_HEAL).min(h.max);
                let tn = self.hero_name(tgt);
                self.note(format!(
                    "{nm} îi dă lui {tn} {} (+{POTION_HEAL} PV).",
                    self.pack.potion
                ));
                self.after_action();
            }
            Move::Search => {
                self.pend = Some(Pend {
                    kind: PK::Search,
                    who,
                    rerolled: false,
                    grade: Grade::Fail,
                });
                self.phase = Phase::Roll;
            }
            Move::Try => {
                self.pend = Some(Pend {
                    kind: PK::Try,
                    who,
                    rerolled: false,
                    grade: Grade::Fail,
                });
                self.phase = Phase::Roll;
            }
            Move::Rest => {
                self.rested = true;
                for h in 0..2 {
                    let he = &mut self.heroes[h];
                    he.hp = (he.hp + REST_HEAL).min(he.max).max(3.min(he.max));
                    match self.pack.res_mode {
                        ResMode::Pool => he.res = (he.res + 1).min(self.pack.res_max),
                        ResMode::Strain => he.res = (he.res - 2).max(0),
                    }
                }
                self.note(format!(
                    "{nm} conduce odihna: eroii se refac (+{REST_HEAL} PV, {} se liniștește).",
                    self.pack.res_name
                ));
                self.after_action();
            }
            Move::Pass => {
                self.passed = true;
                self.note(format!("{nm} cedează inițiativa."));
                self.flip_side();
            }
            Move::Roll => {
                let Some(mut p) = self.pend else {
                    return;
                };
                let (lvl, diff) = self.pend_params(&p);
                let r = check(self.pack.mech, lvl, diff, &mut self.rng);
                p.grade = r.grade;
                let who = p.who;
                let tag = match r.grade {
                    Grade::Crit => "CRITIC",
                    Grade::Success => "succes",
                    Grade::Partial => "parțial",
                    Grade::Fail => "eșec",
                    Grade::CritFail => "EȘEC GRAV",
                };
                let n = self.hero_name(who);
                self.note(format!("{n}: {} → {tag}.", r.text));
                self.pend = Some(p);
                if r.grade <= Grade::Fail && !p.rerolled && self.can_reroll(who) {
                    self.phase = Phase::Reroll;
                } else {
                    self.resolve();
                }
            }
            Move::Accept => {
                self.resolve();
            }
            Move::Reroll => {
                let who = self.pend.map(|p| p.who).unwrap_or(self.side);
                match self.pack.res_mode {
                    ResMode::Pool => self.heroes[who].res -= 1,
                    ResMode::Strain => self.heroes[who].res += 1,
                }
                if let Some(p) = self.pend.as_mut() {
                    p.rerolled = true;
                }
                let n = self.hero_name(who);
                self.note(format!(
                    "{n} cheltuiește {} pentru a reface zarurile.",
                    self.pack.res_name
                ));
                self.phase = Phase::Roll;
            }
            Move::Event => self.do_event(),
            Move::Foes => self.do_foes(),
            Move::Finish => {
                self.over = true;
                self.won = self.pending_end == Some(true);
            }
        }
        if !self.over && self.pending_end.is_none() && self.ticks >= TICK_CAP {
            self.pending_end = Some(false);
            self.note("Timpul aventurii a expirat.".into());
            self.phase = Phase::Finish;
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

    /// Cooperativ: toate zarurile sunt mutări forțate separate; AI-ul alege din starea publică și din
    /// probabilități (calculate cu un generator propriu), niciodată din zarurile care urmează.
    fn evaluate(&self, _p: Player) -> i32 {
        self.team_value()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let pk = self.pack;
        let mut s = format!("{} — {}\n", pk.title, pk.style);
        s.push_str(&format!(
            "Mecanică: {} | Resursa: {} ({})\n",
            pk.mech.desc(),
            pk.res_name,
            match pk.res_mode {
                ResMode::Pool => "rezervă: o cheltui ca să refaci un zar ratat",
                ResMode::Strain => "presiune: refacerea o crește; la maxim te frângi",
            }
        ));
        if self.phase == Phase::Pick {
            s.push_str(&format!("Alege arhetipul pentru J{}:\n", self.picks + 1));
            for (i, a) in pk.arch.iter().enumerate() {
                s.push_str(&format!(
                    "  {}. {} — PV {}, {} {}/{}, {} {}/{}, {} {}/{}, daune {}, apărare {}, special „{}” ({})\n",
                    i + 1,
                    a.name,
                    a.hp,
                    pk.skills[0],
                    a.skills[0],
                    6,
                    pk.skills[1],
                    a.skills[1],
                    6,
                    pk.skills[2],
                    a.skills[2],
                    6,
                    a.dmg,
                    a.def,
                    a.sp_name,
                    match a.sp {
                        Sp::Strike => "lovitură dublă",
                        Sp::Heal => "vindecă 4 PV",
                        Sp::Ward => "dușmanii lovesc mai greu",
                    }
                ));
            }
            s.push_str("Mutări: alege N.\n");
            return s;
        }
        let sc = &pk.scenes[self.scene];
        s.push_str(&format!(
            "Scena {}/{}: {} — {}\n",
            self.scene + 1,
            pk.scenes.len(),
            sc.name,
            sc.text
        ));
        let kind = match &sc.kind {
            SK::Explore => "explorare".to_string(),
            SK::Test(sk, df, need, _) => format!(
                "test ({}, dificultate {}, succese {}/{})",
                pk.skills[usize::from(*sk) % 3],
                df,
                self.progress,
                need
            ),
            SK::Fight(_) => {
                if self.is_final() {
                    "luptă cu stăpânul".into()
                } else {
                    "luptă".into()
                }
            }
            SK::Rest => "odihnă".into(),
        };
        s.push_str(&format!("Tip: {kind}\n"));
        if self.done && !self.fight_active() && !self.is_final() {
            let ex: Vec<String> = sc
                .exits
                .iter()
                .map(|&e| format!("ir {} ({})", e + 1, pk.scenes[e as usize].name))
                .collect();
            s.push_str(&format!("Drumuri: {}\n", ex.join(", ")));
        }
        for h in 0..2 {
            let he = &self.heroes[h];
            let a = &pk.arch[he.arch];
            s.push_str(&format!(
                "J{} {} — PV {}/{}{} — {} {}/{}{}{}\n",
                h + 1,
                a.name,
                he.hp,
                he.max,
                if he.hp == 0 { " (doborât)" } else { "" },
                pk.res_name,
                he.res,
                pk.res_max,
                if he.guard { " [în gardă]" } else { "" },
                if he.used { " [special folosit]" } else { "" }
            ));
        }
        s.push_str(&format!("{}: {}", pk.potion, self.potions));
        if self.fight_active() {
            s.push_str(&format!(" | Runda {}\n", self.round + 1));
            for (i, (id, hp)) in self.foes.iter().enumerate() {
                let f = &pk.foes[*id as usize];
                s.push_str(&format!(
                    "  {}. {} — PV {}/{}, nivel {}, daune {}, apărare {}{}{}{}\n",
                    i + 1,
                    f.name,
                    hp,
                    f.hp,
                    f.lvl,
                    f.dmg,
                    f.def,
                    if f.flags & TERROR != 0 {
                        ", teroare"
                    } else {
                        ""
                    },
                    if f.flags & TOUGH != 0 {
                        ", blindat"
                    } else {
                        ""
                    },
                    if f.flags & BOSS != 0 {
                        ", STĂPÂN"
                    } else {
                        ""
                    }
                ));
            }
        } else {
            s.push('\n');
        }
        s.push_str("Jurnal:\n");
        for l in self.log.iter().rev().take(5).rev() {
            s.push_str(&format!("  {l}\n"));
        }
        if self.over {
            if self.won {
                s.push_str("VICTORIE: aventura e încheiată cu bine.\n");
            } else {
                s.push_str("ÎNFRÂNGERE (remiză): aventura a eșuat.\n");
            }
            return s;
        }
        let who = format!("J{} ({})", self.side + 1, self.arch(self.side).name);
        match self.phase {
            Phase::Roll => s.push_str("Zarurile urmează: zaruri.\n"),
            Phase::Reroll => s.push_str(&format!(
                "Zar ratat. {who}: accept (rămâne rezultatul) sau refac (costă 1 {}).\n",
                pk.res_name
            )),
            Phase::Event => s.push_str("Ceva se petrece în scenă: eveniment.\n"),
            Phase::Foes => s.push_str("Maestrul-AI joacă dușmanii: maestrul.\n"),
            Phase::Finish => s.push_str("Aventura se încheie: sfârșit.\n"),
            _ => s.push_str(&format!(
                "La rând {who}. Mutări: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.\n"
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(a) => format!("alege {}", a + 1),
            Move::Go(e) => format!("ir {}", e + 1),
            Move::Attack(f) => format!("atac {}", f + 1),
            Move::Guard => "parez".into(),
            Move::Special(t) => match self.arch(self.side).sp {
                Sp::Ward => "special".into(),
                _ => format!("special {}", t + 1),
            },
            Move::Potion(h) => format!("bea {}", h + 1),
            Move::Search => "cauta".into(),
            Move::Try => "incearca".into(),
            Move::Rest => "odihna".into(),
            Move::Pass => "cedez".into(),
            Move::Roll => "zaruri".into(),
            Move::Accept => "accept".into(),
            Move::Reroll => "refac".into(),
            Move::Event => "eveniment".into(),
            Move::Foes => "maestrul".into(),
            Move::Finish => "sfârșit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(id: &str, seed: u64) -> Aventura {
        let mut g = Aventura::with_seed(id, seed);
        let mut rng = Rng::new(seed ^ 0xA1);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            n += 1;
            assert!(n < 6000, "{id} seed {seed}: nu se termină");
        }
        g
    }

    #[test]
    fn packs_are_well_formed() {
        assert_eq!(PACKS.len(), 16);
        let mut ids = std::collections::BTreeSet::new();
        for p in PACKS.iter() {
            assert!(ids.insert(p.id), "id dublat {}", p.id);
            let n = p.scenes.len();
            assert!((7..=8).contains(&n), "{}: {} scene", p.id, n);
            assert!((6..=8).contains(&p.foes.len()), "{}: monștri", p.id);
            assert!(
                p.foes[0].flags & BOSS == 0 && p.foes[0].hp <= 6,
                "{}: foe 0 = ambuscadă",
                p.id
            );
            assert!(p.foes.iter().any(|f| f.flags & BOSS != 0), "{}: boss", p.id);
            for (i, sc) in p.scenes.iter().enumerate() {
                for &e in sc.exits {
                    assert!(
                        usize::from(e) > i && usize::from(e) < n,
                        "{} scena {i}",
                        p.id
                    );
                }
                if i + 1 < n {
                    assert!(!sc.exits.is_empty(), "{} scena {i} fără ieșire", p.id);
                }
                if let SK::Fight(ids) = &sc.kind {
                    for &f in ids.iter() {
                        assert!(usize::from(f) < p.foes.len());
                    }
                }
                if let SK::Test(sk, df, need, _) = &sc.kind {
                    assert!(*sk < 3 && *df <= 3 && *need >= 1);
                }
            }
            match &p.scenes[n - 1].kind {
                SK::Fight(ids) => assert!(
                    ids.iter()
                        .any(|&f| p.foes[usize::from(f)].flags & BOSS != 0),
                    "{} boss final",
                    p.id
                ),
                _ => panic!("{}: ultima scenă trebuie să fie lupta cu stăpânul", p.id),
            }
            assert!(
                p.scenes.iter().any(|s| matches!(s.kind, SK::Rest)),
                "{} fără odihnă",
                p.id
            );
            assert!(
                p.scenes.iter().any(|s| matches!(s.kind, SK::Test(..))),
                "{} fără test",
                p.id
            );
            // fiecare scenă ajunge la final
            let mut reach = vec![false; n];
            reach[n - 1] = true;
            for i in (0..n - 1).rev() {
                reach[i] = p.scenes[i].exits.iter().any(|&e| reach[usize::from(e)]);
                assert!(reach[i], "{} scena {i} nu duce la final", p.id);
            }
            for e in p.events.iter() {
                assert!(e.1 <= 6);
            }
            for a in p.arch.iter() {
                assert!(a.skills.iter().all(|&x| (1..=6).contains(&x)));
            }
        }
    }

    #[test]
    fn mechanics_have_sane_odds() {
        for mech in [
            Mech::D20,
            Mech::D20Deg,
            Mech::D100,
            Mech::D100Deg,
            Mech::D20Under,
            Mech::Pool10,
            Mech::Pool6,
            Mech::Highest6,
            Mech::Triple20,
            Mech::TwoD6,
            Mech::D10Exp,
        ] {
            let (a3, p3, _) = probs(mech, 3, 1);
            let (a5, _, _) = probs(mech, 5, 1);
            let (h3, _, _) = probs(mech, 3, 3);
            let (e3, _, _) = probs(mech, 3, 0);
            assert!(a5 + 40 >= a3, "{mech:?}: rating mai mare nu ajută");
            assert!(
                a3 + p3 > 250 && a3 + p3 < 950,
                "{mech:?}: normal {}",
                a3 + p3
            );
            assert!(h3 < a3 + 20, "{mech:?}: extrem mai ușor decât normal");
            assert!(e3 + 20 >= a3, "{mech:?}: ușor mai greu decât normal");
        }
    }

    #[test]
    fn all_packs_complete_with_ai() {
        let mut wins = 0;
        let mut total = 0;
        for p in PACKS.iter() {
            let mut pack_wins = 0;
            for seed in 1..=6u64 {
                let g = play(p.id, seed);
                total += 1;
                if g.won {
                    wins += 1;
                    pack_wins += 1;
                }
                assert!(g.outcome().is_some());
            }
            assert!(pack_wins >= 1, "{}: nicio victorie în 6 partide", p.id);
        }
        eprintln!("victorii AI: {wins}/{total}");
        assert!(wins * 100 >= total * 15, "prea grele: {wins}/{total}");
        assert!(wins * 100 <= total * 97, "prea ușoare: {wins}/{total}");
    }

    #[test]
    fn deterministic_for_fixed_seed() {
        for p in PACKS.iter().take(6) {
            let a = play(p.id, 7);
            let b = play(p.id, 7);
            assert_eq!(a.render(), b.render(), "{}", p.id);
            assert_eq!(a.log, b.log);
            let c = play(p.id, 8);
            assert!(
                a.log != c.log || a.ticks != c.ticks,
                "{}: seed-uri identice",
                p.id
            );
        }
    }

    #[test]
    fn ai_choice_does_not_depend_on_future_dice() {
        for id in [
            "turnul_zorilor",
            "arhiva_abisala",
            "hotii_din_ceata",
            "neon_si_rune",
        ] {
            let mut g = Aventura::with_seed(id, 11);
            let mut r = Rng::new(5);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let seed = r.next_u64();
                let a = polytabletop_core::ai::best_move(&g, seed).unwrap();
                for k in 0..4u64 {
                    let mut h = g.clone();
                    h.rng = Rng::new(1000 + k * 77);
                    let b = polytabletop_core::ai::best_move(&h, seed).unwrap();
                    assert_eq!(
                        a, b,
                        "{id}: AI-ul depinde de zarurile viitoare la pasul {n}"
                    );
                }
                g.apply(&a);
                n += 1;
            }
        }
    }

    #[test]
    fn text_moves_round_trip_and_reject_illegal() {
        use crate::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(Aventura::with_seed("insula_celor_doua_luni", 3));
        assert!(g.play("ir 2").is_err());
        assert_eq!(g.play("alege 1").unwrap(), "alege 1");
        assert!(g.play("alege 1").is_err(), "arhetip luat");
        assert_eq!(g.play("Alege 2").unwrap(), "alege 2");
        assert!(g.legal_moves().iter().any(|m| m.starts_with("ir ")));
        assert!(g.render().contains("Scena 1/"));
    }

    #[test]
    fn hotseat_render_is_seed_independent_at_start() {
        let a = Aventura::new("targul_ciumat").render();
        let b = Aventura::new("targul_ciumat").render();
        assert_eq!(a, b);
    }

    #[test]
    fn rerolls_cost_resource_and_are_bounded() {
        let mut g = Aventura::with_seed("arhiva_abisala", 21);
        g.apply(&Move::Pick(0));
        g.apply(&Move::Pick(1));
        let mut seen_reroll = false;
        let mut n = 0;
        while g.outcome().is_none() && n < 4000 {
            let lm = g.legal_moves();
            if lm.contains(&Move::Reroll) {
                let before = g.heroes[g.pend.unwrap().who].res;
                g.apply(&Move::Reroll);
                assert_eq!(g.heroes[g.pend.unwrap().who].res, before + 1);
                seen_reroll = true;
            } else {
                let m = polytabletop_core::ai::best_move(&g, n as u64).unwrap();
                g.apply(&m);
            }
            n += 1;
        }
        assert!(seen_reroll);
        for h in &g.heroes {
            assert!(h.res >= 0 && h.res < g.pack.res_max);
        }
    }
}
