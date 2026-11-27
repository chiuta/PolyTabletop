//! Motor „trei aruncări” (lista C6): cinci variante originale, 2 jucători, informație publică.
//! `regele_metropolei`, `ere_si_monumente`, `echipa_de_aventurieri`, `oastea_de_zaruri`, `leacul_molimei`.
//!
//! Tura: `tira` (aruncare forțată), apoi `relansez i,j,…` (se aruncă din nou zarurile alese, maxim
//! de două ori) sau `fin`; la `fin` (sau după a treia aruncare) se rezolvă efectele. `eval` estimează
//! valoarea așteptată a zarurilor relansate cu un mediu pe față, nu citește generatorul.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Rege,
    Ere,
    Echipa,
    Oastea,
    Leac,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Pre,
    Roll,
    Keep,
}

#[derive(Clone)]
pub struct Trei {
    kind: Kind,
    rng: u64,
    side: usize,
    ph: Ph,
    dice: Vec<u8>,
    pending: Vec<usize>,
    rolls: u8,
    st: [[i32; 6]; 2],
    mon: Vec<u8>,
    depth: usize,
    turns: u32,
    last: String,
}

impl Trei {
    pub fn new(kind: Kind) -> Self {
        Self::with_seed(kind, seed_now())
    }
    pub fn with_seed(kind: Kind, seed: u64) -> Self {
        let mut st = [[0i32; 6]; 2];
        for p in 0..2 {
            match kind {
                Kind::Rege => st[p][0] = 10,
                Kind::Echipa => st[p][0] = 5,
                Kind::Oastea => st[p][0] = 8,
                _ => {}
            }
        }
        Trei {
            kind,
            rng: seed_state(seed),
            side: 0,
            ph: if kind == Kind::Echipa {
                Ph::Pre
            } else {
                Ph::Roll
            },
            dice: Vec::new(),
            pending: Vec::new(),
            rolls: 0,
            st,
            mon: Vec::new(),
            depth: 0,
            turns: 0,
            last: String::new(),
        }
    }
    fn ndice(&self) -> usize {
        match self.kind {
            Kind::Rege | Kind::Echipa => 6,
            Kind::Ere | Kind::Leac => 5,
            Kind::Oastea => self.st[self.side][0].max(1) as usize,
        }
    }
    fn frozen(&self, f: u8) -> bool {
        match self.kind {
            Kind::Ere => f == 5,
            Kind::Leac => f == 4,
            _ => false,
        }
    }
    fn max_turns(&self) -> u32 {
        match self.kind {
            Kind::Rege => 80,
            Kind::Ere => 16,
            Kind::Echipa => 12,
            Kind::Oastea => 60,
            Kind::Leac => 60,
        }
    }
    fn cnt(d: &[u8], f: u8) -> i32 {
        d.iter().filter(|&&x| x == f).count() as i32
    }

    /// Rezolvă tura jucătorului la mutare cu zarurile `d`, apoi predă tura.
    fn resolve(&mut self, d: &[u8]) {
        let p = self.side;
        let o = 1 - p;
        match self.kind {
            Kind::Rege => {
                for v in 0..3u8 {
                    let n = Self::cnt(d, v);
                    if n >= 3 {
                        self.st[p][1] += v as i32 + 1 + (n - 3);
                    }
                }
                self.st[o][0] -= Self::cnt(d, 3);
                self.st[p][0] = (self.st[p][0] + Self::cnt(d, 4)).min(10);
                self.st[p][2] += Self::cnt(d, 5);
                while self.st[p][2] >= 2 {
                    self.st[p][2] -= 2;
                    self.st[p][1] += 1;
                }
            }
            Kind::Ere => {
                self.st[p][0] += Self::cnt(d, 0) + 2 * Self::cnt(d, 1);
                self.st[p][1] += 2 * Self::cnt(d, 2);
                self.st[p][2] += Self::cnt(d, 3);
                self.st[p][3] += Self::cnt(d, 4);
                let sk = Self::cnt(d, 5);
                if sk >= 3 {
                    self.st[p][0] = 0;
                } else if sk == 2 {
                    self.st[p][3] -= 1;
                }
                while self.st[p][2] >= 3 {
                    self.st[p][2] -= 3;
                    self.st[p][3] += 2;
                }
                while self.st[p][0] >= 6 {
                    self.st[p][0] -= 6;
                    self.st[p][3] += 4;
                }
                if self.st[p][1] >= 2 {
                    self.st[p][1] -= 2;
                } else {
                    self.st[p][3] -= 2 - self.st[p][1];
                    self.st[p][1] = 0;
                }
            }
            Kind::Echipa => {
                let mut have = [0i32; 6];
                for &x in d {
                    have[x as usize] += 1;
                }
                let mut need = [0i32; 4];
                for &m in &self.mon {
                    need[m as usize] += 1;
                }
                let mut killed = 0;
                let mut miss = 0;
                for t in 0..4 {
                    let k = need[t].min(have[t]);
                    killed += k;
                    have[t] -= k;
                    miss += need[t] - k;
                }
                let wild = have[4] + have[5];
                let w = miss.min(wild);
                killed += w;
                miss -= w;
                let depth = self.depth as i32;
                self.st[p][1] += killed * depth + if miss == 0 { depth } else { 0 };
                self.st[p][0] -= miss;
            }
            Kind::Oastea => {
                let dmg = Self::cnt(d, 0) + Self::cnt(d, 1) + Self::cnt(d, 2) + Self::cnt(d, 5) / 2;
                let loss = (dmg - self.st[o][1]).max(0);
                self.st[o][0] = (self.st[o][0] - loss).max(0);
                self.st[p][1] = Self::cnt(d, 3);
                self.st[p][0] = (self.st[p][0] + Self::cnt(d, 4) / 2).min(8);
            }
            Kind::Leac => {
                let mut best = (0, 0);
                for dis in 0..4 {
                    if self.st[p][5] >> dis & 1 == 1 {
                        continue;
                    }
                    let g = Self::cnt(d, dis as u8) + Self::cnt(d, 5);
                    if g > best.0 {
                        best = (g, dis);
                    }
                }
                if best.0 >= 3 {
                    let dis = best.1 as usize;
                    self.st[p][dis] += if best.0 >= 5 { 2 } else { 1 };
                    if self.st[p][dis] >= 3 {
                        self.st[p][dis] = 0;
                        self.st[p][5] |= 1 << dis;
                        self.st[p][4] += 1;
                    }
                }
                if Self::cnt(d, 4) >= 3 {
                    if let Some(i) = (0..4)
                        .filter(|&i| self.st[p][i] > 0)
                        .max_by_key(|&i| self.st[p][i])
                    {
                        self.st[p][i] -= 1;
                    }
                }
            }
        }
        self.dice.clear();
        self.pending.clear();
        self.rolls = 0;
        self.mon.clear();
        self.turns += 1;
        self.side = o;
        self.ph = if self.kind == Kind::Echipa {
            Ph::Pre
        } else {
            Ph::Roll
        };
    }

    fn score_x(&self, p: usize) -> i32 {
        let s = &self.st[p];
        match self.kind {
            Kind::Rege => 100 * s[1] + 25 * s[0] + 50 * s[2],
            Kind::Ere => 100 * s[3] + 60 * s[0] + 25 * s[1] + 30 * s[2],
            Kind::Echipa => 100 * s[1] + 60 * s[0],
            Kind::Oastea => 60 * s[0] + 20 * s[1],
            Kind::Leac => 300 * s[4] + 100 * (s[0] + s[1] + s[2] + s[3]),
        }
    }
    fn diff_x(&self, a: usize) -> i32 {
        self.score_x(a) - self.score_x(1 - a)
    }
    /// Valoarea din perspectiva lui `a` dacă tura se rezolvă acum cu zarurile `d`.
    fn value_if(&self, a: usize, d: &[u8]) -> i32 {
        let mut c = self.clone();
        c.resolve(d);
        c.diff_x(a)
    }
    fn dead(&self) -> Option<usize> {
        for p in 0..2 {
            let lost = match self.kind {
                Kind::Rege => self.st[p][0] <= 0,
                Kind::Echipa => self.st[p][0] <= 0,
                Kind::Oastea => self.st[p][0] <= 0,
                _ => false,
            };
            if lost {
                return Some(p);
            }
        }
        None
    }
}

impl Mini for Trei {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Pre => (1..=3).map(|n| format!("nivel {n}")).collect(),
            Ph::Roll => vec!["tira".into()],
            Ph::Keep => {
                let mut v = vec!["fin".to_string()];
                if self.rolls < 3 {
                    let free: Vec<usize> = (0..self.dice.len())
                        .filter(|&i| !self.frozen(self.dice[i]))
                        .collect();
                    for mask in 1u32..(1 << free.len()) {
                        let l: Vec<String> = (0..free.len())
                            .filter(|&k| mask >> k & 1 == 1)
                            .map(|k| (free[k] + 1).to_string())
                            .collect();
                        v.push(format!("relansez {}", l.join(",")));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Pre => {
                self.depth = m[6..].parse().unwrap();
                self.ph = Ph::Roll;
            }
            Ph::Roll => {
                if self.dice.is_empty() {
                    let n = self.ndice();
                    self.dice = (0..n).map(|_| die(&mut self.rng, 6)).collect();
                    if self.kind == Kind::Echipa {
                        self.mon = (0..self.depth + 1).map(|_| die(&mut self.rng, 4)).collect();
                    }
                } else {
                    for &i in &self.pending.clone() {
                        self.dice[i] = die(&mut self.rng, 6);
                    }
                    self.pending.clear();
                }
                self.rolls += 1;
                self.ph = Ph::Keep;
            }
            Ph::Keep => {
                if m == "fin" {
                    let d = self.dice.clone();
                    self.resolve(&d);
                } else {
                    self.pending = m[9..]
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap() - 1)
                        .collect();
                    self.ph = Ph::Roll;
                }
            }
        }
        if self.ph == Ph::Keep && self.rolls >= 3 {
            let d = self.dice.clone();
            self.last = dstr(&d);
            self.resolve(&d);
        }
    }
    fn result(&self) -> Option<Outcome> {
        let sc = |p: usize| self.score_x(p);
        if let Some(l) = self.dead() {
            if self.kind == Kind::Rege || self.kind == Kind::Oastea {
                return Some(Outcome::Win(pl(1 - l)));
            }
            return Some(Outcome::Win(pl(1 - l)));
        }
        match self.kind {
            Kind::Rege => {
                if self.st[0][1] >= 20 {
                    return Some(Outcome::Win(Player::One));
                }
                if self.st[1][1] >= 20 {
                    return Some(Outcome::Win(Player::Two));
                }
            }
            Kind::Leac => {
                if self.st[0][4] >= 3 {
                    return Some(Outcome::Win(Player::One));
                }
                if self.st[1][4] >= 3 {
                    return Some(Outcome::Win(Player::Two));
                }
            }
            _ => {}
        }
        if self.turns >= self.max_turns() {
            return Some(by_score(sc(0), sc(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = self.diff_x(a);
        if self.ph == Ph::Roll && !self.pending.is_empty() {
            // zarurile păstrate + media zarurilor relansate
            let mut kept: Vec<u8> = Vec::new();
            for (i, &x) in self.dice.iter().enumerate() {
                if !self.pending.contains(&i) {
                    kept.push(x);
                }
            }
            let v0 = self.value_if(a, &kept);
            let mut avg = 0;
            for f in 0..6u8 {
                let mut k2 = kept.clone();
                k2.push(f);
                avg += self.value_if(a, &k2) - v0;
            }
            let ev = v0 + avg / 6 * self.pending.len() as i32;
            return ev;
        }
        base
    }
    fn show(&self) -> String {
        let name = match self.kind {
            Kind::Rege => "Regele metropolei",
            Kind::Ere => "Ere și monumente",
            Kind::Echipa => "Echipa de aventurieri",
            Kind::Oastea => "Oastea de zaruri",
            Kind::Leac => "Leacul molimei",
        };
        let mut s = format!(
            "{name} — tura {}/{} | la mutare {} | aruncarea {}/3\n",
            self.turns + 1,
            self.max_turns(),
            pname(pl(self.side)),
            self.rolls
        );
        for p in 0..2 {
            let t = &self.st[p];
            let line = match self.kind {
                Kind::Rege => format!("viață {}, puncte {}, energie {}", t[0], t[1], t[2]),
                Kind::Ere => format!(
                    "muncitori {}, hrană {}, mărfuri {}, puncte {}",
                    t[0], t[1], t[2], t[3]
                ),
                Kind::Echipa => format!("viață {}, experiență {}", t[0], t[1]),
                Kind::Oastea => format!("unități {}, scuturi {}", t[0], t[1]),
                Kind::Leac => format!("probe {:?}, leacuri {}", &t[0..4], t[4]),
            };
            s.push_str(&format!("J{}: {}\n", p + 1, line));
        }
        if self.kind == Kind::Echipa {
            s.push_str(&format!(
                "Adâncime {} | monștri (tipuri 1–4): {}\n",
                self.depth,
                dstr(&self.mon)
            ));
        }
        if !self.dice.is_empty() {
            s.push_str(&format!("Zaruri: {}\n", dstr(&self.dice)));
        }
        s.push_str(&format!("Fețe: {}\n", self.legend()));
        s.push_str("Mutări: nivel n | tira | relansez i,j | fin\n");
        s
    }
}

impl Trei {
    fn legend(&self) -> &'static str {
        match self.kind {
            Kind::Rege => "1,2,3 = puncte la trei la fel; 4 = atac; 5 = vindecare; 6 = energie",
            Kind::Ere => {
                "1 muncă; 2 muncă ×2; 3 hrană ×2; 4 marfă; 5 monedă; 6 craniu (nu se relansează)"
            }
            Kind::Echipa => {
                "1 luptător; 2 mag; 3 preot; 4 hoț; 5 erou (orice); 6 pergament (orice)"
            }
            Kind::Oastea => "1 corp la corp; 2 tragere; 3 magie; 4 scut; 5 manevră; 6 semn",
            Kind::Leac => "1–4 boli; 5 pericol (nu se relansează); 6 medic (orice boală)",
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;

    fn full(k: Kind, plies: usize) {
        match k {
            Kind::Rege => check(|s| Trei::with_seed(Kind::Rege, s), plies),
            Kind::Ere => check(|s| Trei::with_seed(Kind::Ere, s), plies),
            Kind::Echipa => check(|s| Trei::with_seed(Kind::Echipa, s), plies),
            Kind::Oastea => check(|s| Trei::with_seed(Kind::Oastea, s), plies),
            Kind::Leac => check(|s| Trei::with_seed(Kind::Leac, s), plies),
        }
        let mut g = Trei::with_seed(k, 4);
        advance(&mut g, 60, |m| m.ph == Ph::Keep && m.rolls == 1);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn rege() {
        full(Kind::Rege, 2500);
    }
    #[test]
    fn ere() {
        full(Kind::Ere, 1500);
    }
    #[test]
    fn echipa() {
        full(Kind::Echipa, 1000);
    }
    #[test]
    fn oastea() {
        full(Kind::Oastea, 2500);
    }
    #[test]
    fn leac() {
        full(Kind::Leac, 2500);
    }
}
