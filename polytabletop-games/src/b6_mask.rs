//! Motor comun „alege ce e adevărat dintr-o listă ascunsă” (variante originale B6, 2 jucători).
//!
//! O listă de `n` căsuțe, dintre care `k` (cunoscut public) sunt „bune”, pe o mască ascunsă tuturor.
//! Pe rând alegi `alege i`; mutarea forțată `arata` dezvăluie căsuța. Bună: primești punctele ei și
//! continui; rea: tura trece (în modul cu bancă pierzi punctele nesalvate; `stop` le salvează).
//! Eventual, la începutul fiecărei întrebări se alege o categorie `cat j` (cu alt număr de căsuțe bune).
//! AI-ul decide doar după probabilitatea publică (bune rămase / căsuțe rămase), nu citește masca.
#![allow(clippy::needless_range_loop)]
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct MSpec {
    pub title: &'static str,
    pub n: usize,
    /// Numărul de căsuțe bune pentru fiecare categorie (o singură intrare = fără alegere).
    pub ks: &'static [usize],
    /// Banca: o căsuță rea îți ia punctele nesalvate din tura asta.
    pub bank: bool,
    /// Căsuțele de sus valorează mai mult (3, 2, 1).
    pub ranked: bool,
    pub rounds: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Cat,
    Act,
    Pend(usize),
}

#[derive(Clone)]
pub struct Mask {
    spec: &'static MSpec,
    mask: Vec<bool>,
    shown: Vec<bool>,
    k: usize,
    found: usize,
    bankv: i32,
    ph: Ph,
    mover: usize,
    q: u32,
    pts: [i32; 2],
    rng: Rng,
}

impl Mask {
    pub fn new(spec: &'static MSpec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static MSpec, seed: u64) -> Self {
        let mut g = Mask {
            spec,
            mask: Vec::new(),
            shown: vec![false; spec.n],
            k: spec.ks[0],
            found: 0,
            bankv: 0,
            ph: Ph::Act,
            mover: 0,
            q: 0,
            pts: [0; 2],
            rng: Rng::new(seed),
        };
        g.begin_question();
        g
    }
    fn begin_question(&mut self) {
        self.shown = vec![false; self.spec.n];
        self.found = 0;
        self.bankv = 0;
        self.mover = (self.q % 2) as usize;
        if self.spec.ks.len() > 1 {
            self.ph = Ph::Cat;
        } else {
            self.deal(self.spec.ks[0]);
            self.ph = Ph::Act;
        }
    }
    fn deal(&mut self, k: usize) {
        self.k = k;
        let mut v: Vec<bool> = (0..self.spec.n).map(|i| i < k).collect();
        shuf(&mut v, &mut self.rng);
        self.mask = v;
    }
    fn value(&self, i: usize) -> i32 {
        if self.spec.ranked {
            (3 - (i * 3 / self.spec.n) as i32).max(1)
        } else {
            1
        }
    }
    fn unrevealed(&self) -> usize {
        self.shown.iter().filter(|&&s| !s).count()
    }
    fn end_question(&mut self) {
        self.pts[self.mover] += self.bankv;
        self.bankv = 0;
        self.q += 1;
        if self.q < self.spec.rounds {
            self.begin_question();
        }
    }
}

impl Mini for Mask {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.mover)
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Cat => (0..self.spec.ks.len())
                .map(|j| format!("cat {}", j + 1))
                .collect(),
            Ph::Act => {
                let mut v: Vec<String> = (0..self.spec.n)
                    .filter(|&i| !self.shown[i])
                    .map(|i| format!("alege {}", i + 1))
                    .collect();
                if self.spec.bank && self.bankv > 0 {
                    v.push("stop".into());
                }
                v
            }
            Ph::Pend(_) => vec!["arata".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Cat => {
                let j = m[4..].parse::<usize>().unwrap() - 1;
                self.deal(self.spec.ks[j]);
                self.ph = Ph::Act;
            }
            Ph::Act => {
                if m == "stop" {
                    self.pts[self.mover] += self.bankv;
                    self.bankv = 0;
                    self.mover = 1 - self.mover;
                } else {
                    self.ph = Ph::Pend(m[6..].parse::<usize>().unwrap() - 1);
                }
            }
            Ph::Pend(i) => {
                self.shown[i] = true;
                self.ph = Ph::Act;
                if self.mask[i] {
                    self.found += 1;
                    if self.spec.bank {
                        self.bankv += self.value(i);
                    } else {
                        self.pts[self.mover] += self.value(i);
                    }
                } else {
                    if self.spec.bank {
                        self.bankv = 0;
                    }
                    self.mover = 1 - self.mover;
                }
                if self.found == self.k || self.unrevealed() == 0 {
                    self.end_question();
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.q >= self.spec.rounds {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut base = 100 * (self.pts[a] - self.pts[1 - a]);
        if self.mover == a {
            base += 100 * self.bankv;
        }
        if a != self.mover {
            return base;
        }
        match self.ph {
            Ph::Cat => base,
            Ph::Act => base,
            Ph::Pend(i) => {
                let r = (self.k - self.found) as i32;
                let u = self.unrevealed() as i32;
                let val = 100 * self.value(i);
                let lose = if self.spec.bank { 100 * self.bankv } else { 0 };
                (r * (base + val) + (u - r) * (base - lose)) / u.max(1)
            }
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — întrebarea {}/{} | puncte J1 {}, J2 {}\n",
            self.spec.title,
            (self.q + 1).min(self.spec.rounds),
            self.spec.rounds,
            self.pts[0],
            self.pts[1]
        );
        let cells: Vec<String> = (0..self.spec.n)
            .map(|i| {
                let c = if !self.shown[i] {
                    '?'
                } else if self.mask[i] {
                    '+'
                } else {
                    'x'
                };
                format!("{}[{}]", i + 1, c)
            })
            .collect();
        s.push_str(&cells.join(" "));
        s.push_str(&format!(
            "\nBune în total: {} (găsite {}) | bancă: {} | la mutare: {} (cat j | alege i | stop | arata)\n",
            self.k,
            self.found,
            self.bankv,
            pname(self.side())
        ));
        s
    }
}

/// Teste: AI-ul nu citește masca ascunsă.
#[cfg(test)]
pub fn audit(spec: &'static MSpec) {
    let mut g = Mask::with_seed(spec, 4);
    if g.ph == Ph::Cat {
        g.step("cat 1");
    }
    let mut h = g.clone();
    // permută masca (aceleași k și căsuțe găsite)
    let t = (0..spec.n).find(|&i| g.mask[i]).unwrap();
    let f = (0..spec.n).find(|&i| !g.mask[i]).unwrap();
    h.mask.swap(t, f);
    for seed in 1..6 {
        assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
    }
    assert!(!g.show().contains('+'));
}
