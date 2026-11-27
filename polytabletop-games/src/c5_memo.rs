//! Motor „memorie” (lista C5, 2 jucători): plăci cu fața în jos pe o grilă; `volta i` alege o placă,
//! iar mutarea forțată `arata` o dezvăluie (plăcile văzute rămân cunoscute de amândoi). AI-ul
//! decide doar după plăcile deja văzute, niciodată după cele ascunse.
//! Moduri: `Pairs` (perechi egale se iau; opțional pisici: dezvăluirea unei pisici costă un punct și
//! încheie tura; și jokeri care se potrivesc cu orice), `Target` (țintă publică „A + B”; două plăci
//! cu felurile țintei se iau, altfel se întorc).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MMode {
    Pairs,
    Target,
}

pub struct MemSpec {
    pub title: &'static str,
    pub mode: MMode,
    pub w: usize,
    pub h: usize,
    /// Numărul de feluri obișnuite (Pairs: fiecare fel apare de două ori; Target: egal repartizat).
    pub kinds: usize,
    pub cats: usize,
    pub wilds: usize,
    pub goal: i32,
    pub cap: u32,
    pub names: &'static [&'static str],
}

const CAT: u8 = 200;
const WILD: u8 = 201;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    First,
    Rev1,
    Second,
    Rev2,
}

#[derive(Clone)]
pub struct Memo {
    sp: &'static MemSpec,
    kind: Vec<u8>,
    gone: Vec<bool>,
    seen: Vec<bool>,
    pts: [i32; 2],
    ph: Ph,
    fa: usize,
    fb: usize,
    target: (u8, u8),
    side: usize,
    plies: u32,
    rng: Rng,
}

impl Memo {
    pub fn new(sp: &'static MemSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static MemSpec, seed: u64) -> Self {
        let n = sp.w * sp.h;
        let mut rng = Rng::new(seed);
        let mut kind: Vec<u8> = Vec::new();
        match sp.mode {
            MMode::Pairs => {
                for k in 0..sp.kinds {
                    kind.push(k as u8);
                    kind.push(k as u8);
                }
            }
            MMode::Target => {
                let mut i = 0;
                while kind.len() + sp.cats + sp.wilds < n {
                    kind.push((i % sp.kinds) as u8);
                    i += 1;
                }
            }
        }
        kind.extend(std::iter::repeat_n(CAT, sp.cats));
        kind.extend(std::iter::repeat_n(WILD, sp.wilds));
        kind.truncate(n);
        while kind.len() < n {
            kind.push(CAT);
        }
        shuf(&mut kind, &mut rng);
        let mut m = Memo {
            sp,
            kind,
            gone: vec![false; n],
            seen: vec![false; n],
            pts: [0; 2],
            ph: Ph::First,
            fa: 0,
            fb: 0,
            target: (0, 1),
            side: 0,
            plies: 0,
            rng,
        };
        m.new_target();
        m
    }
    fn new_target(&mut self) {
        if self.sp.mode == MMode::Target {
            let mut ks: Vec<u8> = self
                .alive()
                .iter()
                .map(|&i| self.kind[i])
                .filter(|&k| k != CAT && k != WILD)
                .collect();
            ks.sort();
            ks.dedup();
            if ks.len() >= 2 {
                let a = ks[self.rng.below(ks.len())];
                let mut b = ks[self.rng.below(ks.len())];
                while b == a {
                    b = ks[self.rng.below(ks.len())];
                }
                self.target = (a.min(b), a.max(b));
            }
        }
    }
    fn alive(&self) -> Vec<usize> {
        (0..self.kind.len()).filter(|&i| !self.gone[i]).collect()
    }
    fn nm(&self, k: u8) -> String {
        match k {
            CAT => "pisică".into(),
            WILD => "joker".into(),
            _ => self.sp.names[k as usize % self.sp.names.len()].to_string(),
        }
    }
    fn matches(&self, a: u8, b: u8) -> bool {
        match self.sp.mode {
            MMode::Pairs => {
                (a == b && a != CAT)
                    || (a == WILD && b != CAT && b != WILD)
                    || (b == WILD && a != CAT && a != WILD)
            }
            MMode::Target => {
                let t = (a.min(b), a.max(b));
                t == self.target
            }
        }
    }
    fn left_pairs(&self) -> bool {
        let v = self.alive();
        v.iter().any(|&i| {
            v.iter()
                .any(|&j| j > i && self.matches(self.kind[i], self.kind[j]))
        })
    }
    /// Număr de plăci cunoscute (văzute) care se potrivesc cu `k` (cu excepția lui `skip`).
    fn known_partner(&self, k: u8, skip: usize) -> bool {
        (0..self.kind.len())
            .any(|j| j != skip && !self.gone[j] && self.seen[j] && self.matches(k, self.kind[j]))
    }
}

impl Mini for Memo {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::First => self
                .alive()
                .iter()
                .map(|i| format!("volta {}", i + 1))
                .collect(),
            Ph::Second => self
                .alive()
                .iter()
                .filter(|&&i| i != self.fa)
                .map(|i| format!("volta {}", i + 1))
                .collect(),
            _ => vec!["arata".into()],
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        let me = self.side;
        match self.ph {
            Ph::First => {
                self.fa = m[6..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Rev1;
            }
            Ph::Rev1 => {
                self.seen[self.fa] = true;
                if self.kind[self.fa] == CAT {
                    self.pts[me] = (self.pts[me] - 1).max(0);
                    self.side = 1 - me;
                    self.ph = Ph::First;
                } else {
                    self.ph = Ph::Second;
                }
            }
            Ph::Second => {
                self.fb = m[6..].parse::<usize>().unwrap() - 1;
                self.ph = Ph::Rev2;
            }
            Ph::Rev2 => {
                self.seen[self.fb] = true;
                let (a, b) = (self.kind[self.fa], self.kind[self.fb]);
                if self.matches(a, b) {
                    self.gone[self.fa] = true;
                    self.gone[self.fb] = true;
                    self.pts[me] += 1;
                    self.new_target();
                    self.ph = Ph::First;
                } else {
                    if b == CAT {
                        self.pts[me] = (self.pts[me] - 1).max(0);
                    }
                    self.side = 1 - me;
                    self.ph = Ph::First;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph != Ph::First {
            return None;
        }
        if self.pts[0] >= self.sp.goal || self.pts[1] >= self.sp.goal {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        if !self.left_pairs() || self.plies >= self.sp.cap {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 1000 * (self.pts[a] - self.pts[1 - a]);
        match self.ph {
            Ph::Rev1 => {
                let i = self.fa;
                if self.seen[i] {
                    // placă deja cunoscută: știm ce va fi
                    let k = self.kind[i];
                    if k == CAT {
                        v -= 700;
                    } else if self.known_partner(k, i) {
                        v += 600;
                    } else {
                        v -= 100;
                    }
                } else {
                    v += 50;
                }
            }
            Ph::Rev2 => {
                let j = self.fb;
                let ka = self.kind[self.fa];
                if self.seen[j] {
                    let k = self.kind[j];
                    if k == CAT {
                        v -= 700;
                    } else if self.matches(ka, k) {
                        v += 900;
                    } else {
                        v -= 100;
                    }
                } else {
                    // plăcile necunoscute: șansa e aprox. 1/necunoscute
                    let unk = self
                        .alive()
                        .iter()
                        .filter(|&&x| !self.seen[x])
                        .count()
                        .max(1) as i32;
                    v += 900 / unk - 50;
                }
            }
            _ => {}
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — J1 {} / J2 {} (țintă {} puncte)\n",
            self.sp.title, self.pts[0], self.pts[1], self.sp.goal
        );
        if self.sp.mode == MMode::Target {
            s.push_str(&format!(
                "Ținta rundei: {} + {}\n",
                self.nm(self.target.0),
                self.nm(self.target.1)
            ));
        }
        for r in 0..self.sp.h {
            let mut row = String::new();
            for c in 0..self.sp.w {
                let i = r * self.sp.w + c;
                let t = if self.gone[i] {
                    "  -  ".to_string()
                } else if self.seen[i] || (self.ph == Ph::Rev2 && i == self.fb) {
                    format!("{:>2}:{}", i + 1, lab(self.kind[i]))
                } else {
                    format!("{:>2}:? ", i + 1)
                };
                row.push_str(&t);
                row.push(' ');
            }
            s.push_str(&row);
            s.push('\n');
        }
        let names: Vec<String> = (0..self.sp.kinds)
            .map(|k| format!("{}={}", lab(k as u8), self.nm(k as u8)))
            .collect();
        s.push_str(&format!(
            "Legendă: {}, P=pisică, J=joker\n",
            names.join(", ")
        ));
        s.push_str(&format!(
            "La mutare: {} (volta i | arata)\n",
            pname(self.side())
        ));
        s
    }
}

fn lab(k: u8) -> String {
    match k {
        CAT => "P".into(),
        WILD => "J".into(),
        _ => format!("{}", k + 1),
    }
}
