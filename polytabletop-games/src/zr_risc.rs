//! Familia „riscă sau oprește-te” cu zaruri (lista C6): șase variante originale, 2 jucători.
//!
//! Aruncarea e mereu mutarea forțată `tira`; după ea zarurile sunt publice. `eval` nu citește
//! generatorul (AI cu adâncime fixă 0). Variantele: farfurii_zburatoare, verde_galben_rosu,
//! pofta_zarurilor, purcei_pe_masa, zarul_nebuniei, peritorii_coroanei.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Martian,
    TossUp,
    Greed,
    Pigs,
    Cthulhu,
    Court,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Roll,
    Pick,
    Cont,
    Choose,
}

#[derive(Clone)]
pub struct Risc {
    kind: Kind,
    rng: u64,
    side: usize,
    pts: [i32; 2],
    round: u32,
    rounds: u32,
    ph: Ph,
    /// zarurile aruncate acum (valori 0..5)
    dice: Vec<u8>,
    left: usize,
    kept: [u8; 6],
    taken: u8,
    tanks: u8,
    acc: i32,
    san: [i32; 2],
    last: String,
    turns: u32,
}

const MARTIAN_NAMES: [&str; 5] = ["tanc", "raza", "om", "vaca", "gaina"];

impl Risc {
    pub fn new(kind: Kind) -> Self {
        Self::with_seed(kind, seed_now())
    }
    pub fn with_seed(kind: Kind, seed: u64) -> Self {
        let (left, rounds) = match kind {
            Kind::Martian => (10, 6),
            Kind::TossUp => (10, 6),
            Kind::Greed => (6, 6),
            Kind::Pigs => (2, 0),
            Kind::Cthulhu => (1, 0),
            Kind::Court => (6, 5),
        };
        Risc {
            kind,
            rng: seed_state(seed),
            side: 0,
            pts: [0; 2],
            round: 0,
            rounds,
            ph: Ph::Roll,
            dice: Vec::new(),
            left,
            kept: [0; 6],
            taken: 0,
            tanks: 0,
            acc: 0,
            san: [3, 3],
            last: String::new(),
            turns: 0,
        }
    }
    fn start_left(&self) -> usize {
        match self.kind {
            Kind::Martian | Kind::TossUp => 10,
            Kind::Greed | Kind::Court => 6,
            _ => 2,
        }
    }
    fn end_turn(&mut self, bank: bool) {
        if bank {
            self.pts[self.side] += self.acc;
        }
        self.acc = 0;
        self.kept = [0; 6];
        self.taken = 0;
        self.tanks = 0;
        self.dice.clear();
        self.left = self.start_left();
        self.ph = Ph::Roll;
        self.turns += 1;
        self.side = 1 - self.side;
        if self.side == 0 {
            self.round += 1;
        }
    }
    fn rays(&self) -> u8 {
        self.kept[1]
    }
    fn martian_score(&self) -> i32 {
        let k = &self.kept;
        let mut s = (k[2] + k[3] + k[4]) as i32;
        if k[2] > 0 && k[3] > 0 && k[4] > 0 {
            s += 3;
        }
        s
    }
    fn court_score(&self) -> i32 {
        let mut s = 0;
        for v in 0..6 {
            s += (v as i32 + 1) * self.kept[v] as i32;
        }
        if self.taken == 63 {
            s += 8;
        }
        s
    }
    fn greed_score(c: &[u8; 6]) -> i32 {
        let mut s = 0;
        for v in 0..6 {
            let n = c[v] as i32;
            if n >= 3 {
                let base = if v == 0 { 100 } else { (v as i32 + 1) * 10 };
                s += base << (n - 3);
            } else if v == 0 {
                s += n * 10;
            } else if v == 4 {
                s += n * 5;
            }
        }
        s
    }
    fn greed_options(&self) -> Vec<[u8; 6]> {
        let mut c = [0u8; 6];
        for &d in &self.dice {
            c[d as usize] += 1;
        }
        let mut out: Vec<[u8; 6]> = vec![[0; 6]];
        for v in 0..6 {
            let mut opts: Vec<u8> = vec![0];
            if v == 0 || v == 4 {
                opts.extend(1..=c[v]);
            } else if c[v] >= 3 {
                opts.extend(3..=c[v]);
            }
            let mut nx = Vec::new();
            for o in &out {
                for &k in &opts {
                    let mut n = *o;
                    n[v] = k;
                    nx.push(n);
                }
            }
            out = nx;
        }
        out.retain(|o| o.iter().any(|&x| x > 0));
        out
    }
    /// Pigs: valoarea unei aruncări de doi purcei (`None` = pierzi tot).
    fn pig_score(a: usize, b: usize) -> Option<i32> {
        let val = |x: usize| [0, 0, 5, 10, 15][x];
        if a < 2 && b < 2 {
            return if a == b { Some(1) } else { None };
        }
        let s = val(a) + val(b);
        Some(if a == b { 2 * s } else { s })
    }
    fn risk(&self) -> i32 {
        match self.left {
            0 | 1 => 40,
            2 => 62,
            3 => 78,
            4 => 86,
            _ => 92,
        }
    }
    fn unbanked(&self) -> i32 {
        match self.kind {
            Kind::Martian => {
                let rf = if self.rays() >= self.tanks {
                    self.risk()
                } else {
                    45
                };
                self.acc.max(self.martian_score()) * rf + self.left as i32 * 30
            }
            Kind::TossUp => self.acc * 100 * self.risk() / 100 + self.left as i32 * 22,
            Kind::Greed => self.acc * self.risk() + 40,
            Kind::Court => self.court_score() * self.risk() + self.left as i32 * 25,
            Kind::Pigs => self.acc * 79 + 395,
            Kind::Cthulhu => 0,
        }
    }
}

impl Mini for Risc {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["tira".into()],
            Ph::Pick => match self.kind {
                Kind::Martian => {
                    let mut v = Vec::new();
                    for t in 1..5 {
                        if self.taken & (1 << t) == 0
                            && self.dice.iter().any(|&d| mart_type(d) == t)
                        {
                            v.push(format!("pastrez {}", MARTIAN_NAMES[t]));
                        }
                    }
                    v
                }
                Kind::Court => (0..6)
                    .filter(|&x| self.taken & (1 << x) == 0 && self.dice.contains(&(x as u8)))
                    .map(|x| format!("pastrez {}", x + 1))
                    .collect(),
                Kind::Greed => self
                    .greed_options()
                    .iter()
                    .map(|o| {
                        let mut s = String::from("pastrez");
                        for v in 0..6 {
                            if o[v] > 0 {
                                s.push_str(&format!(" {}x{}", v + 1, o[v]));
                            }
                        }
                        s
                    })
                    .collect(),
                _ => vec![],
            },
            Ph::Cont => {
                let mut v = vec!["continui".to_string()];
                let ok = match self.kind {
                    Kind::Martian => self.rays() >= self.tanks,
                    Kind::Greed => self.acc >= 15,
                    _ => true,
                };
                if ok {
                    v.push("opresc".into());
                }
                if self.kind == Kind::Pigs {
                    v[0] = "arunc".into();
                }
                v
            }
            Ph::Choose => vec!["lovesc".into(), "vindec".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Roll => self.do_roll(),
            Ph::Pick => self.do_pick(m),
            Ph::Cont => {
                if m == "opresc" {
                    let a = match self.kind {
                        Kind::Martian => self.martian_score(),
                        Kind::Court => self.court_score(),
                        _ => self.acc,
                    };
                    self.acc = a;
                    self.end_turn(true);
                } else {
                    self.ph = Ph::Roll;
                }
            }
            Ph::Choose => {
                let s = self.side;
                if m == "lovesc" {
                    self.san[1 - s] -= 1;
                } else {
                    self.san[s] = (self.san[s] + 1).min(4);
                }
                self.end_turn(false);
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        match self.kind {
            Kind::Pigs => {
                if self.pts[0] >= 50 {
                    return Some(Outcome::Win(Player::One));
                }
                if self.pts[1] >= 50 {
                    return Some(Outcome::Win(Player::Two));
                }
                None
            }
            Kind::Cthulhu => {
                if self.san[0] <= 0 && self.san[1] <= 0 {
                    return Some(Outcome::Draw);
                }
                if self.san[0] <= 0 {
                    return Some(Outcome::Win(Player::Two));
                }
                if self.san[1] <= 0 {
                    return Some(Outcome::Win(Player::One));
                }
                if self.turns >= 80 {
                    return Some(by_score(self.san[0], self.san[1]));
                }
                None
            }
            _ => {
                if self.round >= self.rounds {
                    Some(by_score(self.pts[0], self.pts[1]))
                } else {
                    None
                }
            }
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        if self.kind == Kind::Cthulhu {
            return 1000 * (self.san[a] - self.san[o]);
        }
        let mut e = 100 * (self.pts[a] - self.pts[o]);
        if self.turn_started() {
            let u = self.unbanked();
            e += if self.side == a { u } else { -u };
        }
        e
    }
    fn show(&self) -> String {
        let name = match self.kind {
            Kind::Martian => "Farfurii zburătoare",
            Kind::TossUp => "Verde, galben, roșu",
            Kind::Greed => "Pofta zarurilor",
            Kind::Pigs => "Purcei pe masă",
            Kind::Cthulhu => "Zarul nebuniei",
            Kind::Court => "Peritorii coroanei",
        };
        let mut s = format!("{name} — tura {}", self.turns + 1);
        if self.rounds > 0 {
            s.push_str(&format!(
                " (runda {}/{})",
                (self.round + 1).min(self.rounds),
                self.rounds
            ));
        }
        s.push('\n');
        if self.kind == Kind::Cthulhu {
            s.push_str(&format!(
                "Luciditate: J1 {}, J2 {}\n",
                self.san[0], self.san[1]
            ));
        } else {
            s.push_str(&format!("Puncte: J1 {}, J2 {}\n", self.pts[0], self.pts[1]));
        }
        s.push_str(&format!(
            "La mutare: {} | faza {:?} | puncte în tură {} | zaruri rămase {}\n",
            pname(self.side()),
            self.ph,
            match self.kind {
                Kind::Martian => self.martian_score(),
                Kind::Court => self.court_score(),
                _ => self.acc,
            },
            self.left
        ));
        if !self.dice.is_empty() {
            s.push_str(&format!("Zaruri aruncate: {}\n", dstr(&self.dice)));
        }
        if self.kind == Kind::Martian {
            s.push_str(&format!(
                "Păstrate: tancuri {}, raze {}, oameni {}, vaci {}, găini {}\n",
                self.tanks, self.kept[1], self.kept[2], self.kept[3], self.kept[4]
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima aruncare: {}\n", self.last));
        }
        s.push_str("Mutări: tira | pastrez … | continui/arunc | opresc | lovesc/vindec\n");
        s
    }
}

fn mart_type(d: u8) -> usize {
    match d {
        0 => 0,
        1 | 5 => 1,
        x => x as usize,
    }
}

impl Risc {
    fn turn_started(&self) -> bool {
        self.acc > 0
            || self.left != self.start_left()
            || self.taken != 0
            || self.kept.iter().any(|&x| x > 0)
    }
    fn do_roll(&mut self) {
        match self.kind {
            Kind::Martian => {
                let n = self.left;
                self.dice = (0..n).map(|_| die(&mut self.rng, 6)).collect();
                let tk = self.dice.iter().filter(|&&d| d == 0).count();
                self.tanks += tk as u8;
                self.left -= tk;
                self.dice.retain(|&d| d != 0);
                let avail = (1..5).any(|t| {
                    self.taken & (1 << t) == 0 && self.dice.iter().any(|&d| mart_type(d) == t)
                });
                if avail {
                    self.ph = Ph::Pick;
                } else {
                    self.last = "ratat".into();
                    self.end_turn(false);
                }
            }
            Kind::Court => {
                let n = self.left;
                self.dice = (0..n).map(|_| die(&mut self.rng, 6)).collect();
                if (0..6).any(|x| self.taken & (1 << x) == 0 && self.dice.contains(&(x as u8))) {
                    self.ph = Ph::Pick;
                } else {
                    self.last = "ratat".into();
                    self.end_turn(false);
                }
            }
            Kind::TossUp => {
                let n = self.left;
                self.dice = (0..n).map(|_| die(&mut self.rng, 6)).collect();
                let g = self.dice.iter().filter(|&&d| d < 3).count();
                let r = self.dice.iter().filter(|&&d| d == 5).count();
                if g == 0 && r > 0 {
                    self.last = "roșu fără verde: ratat".into();
                    self.end_turn(false);
                    return;
                }
                self.acc += g as i32;
                self.left -= g + r;
                if self.left == 0 {
                    self.end_turn(true);
                } else {
                    self.ph = Ph::Cont;
                }
            }
            Kind::Greed => {
                let n = self.left;
                self.dice = (0..n).map(|_| die(&mut self.rng, 6)).collect();
                if self.greed_options().is_empty() {
                    self.last = "nimic de punctat".into();
                    self.end_turn(false);
                } else {
                    self.ph = Ph::Pick;
                }
            }
            Kind::Pigs => {
                let w = [35, 30, 20, 10, 5];
                let a = weighted(&mut self.rng, &w);
                let b = weighted(&mut self.rng, &w);
                self.dice = vec![a as u8, b as u8];
                let names = ["stânga", "dreapta", "spate", "picioare", "urechi"];
                self.last = format!("{} + {}", names[a], names[b]);
                match Self::pig_score(a, b) {
                    None => self.end_turn(false),
                    Some(s) => {
                        self.acc += s;
                        self.ph = Ph::Cont;
                    }
                }
            }
            Kind::Cthulhu => {
                let f = die(&mut self.rng, 12);
                self.dice = vec![f % 6];
                let s = self.side;
                match f {
                    0 => {
                        self.last = "marele zeu: cel cu mai multă luciditate pierde 1".into();
                        let mx = self.san[0].max(self.san[1]);
                        for p in 0..2 {
                            if self.san[p] == mx {
                                self.san[p] -= 1;
                            }
                        }
                        self.end_turn(false);
                    }
                    1..=4 => {
                        self.last = "tentacul: pierzi 1".into();
                        self.san[s] -= 1;
                        self.end_turn(false);
                    }
                    5..=7 => {
                        self.last = "semn: alegi".into();
                        self.ph = Ph::Choose;
                    }
                    8 | 9 => {
                        self.last = "ochi: cel cu mai puțină luciditate pierde 1".into();
                        let mn = self.san[0].min(self.san[1]);
                        for p in 0..2 {
                            if self.san[p] == mn && self.san[0] != self.san[1] {
                                self.san[p] -= 1;
                            }
                        }
                        if self.san[0] == self.san[1] {
                            self.san[s] -= 1;
                        }
                        self.end_turn(false);
                    }
                    _ => {
                        self.last = "soare: +1".into();
                        self.san[s] = (self.san[s] + 1).min(4);
                        self.end_turn(false);
                    }
                }
            }
        }
    }
    fn do_pick(&mut self, m: &str) {
        match self.kind {
            Kind::Martian => {
                let name = m.split(' ').nth(1).unwrap();
                let t = MARTIAN_NAMES.iter().position(|&x| x == name).unwrap();
                let c = self.dice.iter().filter(|&&d| mart_type(d) == t).count();
                self.kept[t] += c as u8;
                self.taken |= 1 << t;
                self.left -= c;
                self.dice.clear();
                if self.left == 0 {
                    if self.rays() >= self.tanks {
                        self.acc = self.martian_score();
                        self.end_turn(true);
                    } else {
                        self.end_turn(false);
                    }
                } else {
                    self.ph = Ph::Cont;
                }
            }
            Kind::Court => {
                let v: usize = m.split(' ').nth(1).unwrap().parse::<usize>().unwrap() - 1;
                let c = self.dice.iter().filter(|&&d| d as usize == v).count();
                self.kept[v] += c as u8;
                self.taken |= 1 << v;
                self.left -= c;
                self.dice.clear();
                if self.left == 0 {
                    self.acc = self.court_score();
                    self.end_turn(true);
                } else {
                    self.ph = Ph::Cont;
                }
            }
            _ => {
                // Greed
                let mut c = [0u8; 6];
                for t in m.split(' ').skip(1) {
                    let (v, k) = t.split_once('x').unwrap();
                    c[v.parse::<usize>().unwrap() - 1] = k.parse().unwrap();
                }
                let n: u8 = c.iter().sum();
                self.acc += Self::greed_score(&c);
                self.left -= n as usize;
                self.dice.clear();
                if self.left == 0 {
                    self.left = 6;
                    self.last = "toate zarurile punctate: arunci din nou pe toate".into();
                }
                self.ph = Ph::Cont;
            }
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;

    fn all(k: Kind, plies: usize) {
        match k {
            Kind::Martian => check(|s| Risc::with_seed(Kind::Martian, s), plies),
            Kind::TossUp => check(|s| Risc::with_seed(Kind::TossUp, s), plies),
            Kind::Greed => check(|s| Risc::with_seed(Kind::Greed, s), plies),
            Kind::Pigs => check(|s| Risc::with_seed(Kind::Pigs, s), plies),
            Kind::Cthulhu => check(|s| Risc::with_seed(Kind::Cthulhu, s), plies),
            Kind::Court => check(|s| Risc::with_seed(Kind::Court, s), plies),
        }
        // AI-ul nu citește generatorul
        let mut g = Risc::with_seed(k, 4);
        advance(&mut g, 40, |m| m.ph != Ph::Roll);
        no_peek(&g, |m| m.rng ^= 0xABCDEF1234567);
    }

    #[test]
    fn martian() {
        all(Kind::Martian, 600);
    }
    #[test]
    fn tossup() {
        all(Kind::TossUp, 600);
    }
    #[test]
    fn greed() {
        all(Kind::Greed, 800);
    }
    #[test]
    fn pigs() {
        all(Kind::Pigs, 1500);
    }
    #[test]
    fn cthulhu() {
        all(Kind::Cthulhu, 600);
    }
    #[test]
    fn court() {
        all(Kind::Court, 600);
    }
    #[test]
    fn greed_scoring() {
        assert_eq!(Risc::greed_score(&[3, 0, 0, 0, 0, 0]), 100);
        assert_eq!(Risc::greed_score(&[1, 0, 0, 0, 2, 0]), 20);
        assert_eq!(Risc::greed_score(&[0, 3, 0, 0, 0, 0]), 20);
    }
}
