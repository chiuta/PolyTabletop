//! Motor comun „economie cu plasare de muncitori” (variante originale, 2 jucători, informație completă).
//!
//! Fiecare joc are propriile resurse, locuri de acțiune, piață de cărți, conversii și scor final (`Cfg`).
//! Mutări: `pon s` (ocupi un loc liber: plătești `give`, primești `get` + puncte), `cumpar j` (iei o carte
//! din piață: venit permanent + puncte), `conv i` (conversie, limitată pe rundă), `pas`. La sfârșitul
//! rundei, mutarea forțată `ronda`: venituri, întreținere, locurile se eliberează. Piața următoare e
//! publică (se vede în afișare), deci AI-ul nu cunoaște nimic ascuns.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct Slot {
    pub name: &'static str,
    pub give: [i8; 4],
    pub get: [i8; 4],
    pub vp: i8,
}
pub struct Card {
    pub name: &'static str,
    pub cost: [i8; 4],
    pub inc: [i8; 4],
    pub vp: i8,
}
pub const fn sl(name: &'static str, give: [i8; 4], get: [i8; 4], vp: i8) -> Slot {
    Slot {
        name,
        give,
        get,
        vp,
    }
}
pub const fn cd(name: &'static str, cost: [i8; 4], inc: [i8; 4], vp: i8) -> Card {
    Card {
        name,
        cost,
        inc,
        vp,
    }
}

pub struct Cfg {
    pub title: &'static str,
    pub res: [&'static str; 4],
    pub slots: &'static [Slot],
    pub cards: &'static [Card],
    pub conv: &'static [([i8; 4], [i8; 4], i8)],
    pub conv_lim: u8,
    pub rounds: u32,
    pub workers: u8,
    /// Rondel: un loc se poate alege doar la 1..=3 poziții în față de marcajul tău.
    pub rondel: bool,
    pub market: usize,
    /// Întreținere de resursa 0 pe rundă; lipsa costă `upkeep_vp` puncte pe unitate.
    pub upkeep: i8,
    pub upkeep_vp: i8,
    /// Resursa cu bonus de majoritate la final (+6 puncte), sau 255 dacă nu există.
    pub maj: u8,
    /// +2 puncte pentru fiecare set complet (minimul celor 4 resurse).
    pub set_bonus: bool,
    /// 4 unități din resursa a doua = 1 punct (0 = nu).
    pub vp_div: i8,
    pub start: [i8; 4],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Play,
    End,
}

#[derive(Clone)]
pub struct Econ {
    cfg: &'static Cfg,
    res: [[i32; 4]; 2],
    vp: [i32; 2],
    occ: Vec<u8>,
    left: [u8; 2],
    round: u32,
    turn: usize,
    market: Vec<Option<usize>>,
    sched: Vec<Vec<usize>>,
    owned: [Vec<usize>; 2],
    marker: [usize; 2],
    conv_used: [u8; 2],
    ph: Ph,
    first: usize,
}

impl Econ {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut sched = Vec::new();
        for _ in 0..=cfg.rounds {
            let mut row = Vec::new();
            for _ in 0..cfg.market {
                row.push(rng.below(cfg.cards.len().max(1)));
            }
            sched.push(row);
        }
        let mut res = [[0i32; 4]; 2];
        for p in 0..2 {
            for k in 0..4 {
                res[p][k] = cfg.start[k] as i32;
            }
        }
        let market = if cfg.market > 0 {
            sched[0].iter().map(|&c| Some(c)).collect()
        } else {
            Vec::new()
        };
        Econ {
            cfg,
            res,
            vp: [0; 2],
            occ: vec![2; cfg.slots.len()],
            left: [cfg.workers; 2],
            round: 0,
            turn: 0,
            market,
            sched,
            owned: [Vec::new(), Vec::new()],
            marker: [0, cfg.slots.len() / 2],
            conv_used: [0; 2],
            ph: Ph::Play,
            first: 0,
        }
    }
    fn can_pay(&self, p: usize, cost: &[i8; 4]) -> bool {
        (0..4).all(|k| self.res[p][k] >= cost[k] as i32)
    }
    fn dist(&self, p: usize, s: usize) -> usize {
        let n = self.cfg.slots.len();
        (s + n - self.marker[p]) % n
    }
    fn score(&self, p: usize) -> i32 {
        let c = self.cfg;
        let mut s = self.vp[p];
        for &ci in &self.owned[p] {
            s += c.cards[ci].vp as i32;
        }
        if c.vp_div > 0 {
            s += self.res[p][3] / c.vp_div as i32;
        }
        if c.set_bonus {
            s += 2 * self.res[p].iter().copied().min().unwrap();
        }
        if (c.maj as usize) < 4 {
            let k = c.maj as usize;
            if self.res[p][k] > self.res[1 - p][k] {
                s += 6;
            }
        }
        s
    }
    fn next_turn(&mut self) {
        let o = 1 - self.turn;
        if self.left[o] > 0 {
            self.turn = o;
        } else if self.left[self.turn] == 0 {
            self.ph = Ph::End;
        }
    }
}

impl Mini for Econ {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        if self.ph == Ph::End || self.turn == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.ph == Ph::End {
            return vec!["ronda".into()];
        }
        let p = self.turn;
        let c = self.cfg;
        let mut v = vec!["pas".to_string()];
        if self.left[p] > 0 {
            for s in 0..c.slots.len() {
                if self.occ[s] == 2 && self.can_pay(p, &c.slots[s].give) {
                    if c.rondel {
                        let d = self.dist(p, s);
                        if d == 0 || d > 3 {
                            continue;
                        }
                    }
                    v.push(format!("pon {s}"));
                }
            }
            for (j, m) in self.market.iter().enumerate() {
                if let Some(ci) = m {
                    if self.can_pay(p, &c.cards[*ci].cost) {
                        v.push(format!("cumpar {j}"));
                    }
                }
            }
        }
        if self.conv_used[p] < c.conv_lim {
            for (i, (g, _, _)) in c.conv.iter().enumerate() {
                if self.can_pay(p, g) {
                    v.push(format!("conv {i}"));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let c = self.cfg;
        if m == "ronda" {
            for p in 0..2 {
                for &ci in &self.owned[p].clone() {
                    for k in 0..4 {
                        self.res[p][k] += c.cards[ci].inc[k] as i32;
                    }
                }
                if c.upkeep > 0 {
                    let need = c.upkeep as i32;
                    if self.res[p][0] >= need {
                        self.res[p][0] -= need;
                    } else {
                        let miss = need - self.res[p][0];
                        self.res[p][0] = 0;
                        self.vp[p] -= miss * c.upkeep_vp as i32;
                    }
                }
            }
            self.round += 1;
            self.occ = vec![2; c.slots.len()];
            self.left = [c.workers; 2];
            self.conv_used = [0; 2];
            if c.market > 0 && (self.round as usize) < self.sched.len() {
                for j in 0..c.market {
                    if self.market[j].is_none() {
                        self.market[j] = Some(self.sched[self.round as usize][j]);
                    }
                }
            }
            self.first = 1 - self.first;
            self.turn = self.first;
            self.ph = Ph::Play;
            return;
        }
        let p = self.turn;
        let mut it = m.split(' ');
        match it.next().unwrap() {
            "pas" => {
                self.left[p] = 0;
                self.next_turn();
            }
            "pon" => {
                let s: usize = it.next().unwrap().parse().unwrap();
                let sl = &c.slots[s];
                for k in 0..4 {
                    self.res[p][k] += sl.get[k] as i32 - sl.give[k] as i32;
                }
                self.vp[p] += sl.vp as i32;
                self.occ[s] = p as u8;
                self.marker[p] = s;
                self.left[p] -= 1;
                self.next_turn();
            }
            "cumpar" => {
                let j: usize = it.next().unwrap().parse().unwrap();
                let ci = self.market[j].take().unwrap();
                for k in 0..4 {
                    self.res[p][k] -= c.cards[ci].cost[k] as i32;
                }
                self.owned[p].push(ci);
                self.left[p] -= 1;
                self.next_turn();
            }
            _ => {
                let i: usize = it.next().unwrap().parse().unwrap();
                let (g, r, vp) = &c.conv[i];
                for k in 0..4 {
                    self.res[p][k] += r[k] as i32 - g[k] as i32;
                }
                self.vp[p] += *vp as i32;
                self.conv_used[p] += 1;
            }
        }
        if self.ph == Ph::Play && self.left == [0, 0] {
            self.ph = Ph::End;
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.cfg.rounds {
            Some(by_score(self.score(0), self.score(1)))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let left = (self.cfg.rounds - self.round.min(self.cfg.rounds)) as i32;
        let wealth = |q: usize| -> i32 {
            let r: i32 = self.res[q].iter().sum();
            let inc: i32 = self.owned[q]
                .iter()
                .map(|&ci| {
                    self.cfg.cards[ci]
                        .inc
                        .iter()
                        .map(|&x| x as i32)
                        .sum::<i32>()
                })
                .sum();
            r + inc * left / 2
        };
        6 * (self.score(i) - self.score(o)) + wealth(i) - wealth(o)
    }
    fn show(&self) -> String {
        let c = self.cfg;
        let mut s = format!(
            "{} — runda {}/{}\n",
            c.title,
            (self.round + 1).min(c.rounds),
            c.rounds
        );
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: {} {}, {} {}, {} {}, {} {} | puncte {} (scor {}) | muncitori {} | cărți {}\n",
                p + 1,
                self.res[p][0],
                c.res[0],
                self.res[p][1],
                c.res[1],
                self.res[p][2],
                c.res[2],
                self.res[p][3],
                c.res[3],
                self.vp[p],
                self.score(p),
                self.left[p],
                self.owned[p].len()
            ));
        }
        for (i, sl) in c.slots.iter().enumerate() {
            let o = match self.occ[i] {
                0 => "J1",
                1 => "J2",
                _ => "liber",
            };
            s.push_str(&format!(
                "  loc {i}: {} (dai {:?} / primești {:?} / +{}p) — {o}\n",
                sl.name, sl.give, sl.get, sl.vp
            ));
        }
        for (j, m) in self.market.iter().enumerate() {
            if let Some(ci) = m {
                let cc = &c.cards[*ci];
                s.push_str(&format!(
                    "  piață {j}: {} (cost {:?}, venit {:?}, +{}p)\n",
                    cc.name, cc.cost, cc.inc, cc.vp
                ));
            }
        }
        if c.market > 0 && (self.round as usize + 1) < self.sched.len() {
            let nx: Vec<&str> = self.sched[self.round as usize + 1]
                .iter()
                .map(|&ci| c.cards[ci].name)
                .collect();
            s.push_str(&format!("  piața rundei următoare (completări): {nx:?}\n"));
        }
        s
    }
}
