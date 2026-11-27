//! Jocuri pentru copii, partea a doua (lista C3, variante originale): Livada cu Cireșe, Gheața
//! Subțire, Avionul Hapsân, Livada și Corbul (cooperativ), Găinile pe Cerc, Clopoțelul Fructelor.
//! Șansa apare doar în mutări forțate; AI-ul decide din informația publică.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

// ───────────────────────── Livada cu Cireșe ─────────────────────────
// Fiecare tură: `gira` (se învârte, rezultatul prin `roata`, forțat) sau `fixez` (muți cireșele
// din găleată în coș). Roata: 1-4 cireșe în găleată; pasăre, câine, găleată vărsată = pierzi
// găleata (vărsată: și o cireașă din coș). Primul cu 10 cireșe în coș câștigă.
#[derive(Clone)]
pub struct Cirese {
    basket: [i32; 2],
    bucket: i32,
    pend: bool,
    side: usize,
    rng: Rng,
}

pub fn cirese(seed: u64) -> Cirese {
    Cirese {
        basket: [0; 2],
        bucket: 0,
        pend: false,
        side: 0,
        rng: Rng::new(seed),
    }
}

pub fn cirese_new() -> Cirese {
    cirese(seed_now())
}

impl Mini for Cirese {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend {
            return vec!["roata".into()];
        }
        let mut v = vec!["gira".to_string()];
        if self.bucket > 0 {
            v.push("fixez".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        if self.pend {
            self.pend = false;
            match self.rng.below(7) {
                k @ 0..=3 => self.bucket += k as i32 + 1,
                4 | 5 => {
                    self.bucket = 0;
                    self.side = 1 - self.side;
                }
                _ => {
                    self.bucket = 0;
                    self.basket[self.side] = (self.basket[self.side] - 1).max(0);
                    self.side = 1 - self.side;
                }
            }
            return;
        }
        if m == "gira" {
            self.pend = true;
        } else {
            self.basket[self.side] += self.bucket;
            self.bucket = 0;
            self.side = 1 - self.side;
        }
    }
    fn result(&self) -> Option<Outcome> {
        (0..2)
            .find(|&p| self.basket[p] >= 10)
            .map(|p| Outcome::Win(pl(p)))
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 10 * (self.basket[a] - self.basket[1 - a]);
        if self.side == a {
            // găleata ne aparține doar dacă o fixăm; riscul de a o pierde crește cu mărimea ei
            e += self.bucket * 7;
        }
        if self.pend {
            e += 15;
        }
        e
    }
    fn show(&self) -> String {
        format!(
            "Livada cu Cireșe — coș J1 {} J2 {} (țintă 10) | găleată {}\nLa mutare: {} — gira | fixez\n",
            self.basket[0],
            self.basket[1],
            self.bucket,
            pname(self.side())
        )
    }
}

// ───────────────────────── Gheața Subțire ─────────────────────────
// Grilă 3×4 de blocuri; `bat r c` lovește blocul (r, c), iar `izb` (forțat) hotărăște: riscul
// de cădere = 5% + 12% pentru fiecare vecin ortogonal deja scos + 2% pentru fiecare bloc scos.
// Cine face omulețul să cadă pierde.
#[derive(Clone)]
pub struct Gheata {
    gone: [[bool; 4]; 3],
    pend: Option<(usize, usize)>,
    side: usize,
    lost: Option<usize>,
    rng: Rng,
}

pub fn gheata(seed: u64) -> Gheata {
    Gheata {
        gone: [[false; 4]; 3],
        pend: None,
        side: 0,
        lost: None,
        rng: Rng::new(seed),
    }
}

pub fn gheata_new() -> Gheata {
    gheata(seed_now())
}

impl Gheata {
    fn risk(&self, r: usize, c: usize) -> i32 {
        let mut n = 0;
        for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if (0..3).contains(&nr) && (0..4).contains(&nc) && self.gone[nr as usize][nc as usize] {
                n += 1;
            }
        }
        let total = self.gone.iter().flatten().filter(|&&g| g).count() as i32;
        5 + 12 * n + 2 * total
    }
}

impl Mini for Gheata {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["izb".into()];
        }
        let mut v = Vec::new();
        for r in 0..3 {
            for c in 0..4 {
                if !self.gone[r][c] {
                    v.push(format!("bat {} {}", r + 1, c + 1));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        if let Some((r, c)) = self.pend.take() {
            if (self.rng.below(100) as i32) < self.risk(r, c).min(95) {
                self.lost = Some(self.side);
            } else {
                self.gone[r][c] = true;
                self.side = 1 - self.side;
            }
            return;
        }
        let p: Vec<usize> = m[4..]
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        self.pend = Some((p[0], p[1]));
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(l) = self.lost {
            return Some(Outcome::Win(pl(1 - l)));
        }
        if self.gone.iter().flatten().all(|&g| g) {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        match self.pend {
            Some((r, c)) => {
                let k = -self.risk(r, c);
                if pi(p) == self.side {
                    k
                } else {
                    -k
                }
            }
            None => 0,
        }
    }
    fn show(&self) -> String {
        let mut s = String::from("Gheața Subțire — # = bloc rămas, . = scos\n");
        for r in 0..3 {
            for c in 0..4 {
                s.push(if self.gone[r][c] { '.' } else { '#' });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!("La mutare: {} — bat r c\n", pname(self.side())));
        s
    }
}

// ───────────────────────── Avionul Hapsân ─────────────────────────
// Patru stații în cerc; J1 e la 0, J2 la 2. Avionul trece la stația următoare; jucătorul de acolo
// poate `lovesc` (clapeta; nu două ture la rând) cu 60% reușită (hotărâtă de `zbor`, forțat). Dacă
// avionul sosește și clapeta n-a reușit, pierzi o găină din 3. Câștigă cine mai are găini.
#[derive(Clone)]
pub struct Avion {
    pos: usize,
    hens: [i32; 2],
    cool: [bool; 2],
    flip: bool,
    asked: bool,
    rng: Rng,
}

pub fn avion(seed: u64) -> Avion {
    Avion {
        pos: 3,
        hens: [3; 2],
        cool: [false; 2],
        flip: false,
        asked: false,
        rng: Rng::new(seed),
    }
}

pub fn avion_new() -> Avion {
    avion_s(seed_now())
}

fn avion_s(seed: u64) -> Avion {
    avion(seed)
}

impl Avion {
    fn next(&self) -> usize {
        (self.pos + 1) % 4
    }
    fn owner(&self) -> Option<usize> {
        match self.next() {
            0 => Some(0),
            2 => Some(1),
            _ => None,
        }
    }
}

impl Mini for Avion {
    fn side(&self) -> Player {
        pl(self.owner().unwrap_or(0))
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.owner() {
            Some(o) if !self.asked => {
                let mut v = vec!["asteapt".to_string()];
                if !self.cool[o] {
                    v.push("lovesc".into());
                }
                v
            }
            _ => vec!["zbor".into()],
        }
    }
    fn step(&mut self, m: &str) {
        if m == "lovesc" {
            self.flip = true;
            self.asked = true;
            return;
        }
        if m == "asteapt" {
            self.asked = true;
            return;
        }
        // zbor
        let nxt = self.next();
        if let Some(o) = self.owner() {
            let saved = self.flip && self.rng.below(100) < 60;
            if !saved {
                self.hens[o] -= 1;
            }
            self.cool[o] = self.flip;
        }
        self.pos = nxt;
        self.flip = false;
        self.asked = false;
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.hens[p] <= 0 {
                return Some(Outcome::Win(pl(1 - p)));
            }
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 100 * (self.hens[a] - self.hens[1 - a]);
        if self.flip {
            e += if self.owner() == Some(a) { 40 } else { -40 };
        }
        e
    }
    fn show(&self) -> String {
        format!(
            "Avionul Hapsân — avion la stația {} | găini J1 {} J2 {}\nUrmătoarea stație: {}{}\nLa mutare: {} — lovesc | asteapt | zbor\n",
            self.pos + 1,
            self.hens[0],
            self.hens[1],
            self.next() + 1,
            match self.owner() {
                Some(o) => format!(" (a lui J{})", o + 1),
                None => " (goală)".into(),
            },
            pname(self.side())
        )
    }
}

// ───────────────────────── Livada și Corbul (cooperativ) ─────────────────────────
// 4 pomi × 4 fructe. Aruncarea (`zar`, forțată): 4 culori (un fruct de acea culoare), coș (două
// fructe la alegere: `cos c`, apoi iar `cos c`) sau corb (corbul avansează). Echipa câștigă dacă
// strânge toate fructele înainte ca corbul să facă 6 pași (altfel remiză).
#[derive(Clone)]
pub struct Livada {
    fruit: [i32; 4],
    raven: i32,
    face: Option<usize>,
    picks: u8,
    side: usize,
    rng: Rng,
}

pub fn livada(seed: u64) -> Livada {
    Livada {
        fruit: [4; 4],
        raven: 0,
        face: None,
        picks: 0,
        side: 0,
        rng: Rng::new(seed),
    }
}

pub fn livada_new() -> Livada {
    livada(seed_now())
}

impl Mini for Livada {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.picks > 0 {
            let v: Vec<String> = (0..4)
                .filter(|&c| self.fruit[c] > 0)
                .map(|c| format!("cos {}", c + 1))
                .collect();
            return if v.is_empty() {
                vec!["cos 1".into()]
            } else {
                v
            };
        }
        vec!["zar".into()]
    }
    fn step(&mut self, m: &str) {
        if self.picks > 0 {
            let c = m[4..].parse::<usize>().unwrap() - 1;
            if self.fruit[c] > 0 {
                self.fruit[c] -= 1;
            }
            self.picks -= 1;
            if self.picks == 0 || self.fruit.iter().all(|&f| f == 0) {
                self.picks = 0;
                self.side = 1 - self.side;
            }
            return;
        }
        let f = self.rng.below(6);
        match f {
            0..=3 => {
                if self.fruit[f] > 0 {
                    self.fruit[f] -= 1;
                }
                self.side = 1 - self.side;
            }
            4 => self.picks = 2,
            _ => {
                self.raven += 1;
                self.side = 1 - self.side;
            }
        }
        self.face = Some(f);
    }
    fn result(&self) -> Option<Outcome> {
        if self.fruit.iter().all(|&f| f == 0) {
            Some(Outcome::Win(Player::One))
        } else if self.raven >= 6 {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn eval(&self, _p: Player) -> i32 {
        // preferă să strângi din culoarea cu cele mai multe fructe (păstrezi variația)
        let left: i32 = self.fruit.iter().sum();
        let spread = self.fruit.iter().map(|&f| f * f).sum::<i32>();
        -left * 10 - self.raven * 20 - spread
    }
    fn show(&self) -> String {
        format!(
            "Livada și Corbul — fructe rămase pe culori: {} {} {} {} | corb {}/6\nUltima aruncare: {}\nLa mutare: {} — zar | cos c\n",
            self.fruit[0],
            self.fruit[1],
            self.fruit[2],
            self.fruit[3],
            self.raven,
            match self.face {
                None => "-".into(),
                Some(f @ 0..=3) => format!("culoarea {}", f + 1),
                Some(4) => "coș".into(),
                _ => "corb".into(),
            },
            pname(self.side())
        )
    }
}

// ───────────────────────── Găinile pe Cerc (memorie cu vulpi) ─────────────────────────
// 14 plăci: 6 perechi de găini + 2 vulpi. `intorc i` întoarce o placă (ce e dedesubt se arată
// prin `arat`, forțat). Două plăci egale = pereche pentru tine și mai întorci; neegale sau vulpe
// = tura trece (vulpea te costă și o pereche). Plăcile întoarse și neperechi rămân cunoscute.
#[derive(Clone)]
pub struct Gaini {
    tiles: Vec<u8>, // 0..5 găini, 6 vulpe
    taken: Vec<bool>,
    seen: Vec<bool>,
    first: Option<usize>,
    pend: Option<usize>,
    pairs: [i32; 2],
    side: usize,
    gone: usize,
}

pub fn gaini(seed: u64) -> Gaini {
    let mut rng = Rng::new(seed);
    let mut t: Vec<u8> = (0..6u8).chain(0..6u8).chain([6, 6]).collect();
    shuf(&mut t, &mut rng);
    Gaini {
        taken: vec![false; 14],
        seen: vec![false; 14],
        tiles: t,
        first: None,
        pend: None,
        pairs: [0; 2],
        side: 0,
        gone: 0,
    }
}

pub fn gaini_new() -> Gaini {
    gaini(seed_now())
}

impl Mini for Gaini {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["arat".into()];
        }
        (0..14)
            .filter(|&i| !self.taken[i] && Some(i) != self.first)
            .map(|i| format!("intorc {}", i + 1))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if let Some(i) = self.pend.take() {
            self.seen[i] = true;
            let t = self.tiles[i];
            if t == 6 {
                if self.pairs[self.side] > 0 {
                    self.pairs[self.side] -= 1;
                }
                self.first = None;
                self.side = 1 - self.side;
            } else if let Some(f) = self.first.take() {
                if self.tiles[f] == t {
                    self.taken[f] = true;
                    self.taken[i] = true;
                    self.pairs[self.side] += 1;
                    self.gone += 1;
                } else {
                    self.side = 1 - self.side;
                }
            } else {
                self.first = Some(i);
            }
            return;
        }
        self.pend = Some(m[7..].parse::<usize>().unwrap() - 1);
    }
    fn result(&self) -> Option<Outcome> {
        if self.gone >= 6 {
            Some(by_score(self.pairs[0], self.pairs[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 100 * (self.pairs[a] - self.pairs[1 - a]);
        if let Some(i) = self.pend {
            // doar ce se știe public despre placa aleasă
            let sign = if self.side == a { 1 } else { -1 };
            if self.seen[i] {
                let t = self.tiles[i];
                if let Some(f) = self.first {
                    if self.tiles[f] == t {
                        e += sign * 100;
                    } else {
                        e -= sign * 5;
                    }
                }
                if t == 6 {
                    e -= sign * 100;
                }
            }
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Găinile pe Cerc — perechi J1 {} J2 {}\n",
            self.pairs[0], self.pairs[1]
        );
        for i in 0..14 {
            let c = if self.taken[i] {
                " .".to_string()
            } else if Some(i) == self.first || self.pend == Some(i) && self.seen[i] {
                format!(" {}", self.tile_name(i))
            } else if self.seen[i] {
                format!(" ({})", self.tile_name(i))
            } else {
                " ?".to_string()
            };
            s.push_str(&format!("{}{} ", i + 1, c));
        }
        s.push_str(&format!(
            "\n(în paranteză: văzute înainte; ? necunoscute)\nLa mutare: {} — intorc i\n",
            pname(self.side())
        ));
        s
    }
}

impl Gaini {
    fn tile_name(&self, i: usize) -> String {
        match self.tiles[i] {
            6 => "vulpe".into(),
            k => format!("găina{}", k + 1),
        }
    }
}

// ───────────────────────── Clopoțelul Fructelor ─────────────────────────
// Fiecare are 20 de cărți cu fructe (4 feluri, 1-4 bucăți). `rasf` întoarce cartea de sus pe masa
// ta. Dacă un fel de fruct are exact 5 pe cele două cărți vizibile, orice jucător poate `suna`;
// corect: duelul `duel` (forțat; cine a sunat câștigă masa cu 65%, altfel o ia celălalt);
// greșit: dai o carte adversarului. `trec` lasă clopoțelul.
#[derive(Clone)]
pub struct Halli {
    hand: [Vec<(u8, u8)>; 2],
    table: [Vec<(u8, u8)>; 2],
    phase: u8, // 0 răsfoiește, 1 clopot pentru cel care a întors, 2 clopot pentru celălalt, 3 duel
    flipper: usize,
    ringer: usize,
    plies: u32,
    rng: Rng,
}

pub fn halli(seed: u64) -> Halli {
    let mut rng = Rng::new(seed);
    let mut deck: Vec<(u8, u8)> = Vec::new();
    for f in 0..4u8 {
        for n in [1, 1, 1, 2, 2, 3, 3, 4, 4, 5] {
            deck.push((f, n));
        }
    }
    shuf(&mut deck, &mut rng);
    let b = deck.split_off(20);
    Halli {
        hand: [deck, b],
        table: [Vec::new(), Vec::new()],
        phase: 0,
        flipper: 0,
        ringer: 0,
        plies: 0,
        rng,
    }
}

pub fn halli_new() -> Halli {
    halli(seed_now())
}

impl Halli {
    fn five(&self) -> bool {
        let mut sum = [0u8; 4];
        for t in &self.table {
            if let Some(&(f, n)) = t.last() {
                sum[f as usize] += n;
            }
        }
        sum.contains(&5)
    }
    fn to_play(&self) -> usize {
        match self.phase {
            0 => self.flipper,
            1 => self.flipper,
            2 => 1 - self.flipper,
            _ => self.ringer,
        }
    }
}

impl Mini for Halli {
    fn side(&self) -> Player {
        pl(self.to_play())
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.phase {
            0 => vec!["rasf".into()],
            1 | 2 => vec!["suna".into(), "trec".into()],
            _ => vec!["duel".into()],
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        match self.phase {
            0 => {
                let c = self.hand[self.flipper].remove(0);
                self.table[self.flipper].push(c);
                self.phase = 1;
            }
            1 | 2 => {
                let who = self.to_play();
                if m == "suna" {
                    if self.five() {
                        self.ringer = who;
                        self.phase = 3;
                    } else if let Some(c) = self.hand[who].pop() {
                        self.hand[1 - who].insert(0, c);
                        self.next_phase();
                    } else {
                        self.next_phase();
                    }
                } else {
                    self.next_phase();
                }
            }
            _ => {
                let win = if self.rng.below(100) < 65 {
                    self.ringer
                } else {
                    1 - self.ringer
                };
                let mut t = std::mem::take(&mut self.table[0]);
                t.append(&mut self.table[1]);
                shuf(&mut t, &mut self.rng);
                for c in t {
                    self.hand[win].push(c);
                }
                self.phase = 0;
                self.flipper = 1 - self.flipper;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.phase == 0 {
            let f = self.flipper;
            if self.hand[f].is_empty() {
                return Some(Outcome::Win(pl(1 - f)));
            }
        }
        if self.plies > 600 {
            return Some(by_score(
                self.hand[0].len() as i32,
                self.hand[1].len() as i32,
            ));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 10 * (self.hand[a].len() as i32 - self.hand[1 - a].len() as i32);
        if self.phase == 3 {
            let t = (self.table[0].len() + self.table[1].len()) as i32;
            e += if self.ringer == a { t * 3 } else { -t * 3 };
        }
        if (self.phase == 1 || self.phase == 2) && self.five() {
            let t = (self.table[0].len() + self.table[1].len()) as i32;
            e += if self.to_play() == a { t * 5 } else { -t * 5 };
        }
        e
    }
    fn show(&self) -> String {
        let top = |p: usize| match self.table[p].last() {
            Some(&(f, n)) => format!("{n}×fruct{}", f + 1),
            None => "-".into(),
        };
        format!(
            "Clopoțelul Fructelor — cărți J1 {} J2 {} | pe masă: J1 {}, J2 {}{}\nLa mutare: {} — rasf | suna | trec | duel\n",
            self.hand[0].len(),
            self.hand[1].len(),
            top(0),
            top(1),
            if self.five() { " (sunt 5!)" } else { "" },
            pname(self.side())
        )
    }
}

impl Halli {
    fn next_phase(&mut self) {
        if self.phase == 1 {
            self.phase = 2;
        } else {
            self.phase = 0;
            self.flipper = 1 - self.flipper;
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        check(cirese, 4000);
        check(gheata, 200);
        check(avion, 4000);
        check(livada, 400);
        check(gaini, 1000);
        check(halli, 2000);
    }
}
