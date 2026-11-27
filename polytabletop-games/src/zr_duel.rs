//! Cinci dueluri cu zaruri (lista C6, variante originale, 2 jucători, informație publică):
//! `cifra_cea_mare`, `zece_la_fel`, `vartej_de_zaruri`, `petale_si_spini`, `zaruri_in_duel`.
//! Aruncările sunt mutări forțate `tira`; `eval` nu citește generatorul.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

// ---------------------------------------------------------------- cifra_cea_mare
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CPh {
    Size,
    RollS,
    ArrS,
    Resp,
    RollR,
    ArrR,
}

#[derive(Clone)]
pub struct Cifra {
    rng: u64,
    round: u32,
    ph: CPh,
    n: usize,
    dice: Vec<u8>,
    xs: i64,
    pts: [i32; 2],
}

fn perms(d: &[u8]) -> Vec<Vec<u8>> {
    if d.len() <= 1 {
        return vec![d.to_vec()];
    }
    let mut out: Vec<Vec<u8>> = Vec::new();
    for i in 0..d.len() {
        if d[..i].contains(&d[i]) {
            continue;
        }
        let mut r = d.to_vec();
        let x = r.remove(i);
        for mut p in perms(&r) {
            p.insert(0, x);
            out.push(p);
        }
    }
    out
}

fn num(d: &[u8]) -> i64 {
    d.iter().fold(0, |a, &x| a * 10 + x as i64 + 1)
}

/// (P(număr mai mare ca x), P(mai mic)) pentru `n` zaruri aranjate optim, în mii.
fn cmp_prob(n: usize, x: i64) -> (i32, i32) {
    let tot = 6i32.pow(n as u32);
    let (mut gt, mut lt) = (0, 0);
    for k in 0..tot {
        let mut d = Vec::new();
        let mut t = k;
        for _ in 0..n {
            d.push((t % 6) as u8);
            t /= 6;
        }
        d.sort_unstable_by(|a, b| b.cmp(a));
        let v = num(&d);
        if v > x {
            gt += 1;
        } else if v < x {
            lt += 1;
        }
    }
    (gt * 1000 / tot, lt * 1000 / tot)
}

impl Cifra {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Cifra {
            rng: seed_state(seed),
            round: 0,
            ph: CPh::Size,
            n: 2,
            dice: vec![],
            xs: 0,
            pts: [0; 2],
        }
    }
    fn starter(&self) -> usize {
        (self.round % 2) as usize
    }
    fn finish(&mut self, resp_num: Option<i64>) {
        let s = self.starter();
        let w = (self.n - 1) as i32;
        match resp_num {
            None => self.pts[s] += 1,
            Some(r) => {
                if r > self.xs {
                    self.pts[1 - s] += w;
                } else if r < self.xs {
                    self.pts[s] += w;
                }
            }
        }
        self.round += 1;
        self.ph = CPh::Size;
        self.dice.clear();
    }
}

impl Default for Cifra {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cifra {
    fn side(&self) -> Player {
        let s = self.starter();
        pl(match self.ph {
            CPh::Size | CPh::RollS | CPh::ArrS => s,
            _ => 1 - s,
        })
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            CPh::Size => (2..=4).map(|n| format!("zaruri {n}")).collect(),
            CPh::RollS | CPh::RollR => vec!["tira".into()],
            CPh::ArrS | CPh::ArrR => perms(&self.dice)
                .iter()
                .map(|p| format!("ordin {}", dstr(p).replace(' ', "")))
                .collect(),
            CPh::Resp => vec!["primesc".into(), "renunt".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            CPh::Size => {
                self.n = m[7..].parse().unwrap();
                self.ph = CPh::RollS;
            }
            CPh::RollS | CPh::RollR => {
                self.dice = (0..self.n).map(|_| die(&mut self.rng, 6)).collect();
                self.ph = if self.ph == CPh::RollS {
                    CPh::ArrS
                } else {
                    CPh::ArrR
                };
            }
            CPh::ArrS => {
                self.xs = m[6..].parse().unwrap();
                self.dice.clear();
                self.ph = CPh::Resp;
            }
            CPh::ArrR => {
                let r: i64 = m[6..].parse().unwrap();
                self.finish(Some(r));
            }
            CPh::Resp => {
                if m == "renunt" {
                    self.finish(None);
                } else {
                    self.ph = CPh::RollR;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 6 {
            Some(by_score(self.pts[0], self.pts[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let s = self.starter();
        let mut e = 1000 * (self.pts[a] - self.pts[1 - a]);
        if matches!(self.ph, CPh::Resp | CPh::RollR) {
            let (gt, lt) = cmp_prob(self.n, self.xs);
            let w = (self.n - 1) as i32;
            let mut net = (lt - gt) * w;
            if self.ph == CPh::Resp {
                net = net.min(1000);
            }
            e += if a == s { net } else { -net };
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Cifra cea mare — runda {}/6 | puncte J1 {}, J2 {}\nÎncepe runda: {} | zaruri/rundă: {}\n",
            (self.round + 1).min(6),
            self.pts[0],
            self.pts[1],
            pname(pl(self.starter())),
            self.n
        );
        s.push_str(&format!(
            "Faza {:?}; la mutare {}\n",
            self.ph,
            pname(self.side())
        ));
        if matches!(self.ph, CPh::Resp | CPh::RollR) {
            s.push_str(&format!("Numărul de depășit: {}\n", self.xs));
        }
        if !self.dice.is_empty() {
            s.push_str(&format!("Zaruri: {}\n", dstr(&self.dice)));
        }
        s.push_str("Mutări: zaruri n | tira | ordin cifre | primesc | renunt\n");
        s
    }
}

// ---------------------------------------------------------------- zece_la_fel
#[derive(Clone)]
pub struct Zece {
    rng: u64,
    side: usize,
    target: [Option<u8>; 2],
    locked: [u8; 2],
    dice: Vec<u8>,
    rolled: bool,
    turns: u32,
    done: [bool; 2],
}

impl Zece {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Zece {
            rng: seed_state(seed),
            side: 0,
            target: [None; 2],
            locked: [0; 2],
            dice: vec![],
            rolled: false,
            turns: 0,
            done: [false; 2],
        }
    }
    fn cnt(&self, v: u8) -> u8 {
        self.dice.iter().filter(|&&d| d == v).count() as u8
    }
    fn nxt(&mut self) {
        self.dice.clear();
        self.rolled = false;
        if self.locked[self.side] >= 10 {
            self.done[self.side] = true;
        }
        self.side = 1 - self.side;
        self.turns += 1;
    }
}

impl Default for Zece {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Zece {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if !self.rolled {
            return vec!["tira".into()];
        }
        let s = self.side;
        let mut v = Vec::new();
        match self.target[s] {
            None => {
                for t in 0..6u8 {
                    if self.cnt(t) > 0 {
                        v.push(format!("tinta {}", t + 1));
                    }
                }
            }
            Some(t) => {
                v.push("bloc".into());
                for x in 0..6u8 {
                    if x != t && self.cnt(x) > self.locked[s] {
                        v.push(format!("schimb {}", x + 1));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let s = self.side;
        if !self.rolled {
            let n = 10 - self.locked[s] as usize;
            self.dice = (0..n).map(|_| die(&mut self.rng, 6)).collect();
            self.rolled = true;
            return;
        }
        if m == "bloc" {
            let t = self.target[s].unwrap();
            self.locked[s] += self.cnt(t);
        } else {
            let t: u8 = m.split(' ').nth(1).unwrap().parse::<u8>().unwrap() - 1;
            // schimbarea (sau alegerea ținței) blochează toate zarurile cu acea față
            self.target[s] = Some(t);
            self.locked[s] = self.cnt(t);
        }
        self.nxt();
    }
    fn result(&self) -> Option<Outcome> {
        if self.done[0] && self.done[1] {
            return Some(Outcome::Draw);
        }
        if self.done[0] && self.side == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.done[1] {
            return Some(Outcome::Win(Player::Two));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.locked[a] as i32 - self.locked[1 - a] as i32)
    }
    fn show(&self) -> String {
        format!(
            "Zece la fel — blocate: J1 {} (țintă {:?}), J2 {} (țintă {:?})\nLa mutare: {}\nZaruri aruncate: {}\nMutări: tira | tinta v | bloc | schimb v\n",
            self.locked[0],
            self.target[0].map(|x| x + 1),
            self.locked[1],
            self.target[1].map(|x| x + 1),
            pname(self.side()),
            dstr(&self.dice)
        )
    }
}

// ---------------------------------------------------------------- vartej_de_zaruri
#[derive(Clone)]
pub struct Vartej {
    rng: u64,
    side: usize,
    remain: [usize; 2],
    hand: Vec<u8>,
    piles: [u8; 3],
    rolled: bool,
    turns: u32,
}

impl Vartej {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = seed_state(seed);
        let piles = [die(&mut rng, 6), die(&mut rng, 6), die(&mut rng, 6)];
        Vartej {
            rng,
            side: 0,
            remain: [8, 8],
            hand: vec![],
            piles,
            rolled: false,
            turns: 0,
        }
    }
}

impl Default for Vartej {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Vartej {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if !self.rolled {
            return vec!["tira".into()];
        }
        let mut v = Vec::new();
        for val in 0..6u8 {
            if self.hand.contains(&val) {
                for (j, &t) in self.piles.iter().enumerate() {
                    if (val as i32 - t as i32).abs() <= 1 {
                        v.push(format!("joc {} {}", val + 1, j + 1));
                    }
                }
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        if !self.rolled {
            self.hand = (0..self.remain[self.side])
                .map(|_| die(&mut self.rng, 6))
                .collect();
            self.rolled = true;
            return;
        }
        if m == "pas" {
            self.hand.clear();
            self.rolled = false;
            self.side = 1 - self.side;
            self.turns += 1;
            return;
        }
        let mut it = m.split(' ').skip(1);
        let val: u8 = it.next().unwrap().parse::<u8>().unwrap() - 1;
        let j: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let k = self.hand.iter().position(|&d| d == val).unwrap();
        self.hand.remove(k);
        self.piles[j] = val;
        self.remain[self.side] -= 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.remain[0] == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.remain[1] == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.turns >= 120 {
            return Some(by_score(
                8 - self.remain[0] as i32,
                8 - self.remain[1] as i32,
            ));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.remain[1 - a] as i32 - self.remain[a] as i32)
    }
    fn show(&self) -> String {
        format!(
            "Vârtej de zaruri — rămase: J1 {}, J2 {}\nGrămezi (vârfuri): {}\nLa mutare: {} | zaruri în mână: {}\nMutări: tira | joc v g (zar v pe grămada g, diferență ≤ 1) | pas\n",
            self.remain[0],
            self.remain[1],
            dstr(&self.piles),
            pname(self.side()),
            dstr(&self.hand)
        )
    }
}

// ---------------------------------------------------------------- petale_si_spini
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PPh {
    Count,
    Roll,
    Guess,
}

#[derive(Clone)]
pub struct Petale {
    rng: u64,
    round: u32,
    ph: PPh,
    n: usize,
    dice: Vec<u8>,
    pts: [i32; 2],
}

fn petals(d: &[u8]) -> usize {
    d.iter()
        .map(|&x| match x {
            2 => 2,
            4 => 4,
            _ => 0,
        })
        .sum()
}

impl Petale {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Petale {
            rng: seed_state(seed),
            round: 0,
            ph: PPh::Count,
            n: 5,
            dice: vec![],
            pts: [0; 2],
        }
    }
    fn roller(&self) -> usize {
        (self.round % 2) as usize
    }
}

impl Default for Petale {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Petale {
    fn side(&self) -> Player {
        match self.ph {
            PPh::Guess => pl(1 - self.roller()),
            _ => pl(self.roller()),
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            PPh::Count => (3..=5).map(|n| format!("zaruri {n}")).collect(),
            PPh::Roll => vec!["tira".into()],
            PPh::Guess => {
                let mut v = vec!["trec".to_string()];
                v.extend((0..=self.n * 4).step_by(2).map(|k| format!("ghic {k}")));
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            PPh::Count => {
                self.n = m[7..].parse().unwrap();
                self.ph = PPh::Roll;
            }
            PPh::Roll => {
                self.dice = (0..self.n).map(|_| die(&mut self.rng, 6)).collect();
                self.ph = PPh::Guess;
            }
            PPh::Guess => {
                let g = 1 - self.roller();
                if let Some(k) = m.strip_prefix("ghic ") {
                    let k: usize = k.parse().unwrap();
                    if k == petals(&self.dice) {
                        self.pts[g] += self.n as i32 - 2;
                    } else {
                        self.pts[self.roller()] += 1;
                    }
                }
                self.round += 1;
                self.ph = PPh::Count;
                self.dice.clear();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 8 {
            Some(by_score(self.pts[0], self.pts[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        1000 * (self.pts[a] - self.pts[1 - a])
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Petale și spini — runda {}/8 | puncte J1 {}, J2 {}\nAruncă: {} | la mutare: {}\n",
            (self.round + 1).min(8),
            self.pts[0],
            self.pts[1],
            pname(pl(self.roller())),
            pname(self.side())
        );
        if !self.dice.is_empty() {
            s.push_str(&format!("Zaruri: {}\n", dstr(&self.dice)));
        }
        if self.round >= 3 {
            s.push_str("Regula descoperită: zarul 3 valorează 2 petale, zarul 5 valorează 4, restul 0; răspunsul e suma petalelor.\n");
        } else {
            s.push_str("Ghicește suma „petalelor” din zaruri: există o regulă ascunsă (se dezvăluie după runda 3).\n");
        }
        s.push_str("Mutări: zaruri n | tira | ghic k | trec\n");
        s
    }
}

// ---------------------------------------------------------------- zaruri_in_duel
const SIDES: [u8; 5] = [4, 6, 8, 12, 20];

#[derive(Clone)]
pub struct Butoane {
    rng: u64,
    side: usize,
    /// (fețe, valoare) per jucător
    dice: [Vec<(u8, u8)>; 2],
    pending: Vec<(usize, usize)>,
    passes: u8,
    turns: u32,
}

impl Butoane {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let d: Vec<(u8, u8)> = SIDES.iter().map(|&s| (s, 0)).collect();
        let mut pend = Vec::new();
        for p in 0..2 {
            for i in 0..5 {
                pend.push((p, i));
            }
        }
        Butoane {
            rng: seed_state(seed),
            side: 0,
            dice: [d.clone(), d],
            pending: pend,
            passes: 0,
            turns: 0,
        }
    }
    fn total(&self, p: usize) -> i32 {
        self.dice[p].iter().map(|x| x.0 as i32).sum()
    }
    fn subsets(&self) -> Vec<(Vec<usize>, usize)> {
        // atacuri de îndemânare: suma a ≥ 2 zaruri proprii = valoarea unei ținte
        let s = self.side;
        let mine = &self.dice[s];
        let mut out = Vec::new();
        for mask in 1u32..(1 << mine.len()) {
            if mask.count_ones() < 2 {
                continue;
            }
            let idx: Vec<usize> = (0..mine.len()).filter(|&i| mask >> i & 1 == 1).collect();
            let sum: i32 = idx.iter().map(|&i| mine[i].1 as i32).sum();
            for (t, d) in self.dice[1 - s].iter().enumerate() {
                if d.1 as i32 == sum {
                    out.push((idx.clone(), t));
                }
            }
        }
        out
    }
}

impl Default for Butoane {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Butoane {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if !self.pending.is_empty() {
            return vec!["tira".into()];
        }
        let s = self.side;
        let mut v = vec!["pas".to_string()];
        for (a, da) in self.dice[s].iter().enumerate() {
            for (t, dt) in self.dice[1 - s].iter().enumerate() {
                if da.1 >= dt.1 {
                    v.push(format!("forta {} {}", a + 1, t + 1));
                }
            }
        }
        for (idx, t) in self.subsets() {
            let l: Vec<String> = idx.iter().map(|i| (i + 1).to_string()).collect();
            v.push(format!("indemanare {} {}", l.join(","), t + 1));
        }
        v
    }
    fn step(&mut self, m: &str) {
        if !self.pending.is_empty() {
            let pend = std::mem::take(&mut self.pending);
            for (p, i) in pend {
                let f = self.dice[p][i].0;
                self.dice[p][i].1 = die(&mut self.rng, f as u32) + 1;
            }
            return;
        }
        let s = self.side;
        self.turns += 1;
        if m == "pas" {
            self.passes += 1;
            self.side = 1 - s;
            return;
        }
        self.passes = 0;
        let mut it = m.split(' ');
        let kind = it.next().unwrap();
        let a_s = it.next().unwrap();
        let t: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let atk: Vec<usize> = a_s
            .split(',')
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        let _ = kind;
        self.dice[1 - s].remove(t);
        for &i in &atk {
            self.pending.push((s, i));
        }
        self.side = 1 - s;
    }
    fn result(&self) -> Option<Outcome> {
        if self.dice[0].is_empty() {
            return Some(Outcome::Win(Player::Two));
        }
        if self.dice[1].is_empty() {
            return Some(Outcome::Win(Player::One));
        }
        if self.passes >= 2 || self.turns >= 60 {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let val = |q: usize| -> i32 {
            self.dice[q]
                .iter()
                .map(|d| 10 * d.0 as i32 + 3 * d.1 as i32)
                .sum::<i32>()
        };
        val(a) - val(1 - a)
    }
    fn show(&self) -> String {
        let f = |p: usize| {
            self.dice[p]
                .iter()
                .enumerate()
                .map(|(i, d)| format!("{}:d{}={}", i + 1, d.0, d.1))
                .collect::<Vec<_>>()
                .join(" ")
        };
        format!(
            "Zaruri în duel — J1: {}\n               J2: {}\nLa mutare: {} | de aruncat: {}\nMutări: tira | forta a t | indemanare a,b t | pas\n",
            f(0),
            f(1),
            pname(self.side()),
            self.pending.len()
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn cifra() {
        check(Cifra::with_seed, 200);
        let mut g = Cifra::with_seed(3);
        advance(&mut g, 30, |m| m.ph == CPh::Resp);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
        assert_eq!(num(&[1, 0]), 21);
    }
    #[test]
    fn zece() {
        check(Zece::with_seed, 3000);
        let mut g = Zece::with_seed(3);
        advance(&mut g, 30, |m| m.rolled);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn vartej() {
        check(Vartej::with_seed, 3000);
        let mut g = Vartej::with_seed(3);
        advance(&mut g, 30, |m| m.rolled);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn petale() {
        check(Petale::with_seed, 200);
        let mut g = Petale::with_seed(3);
        advance(&mut g, 30, |m| m.ph == PPh::Guess);
        let right = format!("ghic {}", petals(&g.dice));
        assert_eq!(ai_of(&g, 1).unwrap(), right);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn butoane() {
        check(Butoane::with_seed, 400);
        let mut g = Butoane::with_seed(3);
        advance(&mut g, 30, |m| m.pending.is_empty());
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
}
