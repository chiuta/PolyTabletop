//! Motor comun „licitații și seturi” (variante originale, 2 jucători).
//!
//! Două moduri: licitație ascendentă (`ofer n` / `pas`, cu loturi individuale sau un singur pachet
//! pe rundă) și „constructorul-șef” (un jucător fixează prețurile loturilor, celălalt alege primul).
//! Loturile (fel 0..3, valoare 1..4) se trag prin mutarea forțată `trag`, deci AI-ul nu le cunoaște
//! dinainte. Banii rămași contează 1 punct la 3 monede. Informație publică după tragere.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct Cfg {
    pub title: &'static str,
    pub kinds: [&'static str; 4],
    pub lots: usize,
    pub bundle: bool,
    pub master: bool,
    pub rounds: u32,
    pub money: i32,
    pub income: i32,
    /// 0 = monumente (Ra), 1 = varietate (Princes), 2 = majorități (Taj), 3 = grupuri (Castles)
    pub scoring: u8,
}

type Items = [[i32; 2]; 4]; // pe fel: [număr, sumă valori]
type Lot = (u8, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Bid,
    Price,
    ChooseO,
    ChooseM,
}

#[derive(Clone)]
pub struct Auc {
    cfg: &'static Cfg,
    gold: [i32; 2],
    items: [Items; 2],
    round: u32,
    ph: Ph,
    lots: Vec<Lot>,
    taken: Vec<bool>,
    prices: Vec<i32>,
    li: usize,
    cur: i32,
    bidder: Option<usize>,
    first_pass: bool,
    to_act: usize,
    rng: Rng,
}

fn perms3() -> Vec<[i32; 3]> {
    vec![
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ]
}

impl Auc {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        Auc {
            cfg,
            gold: [cfg.money; 2],
            items: [[[0; 2]; 4]; 2],
            round: 0,
            ph: Ph::Deal,
            lots: Vec::new(),
            taken: Vec::new(),
            prices: Vec::new(),
            li: 0,
            cur: 0,
            bidder: None,
            first_pass: false,
            to_act: 0,
            rng: Rng::new(seed),
        }
    }
    fn master(&self) -> usize {
        (self.round % 2) as usize
    }
    fn score_items(&self, mine: &Items, other: &Items) -> i32 {
        let mut s = 0;
        match self.cfg.scoring {
            0 => {
                for k in 0..4 {
                    if mine[k][0] == 0 {
                        s -= 2;
                    } else {
                        s += mine[k][1] + if mine[k][0] >= 3 { 4 } else { 0 };
                    }
                }
            }
            1 => {
                let d = mine.iter().filter(|x| x[0] > 0).count() as i32;
                s += d * d;
                for k in 0..4 {
                    s += mine[k][1] / 2;
                }
            }
            2 => {
                for k in 0..4 {
                    if mine[k][0] > other[k][0] {
                        s += 6;
                    } else if mine[k][0] == other[k][0] && mine[k][0] > 0 {
                        s += 2;
                    }
                    s += mine[k][1] / 3;
                }
            }
            _ => {
                for k in 0..4 {
                    s += mine[k][0] * mine[k][0] + mine[k][1] / 2;
                }
            }
        }
        s
    }
    fn score(&self, p: usize) -> i32 {
        self.score_items(&self.items[p], &self.items[1 - p]) + self.gold[p] / 3
    }
    fn add(items: &mut Items, l: &[Lot]) {
        for &(k, v) in l {
            items[k as usize][0] += 1;
            items[k as usize][1] += v as i32;
        }
    }
    fn gain(&self, p: usize, l: &[Lot]) -> i32 {
        let mut it = self.items[p];
        Self::add(&mut it, l);
        self.score_items(&it, &self.items[1 - p])
            - self.score_items(&self.items[p], &self.items[1 - p])
    }
    fn cur_lot(&self) -> Vec<Lot> {
        if self.cfg.bundle {
            self.lots.clone()
        } else {
            vec![self.lots[self.li]]
        }
    }
    fn n_lots(&self) -> usize {
        if self.cfg.bundle {
            1
        } else {
            self.lots.len()
        }
    }
    fn next_lot(&mut self) {
        self.li += 1;
        self.cur = 0;
        self.bidder = None;
        self.first_pass = false;
        if self.li >= self.n_lots() {
            self.end_round();
        } else {
            self.to_act = (self.round as usize + self.li) % 2;
        }
    }
    fn end_round(&mut self) {
        self.round += 1;
        for p in 0..2 {
            self.gold[p] += self.cfg.income;
        }
        self.ph = Ph::Deal;
    }
    fn net(&self, p: usize, l: usize) -> i32 {
        5 * self.gain(p, &[self.lots[l]]) - 2 * self.prices[l]
    }
}

impl Mini for Auc {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Deal => 0,
            Ph::Bid => self.to_act,
            Ph::Price | Ph::ChooseM => self.master(),
            Ph::ChooseO => 1 - self.master(),
        };
        if t == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Bid => {
                let p = self.to_act;
                let mut v = vec!["pas".to_string()];
                for n in self.cur + 1..=self.cur + 3 {
                    if n <= self.gold[p] {
                        v.push(format!("ofer {n}"));
                    }
                }
                v
            }
            Ph::Price => perms3()
                .iter()
                .map(|p| format!("prec {},{},{}", p[0], p[1], p[2]))
                .collect(),
            Ph::ChooseO | Ph::ChooseM => {
                let p = if self.ph == Ph::ChooseO {
                    1 - self.master()
                } else {
                    self.master()
                };
                let mut v = vec!["nu".to_string()];
                for l in 0..self.lots.len() {
                    if !self.taken[l] && self.gold[p] >= self.prices[l] {
                        v.push(format!("ia {l}"));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let c = self.cfg;
        let mut it = m.split([' ', ',']);
        match it.next().unwrap() {
            "trag" => {
                self.lots.clear();
                for _ in 0..c.lots {
                    let k = self.rng.below(4) as u8;
                    let v = 1 + self.rng.below(4) as u8;
                    self.lots.push((k, v));
                }
                self.taken = vec![false; c.lots];
                self.prices = vec![0; c.lots];
                self.li = 0;
                self.cur = 0;
                self.bidder = None;
                self.first_pass = false;
                if c.master {
                    self.ph = Ph::Price;
                } else {
                    self.ph = Ph::Bid;
                    self.to_act = (self.round as usize) % 2;
                }
            }
            "ofer" => {
                let n: i32 = it.next().unwrap().parse().unwrap();
                self.cur = n;
                self.bidder = Some(self.to_act);
                self.to_act = 1 - self.to_act;
            }
            "pas" => match self.bidder {
                Some(b) => {
                    self.gold[b] -= self.cur;
                    let l = self.cur_lot();
                    Self::add(&mut self.items[b], &l);
                    self.next_lot();
                }
                None => {
                    if self.first_pass {
                        self.next_lot();
                    } else {
                        self.first_pass = true;
                        self.to_act = 1 - self.to_act;
                    }
                }
            },
            "prec" => {
                for l in 0..c.lots {
                    self.prices[l] = it.next().unwrap().parse().unwrap();
                }
                self.ph = Ph::ChooseO;
            }
            "nu" => {
                if self.ph == Ph::ChooseO {
                    self.ph = Ph::ChooseM;
                } else {
                    self.end_round();
                }
            }
            _ => {
                let l: usize = it.next().unwrap().parse().unwrap();
                self.taken[l] = true;
                let lot = [self.lots[l]];
                if self.ph == Ph::ChooseO {
                    let o = 1 - self.master();
                    self.gold[o] -= self.prices[l];
                    self.gold[self.master()] += self.prices[l];
                    Self::add(&mut self.items[o], &lot);
                    self.ph = Ph::ChooseM;
                } else {
                    let ms = self.master();
                    self.gold[ms] -= self.prices[l];
                    Self::add(&mut self.items[ms], &lot);
                    self.end_round();
                }
            }
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
        let mut e = 5 * (self.score(i) - self.score(o)) + (self.gold[i] - self.gold[o]);
        match self.ph {
            Ph::Bid => {
                if let Some(b) = self.bidder {
                    let l = self.cur_lot();
                    let g = 5 * self.gain(b, &l) - 2 * self.cur;
                    e += if b == i { g } else { -g };
                }
            }
            Ph::ChooseO => {
                let ms = self.master();
                let ch = 1 - ms;
                let mut best = (0, None);
                for l in 0..self.lots.len() {
                    let v = self.net(ch, l);
                    if v > best.0 {
                        best = (v, Some(l));
                    }
                }
                let mut mnet = 0;
                for l in 0..self.lots.len() {
                    if Some(l) != best.1 {
                        mnet = mnet.max(self.net(ms, l));
                    }
                }
                let inc = best.1.map(|l| 2 * self.prices[l]).unwrap_or(0);
                let d = mnet + inc - best.0;
                e += if i == ms { d } else { -d };
            }
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let c = self.cfg;
        let mut s = format!(
            "{} — runda {}/{} | aur J1 {} / J2 {} | scor J1 {} / J2 {}\n",
            c.title,
            (self.round + 1).min(c.rounds),
            c.rounds,
            self.gold[0],
            self.gold[1],
            self.score(0),
            self.score(1)
        );
        for p in 0..2 {
            s.push_str(&format!("J{}:", p + 1));
            for k in 0..4 {
                s.push_str(&format!(
                    " {} {}x(val {})",
                    c.kinds[k], self.items[p][k][0], self.items[p][k][1]
                ));
            }
            s.push('\n');
        }
        for (l, (k, v)) in self.lots.iter().enumerate() {
            s.push_str(&format!(
                "  lot {l}: {} val {}{}{}\n",
                c.kinds[*k as usize],
                v,
                if self.taken.get(l).copied().unwrap_or(false) {
                    " (luat)"
                } else {
                    ""
                },
                if c.master && self.ph != Ph::Price {
                    format!(" preț {}", self.prices[l])
                } else {
                    String::new()
                }
            ));
        }
        if self.ph == Ph::Bid {
            s.push_str(&format!(
                "Licitație lot {}: ofertă curentă {} (J{}), la rând J{}\n",
                self.li,
                self.cur,
                self.bidder.map(|b| b + 1).unwrap_or(0),
                self.to_act + 1
            ));
        }
        s
    }
}
