//! Familia „fișe simple, MJ automat” (lista T1): duel/drum de aventurieri pentru 2 jucători.
//!
//! Un singur motor parametrizat prin `Spec` (vezi `t1_specs.rs`, generat): fiecare joc are propriul
//! mecanism de verificare (`Res`), decizie după aruncare (`Post`), vrajă/meșteșug (`Art`),
//! condiție de câștig (`Win`), regulă specială (`Twist`), tabel de evenimente al MJ-ului automat,
//! fișe diferite pentru cele două părți și valori proprii. Informație complet publică.
//!
//! Aruncările se fac doar în mutarea forțată `tira`; `eval` nu citește generatorul (estimează
//! rezultatul așteptat cu un generator local fix), iar AI-ul are adâncime 0: nu poate trișa.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::t1_specs::SPECS;
use crate::zr_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Res {
    /// d100 sub valoare
    Under100,
    /// 3d6 sub valoare
    Under3d6,
    /// d20 + 2·pricepere față de prag
    D20,
    /// zaruri d6 numărate (succese)
    Pool,
    /// patru zaruri cu +/0/- (propriu)
    Quad,
    /// 2d6 cu praguri 7 și 10
    Twin,
    /// un zar de mărime variabilă, explodează
    Step,
    /// cel mai bun zar dintr-un lot
    Best,
    /// zaruri d10, se păstrează cele mai bune, suma
    Keep,
    /// un zar de acțiune contra a două zaruri ale provocării
    Contest,
    /// zaruri d6; dublurile adaugă o nouă aruncare
    Doubles,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Post {
    /// niciun pas după aruncare
    None,
    /// cheltuiești 1 noroc: +1 treaptă
    Luck,
    /// forțezi: +1 stres și reiei aruncarea (o dată)
    Push,
    /// la „parțial” poți plăti 1 PV pentru reușită
    Cost,
    /// cu 2 noroc „arzi” momentul: rezultatul devine reușită
    Burn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Art {
    Bolt,
    Heal,
    Drain,
    Curse,
    Haste,
    Ward,
    Shatter,
    Mark,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Win {
    Kill,
    Road,
    Glory,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Twist {
    None,
    /// lovitură de 3+ PV: +1 stres țintei
    Wound,
    /// cine e lovit câștigă +1 focus
    Rage,
    /// reușita excelentă dă o acțiune în plus
    Surge,
    /// stres ≥ jumătate din maxim: −1 la verificări
    Fear,
    /// ratarea totală costă 1 PV
    Fumble,
    /// al doilea jucător din rundă: +1 la verificări
    Edge,
    /// fiecare vrajă costă +1 stres
    Tithe,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EvK {
    Pass,
    Hurt,
    Lead,
    Mend,
    Gift,
    Fortune,
    Fright,
    Setback,
    Rally,
    Calm,
}

#[derive(Clone, Copy)]
pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub res: Res,
    pub post: Post,
    pub art: Art,
    pub win: Win,
    pub twist: Twist,
    pub soft: bool,
    pub fail_stress: bool,
    pub hp: i32,
    pub foc0: i32,
    pub luck0: i32,
    /// pentru fiecare parte: [forță, iscusință, minte]
    pub st: [[i32; 3]; 2],
    pub dmg: [i32; 4],
    pub guard: [i32; 4],
    pub step: [i32; 4],
    pub art_cost: i32,
    pub art_pow: i32,
    pub stress_max: i32,
    pub goal: i32,
    pub rounds: u32,
    pub tn: i32,
    pub base: i32,
    pub rest_hp: i32,
    pub rest_foc: i32,
    pub ev: [(EvK, i32); 6],
    pub cls: [&'static str; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Event,
    Choose,
    Roll,
    Post,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Act {
    Atac,
    Apar,
    Arta,
    Tinta,
}

#[derive(Clone, Copy, Default)]
struct Hero {
    hp: i32,
    foc: i32,
    luck: i32,
    stress: i32,
    pos: i32,
    glory: i32,
    guard: i32,
    curse: i32,
    mark: i32,
}

#[derive(Clone)]
pub struct Rpg {
    sp: &'static Spec,
    rng: u64,
    side: usize,
    h: [Hero; 2],
    round: u32,
    ph: Ph,
    act: Act,
    deg: i32,
    info: i32,
    rerolled: bool,
    acted: u8,
    again: bool,
    bonus: bool,
    last: String,
}

fn d_(r: &mut u64, n: u32) -> i32 {
    die(r, n) as i32 + 1
}

const DEG: [&str; 4] = ["ratat", "parțial", "reușit", "excelent"];

/// Verificarea: întoarce (treapta 0..=3, numărul principal aruncat, pentru jurnal).
fn roll_deg(sp: &Spec, r: &mut u64, s: i32, d: i32) -> (i32, i32) {
    let s = s.clamp(0, 7);
    let d = d.clamp(0, 7);
    match sp.res {
        Res::Under100 => {
            let t = (50 + 12 * (s - d)).clamp(8, 95);
            let x = d_(r, 100);
            let deg = if x <= t / 5 {
                3
            } else if x <= t {
                2
            } else if sp.soft && x <= t + 12 {
                1
            } else {
                0
            };
            (deg, x)
        }
        Res::Under3d6 => {
            let t = (11 + 2 * (s - d)).clamp(5, 16);
            let x = d_(r, 6) + d_(r, 6) + d_(r, 6);
            let deg = if x <= t - 5 {
                3
            } else if x <= t {
                2
            } else if sp.soft && x <= t + 2 {
                1
            } else {
                0
            };
            (deg, x)
        }
        Res::D20 => {
            let x = d_(r, 20);
            let tot = x + 2 * s;
            let dc = 8 + 3 * d;
            let deg = if x == 1 {
                0
            } else if x == 20 || tot >= dc + 7 {
                3
            } else if tot >= dc {
                2
            } else if sp.soft && tot >= dc - 3 {
                1
            } else {
                0
            };
            (deg, tot)
        }
        Res::Pool => {
            let n = (s + 2).clamp(1, 9);
            let mut succ = 0;
            for _ in 0..n {
                if d_(r, 6) >= sp.tn {
                    succ += 1;
                }
            }
            let need = 1 + d / 3;
            let deg = if succ == 0 {
                0
            } else if succ < need {
                1
            } else if succ >= need + 2 {
                3
            } else {
                2
            };
            (deg, succ)
        }
        Res::Quad => {
            let mut f = 0;
            for _ in 0..4 {
                f += d_(r, 3) - 2;
            }
            let t = f + s - d;
            let deg = if t < 0 {
                if sp.soft && t == -1 {
                    1
                } else {
                    0
                }
            } else if t == 0 {
                1
            } else if t <= 2 {
                2
            } else {
                3
            };
            (deg, t)
        }
        Res::Twin => {
            let t = d_(r, 6) + d_(r, 6) + s - d;
            let deg = if t <= 6 {
                0
            } else if t <= 9 {
                1
            } else if t <= 12 {
                2
            } else {
                3
            };
            (deg, t)
        }
        Res::Step => {
            let size = (4 + 2 * s).clamp(4, 12) as u32;
            let mut tot = 0;
            for _ in 0..3 {
                let x = d_(r, size);
                tot += x;
                if x != size as i32 {
                    break;
                }
            }
            let t = 3 + d;
            let deg = if tot >= t + 4 {
                3
            } else if tot >= t {
                2
            } else if sp.soft && tot >= t - 1 {
                1
            } else {
                0
            };
            (deg, tot)
        }
        Res::Best => {
            let mut hi;
            let mut sixes = 0;
            if s <= 0 {
                let a = d_(r, 6);
                let b = d_(r, 6);
                hi = a.min(b);
                if hi == 6 {
                    sixes = 1;
                }
            } else {
                hi = 0;
                for _ in 0..s.min(6) {
                    let x = d_(r, 6);
                    if x == 6 {
                        sixes += 1;
                    }
                    hi = hi.max(x);
                }
            }
            if d >= 4 && hi > 1 {
                hi -= 1;
                if hi == 5 {
                    sixes = 0;
                }
            }
            let deg = if hi >= 6 {
                if sixes >= 2 {
                    3
                } else {
                    2
                }
            } else if hi >= 4 {
                1
            } else {
                0
            };
            (deg, hi)
        }
        Res::Keep => {
            let k = (s + 2).clamp(2, 8) as usize;
            let keep = (s.clamp(1, 6) as usize).min(k);
            let mut v: Vec<i32> = Vec::with_capacity(k + 2);
            for _ in 0..k {
                let mut x = d_(r, 10);
                if x == 10 {
                    x += d_(r, 10);
                }
                v.push(x);
            }
            v.sort_unstable_by(|a, b| b.cmp(a));
            let tot: i32 = v.iter().take(keep).sum();
            let t = 8 + 6 * d;
            let deg = if tot >= t + 8 {
                3
            } else if tot >= t {
                2
            } else if sp.soft && tot >= t - 4 {
                1
            } else {
                0
            };
            (deg, tot)
        }
        Res::Contest => {
            let act = d_(r, 6) + s - (d - 1).max(0);
            let c1 = d_(r, 10);
            let c2 = d_(r, 10);
            let beats = (act > c1) as i32 + (act > c2) as i32;
            let deg = match beats {
                2 => {
                    if c1 == c2 {
                        3
                    } else {
                        2
                    }
                }
                1 => 1,
                _ => 0,
            };
            (deg, act)
        }
        Res::Doubles => {
            let n = (s + 1).clamp(2, 6) as usize;
            let mut tot = 0;
            let mut seen = [0u8; 7];
            for _ in 0..n {
                let x = d_(r, 6);
                tot += x;
                seen[x as usize] += 1;
            }
            let mut bonus = 0;
            for v in 1..7 {
                if seen[v] >= 2 && bonus < 2 {
                    tot += d_(r, 6);
                    bonus += 1;
                }
            }
            let t = 8 + 4 * d;
            let deg = if tot >= t + 8 {
                3
            } else if tot >= t {
                2
            } else if sp.soft && tot >= t - 3 {
                1
            } else {
                0
            };
            (deg, tot)
        }
    }
}

fn ev_text(k: EvK, n: i32) -> String {
    match k {
        EvK::Pass => "liniște: nu se întâmplă nimic".to_string(),
        EvK::Hurt => format!("capcană: amândoi pierd {n} PV"),
        EvK::Lead => format!("primejdia îl lovește pe cel din față: −{n} PV"),
        EvK::Mend => format!("ajutor: cel din urmă vindecă {n} PV"),
        EvK::Gift => format!("izvor: amândoi primesc +{n} focus"),
        EvK::Fortune => format!("șansă: cel din urmă primește +{n} noroc"),
        EvK::Fright => format!("spaimă: amândoi +{n} stres"),
        EvK::Setback => format!("piedică: cel din față pierde {n} din progres (sau focus)"),
        EvK::Rally => format!("avânt: cel din urmă câștigă {n} (progres sau PV)"),
        EvK::Calm => format!("răgaz: amândoi −{n} stres"),
    }
}

impl Rpg {
    pub fn new(idx: usize) -> Self {
        Self::with_seed(idx, seed_now())
    }

    pub fn with_seed(idx: usize, seed: u64) -> Self {
        Self::with_spec(&SPECS[idx], seed)
    }

    pub fn with_spec(sp: &'static Spec, seed: u64) -> Self {
        let hero = Hero {
            hp: sp.hp,
            foc: sp.foc0,
            luck: sp.luck0,
            ..Hero::default()
        };
        Rpg {
            sp,
            rng: seed_state(seed),
            side: 0,
            h: [hero; 2],
            round: 0,
            ph: Ph::Event,
            act: Act::Atac,
            deg: 0,
            info: 0,
            rerolled: false,
            acted: 0,
            again: false,
            bonus: false,
            last: String::new(),
        }
    }

    /// Oboseala: din ultima treime a jocului, fiecare eveniment al MJ-ului costă 1 PV eroului din urmă.
    fn fatigue(&self) -> bool {
        self.round * 3 >= self.sp.rounds * 2
    }

    fn lost(&self, i: usize) -> bool {
        let h = &self.h[i];
        h.hp <= 0 || (self.sp.stress_max > 0 && h.stress >= self.sp.stress_max)
    }

    fn progress(&self, i: usize) -> i32 {
        match self.sp.win {
            Win::Kill => self.h[i].hp,
            Win::Road => self.h[i].pos,
            Win::Glory => self.h[i].glory,
        }
    }

    /// Cine e „în față” (progres, apoi PV); `None` la egalitate.
    fn ahead(&self) -> Option<usize> {
        let a = (self.progress(0), self.h[0].hp);
        let b = (self.progress(1), self.h[1].hp);
        match a.cmp(&b) {
            std::cmp::Ordering::Greater => Some(0),
            std::cmp::Ordering::Less => Some(1),
            std::cmp::Ordering::Equal => None,
        }
    }

    fn apply_ev(&mut self, k: EvK, n: i32) {
        let ahead = self.ahead();
        let max = self.sp.hp;
        let win = self.sp.win;
        match k {
            EvK::Pass => {}
            EvK::Hurt => {
                for h in self.h.iter_mut() {
                    h.hp -= n;
                }
            }
            EvK::Lead => {
                if let Some(a) = ahead {
                    self.h[a].hp -= n;
                }
            }
            EvK::Mend => {
                if let Some(a) = ahead {
                    let b = &mut self.h[1 - a];
                    b.hp = (b.hp + n).min(max);
                }
            }
            EvK::Gift => {
                for h in self.h.iter_mut() {
                    h.foc = (h.foc + n).min(6);
                }
            }
            EvK::Fortune => {
                if let Some(a) = ahead {
                    let b = &mut self.h[1 - a];
                    b.luck = (b.luck + n).min(5);
                }
            }
            EvK::Fright => {
                for h in self.h.iter_mut() {
                    h.stress += n;
                }
            }
            EvK::Setback => {
                if let Some(a) = ahead {
                    let h = &mut self.h[a];
                    match win {
                        Win::Road => h.pos = (h.pos - n).max(0),
                        Win::Glory => h.glory = (h.glory - n).max(0),
                        Win::Kill => h.foc = (h.foc - n).max(0),
                    }
                }
            }
            EvK::Rally => {
                if let Some(a) = ahead {
                    let h = &mut self.h[1 - a];
                    match win {
                        Win::Road => h.pos += n,
                        Win::Glory => h.glory += n,
                        Win::Kill => h.hp = (h.hp + n).min(max),
                    }
                }
            }
            EvK::Calm => {
                for h in self.h.iter_mut() {
                    h.stress = (h.stress - n).max(0);
                }
            }
        }
    }

    fn skill(&self, a: usize, act: Act) -> i32 {
        let s = &self.sp.st[a];
        let mut v = match act {
            Act::Atac => s[0],
            Act::Apar => s[1],
            Act::Arta => s[2],
            Act::Tinta => {
                if self.sp.win == Win::Road {
                    s[1]
                } else {
                    s[2]
                }
            }
        };
        v -= self.h[a].curse;
        match self.sp.twist {
            Twist::Edge if self.acted == 1 => v += 1,
            Twist::Fear if self.sp.stress_max > 0 && self.h[a].stress * 2 >= self.sp.stress_max => {
                v -= 1
            }
            _ => {}
        }
        v
    }

    fn difficulty(&self, a: usize, act: Act) -> i32 {
        let b = self.sp.base;
        match act {
            Act::Atac => b + self.sp.st[1 - a][1] - 1,
            Act::Tinta => b + 1,
            _ => b,
        }
    }

    fn post_moves(&self) -> Vec<&'static str> {
        let a = &self.h[self.side];
        let mut v = vec!["accept"];
        match self.sp.post {
            Post::None => {}
            Post::Luck => {
                if a.luck >= 1 && self.deg < 3 {
                    v.push("noroc");
                }
            }
            Post::Push => {
                if self.deg < 2 && !self.rerolled {
                    v.push("forteaza");
                }
            }
            Post::Cost => {
                if self.deg == 1 && a.hp > 1 {
                    v.push("plata");
                }
            }
            Post::Burn => {
                if a.luck >= 2 && self.deg < 2 {
                    v.push("arde");
                }
            }
        }
        v
    }

    /// Efectul acțiunii curente, cu treapta `self.deg` (fără schimbarea turei).
    fn apply_effect(&mut self) {
        let a = self.side;
        let o = 1 - a;
        let deg = self.deg.clamp(0, 3);
        let sp = self.sp;
        let max = sp.hp;
        match self.act {
            Act::Atac => {
                let mut dmg = sp.dmg[deg as usize];
                if dmg > 0 {
                    dmg += self.h[o].mark;
                    self.h[o].mark = 0;
                    dmg = (dmg - self.h[o].guard).max(0);
                    self.h[o].guard = 0;
                    self.h[o].hp -= dmg;
                    if sp.twist == Twist::Wound && dmg >= 3 {
                        self.h[o].stress += 1;
                    }
                    if sp.twist == Twist::Rage && dmg > 0 {
                        self.h[o].foc = (self.h[o].foc + 1).min(6);
                    }
                } else if dmg < 0 {
                    self.h[a].hp += dmg;
                }
            }
            Act::Apar => {
                self.h[a].guard += sp.guard[deg as usize];
            }
            Act::Arta => {
                let pow = sp.art_pow;
                let p = match deg {
                    0 => 0,
                    1 => (pow + 1) / 2,
                    2 => pow,
                    _ => pow + 1,
                };
                match sp.art {
                    Art::Bolt => self.h[o].hp -= p,
                    Art::Heal => self.h[a].hp = (self.h[a].hp + p).min(max),
                    Art::Drain => {
                        self.h[o].hp -= p;
                        self.h[a].hp = (self.h[a].hp + (p + 1) / 2).min(max);
                    }
                    Art::Curse => {
                        self.h[o].curse = (self.h[o].curse + (p + 1) / 2).min(3);
                        self.h[o].hp -= p / 2;
                    }
                    Art::Haste => {
                        if deg >= 2 && !self.bonus {
                            self.again = true;
                        } else if deg == 1 {
                            self.h[a].luck = (self.h[a].luck + 1).min(5);
                        }
                    }
                    Art::Ward => {
                        if p > 0 {
                            self.h[a].guard += p + 1;
                            self.h[a].hp = (self.h[a].hp + p / 2).min(max);
                        }
                    }
                    Art::Shatter => {
                        self.h[o].foc = (self.h[o].foc - p).max(0);
                        self.h[o].guard = 0;
                        self.h[o].hp -= (p + 1) / 2;
                    }
                    Art::Mark => self.h[o].mark = (self.h[o].mark + p).min(4),
                }
            }
            Act::Tinta => {
                let g = sp.step[deg as usize];
                match sp.win {
                    Win::Road => self.h[a].pos += g,
                    Win::Glory => self.h[a].glory += g,
                    Win::Kill => {}
                }
            }
        }
        if deg == 3 || (sp.post == Post::Burn && deg == 2) {
            self.h[a].luck = (self.h[a].luck + 1).min(6);
        }
        if deg == 0 {
            if sp.fail_stress {
                self.h[a].stress += 1;
            }
            if sp.twist == Twist::Fumble {
                self.h[a].hp -= 1;
            }
        }
        if sp.twist == Twist::Surge && deg == 3 && !self.bonus {
            self.again = true;
        }
    }

    fn resolve(&mut self) {
        self.apply_effect();
        let a = self.side;
        self.h[a].curse = 0;
        self.end_turn();
    }

    fn begin_turn(&mut self) {
        let a = self.side;
        self.h[a].guard = 0;
    }

    fn end_turn(&mut self) {
        if self.again && self.result().is_none() {
            self.again = false;
            self.bonus = true;
            self.ph = Ph::Choose;
            return;
        }
        self.again = false;
        self.bonus = false;
        self.acted += 1;
        if self.acted >= 2 {
            self.round += 1;
            self.acted = 0;
            self.ph = Ph::Event;
            self.side = (self.round % 2) as usize;
        } else {
            self.side = 1 - self.side;
            self.ph = Ph::Choose;
            self.begin_turn();
        }
    }

    fn value(&self, i: usize) -> i32 {
        let h = &self.h[i];
        let sp = self.sp;
        let mut v = h.hp * 10 + h.foc * 3 + h.luck * 3 + h.guard * 2 - h.curse * 5 - h.mark * 4;
        if sp.stress_max > 0 {
            v -= h.stress * (90 / sp.stress_max);
        }
        match sp.win {
            Win::Road => v += h.pos * (170 / sp.goal.max(1)),
            Win::Glory => v += h.glory * (170 / sp.goal.max(1)),
            Win::Kill => {}
        }
        v
    }
}

impl Mini for Rpg {
    fn side(&self) -> Player {
        pl(self.side)
    }

    fn moves(&self) -> Vec<String> {
        let a = &self.h[self.side];
        let mut v: Vec<&str> = Vec::new();
        match self.ph {
            Ph::Event | Ph::Roll => v.push("tira"),
            Ph::Choose => {
                v.push("atac");
                v.push("aparare");
                if a.foc >= self.sp.art_cost {
                    v.push("arta");
                }
                v.push("odihna");
                if self.sp.win != Win::Kill {
                    v.push("tinta");
                }
            }
            Ph::Post => v = self.post_moves(),
        }
        v.into_iter().map(String::from).collect()
    }

    fn step(&mut self, m: &str) {
        let a = self.side;
        match self.ph {
            Ph::Event => {
                let k = die(&mut self.rng, 6) as usize;
                let (kk, n) = self.sp.ev[k];
                self.apply_ev(kk, n);
                if self.fatigue() {
                    let t = match self.ahead() {
                        Some(a) => 1 - a,
                        None => 1 - self.side,
                    };
                    self.h[t].hp -= 1;
                }
                self.last = format!("MJ (d6={}): {}", k + 1, ev_text(kk, n));
                self.ph = Ph::Choose;
                self.acted = 0;
                self.bonus = false;
                self.again = false;
                self.begin_turn();
            }
            Ph::Choose => {
                let act = match m {
                    "odihna" => {
                        let sp = self.sp;
                        let h = &mut self.h[a];
                        h.hp = (h.hp + sp.rest_hp).min(sp.hp);
                        h.foc = (h.foc + sp.rest_foc).min(6);
                        h.stress = (h.stress - 1).max(0);
                        self.last = format!("{} se odihnește", pname(pl(a)));
                        self.end_turn();
                        return;
                    }
                    "atac" => Act::Atac,
                    "aparare" => Act::Apar,
                    "arta" => Act::Arta,
                    _ => Act::Tinta,
                };
                if act == Act::Arta {
                    self.h[a].foc -= self.sp.art_cost;
                    if self.sp.twist == Twist::Tithe {
                        self.h[a].stress += 1;
                    }
                }
                self.act = act;
                self.rerolled = false;
                self.ph = Ph::Roll;
            }
            Ph::Roll => {
                let s = self.skill(a, self.act);
                let d = self.difficulty(a, self.act);
                let (deg, info) = roll_deg(self.sp, &mut self.rng, s, d);
                self.deg = deg;
                self.info = info;
                self.last = format!(
                    "{} ({}): aruncare {} → {}",
                    pname(pl(a)),
                    match self.act {
                        Act::Atac => "atac",
                        Act::Apar => "apărare",
                        Act::Arta => "artă",
                        Act::Tinta => "țintă",
                    },
                    info,
                    DEG[deg as usize]
                );
                if self.post_moves().len() > 1 {
                    self.ph = Ph::Post;
                } else {
                    self.resolve();
                }
            }
            Ph::Post => {
                match m {
                    "noroc" => {
                        self.h[a].luck -= 1;
                        self.deg = (self.deg + 1).min(3);
                    }
                    "forteaza" => {
                        self.h[a].stress += 1;
                        self.rerolled = true;
                        self.ph = Ph::Roll;
                        return;
                    }
                    "plata" => {
                        self.h[a].hp -= 1;
                        self.deg = 2;
                    }
                    "arde" => {
                        self.h[a].luck = 0;
                        self.deg = 2;
                    }
                    _ => {}
                }
                self.resolve();
            }
        }
    }

    fn result(&self) -> Option<Outcome> {
        let (l0, l1) = (self.lost(0), self.lost(1));
        if l0 && l1 {
            return Some(Outcome::Draw);
        }
        if l0 {
            return Some(Outcome::Win(Player::Two));
        }
        if l1 {
            return Some(Outcome::Win(Player::One));
        }
        let sp = self.sp;
        if sp.win != Win::Kill {
            let reach = |i: usize| self.progress(i) >= sp.goal;
            match (reach(0), reach(1)) {
                (true, true) => {
                    return Some(by_score(self.h[0].hp, self.h[1].hp));
                }
                (true, false) => return Some(Outcome::Win(Player::One)),
                (false, true) => return Some(Outcome::Win(Player::Two)),
                _ => {}
            }
        }
        if self.round >= sp.rounds {
            let a = (
                self.progress(0),
                self.h[0].hp,
                self.h[0].luck,
                -self.h[0].stress,
            );
            let b = (
                self.progress(1),
                self.h[1].hp,
                self.h[1].luck,
                -self.h[1].stress,
            );
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn eval(&self, p: Player) -> i32 {
        let me = pi(p);
        let ot = 1 - me;
        let base = self.value(me) - self.value(ot);
        if self.ph != Ph::Roll {
            return base;
        }
        // rezultat așteptat al aruncării în așteptare, cu un generator local fix (nu citește viitorul)
        let n = 12;
        let mut lr: u64 = seed_state(0x7157 ^ (self.round as u64 * 977) ^ (self.act as u64));
        let a = self.side;
        let s = self.skill(a, self.act);
        let d = self.difficulty(a, self.act);
        let mut tot = 0;
        for _ in 0..n {
            let (deg, _) = roll_deg(self.sp, &mut lr, s, d);
            let mut c = self.clone();
            c.deg = deg;
            c.apply_effect();
            tot += c.value(me) - c.value(ot);
        }
        tot / n
    }

    fn show(&self) -> String {
        let sp = self.sp;
        let mut s = format!(
            "=== {} === runda {}/{} · la rând: {}\n",
            sp.title,
            (self.round + 1).min(sp.rounds),
            sp.rounds,
            pname(pl(self.side))
        );
        for i in 0..2 {
            let h = &self.h[i];
            let mut line = format!(
                "{} [{}] PV {}/{} · focus {} · noroc {}",
                pname(pl(i)),
                sp.cls[i],
                h.hp.max(0),
                sp.hp,
                h.foc,
                h.luck
            );
            if sp.stress_max > 0 {
                line += &format!(" · stres {}/{}", h.stress, sp.stress_max);
            }
            match sp.win {
                Win::Road => line += &format!(" · drum {}/{}", h.pos, sp.goal),
                Win::Glory => line += &format!(" · faimă {}/{}", h.glory, sp.goal),
                Win::Kill => {}
            }
            if h.guard > 0 {
                line += &format!(" · gardă {}", h.guard);
            }
            if h.curse > 0 {
                line += &format!(" · blestem {}", h.curse);
            }
            if h.mark > 0 {
                line += &format!(" · marcat {}", h.mark);
            }
            s += &line;
            s.push('\n');
        }
        if self.fatigue() {
            s += "Oboseală: fiecare eveniment costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea).\n";
        }
        s += "Evenimente MJ (d6): ";
        for (i, (k, n)) in sp.ev.iter().enumerate() {
            s += &format!("{}={} ", i + 1, ev_text(*k, *n));
            if i < 5 {
                s += "; ";
            }
        }
        s.push('\n');
        match self.ph {
            Ph::Event => s += "MJ-ul trage un eveniment (tira).\n",
            Ph::Choose => s += "Alege acțiunea: atac, aparare, arta, odihna, tinta.\n",
            Ph::Roll => s += "Aruncă zarurile (tira).\n",
            Ph::Post => {
                s += &format!(
                    "Rezultat: {} (aruncare {}). Decide: {}.\n",
                    DEG[self.deg.clamp(0, 3) as usize],
                    self.info,
                    self.post_moves().join(" / ")
                )
            }
        }
        if !self.last.is_empty() {
            s += &format!("Ultima: {}\n", self.last);
        }
        s
    }

    fn depth(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_does_not_read_the_future() {
        for idx in 0..SPECS.len() {
            let mut m = Rpg::with_seed(idx, 11 + idx as u64);
            advance(&mut m, 200, |m| m.ph == Ph::Roll);
            if m.ph == Ph::Roll {
                no_peek(&m, |h| h.rng ^= 0x5DEE_CE66_D1CE_4E5B);
            }
        }
    }

    #[test]
    fn spec_sanity() {
        for (i, sp) in SPECS.iter().enumerate() {
            assert!(sp.hp > 0 && sp.rounds >= 8, "{}", sp.id);
            if sp.win != Win::Kill {
                assert!(sp.goal > 0, "{}", sp.id);
            }
            let sum = |k: usize| sp.st[k].iter().sum::<i32>();
            assert!(
                (sum(0) - sum(1)).abs() <= 1,
                "{} fișe dezechilibrate",
                sp.id
            );
            for j in 0..i {
                assert_ne!(SPECS[j].id, sp.id);
            }
        }
    }

    #[test]
    fn ai_beats_random_overall() {
        use polytabletop_core::rng::Rng;
        use polytabletop_core::GameObj;
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let (mut ai_w, mut rnd_w) = (0, 0);
        for idx in (0..SPECS.len()).step_by(3) {
            for seed in 1..=2u64 {
                let ai_side = (seed % 2) as usize;
                let mut g = B(Rpg::with_seed(idx, seed));
                let mut rng = Rng::new(seed + 99);
                let mut n = 0;
                while GameObj::outcome(&g).is_none() && n < 900 {
                    let lm = GameObj::legal_moves(&g);
                    let mv = if g.0.side == ai_side {
                        GameObj::ai_move(&g, rng.next_u64()).unwrap()
                    } else {
                        lm[rng.below(lm.len())].clone()
                    };
                    GameObj::play(&mut g, &mv).unwrap();
                    n += 1;
                }
                if let Some(Outcome::Win(p)) = GameObj::outcome(&g) {
                    if pi(p) == ai_side {
                        ai_w += 1
                    } else {
                        rnd_w += 1
                    }
                }
            }
        }
        assert!(ai_w > rnd_w, "AI {ai_w} vs aleator {rnd_w}");
    }
}
