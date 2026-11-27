//! Motor comun „construcție de pachet” (variante originale, 2 jucători).
//!
//! Fiecare începe cu 8 „novici” (1 rune) și 2 „străji” (1 forță). Tura: ai 5 cărți în mână (trase
//! prin mutarea forțată de la sfârșitul turei precedente, deci AI-ul nu le cunoaște dinainte);
//! `joc` joacă toată mâna (rune, forță, lumină); apoi `cumpar j` (erou din rândul central, costă
//! rune), `lupt j` (monstru, costă forță; în varianta „peșteră”, întunericul scade forța cu
//! lumina lipsă), `fixa` (erou mereu disponibil), `fin`. Câștigă cine ajunge primul la
//! punctele-țintă sau cine are mai multe după ultima tură.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct C {
    pub name: &'static str,
    /// 0 = erou, 1 = monstru
    pub kind: u8,
    /// erou: cost în rune; monstru: forță necesară
    pub cost: i8,
    pub rune: i8,
    pub power: i8,
    pub light: i8,
    /// puncte (erou: puncte la sfârșit; monstru: puncte la înfrângere)
    pub vp: i8,
    /// monstru: runele primite la înfrângere; întuneric = `dark`
    pub reward: i8,
    pub dark: i8,
}
pub const fn hero(name: &'static str, cost: i8, rune: i8, power: i8, light: i8, vp: i8) -> C {
    C {
        name,
        kind: 0,
        cost,
        rune,
        power,
        light,
        vp,
        reward: 0,
        dark: 0,
    }
}
pub const fn mon(name: &'static str, cost: i8, vp: i8, reward: i8, dark: i8) -> C {
    C {
        name,
        kind: 1,
        cost,
        rune: 0,
        power: 0,
        light: 0,
        vp,
        reward,
        dark,
    }
}

pub struct Cfg {
    pub title: &'static str,
    /// pool[0] = novice, pool[1] = strajă, pool[2] = eroul fix; restul alimentează rândul.
    pub pool: &'static [C],
    pub row: usize,
    /// primele `hrow` locuri sunt pentru eroi, restul pentru monștri (0 = rând mixt)
    pub hrow: usize,
    pub turns: u32,
    pub target: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Main,
    Draw,
}

#[derive(Clone)]
pub struct Deck {
    cfg: &'static Cfg,
    viewer: Option<Player>,
    deck: [Vec<u8>; 2],
    disc: [Vec<u8>; 2],
    hand: [Vec<u8>; 2],
    row: Vec<Option<u8>>,
    runes: i32,
    power: i32,
    light: i32,
    played: bool,
    vp: [i32; 2],
    turn: usize,
    turns: [u32; 2],
    ph: Ph,
    rng: Rng,
}

impl Deck {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let mut d = Deck {
            cfg,
            viewer: None,
            deck: [Vec::new(), Vec::new()],
            disc: [Vec::new(), Vec::new()],
            hand: [Vec::new(), Vec::new()],
            row: vec![None; cfg.row],
            runes: 0,
            power: 0,
            light: 0,
            played: false,
            vp: [0; 2],
            turn: 0,
            turns: [0; 2],
            ph: Ph::Main,
            rng: Rng::new(seed),
        };
        for p in 0..2 {
            for _ in 0..8 {
                d.deck[p].push(0);
            }
            for _ in 0..2 {
                d.deck[p].push(1);
            }
            let mut dk = std::mem::take(&mut d.deck[p]);
            shuf(&mut dk, &mut d.rng);
            d.deck[p] = dk;
            d.draw(p, 5);
        }
        d.refill();
        d
    }
    fn draw(&mut self, p: usize, n: usize) {
        for _ in 0..n {
            if self.deck[p].is_empty() {
                let mut dk = std::mem::take(&mut self.disc[p]);
                shuf(&mut dk, &mut self.rng);
                self.deck[p] = dk;
            }
            if let Some(c) = self.deck[p].pop() {
                self.hand[p].push(c);
            }
        }
    }
    fn pick(&mut self, kind: Option<u8>) -> u8 {
        let pool = self.cfg.pool;
        loop {
            let i = 3 + self.rng.below(pool.len() - 3);
            if kind.is_none_or(|k| pool[i].kind == k) {
                return i as u8;
            }
        }
    }
    fn refill(&mut self) {
        for j in 0..self.cfg.row {
            if self.row[j].is_none() {
                let kind = if self.cfg.hrow == 0 {
                    None
                } else if j < self.cfg.hrow {
                    Some(0)
                } else {
                    Some(1)
                };
                self.row[j] = Some(self.pick(kind));
            }
        }
    }
    /// Pentru teste (audit AI): amestecă tot ce nu poate vedea jucătorul `1 - hide`: mâna și pachetul
    /// lui `hide`, ordinea propriului pachet al celui la mutare și viitoarele extrageri din rând.
    #[cfg(test)]
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut pool: Vec<u8> = self.hand[hide].clone();
        pool.extend(self.deck[hide].iter().copied());
        let n = self.hand[hide].len();
        crate::p3_comun::shuffle_with(&mut pool, seed);
        self.hand[hide] = pool.drain(..n).collect();
        self.deck[hide] = pool;
        crate::p3_comun::shuffle_with(&mut self.deck[1 - hide], seed ^ 0x77);
        self.rng = Rng::new(seed ^ 0xABCD);
    }

    fn all_cards(&self, p: usize) -> impl Iterator<Item = &u8> {
        self.deck[p]
            .iter()
            .chain(&self.disc[p])
            .chain(&self.hand[p])
    }
    fn eff_power(&self, dark: i8) -> i32 {
        self.power - (dark as i32 - self.light).max(0)
    }
    fn total(&self, p: usize) -> i32 {
        let pool = self.cfg.pool;
        self.vp[p]
            + self
                .all_cards(p)
                .map(|&c| {
                    if pool[c as usize].kind == 0 {
                        pool[c as usize].vp as i32
                    } else {
                        0
                    }
                })
                .sum::<i32>()
    }
}

impl Mini for Deck {
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
        if self.ph == Ph::Draw {
            return vec!["trag".into()];
        }
        let pool = self.cfg.pool;
        let mut v = vec!["fin".to_string()];
        if !self.played {
            v.push("joc".into());
            return v;
        }
        if self.runes >= pool[2].cost as i32 {
            v.push("fixa".into());
        }
        for (j, c) in self.row.iter().enumerate() {
            if let Some(ci) = c {
                let c = &pool[*ci as usize];
                if c.kind == 0 && self.runes >= c.cost as i32 {
                    v.push(format!("cumpar {j}"));
                }
                if c.kind == 1 && self.eff_power(c.dark) >= c.cost as i32 {
                    v.push(format!("lupt {j}"));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let pool = self.cfg.pool;
        let p = self.turn;
        if m == "trag" {
            let hand = std::mem::take(&mut self.hand[p]);
            self.disc[p].extend(hand);
            self.turns[p] += 1;
            self.draw(p, 5);
            self.refill();
            self.runes = 0;
            self.power = 0;
            self.light = 0;
            self.played = false;
            self.ph = Ph::Main;
            self.turn = 1 - p;
            return;
        }
        let mut it = m.split(' ');
        match it.next().unwrap() {
            "fin" => self.ph = Ph::Draw,
            "joc" => {
                for &c in &self.hand[p] {
                    let c = &pool[c as usize];
                    self.runes += c.rune as i32;
                    self.power += c.power as i32;
                    self.light += c.light as i32;
                }
                self.played = true;
            }
            "fixa" => {
                self.runes -= pool[2].cost as i32;
                self.disc[p].push(2);
            }
            "cumpar" => {
                let j: usize = it.next().unwrap().parse().unwrap();
                let ci = self.row[j].take().unwrap();
                self.runes -= pool[ci as usize].cost as i32;
                self.disc[p].push(ci);
                self.vp[p] += 0;
            }
            _ => {
                let j: usize = it.next().unwrap().parse().unwrap();
                let ci = self.row[j].take().unwrap();
                let c = &pool[ci as usize];
                self.power -= c.cost as i32;
                self.vp[p] += c.vp as i32;
                self.runes += c.reward as i32;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        let hit = (0..2)
            .find(|&p| self.total(p) >= self.cfg.target && self.ph == Ph::Main && self.turn != p);
        let over = self.turns[0] >= self.cfg.turns && self.turns[1] >= self.cfg.turns;
        if over || hit.is_some() {
            Some(by_score(self.total(0), self.total(1)))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let pool = self.cfg.pool;
        let q = |x: usize| -> i32 {
            self.all_cards(x)
                .map(|&c| {
                    let c = &pool[c as usize];
                    c.rune as i32 + c.power as i32 + c.light as i32 / 2
                })
                .sum()
        };
        12 * (self.total(i) - self.total(o))
            + 3 * (q(i) - q(o))
            + if self.turn == i {
                self.runes + self.power
            } else {
                0
            }
    }
    fn show(&self) -> String {
        let pool = self.cfg.pool;
        let me = self.viewer.map(pi).unwrap_or(self.turn);
        let mut s = format!(
            "{} — tura {}/{} | J1 {} p / J2 {} p (țintă {})\n",
            self.cfg.title,
            self.turns[self.turn] + 1,
            self.cfg.turns,
            self.total(0),
            self.total(1),
            self.cfg.target
        );
        s.push_str(&format!(
            "La mutare J{}: rune {}, forță {}, lumină {}{}\n",
            self.turn + 1,
            self.runes,
            self.power,
            self.light,
            if self.played {
                ""
            } else {
                " (mâna nejucată)"
            }
        ));
        let names: Vec<&str> = self.hand[me]
            .iter()
            .map(|&c| pool[c as usize].name)
            .collect();
        s.push_str(&format!("Mâna lui J{}: {:?}\n", me + 1, names));
        s.push_str(&format!(
            "Pachete: J1 {} cărți, J2 {} cărți\nRând central:\n",
            self.all_cards(0).count(),
            self.all_cards(1).count()
        ));
        s.push_str(&format!(
            "  fixa: {} (cost {} rune)\n",
            pool[2].name, pool[2].cost
        ));
        for (j, c) in self.row.iter().enumerate() {
            if let Some(ci) = c {
                let c = &pool[*ci as usize];
                if c.kind == 0 {
                    s.push_str(&format!(
                        "  {j}: erou {} cost {} (rune {}, forță {}, lumină {}, {}p)\n",
                        c.name, c.cost, c.rune, c.power, c.light, c.vp
                    ));
                } else {
                    s.push_str(&format!(
                        "  {j}: monstru {} forță {} întuneric {} (+{}p, +{} rune)\n",
                        c.name, c.cost, c.dark, c.vp, c.reward
                    ));
                }
            }
        }
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}
