//! Motor „drum cu întâlniri” (familia T2, variante originale pentru doi jucători).
//!
//! Fiecare jucător își alege un erou (`alege k`, public) și parcurge propriul drum de `stations`
//! stații; la fiecare stație Povestitorul automat (zarul întâmplării, mutarea forțată `tira`)
//! scoate o întâlnire dintr-un tabel propriu jocului. Pe rând alegeți abordarea: `infrunt`,
//! `furis`, `vorbesc` (cu variantele `+` = un zar în plus pe resursă), `odihnesc`, `capcana`
//! (la adversar), `obiect`. Zarurile se aruncă doar în `tira`; AI-ul nu le vede dinainte.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct Enc {
    pub name: &'static str,
    /// 0 = primejdie (se înfruntă), 1 = capcană/teren (se furișează), 2 = om (se vorbește)
    pub kind: u8,
    pub threat: i8,
    pub reward: i8,
    pub dmg: i8,
}

pub struct Hero {
    pub name: &'static str,
    pub hp: i8,
    pub fight: i8,
    pub sneak: i8,
    pub talk: i8,
    pub item: &'static str,
}

pub const TRAP: u8 = 1;
pub const ITEM: u8 = 2;
pub const BLOOD: u8 = 4;
pub const CALM: u8 = 8;

pub struct Cfg {
    pub title: &'static str,
    pub res: &'static str,
    pub sides: i8,
    pub tn: i8,
    pub flags: u8,
    pub stations: usize,
    pub turns: u32,
    pub bonus: i32,
    pub heroes: &'static [Hero],
    pub enc: &'static [Enc],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Pick,
    Act,
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Act {
    Fight(bool),
    Sneak(bool),
    Talk(bool),
    Item,
}

#[derive(Clone)]
pub struct Drum {
    cfg: &'static Cfg,
    ph: Ph,
    picked: [bool; 2],
    cls: [usize; 2],
    hp: [i32; 2],
    pos: [usize; 2],
    score: [i32; 2],
    res: [i32; 2],
    item: [bool; 2],
    trap: [i32; 2],
    cur: [usize; 2],
    done: [bool; 2],
    first: Option<usize>,
    n: [u32; 2],
    turn: usize,
    pend: Option<Act>,
    rng: Rng,
}

/// Probabilitatea (în miimi) de cel puțin `need` reușite din `n` zaruri cu șansa `p` (în miimi).
fn tail(n: i32, p: i32, need: i32) -> i32 {
    if need <= 0 {
        return 1000;
    }
    if need > n {
        return 0;
    }
    let mut dp = vec![0i64; (n + 1) as usize];
    dp[0] = 1_000_000;
    for _ in 0..n {
        let mut nx = vec![0i64; (n + 1) as usize];
        for k in 0..=n as usize {
            if dp[k] == 0 {
                continue;
            }
            nx[k] += dp[k] * (1000 - p as i64) / 1000;
            if k < n as usize {
                nx[k + 1] += dp[k] * p as i64 / 1000;
            }
        }
        dp = nx;
    }
    (dp[need as usize..].iter().sum::<i64>() / 1000) as i32
}

impl Drum {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let mut d = Drum {
            cfg,
            ph: Ph::Pick,
            picked: [false; 2],
            cls: [0; 2],
            hp: [0; 2],
            pos: [0; 2],
            score: [0; 2],
            res: [0; 2],
            item: [false; 2],
            trap: [0; 2],
            cur: [0; 2],
            done: [false; 2],
            first: None,
            n: [0; 2],
            turn: 0,
            pend: None,
            rng: Rng::new(seed),
        };
        for p in 0..2 {
            d.cur[p] = d.rng.below(cfg.enc.len());
        }
        d
    }
    fn h(&self, p: usize) -> &'static Hero {
        &self.cfg.heroes[self.cls[p]]
    }
    fn has(&self, f: u8) -> bool {
        self.cfg.flags & f != 0
    }
    fn e(&self, p: usize) -> &'static Enc {
        &self.cfg.enc[self.cur[p]]
    }
    fn p1000(&self) -> i32 {
        ((self.cfg.sides as i32 - self.cfg.tn as i32 + 1) * 1000 / self.cfg.sides as i32)
            .clamp(0, 1000)
    }
    /// (zaruri, necesar, recompensă, pierdere la eșec, reușită sigură)
    fn plan(&self, t: usize, act: Act) -> (i32, i32, i32, i32, bool) {
        let h = self.h(t);
        let e = self.e(t);
        let th = e.threat as i32;
        let k = e.kind as usize;
        let full = e.reward as i32;
        let half = (full + 1) / 2;
        let blood = if self.has(BLOOD) { 1 } else { 0 };
        let dm = e.dmg as i32 + blood;
        let trap = self.trap[t];
        let (dice, need, rew, loss, sure) = match act {
            Act::Fight(pl) => (
                h.fight as i32 + pl as i32,
                [th, th + 1, th + 1][k],
                if k == 0 { full } else { half },
                dm,
                false,
            ),
            Act::Sneak(pl) => (
                h.sneak as i32 + pl as i32,
                [th, th - 1, th][k],
                if k == 1 { full } else { half },
                (dm + 1) / 2,
                false,
            ),
            Act::Talk(pl) => (
                h.talk as i32 + pl as i32,
                [th + 1, th + 1, th - 1][k],
                if k == 2 { full } else { half },
                1 + blood,
                false,
            ),
            Act::Item => (0, 0, full, 0, true),
        };
        (dice, (need + trap).max(1), rew + blood, loss, sure)
    }
    fn finish_turn(&mut self) {
        self.n[self.turn] += 1;
        self.ph = Ph::Act;
        let o = 1 - self.turn;
        if !self.done[o] {
            self.turn = o;
        }
    }
    fn resolve(&mut self) {
        let t = self.turn;
        let act = self.pend.take().unwrap();
        let (dice, need, rew, loss, sure) = self.plan(t, act);
        let mut hits = 0;
        if !sure {
            for _ in 0..dice {
                if 1 + self.rng.below(self.cfg.sides as usize) as i32 >= self.cfg.tn as i32 {
                    hits += 1;
                }
            }
        }
        self.trap[t] = 0;
        if sure || hits >= need {
            self.score[t] += rew;
            self.res[t] = (self.res[t] + 1).min(3);
            self.pos[t] += 1;
            if self.pos[t] >= self.cfg.stations {
                self.done[t] = true;
                if self.first.is_none() {
                    self.first = Some(t);
                    self.score[t] += self.cfg.bonus;
                }
            } else {
                self.cur[t] = self.rng.below(self.cfg.enc.len());
            }
        } else {
            self.hp[t] -= loss;
            if self.hp[t] <= 0 {
                self.score[t] = (self.score[t] - 2).max(0);
                self.hp[t] = (self.h(t).hp as i32 + 1) / 2;
                self.res[t] = 0;
            }
        }
    }
    fn total(&self, p: usize) -> i32 {
        self.score[p]
    }
}

impl Mini for Drum {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.turn == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let t = self.turn;
        match self.ph {
            Ph::Pick => (0..self.cfg.heroes.len())
                .map(|k| format!("alege {k}"))
                .collect(),
            Ph::Roll => vec!["tira".into()],
            Ph::Act => {
                let mut v = vec!["infrunt".to_string(), "furis".into(), "vorbesc".into()];
                if self.res[t] >= 1 {
                    v.push("infrunt+".into());
                    v.push("furis+".into());
                    v.push("vorbesc+".into());
                }
                if self.hp[t] < self.h(t).hp as i32 {
                    v.push("odihnesc".into());
                }
                if self.has(TRAP) && self.res[t] >= 1 && !self.done[1 - t] {
                    v.push("capcana".into());
                }
                if self.has(ITEM) && self.item[t] {
                    v.push("obiect".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let t = self.turn;
        let mut it = m.split(' ');
        let w = it.next().unwrap();
        match w {
            "alege" => {
                let k: usize = it.next().unwrap().parse().unwrap();
                self.cls[t] = k;
                self.picked[t] = true;
                self.hp[t] = self.cfg.heroes[k].hp as i32;
                self.res[t] = 1;
                self.item[t] = true;
                if t == 0 {
                    self.turn = 1;
                } else {
                    self.turn = 0;
                    self.ph = Ph::Act;
                }
            }
            "infrunt" | "furis" | "vorbesc" | "infrunt+" | "furis+" | "vorbesc+" => {
                let plus = w.ends_with('+');
                if plus {
                    self.res[t] -= 1;
                }
                self.pend = Some(match w.trim_end_matches('+') {
                    "infrunt" => Act::Fight(plus),
                    "furis" => Act::Sneak(plus),
                    _ => Act::Talk(plus),
                });
                self.ph = Ph::Roll;
            }
            "obiect" => {
                self.item[t] = false;
                self.pend = Some(Act::Item);
                self.ph = Ph::Roll;
            }
            "odihnesc" => {
                let heal = if self.has(CALM) { 3 } else { 2 };
                self.hp[t] = (self.hp[t] + heal).min(self.h(t).hp as i32);
                self.res[t] = (self.res[t] + 1).min(3);
                self.finish_turn();
            }
            "capcana" => {
                self.res[t] -= 1;
                self.trap[1 - t] += 1;
                self.finish_turn();
            }
            _ => {
                self.resolve();
                self.finish_turn();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph == Ph::Pick {
            return None;
        }
        let over = (self.done[0] && self.done[1])
            || (self.n[0] >= self.cfg.turns && self.n[1] >= self.cfg.turns && self.ph == Ph::Act);
        if !over {
            return None;
        }
        let a = (self.total(0), self.pos[0] as i32, self.hp[0]);
        let b = (self.total(1), self.pos[1] as i32, self.hp[1]);
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        if self.ph == Ph::Pick || !(self.picked[0] && self.picked[1]) {
            let f = |x: usize| {
                if self.picked[x] {
                    let h = self.h(x);
                    h.hp as i32 + 2 * (h.fight + h.sneak + h.talk) as i32
                } else {
                    0
                }
            };
            return 10 * (f(i) - f(o));
        }
        let mut s = 10 * (self.total(i) - self.total(o))
            + 6 * (self.pos[i] as i32 - self.pos[o] as i32)
            + 2 * (self.hp[i] - self.hp[o])
            + 2 * (self.res[i] - self.res[o])
            + 3 * (self.trap[o] - self.trap[i])
            + if self.item[i] { 2 } else { 0 }
            - if self.item[o] { 2 } else { 0 };
        if let Some(act) = self.pend {
            let t = self.turn;
            let (dice, need, rew, loss, sure) = self.plan(t, act);
            let ps = if sure {
                1000
            } else {
                tail(dice, self.p1000(), need)
            };
            let ev = (ps * (rew * 10 + 6) - (1000 - ps) * loss * 3) / 1000;
            if t == i {
                s += ev;
            } else {
                s -= ev;
            }
        }
        s
    }
    fn show(&self) -> String {
        let cfg = self.cfg;
        let mut s = format!(
            "{} — drum cu întâlniri, {} stații, tura {}/{}\n",
            cfg.title,
            cfg.stations,
            self.n[self.turn.min(1)] + 1,
            cfg.turns
        );
        for (k, h) in cfg.heroes.iter().enumerate() {
            s.push_str(&format!(
                "  erou {k}: {} (PV {}, luptă {}, furiș {}, vorbă {}, obiect «{}»)\n",
                h.name, h.hp, h.fight, h.sneak, h.talk, h.item
            ));
        }
        for p in 0..2 {
            if !self.picked[p] {
                s.push_str(&format!("J{}: încă nu a ales eroul\n", p + 1));
                continue;
            }
            let h = self.h(p);
            s.push_str(&format!(
                "J{}: {} PV {}/{} | stația {}/{} | puncte {} | {} {}{}{}{}\n",
                p + 1,
                h.name,
                self.hp[p],
                h.hp,
                self.pos[p],
                cfg.stations,
                self.score[p],
                cfg.res,
                self.res[p],
                if self.item[p] { " | obiect" } else { "" },
                if self.trap[p] > 0 {
                    " | în capcană"
                } else {
                    ""
                },
                if self.turn == p { " ◄" } else { "" }
            ));
            if !self.done[p] {
                let e = self.e(p);
                s.push_str(&format!(
                    "     întâlnire: {} ({}, pericol {}, câștig {}, rană {})\n",
                    e.name,
                    ["primejdie", "teren", "om"][e.kind as usize],
                    e.threat,
                    e.reward,
                    e.dmg
                ));
            } else {
                s.push_str("     a terminat drumul\n");
            }
        }
        match self.ph {
            Ph::Pick => s.push_str(&format!("J{} alege eroul: alege k\n", self.turn + 1)),
            Ph::Roll => s.push_str("Povestitorul așteaptă: tira\n"),
            Ph::Act => {}
        }
        s
    }
}

#[cfg(test)]
impl Drum {
    fn reseed(&mut self, s: u64) {
        self.rng = Rng::new(s);
    }
}

#[cfg(test)]
pub fn check_cfg(cfg: &'static Cfg) {
    crate::t2_util::check_fn(&|s| Drum::with_seed(cfg, s), 600, Drum::reseed);
}
