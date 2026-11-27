//! Motor cooperativ „temniță pe grilă” (lista C3, variante originale, 2 jucători = doi eroi).
//!
//! Doi eroi (J1, J2) pe o grilă; monștrii sunt conduși de joc, fără zaruri: faza forțată `mon`
//! (după tura ambilor eroi) mută fiecare monstru un pas spre cel mai apropiat erou sau îl lovește
//! dacă e alături; noi monștri apar din rezerva de val (poziții la întâmplare, doar în `mon`).
//! Acțiuni eroi: `mers r c` (pas ortogonal), `lov r c` (lovește monstrul alăturat), `vindec`
//! (+2 viață, de cel mult 2 ori pe partidă la echipă), `pas`.
//! Misiuni: Curățare (toți monștrii), Ieșire (ambii eroi pe ieșiri), Căpetenie (monstrul mare),
//! Rezistență (supraviețuiește `rounds` runde). Echipa pierde dacă un erou cade; la depășirea
//! rundelor (afară de Rezistență, unde e victorie) e remiză.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Goal {
    Clear,
    Exit,
    Boss,
    Survive,
}

pub struct GSpec {
    pub title: &'static str,
    pub w: i32,
    pub h: i32,
    pub hp: i32,
    pub dmg: i32,
    pub mdmg: i32,
    pub mhp: i32,
    pub monsters: usize,
    pub wave_every: u32,
    pub wave_size: usize,
    pub rounds: u32,
    pub ap: u8,
    pub goal: Goal,
    pub boss_hp: i32,
    pub walls: usize,
}

pub static SPECS: [GSpec; 6] = [
    GSpec {
        title: "Ritualul Adâncului",
        w: 6,
        h: 6,
        hp: 7,
        dmg: 1,
        mdmg: 1,
        mhp: 2,
        monsters: 4,
        wave_every: 3,
        wave_size: 1,
        rounds: 14,
        ap: 2,
        goal: Goal::Survive,
        boss_hp: 0,
        walls: 3,
    },
    GSpec {
        title: "Ceata Tenebrelor",
        w: 7,
        h: 6,
        hp: 8,
        dmg: 2,
        mdmg: 1,
        mhp: 3,
        monsters: 5,
        wave_every: 4,
        wave_size: 2,
        rounds: 24,
        ap: 2,
        goal: Goal::Clear,
        boss_hp: 0,
        walls: 4,
    },
    GSpec {
        title: "Cripta Eroilor",
        w: 6,
        h: 6,
        hp: 8,
        dmg: 2,
        mdmg: 2,
        mhp: 2,
        monsters: 4,
        wave_every: 0,
        wave_size: 0,
        rounds: 24,
        ap: 2,
        goal: Goal::Clear,
        boss_hp: 0,
        walls: 3,
    },
    GSpec {
        title: "Cetele Pustiei",
        w: 7,
        h: 5,
        hp: 9,
        dmg: 2,
        mdmg: 2,
        mhp: 3,
        monsters: 4,
        wave_every: 5,
        wave_size: 2,
        rounds: 26,
        ap: 3,
        goal: Goal::Boss,
        boss_hp: 8,
        walls: 3,
    },
    GSpec {
        title: "Călătoria Cetei",
        w: 8,
        h: 5,
        hp: 8,
        dmg: 1,
        mdmg: 1,
        mhp: 2,
        monsters: 5,
        wave_every: 0,
        wave_size: 0,
        rounds: 22,
        ap: 3,
        goal: Goal::Exit,
        boss_hp: 0,
        walls: 4,
    },
    GSpec {
        title: "Pădurea Jurămintelor",
        w: 7,
        h: 6,
        hp: 10,
        dmg: 2,
        mdmg: 2,
        mhp: 3,
        monsters: 3,
        wave_every: 4,
        wave_size: 1,
        rounds: 28,
        ap: 3,
        goal: Goal::Boss,
        boss_hp: 12,
        walls: 4,
    },
];

#[derive(Clone)]
struct Mon {
    r: i32,
    c: i32,
    hp: i32,
    boss: bool,
}

#[derive(Clone)]
pub struct Dung {
    sp: &'static GSpec,
    wall: Vec<bool>,
    hero: [(i32, i32); 2],
    hhp: [i32; 2],
    mons: Vec<Mon>,
    heals: u8,
    ap_left: u8,
    side: usize,
    round: u32,
    kills: u32,
    mon_phase: bool,
    rng: Rng,
}

pub fn new(i: usize) -> Dung {
    with_seed(i, seed_now())
}

pub fn with_seed(i: usize, seed: u64) -> Dung {
    let sp = &SPECS[i];
    let mut rng = Rng::new(seed);
    let (w, h) = (sp.w, sp.h);
    let mut wall = vec![false; (w * h) as usize];
    let heroes = [(h - 1, 0), (h - 1, 1)];
    let mut k = 0;
    while k < sp.walls {
        let r = rng.below(h as usize - 2) as i32 + 1;
        let c = rng.below(w as usize) as i32;
        if !wall[(r * w + c) as usize] && !(r >= h - 2 && c < 3) && !(r <= 1 && c >= w - 3) {
            wall[(r * w + c) as usize] = true;
            k += 1;
        }
    }
    let mut mons = Vec::new();
    let mut tries = 0;
    let want = sp.monsters + if sp.goal == Goal::Boss { 1 } else { 0 };
    while mons.len() < want && tries < 500 {
        tries += 1;
        let r = rng.below((h as usize).div_ceil(2)) as i32;
        let c = rng.below(w as usize) as i32;
        if wall[(r * w + c) as usize] || mons.iter().any(|m: &Mon| (m.r, m.c) == (r, c)) {
            continue;
        }
        let boss = sp.goal == Goal::Boss && mons.is_empty();
        mons.push(Mon {
            r,
            c,
            hp: if boss { sp.boss_hp } else { sp.mhp },
            boss,
        });
    }
    Dung {
        sp,
        wall,
        hero: heroes,
        hhp: [sp.hp; 2],
        mons,
        heals: 2,
        ap_left: sp.ap,
        side: 0,
        round: 0,
        kills: 0,
        mon_phase: false,
        rng,
    }
}

impl Dung {
    fn free(&self, r: i32, c: i32) -> bool {
        r >= 0
            && c >= 0
            && r < self.sp.h
            && c < self.sp.w
            && !self.wall[(r * self.sp.w + c) as usize]
            && !self.mons.iter().any(|m| (m.r, m.c) == (r, c))
            && !self
                .hero
                .iter()
                .enumerate()
                .any(|(i, h)| self.hhp[i] > 0 && *h == (r, c))
    }
    fn exits(&self) -> [(i32, i32); 2] {
        [(0, self.sp.w - 1), (1, self.sp.w - 1)]
    }
    fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }
    fn won(&self) -> bool {
        match self.sp.goal {
            Goal::Clear => self.mons.is_empty(),
            Goal::Boss => !self.mons.iter().any(|m| m.boss),
            Goal::Exit => {
                let e = self.exits();
                (self.hero[0] == e[0] && self.hero[1] == e[1])
                    || (self.hero[0] == e[1] && self.hero[1] == e[0])
            }
            Goal::Survive => self.round >= self.sp.rounds,
        }
    }
}

impl Mini for Dung {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.mon_phase {
            return vec!["mon".into()];
        }
        let (r, c) = self.hero[self.side];
        let mut v = Vec::new();
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nr, nc) = (r + dr, c + dc);
            if self.free(nr, nc) {
                v.push(format!("mers {} {}", nr + 1, nc + 1));
            }
            if self.mons.iter().any(|m| (m.r, m.c) == (nr, nc)) {
                v.push(format!("lov {} {}", nr + 1, nc + 1));
            }
        }
        if self.heals > 0 && self.hhp[self.side] < self.sp.hp {
            v.push("vindec".into());
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        if self.mon_phase {
            // monștrii acționează
            let n = self.mons.len();
            for i in 0..n {
                let (mr, mc) = (self.mons[i].r, self.mons[i].c);
                let alive: Vec<usize> = (0..2).filter(|&h| self.hhp[h] > 0).collect();
                let Some(&tgt) = alive
                    .iter()
                    .min_by_key(|&&h| (Self::dist((mr, mc), self.hero[h]), self.hhp[h], h))
                else {
                    break;
                };
                let hp = self.hero[tgt];
                if Self::dist((mr, mc), hp) == 1 {
                    self.hhp[tgt] -= self.sp.mdmg + self.mons[i].boss as i32;
                } else {
                    let mut best = (Self::dist((mr, mc), hp), mr, mc);
                    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (nr, nc) = (mr + dr, mc + dc);
                        if self.free(nr, nc) {
                            let d = Self::dist((nr, nc), hp);
                            if d < best.0 {
                                best = (d, nr, nc);
                            }
                        }
                    }
                    self.mons[i].r = best.1;
                    self.mons[i].c = best.2;
                }
            }
            self.round += 1;
            if self.sp.wave_every > 0 && self.round.is_multiple_of(self.sp.wave_every) {
                for _ in 0..self.sp.wave_size {
                    for _ in 0..30 {
                        let r = self.rng.below((self.sp.h as usize).div_ceil(2)) as i32;
                        let c = self.rng.below(self.sp.w as usize) as i32;
                        if self.free(r, c) {
                            self.mons.push(Mon {
                                r,
                                c,
                                hp: self.sp.mhp,
                                boss: false,
                            });
                            break;
                        }
                    }
                }
            }
            self.mon_phase = false;
            self.side = 0;
            self.ap_left = self.sp.ap;
            return;
        }
        let me = self.side;
        let mut it = m.split(' ');
        match it.next().unwrap() {
            "mers" => {
                let r = it.next().unwrap().parse::<i32>().unwrap() - 1;
                let c = it.next().unwrap().parse::<i32>().unwrap() - 1;
                self.hero[me] = (r, c);
            }
            "lov" => {
                let r = it.next().unwrap().parse::<i32>().unwrap() - 1;
                let c = it.next().unwrap().parse::<i32>().unwrap() - 1;
                let i = self.mons.iter().position(|x| (x.r, x.c) == (r, c)).unwrap();
                self.mons[i].hp -= self.sp.dmg;
                if self.mons[i].hp <= 0 {
                    self.mons.remove(i);
                    self.kills += 1;
                }
            }
            "vindec" => {
                self.hhp[me] = (self.hhp[me] + 2).min(self.sp.hp);
                self.heals -= 1;
            }
            _ => {
                self.ap_left = 1;
            }
        }
        self.ap_left -= 1;
        if self.ap_left == 0 || self.won() {
            self.ap_left = self.sp.ap;
            if self.side == 0 {
                self.side = 1;
            } else {
                self.mon_phase = true;
                self.side = 0;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.hhp[0] <= 0 || self.hhp[1] <= 0 {
            return Some(Outcome::Draw);
        }
        if self.won() {
            return Some(Outcome::Win(Player::One));
        }
        if self.round >= self.sp.rounds {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, _p: Player) -> i32 {
        let hp: i32 = self.hhp.iter().sum();
        let mut e = self.kills as i32 * 100 + hp * 8;
        let me = self.hero[self.side];
        match self.sp.goal {
            Goal::Exit => {
                let ex = self.exits();
                let d = |h: (i32, i32)| Self::dist(h, ex[0]).min(Self::dist(h, ex[1]));
                e -= (d(self.hero[0]) + d(self.hero[1])) * 12;
            }
            _ => {
                let near = self
                    .mons
                    .iter()
                    .map(|m| Self::dist(me, (m.r, m.c)))
                    .min()
                    .unwrap_or(0);
                e -= near * 4;
                let tot: i32 = self.mons.iter().map(|m| m.hp).sum();
                e -= tot * 20;
            }
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | viață J1 {} J2 {} | monștri {} | vindecări {}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.hhp[0],
            self.hhp[1],
            self.mons.len(),
            self.heals
        );
        for r in 0..self.sp.h {
            for c in 0..self.sp.w {
                let ch = if self.hero[0] == (r, c) && self.hhp[0] > 0 {
                    '1'
                } else if self.hero[1] == (r, c) && self.hhp[1] > 0 {
                    '2'
                } else if let Some(m) = self.mons.iter().find(|m| (m.r, m.c) == (r, c)) {
                    if m.boss {
                        'B'
                    } else {
                        'm'
                    }
                } else if self.wall[(r * self.sp.w + c) as usize] {
                    '#'
                } else if self.sp.goal == Goal::Exit && self.exits().contains(&(r, c)) {
                    'E'
                } else {
                    '.'
                };
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        let ms: Vec<String> = self
            .mons
            .iter()
            .map(|m| {
                format!(
                    "({},{}) v{}{}",
                    m.r + 1,
                    m.c + 1,
                    m.hp,
                    if m.boss { " căpetenie" } else { "" }
                )
            })
            .collect();
        s.push_str(&format!("Monștri: {}\n", ms.join("; ")));
        s.push_str(&format!(
            "La mutare: {} ({} acțiuni) — mers r c | lov r c | vindec | pas\n",
            pname(self.side()),
            self.ap_left
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        check(|s| with_seed(0, s), 4000);
        check(|s| with_seed(1, s), 4000);
        check(|s| with_seed(2, s), 4000);
        check(|s| with_seed(3, s), 4000);
        check(|s| with_seed(4, s), 4000);
        check(|s| with_seed(5, s), 4000);
    }
}
