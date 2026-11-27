//! Trei jocuri cu zaruri de sine stătătoare (lista C6, variante originale, 2 jucători):
//! `scufundarea_adanca`, `cursa_cuburilor`, `fitilul_aprins`. Aruncările/extragerile sunt
//! mutări forțate `tira`; `eval` nu citește generatorul.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_loc::Rule;
use crate::zr_util::*;

// ---------------------------------------------------------------- scufundarea_adanca
const TREASURE: [i32; 16] = [1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DPh {
    Dir,
    Roll,
    Act,
}

#[derive(Clone)]
pub struct Scufundare {
    rng: u64,
    round: u32,
    side: usize,
    ph: DPh,
    air: i32,
    /// poziții 0 = submarin, 1..=16 = plăci
    pos: [i32; 2],
    up: [bool; 2],
    back: [bool; 2],
    carry: [Vec<i32>; 2],
    saved: [i32; 2],
    tiles: Vec<i32>,
    moved: i32,
    turns: u32,
}

impl Scufundare {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Scufundare {
            rng: seed_state(seed),
            round: 0,
            side: 0,
            ph: DPh::Dir,
            air: 25,
            pos: [0; 2],
            up: [false; 2],
            back: [false; 2],
            carry: [Vec::new(), Vec::new()],
            saved: [0; 2],
            tiles: TREASURE.to_vec(),
            moved: 0,
            turns: 0,
        }
    }
    fn tile_at(&self, p: i32) -> Option<usize> {
        if p >= 1 && (p as usize) <= self.tiles.len() {
            Some(p as usize - 1)
        } else {
            None
        }
    }
    fn occupied(&self, p: i32, who: usize) -> bool {
        p > 0 && self.pos[1 - who] == p && !self.back[1 - who]
    }
    fn next_side(&mut self) {
        let o = 1 - self.side;
        if !self.back[o] {
            self.side = o;
        }
        self.ph = DPh::Dir;
    }
    fn end_round_check(&mut self) -> bool {
        if (self.back[0] && self.back[1]) || self.air <= 0 {
            for p in 0..2 {
                if self.back[p] {
                    self.saved[p] += self.carry[p].iter().sum::<i32>();
                } else {
                    // înecat: comorile se pierd
                    self.carry[p].clear();
                }
                self.carry[p].clear();
                self.pos[p] = 0;
                self.up[p] = false;
                self.back[p] = false;
            }
            self.round += 1;
            self.air = 25;
            self.side = (self.round % 2) as usize;
            // plăcile rămase se păstrează; cele luate dispar
            self.tiles.retain(|&t| t != 0);
            self.ph = DPh::Dir;
            return true;
        }
        false
    }
}

impl Default for Scufundare {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Scufundare {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        let s = self.side;
        match self.ph {
            DPh::Dir => {
                let mut v = vec![];
                if self.up[s] {
                    v.push("urca".to_string());
                } else {
                    v.push("coboara".to_string());
                    if self.pos[s] > 0 {
                        v.push("urca".to_string());
                    }
                }
                v
            }
            DPh::Roll => vec!["tira".into()],
            DPh::Act => {
                let mut v = vec!["nimic".to_string()];
                if let Some(i) = self.tile_at(self.pos[s]) {
                    if self.tiles[i] > 0 {
                        v.push("iau".into());
                    } else if !self.carry[s].is_empty() {
                        v.push("las".into());
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let s = self.side;
        match self.ph {
            DPh::Dir => {
                if m == "urca" {
                    self.up[s] = true;
                }
                self.air -= self.carry[s].len() as i32;
                self.air = self.air.max(0);
                self.ph = DPh::Roll;
            }
            DPh::Roll => {
                let r = die(&mut self.rng, 3) as i32 + 1 + die(&mut self.rng, 3) as i32 + 1;
                let mut steps = (r - self.carry[s].len() as i32).max(0);
                self.moved = steps;
                let dir = if self.up[s] { -1 } else { 1 };
                let mut p = self.pos[s];
                while steps > 0 {
                    let np = p + dir;
                    if np <= 0 {
                        p = 0;
                        break;
                    }
                    if np as usize > self.tiles.len() {
                        break;
                    }
                    p = np;
                    if self.occupied(p, s) {
                        continue;
                    }
                    steps -= 1;
                }
                // dacă s-a terminat pe loc ocupat, mergi mai departe pe un loc liber
                while self.occupied(p, s) && p > 0 {
                    let np = p + dir;
                    if np <= 0 || np as usize > self.tiles.len() {
                        break;
                    }
                    p = np;
                }
                self.pos[s] = p;
                if p == 0 {
                    self.back[s] = true;
                    self.ph = DPh::Dir;
                    self.turns += 1;
                    if !self.end_round_check() {
                        self.next_side();
                    }
                } else {
                    self.ph = DPh::Act;
                }
            }
            DPh::Act => {
                if let Some(i) = self.tile_at(self.pos[s]) {
                    if m == "iau" {
                        self.carry[s].push(self.tiles[i]);
                        self.tiles[i] = 0;
                    } else if m == "las" {
                        let v = self.carry[s].pop().unwrap();
                        self.tiles[i] = v;
                    }
                }
                self.turns += 1;
                if !self.end_round_check() {
                    self.next_side();
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 3 || self.turns >= 400 {
            Some(by_score(self.saved[0], self.saved[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let val = |q: usize| -> i32 {
            let c: i32 = self.carry[q].iter().sum();
            let danger = if self.back[q] {
                0
            } else {
                (self.pos[q] * 2 + self.carry[q].len() as i32) - self.air
            };
            let surv = if self.back[q] {
                100
            } else if danger > 0 {
                20
            } else {
                80
            };
            100 * self.saved[q] + c * surv
        };
        val(a) - val(1 - a)
    }
    fn show(&self) -> String {
        format!(
            "Scufundarea adâncă — runda {}/3 | aer {} | la mutare {}\nPoziții: J1 {} ({}), J2 {} ({}) | comori purtate: J1 {:?}, J2 {:?}\nPuncte salvate: J1 {}, J2 {}\nPlăci (adâncime 1…): {:?}\nMutări: coboara | urca | tira | iau | las | nimic\n",
            (self.round + 1).min(3),
            self.air,
            pname(pl(self.side)),
            self.pos[0],
            if self.up[0] { "urcă" } else { "coboară" },
            self.pos[1],
            if self.up[1] { "urcă" } else { "coboară" },
            self.carry[0],
            self.carry[1],
            self.saved[0],
            self.saved[1],
            self.tiles
        )
    }
}

// ---------------------------------------------------------------- cursa_cuburilor
const CUBE_FACES: [[(i32, i32); 6]; 4] = [
    [(1, 0), (1, 0), (2, 0), (2, 0), (0, 1), (0, 1)],
    [(2, 0), (2, 0), (3, 0), (3, 0), (0, 0), (1, 1)],
    [(0, 1), (0, 1), (0, 2), (1, 1), (1, 1), (0, 2)],
    [(3, 0), (0, 0), (4, 0), (0, 0), (3, 1), (2, 0)],
];
const CUBE_COST: [i32; 4] = [0, 4, 3, 6];
const CUBE_NAMES: [&str; 4] = ["simplu", "alergător", "negustor", "turbo"];
const TRACK: i32 = 28;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CPh {
    Draw,
    Decide,
    Buy,
}

#[derive(Clone)]
pub struct Cuburi {
    rng: u64,
    side: usize,
    ph: CPh,
    bag: [[u8; 4]; 2],
    disc: [[u8; 4]; 2],
    drawn: Vec<(u8, u8)>,
    pos: [i32; 2],
    coins: [i32; 2],
    gained: (i32, i32),
    bust: bool,
    done_last: bool,
    turns: u32,
}

impl Cuburi {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Cuburi {
            rng: seed_state(seed),
            side: 0,
            ph: CPh::Draw,
            bag: [[5, 0, 0, 0]; 2],
            disc: [[0; 4]; 2],
            drawn: Vec::new(),
            pos: [0; 2],
            coins: [0; 2],
            gained: (0, 0),
            bust: false,
            done_last: false,
            turns: 0,
        }
    }
    fn draw_one(&mut self) -> bool {
        let p = self.side;
        if self.bag[p].iter().all(|&x| x == 0) {
            self.bag[p] = self.disc[p];
            self.disc[p] = [0; 4];
        }
        if self.bag[p].iter().all(|&x| x == 0) {
            return false;
        }
        let w: Vec<u32> = self.bag[p].iter().map(|&x| x as u32).collect();
        let t = weighted(&mut self.rng, &w);
        self.bag[p][t] -= 1;
        let f = die(&mut self.rng, 6);
        self.drawn.push((t as u8, f));
        true
    }
    fn tally(&mut self) {
        let mut mv = 0;
        let mut c = 0;
        let mut zero = 0;
        for &(t, f) in &self.drawn {
            let (m, k) = CUBE_FACES[t as usize][f as usize];
            mv += m;
            c += k;
            if m == 0 && k == 0 {
                zero += 1;
            }
        }
        self.bust = zero >= 2;
        self.gained = if self.bust { (0, 0) } else { (mv, c) };
    }
    fn total(&self, p: usize) -> i32 {
        self.bag[p]
            .iter()
            .chain(self.disc[p].iter())
            .map(|&x| x as i32)
            .sum::<i32>()
            + if p == self.side {
                self.drawn.len() as i32
            } else {
                0
            }
    }
    fn quality(&self, p: usize) -> i32 {
        let mut q = 0;
        for t in 0..4 {
            let n = (self.bag[p][t] + self.disc[p][t]) as i32
                + if p == self.side {
                    self.drawn.iter().filter(|x| x.0 as usize == t).count() as i32
                } else {
                    0
                };
            let avg: i32 = CUBE_FACES[t]
                .iter()
                .map(|&(m, k)| 100 * m + 60 * k)
                .sum::<i32>()
                / 6;
            q += n * avg;
        }
        q
    }
}

impl Default for Cuburi {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cuburi {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            CPh::Draw => vec!["tira".into()],
            CPh::Decide => {
                let mut v = vec!["opresc".to_string()];
                if !self.bust
                    && self.drawn.len() < 6
                    && self.total(self.side) > self.drawn.len() as i32
                {
                    v.push("trag".into());
                }
                v
            }
            CPh::Buy => {
                let mut v = vec!["pas".to_string()];
                for t in 1..4 {
                    if self.coins[self.side] >= CUBE_COST[t] {
                        v.push(format!("cumpar {}", CUBE_NAMES[t]));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let p = self.side;
        match self.ph {
            CPh::Draw => {
                self.drawn.clear();
                for _ in 0..3 {
                    self.draw_one();
                }
                self.tally();
                self.ph = CPh::Decide;
            }
            CPh::Decide => {
                if m == "trag" {
                    self.draw_one();
                    self.tally();
                    if self.bust {
                        // ratat: nu se mai poate trage
                    }
                } else {
                    self.pos[p] += self.gained.0;
                    self.coins[p] += self.gained.1;
                    self.ph = CPh::Buy;
                }
            }
            CPh::Buy => {
                if let Some(n) = m.strip_prefix("cumpar ") {
                    let t = CUBE_NAMES.iter().position(|&x| x == n).unwrap();
                    self.coins[p] -= CUBE_COST[t];
                    self.disc[p][t] += 1;
                }
                for &(t, _) in &self.drawn {
                    self.disc[p][t as usize] += 1;
                }
                self.drawn.clear();
                self.gained = (0, 0);
                self.bust = false;
                self.ph = CPh::Draw;
                self.turns += 1;
                if p == 1 {
                    self.done_last = self.pos[0] >= TRACK || self.pos[1] >= TRACK;
                }
                self.side = 1 - p;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.done_last || self.turns >= 120 {
            return Some(by_score(self.pos[0], self.pos[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let mut e = 100 * (self.pos[a] - self.pos[o])
            + 20 * (self.coins[a] - self.coins[o])
            + (self.quality(a) - self.quality(o)) / 20;
        if self.ph == CPh::Decide && self.side == a {
            e += 100 * self.gained.0 + 20 * self.gained.1;
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Cursa cuburilor — tura {} | la mutare {} (pista {TRACK})\n",
            self.turns + 1,
            pname(pl(self.side))
        );
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: poziție {}, monede {}, săculeț {:?}, aruncate {:?} (tipuri: {})\n",
                p + 1,
                self.pos[p],
                self.coins[p],
                self.bag[p],
                self.disc[p],
                CUBE_NAMES.join("/")
            ));
        }
        if !self.drawn.is_empty() {
            let d: Vec<String> = self
                .drawn
                .iter()
                .map(|&(t, f)| format!("{}:{}", CUBE_NAMES[t as usize], f + 1))
                .collect();
            s.push_str(&format!(
                "Extrase: {} → mișcare {}, monede {}{}\n",
                d.join(" "),
                self.gained.0,
                self.gained.1,
                if self.bust { " (RATAT)" } else { "" }
            ));
        }
        s.push_str("Mutări: tira | trag | opresc | cumpar tip | pas\n");
        s
    }
}

// ---------------------------------------------------------------- fitilul_aprins
const BOMBS: [(&[Rule], i32); 4] = [
    (&[Rule::Exact(2), Rule::Min(5), Rule::Even, Rule::Odd], 3),
    (
        &[
            Rule::Max(2),
            Rule::Exact(6),
            Rule::Odd,
            Rule::Min(4),
            Rule::Even,
        ],
        4,
    ),
    (
        &[Rule::Exact(1), Rule::Exact(3), Rule::Min(5), Rule::Any],
        3,
    ),
    (
        &[
            Rule::Even,
            Rule::Even,
            Rule::Odd,
            Rule::Min(3),
            Rule::Max(4),
            Rule::Exact(5),
        ],
        5,
    ),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FPh {
    Roll,
    Play,
}

#[derive(Clone)]
pub struct Fitil {
    rng: u64,
    side: usize,
    ph: FPh,
    bomb: usize,
    filled: Vec<bool>,
    fuse: i32,
    hand: Vec<u8>,
    pts: [i32; 2],
    turns: u32,
    last: String,
}

impl Fitil {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Fitil {
            rng: seed_state(seed),
            side: 0,
            ph: FPh::Roll,
            bomb: 0,
            filled: vec![false; BOMBS[0].0.len()],
            fuse: 5,
            hand: vec![],
            pts: [0; 2],
            turns: 0,
            last: String::new(),
        }
    }
    fn next_bomb(&mut self) {
        self.bomb += 1;
        if self.bomb < BOMBS.len() {
            self.filled = vec![false; BOMBS[self.bomb].0.len()];
            self.fuse = 5;
        }
    }
}

impl Default for Fitil {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Fitil {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            FPh::Roll => vec!["tira".into()],
            FPh::Play => {
                let mut v = vec!["pas".to_string()];
                let rules = BOMBS[self.bomb].0;
                let mut vals = self.hand.clone();
                vals.sort_unstable();
                vals.dedup();
                for &val in &vals {
                    for (s, r) in rules.iter().enumerate() {
                        if !self.filled[s] && r.ok(val + 1) {
                            v.push(format!("des {} {}", val + 1, s + 1));
                        }
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            FPh::Roll => {
                self.hand = (0..5).map(|_| die(&mut self.rng, 6)).collect();
                self.ph = FPh::Play;
            }
            FPh::Play => {
                let p = self.side;
                if m == "pas" {
                    self.hand.clear();
                    self.fuse -= 1;
                    self.turns += 1;
                    self.ph = FPh::Roll;
                    if self.fuse <= 0 {
                        self.pts[p] -= 2;
                        self.last = "bombă explodată".into();
                        self.next_bomb();
                    }
                    self.side = 1 - p;
                    return;
                }
                let mut it = m.split(' ').skip(1);
                let v: u8 = it.next().unwrap().parse::<u8>().unwrap() - 1;
                let s: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let k = self.hand.iter().position(|&d| d == v).unwrap();
                self.hand.remove(k);
                self.filled[s] = true;
                if self.filled.iter().all(|&x| x) {
                    self.pts[p] += BOMBS[self.bomb].1;
                    self.last = "bombă dezamorsată".into();
                    self.hand.clear();
                    self.next_bomb();
                    self.turns += 1;
                    self.ph = FPh::Roll;
                    self.side = 1 - p;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.bomb >= BOMBS.len() || self.turns >= 120 {
            Some(by_score(self.pts[0], self.pts[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 1000 * (self.pts[a] - self.pts[1 - a]);
        if self.bomb < BOMBS.len() {
            let left = self.filled.iter().filter(|&&x| !x).count() as i32;
            let val = BOMBS[self.bomb].1 * 1000;
            // cine lasă exact un loc liber îl oferă adversarului
            if self.ph == FPh::Roll && left == 1 {
                e += if self.side == a { -val / 2 } else { val / 2 };
            }
            if self.ph == FPh::Roll && self.fuse == 1 {
                e += if self.side == a { -1000 } else { 1000 };
            }
        }
        e
    }
    fn show(&self) -> String {
        let b = BOMBS.get(self.bomb);
        let mut s = format!(
            "Fitilul aprins — bomba {}/{} | puncte J1 {}, J2 {} | la mutare {}\n",
            (self.bomb + 1).min(BOMBS.len()),
            BOMBS.len(),
            self.pts[0],
            self.pts[1],
            pname(pl(self.side))
        );
        if let Some((rules, val)) = b {
            s.push_str(&format!("Fitil: {} | valoare {}\n", self.fuse, val));
            for (i, r) in rules.iter().enumerate() {
                s.push_str(&format!(
                    "  fir {}: {:?} {}\n",
                    i + 1,
                    r,
                    if self.filled[i] { "TĂIAT" } else { "" }
                ));
            }
        }
        s.push_str(&format!(
            "Zaruri în mână: {}\n{}\nMutări: tira | des v fir | pas\n",
            dstr(&self.hand),
            self.last
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn scufundare() {
        check(Scufundare::with_seed, 1500);
        let mut g = Scufundare::with_seed(3);
        advance(&mut g, 30, |m| m.ph == DPh::Roll);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn cuburi() {
        check(Cuburi::with_seed, 1500);
        let mut g = Cuburi::with_seed(3);
        advance(&mut g, 30, |m| m.ph == CPh::Decide);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn fitil() {
        check(Fitil::with_seed, 1500);
        let mut g = Fitil::with_seed(3);
        advance(&mut g, 30, |m| m.ph == FPh::Play);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
}
