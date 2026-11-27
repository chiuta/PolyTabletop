//! Motor „carduri activate de zaruri” (lista C6): `baza_stelara`, `orasul_in_crestere`,
//! `infirmeria_de_noapte`, `descalecatorii` — variante originale, 2 jucători, informație publică.
//!
//! În fiecare rundă se aruncă două zaruri (mutarea forțată `tira`); cărțile cu numărul
//! potrivit produc pentru proprietar (cele „pasive” produc și la aruncarea adversarului). Apoi
//! fiecare jucător cumpără cel mult o carte din piața publică (`compro i` sau `pas`).
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Trig {
    /// suma celor două zaruri
    Sum,
    /// oricare dintre zaruri
    Single,
}

pub struct Card {
    pub name: &'static str,
    pub cost: i32,
    pub num: u8,
    pub gain: [i8; 2],
    pub vp: i8,
    pub passive: bool,
}

pub const fn cd(
    name: &'static str,
    cost: i32,
    num: u8,
    gain: [i8; 2],
    vp: i8,
    passive: bool,
) -> Card {
    Card {
        name,
        cost,
        num,
        gain,
        vp,
        passive,
    }
}

pub struct Cfg {
    pub title: &'static str,
    pub rn: [&'static str; 2],
    pub cards: &'static [Card],
    pub trig: Trig,
    pub rounds: u32,
    pub start: [i32; 2],
    /// câte unități din resursa a doua valorează 1 punct la final
    pub div: i32,
    pub copies: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Roll,
    Buy(u8),
}

#[derive(Clone)]
pub struct Flota {
    cfg: &'static Cfg,
    rng: u64,
    round: u32,
    ph: Ph,
    dice: [u8; 2],
    res: [[i32; 2]; 2],
    vp: [i32; 2],
    stock: Vec<u8>,
    owned: [Vec<usize>; 2],
    last: String,
}

impl Flota {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        Flota {
            cfg,
            rng: seed_state(seed),
            round: 0,
            ph: Ph::Roll,
            dice: [0; 2],
            res: [cfg.start; 2],
            vp: [0; 2],
            stock: vec![cfg.copies; cfg.cards.len()],
            owned: [Vec::new(), Vec::new()],
            last: String::new(),
        }
    }
    fn roller(&self) -> usize {
        (self.round % 2) as usize
    }
    fn hits(&self, num: u8) -> bool {
        match self.cfg.trig {
            Trig::Sum => self.dice[0] + self.dice[1] + 2 == num,
            Trig::Single => self.dice[0] + 1 == num || self.dice[1] + 1 == num,
        }
    }
    fn prob(&self, num: u8) -> i32 {
        // în miimi
        match self.cfg.trig {
            Trig::Sum => (6 - (num as i32 - 7).abs()).max(0) * 1000 / 36,
            Trig::Single => 11 * 1000 / 36,
        }
    }
    fn score(&self, p: usize) -> i32 {
        self.vp[p] + self.res[p][1] / self.cfg.div
    }
    fn side_idx(&self) -> usize {
        match self.ph {
            Ph::Roll => self.roller(),
            Ph::Buy(0) => self.roller(),
            Ph::Buy(_) => 1 - self.roller(),
        }
    }
}

impl Mini for Flota {
    fn side(&self) -> Player {
        pl(self.side_idx())
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["tira".into()],
            Ph::Buy(_) => {
                let p = self.side_idx();
                let mut v = vec!["pas".to_string()];
                for (i, c) in self.cfg.cards.iter().enumerate() {
                    if self.stock[i] > 0 && self.res[p][0] >= c.cost {
                        v.push(format!("compro {}", i + 1));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Roll => {
                self.dice = [die(&mut self.rng, 6), die(&mut self.rng, 6)];
                let r = self.roller();
                for p in 0..2 {
                    for &ci in &self.owned[p].clone() {
                        let c = &self.cfg.cards[ci];
                        if self.hits(c.num) && (p == r || c.passive) {
                            for k in 0..2 {
                                self.res[p][k] += c.gain[k] as i32;
                            }
                        }
                    }
                }
                self.last = format!("{} + {}", self.dice[0] + 1, self.dice[1] + 1);
                self.ph = Ph::Buy(0);
            }
            Ph::Buy(w) => {
                let p = self.side_idx();
                if let Some(i) = m.strip_prefix("compro ") {
                    let i: usize = i.parse::<usize>().unwrap() - 1;
                    let c = &self.cfg.cards[i];
                    self.res[p][0] -= c.cost;
                    self.vp[p] += c.vp as i32;
                    self.stock[i] -= 1;
                    self.owned[p].push(i);
                }
                if w == 0 {
                    self.ph = Ph::Buy(1);
                } else {
                    self.round += 1;
                    self.ph = Ph::Roll;
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
        let a = pi(p);
        let left = (self.cfg.rounds - self.round.min(self.cfg.rounds)) as i32;
        let val = |q: usize| -> i32 {
            let mut e =
                1000 * self.vp[q] + 1000 * self.res[q][1] / self.cfg.div + 250 * self.res[q][0];
            for &ci in &self.owned[q] {
                let c = &self.cfg.cards[ci];
                let pr = self.prob(c.num) * if c.passive { 2 } else { 1 }
                    / if c.passive { 1 } else { 2 };
                let g = c.gain[0] as i32 * 250 + c.gain[1] as i32 * 1000 / self.cfg.div;
                e += pr * g / 1000 * left;
            }
            e
        };
        val(a) - val(1 - a)
    }
    fn show(&self) -> String {
        let c = self.cfg;
        let mut s = format!(
            "{} — runda {}/{} | aruncă J{} | zaruri {}\n",
            c.title,
            (self.round + 1).min(c.rounds),
            c.rounds,
            self.roller() + 1,
            self.last
        );
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: {} {}, {} {}, +{}p (scor {})\n",
                p + 1,
                c.rn[0],
                self.res[p][0],
                c.rn[1],
                self.res[p][1],
                self.vp[p],
                self.score(p)
            ));
        }
        for (i, cc) in c.cards.iter().enumerate() {
            s.push_str(&format!(
                "  carte {}: {} — cost {}, număr {}{}, dă {:?}, +{}p, rămase {}\n",
                i + 1,
                cc.name,
                cc.cost,
                cc.num,
                if cc.passive { " (și la adversar)" } else { "" },
                cc.gain,
                cc.vp,
                self.stock[i]
            ));
        }
        let own = |p: usize| {
            self.owned[p]
                .iter()
                .map(|&i| c.cards[i].name)
                .collect::<Vec<_>>()
                .join(", ")
        };
        s.push_str(&format!(
            "Cărți J1: {}\nCărți J2: {}\nMutări: tira | compro i | pas\n",
            own(0),
            own(1)
        ));
        s
    }
}

macro_rules! cfgs {
    ($($name:ident => $e:expr;)*) => { $(pub static $name: Cfg = $e;)* };
}

cfgs! {
    BAZA => Cfg { title: "Baza stelară", rn: ["credite", "energie"], trig: Trig::Sum, rounds: 10, start: [4, 0], div: 3, copies: 2,
        cards: &[cd("Sondă", 3, 7, [3, 0], 0, false), cd("Dronă minieră", 4, 6, [2, 1], 0, false), cd("Panou solar", 5, 8, [0, 2], 0, true),
                 cd("Transportor", 6, 5, [4, 0], 1, false), cd("Reactor", 7, 9, [1, 3], 1, false), cd("Antenă", 4, 4, [2, 0], 0, true),
                 cd("Hangar", 8, 10, [3, 3], 2, false), cd("Cupolă", 9, 6, [0, 0], 4, false)] };
    ORAS => Cfg { title: "Orașul în creștere", rn: ["lemn", "faimă"], trig: Trig::Single, rounds: 9, start: [3, 0], div: 2, copies: 2,
        cards: &[cd("Pădure", 2, 1, [2, 0], 0, false), cd("Carieră", 3, 2, [2, 0], 0, false), cd("Târg", 4, 3, [1, 1], 0, true),
                 cd("Moară", 4, 4, [3, 0], 0, false), cd("Templu", 5, 5, [0, 2], 1, false), cd("Pod", 5, 6, [1, 1], 1, true),
                 cd("Turn", 7, 3, [0, 3], 2, false), cd("Zid", 6, 6, [0, 0], 3, false)] };
    INFIRM => Cfg { title: "Infirmeria de noapte", rn: ["fonduri", "vindecări"], trig: Trig::Single, rounds: 8, start: [3, 0], div: 2, copies: 3,
        cards: &[cd("Pat simplu", 2, 1, [1, 1], 0, false), cd("Pat dublu", 3, 2, [1, 2], 0, false), cd("Cabinet", 4, 3, [3, 0], 0, true),
                 cd("Sală de operații", 6, 4, [0, 4], 1, false), cd("Farmacie", 4, 5, [2, 1], 0, false), cd("Voluntari", 3, 6, [2, 0], 0, true),
                 cd("Laborator", 7, 5, [0, 3], 2, false)] };
    DESCAL => Cfg { title: "Descălecătorii", rn: ["merinde", "pământ"], trig: Trig::Sum, rounds: 9, start: [3, 0], div: 2, copies: 2,
        cards: &[cd("Câmp", 2, 6, [2, 0], 0, false), cd("Colibă", 3, 8, [1, 1], 0, false), cd("Fântână", 3, 7, [3, 0], 0, true),
                 cd("Sat", 5, 5, [0, 2], 1, false), cd("Moară", 5, 9, [2, 2], 0, false), cd("Fortăreață", 8, 10, [0, 3], 3, false),
                 cd("Drum", 4, 4, [1, 1], 0, true)] };
}

#[cfg(test)]
mod t {
    use super::*;

    fn run(mk: fn(u64) -> Flota) {
        check(mk, 600);
        let mut g = mk(4);
        advance(&mut g, 60, |m| m.ph == Ph::Buy(0));
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn baza() {
        run(|s| Flota::with_seed(&BAZA, s));
    }
    #[test]
    fn oras() {
        run(|s| Flota::with_seed(&ORAS, s));
    }
    #[test]
    fn infirm() {
        run(|s| Flota::with_seed(&INFIRM, s));
    }
    #[test]
    fn descal() {
        run(|s| Flota::with_seed(&DESCAL, s));
    }
}
