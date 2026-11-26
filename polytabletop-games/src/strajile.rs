//! Străjile Codrului — joc cooperativ de apărare cu cărți de putere (variantă originală, partener AI).
//!
//! Voi doi sunteți duhurile unei văi (Duhul Rădăcinilor și Duhul Furtunii) care alungă
//! coloniștii veniți de peste mare. Mecanica „cooperativ de apărare cu cărți de putere,
//! prezență pe hartă, energie, pachet de invadatori în trei trepte (explorare → construire →
//! devastare) și frică” este generică; harta, duhurile, cărțile și textele sunt ale noastre.
//!
//! * Harta: 8 ținuturi (M munte, P pădure, B mlaștină, C câmpie; 4 de coastă, câte una din fiecare fel), un inel cu 3
//!   scurtături. Pe ținuturi stau invadatori — pionieri (forță/viață 1), colonii (2), fortărețe (3) —,
//!   localnici (apără valea), blestem (blight) și prezența duhurilor.
//! * Tura unui duh: (1) creștere, o singură alegere: `creste N` (prezență într-un ținut vecin
//!   cu prezența ta, +1 energie), `aduna` (+2 energie) sau `reia` (cărțile jucate revin în mână, +1
//!   energie); (2) cel mult două cărți de putere: `joc spini 3` (plătești costul; ținutul trebuie să
//!   fie în raza cărții față de prezența ta); (3) `gata`.
//! * După ambele duhuri vine runda invadatorilor, în trei mutări forțate: `devastare` (ținuturile de
//!   tipul cărții din slotul „devastare”), `constructie` (cărții din slotul „construire”),
//!   `explorare` (cartea din slotul „explorare”; pe țărm sau lângă invadatori apar 3 pionieri (4 în a doua jumătate a pachetului));
//!   cărțile avansează un slot. Cele 12 cărți ale pachetului hotărăsc durata (14 runde).
//! * Construire: pionierii devin colonii; o colonie existentă aduce un pionier nou și (în teroare 1) o fortăreață.
//! * Devastare: forța invadatorilor minus apărarea ținutului; fiecare 2 puncte ucid un localnic; restul
//!   ≥ 1 aduce un blestem. Localnicii rămași lovesc înapoi (2 viață de invadatori fiecare).
//! * Frică: fiecare colonie ucisă dă 1, fortăreață 2 (+ efectele cărților). Frică ≥ 14: „teroare 2” —
//!   invadatorii nu mai construiesc fortărețe.
//! * Efectele sunt în ordinea: lovitură (omoară întâi cei mai tari care încap), apărare, împingere
//!   (spre vecinul cu cei mai mulți localnici), adunare de localnici, frică, vindecare de blestem.
//!
//! Rezultat: echipa câștigă dacă frica ajunge la 28 sau dacă, după ultima carte de invadatori,
//! valea e curată de invadatori (J1 „câștigă”); 10 blesteme sau invadatori rămași la sfârșit =
//! înfrângere = REMIZĂ (documentat: nu există adversar). Sfârșitul e o mutare forțată (`sfârșit`).
#![allow(clippy::needless_range_loop)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const L: usize = 8;
pub const FEAR_WIN: i32 = 28;
pub const BLIGHT_LOSE: i32 = 10;
const TERROR2: i32 = 14;
const TERRAIN: [u8; L] = [3, 2, 0, 1, 3, 1, 0, 2];
const TERRAIN_LET: [char; 4] = ['M', 'P', 'B', 'C'];
const COAST: [bool; L] = [true, true, false, true, false, false, true, false];
pub const LAND_NAMES: [&str; L] = [
    "Țărm",
    "Stufăriș",
    "Creastă",
    "Dumbravă",
    "Luncă",
    "Codru",
    "Piscuri",
    "Mlaștina Neagră",
];
const EDGES: [(usize, usize); 3] = [(1, 5), (2, 6), (3, 7)];
const HP: [u32; 3] = [1, 2, 3];

#[derive(Clone, Copy, Debug)]
pub struct Power {
    pub id: &'static str,
    pub name: &'static str,
    pub cost: i32,
    pub range: u8,
    pub dmg: u32,
    pub defend: u8,
    pub push: u8,
    pub gather: u8,
    pub fear: i32,
    pub heal: u8,
}

#[allow(clippy::too_many_arguments)]
const fn pw(
    id: &'static str,
    name: &'static str,
    cost: i32,
    range: u8,
    dmg: u32,
    defend: u8,
    push: u8,
    gather: u8,
    fear: i32,
    heal: u8,
) -> Power {
    Power {
        id,
        name,
        cost,
        range,
        dmg,
        defend,
        push,
        gather,
        fear,
        heal,
    }
}

/// Cărțile 0..6 aparțin Duhului Rădăcinilor (J1), 6..12 Duhului Furtunii (J2).
pub const POWERS: [Power; 12] = [
    pw("radacini", "Rădăcini Încurcate", 1, 1, 0, 2, 0, 0, 0, 0),
    pw("spini", "Perete de Spini", 2, 1, 2, 0, 0, 0, 0, 0),
    pw("seva", "Seva Vindecătoare", 1, 0, 0, 0, 0, 0, 0, 1),
    pw("ceata", "Ceață Deasă", 1, 2, 0, 1, 1, 0, 0, 0),
    pw("strigat", "Strigătul Codrului", 2, 2, 0, 0, 0, 1, 1, 0),
    pw("furtuna", "Furtună de Ghinde", 3, 1, 3, 0, 0, 0, 1, 0),
    pw("fulger", "Fulger Gol", 2, 2, 2, 0, 0, 0, 0, 0),
    pw("vant", "Vânt Răzvrătit", 1, 2, 0, 0, 2, 0, 0, 0),
    pw("ecou", "Ecoul Văii", 1, 2, 0, 0, 0, 2, 0, 0),
    pw("grindina", "Grindină Mare", 3, 2, 3, 0, 0, 0, 0, 0),
    pw("spaima", "Spaimă Mută", 2, 3, 0, 0, 0, 0, 2, 0),
    pw("ploaie", "Ploaie Binecuvântată", 2, 1, 0, 1, 0, 0, 0, 1),
];

pub const SPIRIT_NAMES: [&str; 2] = ["Duhul Rădăcinilor", "Duhul Furtunii"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Grow,
    Power,
    Ravage,
    Build,
    Explore,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Grow(u8),
    Gold,
    Reclaim,
    Play(u8, u8),
    Done,
    Ravage,
    Build,
    Explore,
    Finish,
}

#[derive(Clone)]
pub struct Strajile {
    inv: [[u8; 3]; L],
    locals: [u8; L],
    blight: [u8; L],
    defend: [u8; L],
    presence: [[bool; L]; 2],
    energy: [i32; 2],
    hand: [Vec<u8>; 2],
    discard: [Vec<u8>; 2],
    plays: u8,
    fear: i32,
    deck: Vec<u8>,
    explore: Option<u8>,
    build: Option<u8>,
    ravage: Option<u8>,
    step: Step,
    side: Player,
    adj: Vec<Vec<usize>>,
    dist: [[u8; L]; L],
    pending: Option<bool>,
    over: bool,
    won: bool,
    round: u32,
    last: String,
}

impl Default for Strajile {
    fn default() -> Self {
        Self::new()
    }
}

impl Strajile {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..12).map(|i| (i % 4) as u8).collect();
        shuffle(&mut deck, &mut rng);
        let mut adj = vec![Vec::new(); L];
        for i in 0..L {
            let j = (i + 1) % L;
            adj[i].push(j);
            adj[j].push(i);
        }
        for (a, b) in EDGES {
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut dist = [[99u8; L]; L];
        for s in 0..L {
            dist[s][s] = 0;
            let mut q = std::collections::VecDeque::from([s]);
            while let Some(x) = q.pop_front() {
                for &y in &adj[x] {
                    if dist[s][y] == 99 {
                        dist[s][y] = dist[s][x] + 1;
                        q.push_back(y);
                    }
                }
            }
        }
        let mut presence = [[false; L]; 2];
        presence[0][5] = true;
        presence[1][3] = true;
        let mut g = Strajile {
            inv: [[0; 3]; L],
            locals: [0, 1, 2, 1, 1, 2, 1, 1],
            blight: [0; L],
            defend: [0; L],
            presence,
            energy: [2, 2],
            hand: [(0..6).collect(), (6..12).collect()],
            discard: [Vec::new(), Vec::new()],
            plays: 0,
            fear: 0,
            deck,
            explore: None,
            build: None,
            ravage: None,
            step: Step::Grow,
            side: Player::One,
            adj,
            dist,
            pending: None,
            over: false,
            won: false,
            round: 1,
            last: String::new(),
        };
        let first = g.deck.pop().unwrap();
        g.explore_terrain(first);
        g.build = Some(first);
        g.explore = g.deck.pop();
        g
    }

    pub fn fear(&self) -> i32 {
        self.fear
    }

    pub fn blight_total(&self) -> i32 {
        self.blight.iter().map(|x| i32::from(*x)).sum()
    }

    pub fn invader_count(&self) -> u32 {
        self.inv
            .iter()
            .map(|a| a.iter().map(|x| u32::from(*x)).sum::<u32>())
            .sum()
    }

    pub fn is_won(&self) -> bool {
        self.over && self.won
    }

    /// Venit de energie la începutul turei unui duh: 1 + un sfert din prezență.
    fn income(&self, p: usize) -> i32 {
        1 + self.presence[p].iter().filter(|x| **x).count() as i32 / 4
    }

    fn terror2(&self) -> bool {
        self.fear >= TERROR2
    }

    fn explore_terrain(&mut self, t: u8) {
        let had: Vec<bool> = (0..L).map(|l| self.inv[l].iter().any(|x| *x > 0)).collect();
        for l in 0..L {
            if TERRAIN[l] != t {
                continue;
            }
            let esc = u8::from(self.deck.len() <= 5);
            if COAST[l] || self.adj[l].iter().any(|&y| had[y]) {
                self.inv[l][0] += 3 + esc;
            }
        }
    }

    fn near(&self, p: usize, land: usize, range: u8) -> bool {
        (0..L).any(|l| self.presence[p][l] && self.dist[l][land] <= range)
    }

    /// Ucide invadatori cu un buget de lovitură; întoarce frica obținută.
    fn kill_invaders(&mut self, l: usize, mut budget: u32, with_fear: bool) -> i32 {
        let mut fear = 0;
        'outer: loop {
            for tier in (0..3).rev() {
                if self.inv[l][tier] > 0 && HP[tier] <= budget {
                    self.inv[l][tier] -= 1;
                    budget -= HP[tier];
                    if with_fear {
                        fear += tier as i32;
                    }
                    continue 'outer;
                }
            }
            break;
        }
        fear
    }

    /// (forța efectivă, localnici uciși, blestem adăugat) dacă `l` ar fi devastat acum.
    fn ravage_calc(&self, l: usize) -> (u32, u8, u8) {
        let raw = u32::from(self.inv[l][0])
            + 2 * u32::from(self.inv[l][1])
            + 3 * u32::from(self.inv[l][2]);
        let eff = raw.saturating_sub(u32::from(self.defend[l]));
        let kl = (eff / 2).min(u32::from(self.locals[l]));
        let rem = eff - 2 * kl;
        (eff, kl as u8, u8::from(rem >= 1))
    }

    fn set_end(&mut self, won: bool) {
        if self.pending.is_none() {
            self.pending = Some(won);
        }
    }

    fn apply_power(&mut self, p: usize, card: u8, land: usize) {
        let pw = POWERS[card as usize];
        if pw.dmg > 0 {
            let f = self.kill_invaders(land, pw.dmg, true);
            self.fear += f;
        }
        self.defend[land] += pw.defend;
        for _ in 0..pw.push {
            let tier = (0..3).rev().find(|&t| self.inv[land][t] > 0);
            let Some(t) = tier else { break };
            let dest = self.adj[land]
                .iter()
                .copied()
                .max_by_key(|&y| (self.locals[y], std::cmp::Reverse(y)));
            if let Some(d) = dest {
                self.inv[land][t] -= 1;
                self.inv[d][t] += 1;
            }
        }
        for _ in 0..pw.gather {
            let src = self.adj[land]
                .iter()
                .copied()
                .filter(|&y| self.locals[y] > 0)
                .max_by_key(|&y| (self.locals[y], std::cmp::Reverse(y)));
            if let Some(y) = src {
                self.locals[y] -= 1;
                self.locals[land] += 1;
            }
        }
        self.fear += pw.fear;
        for _ in 0..pw.heal {
            if self.blight[land] > 0 {
                self.blight[land] -= 1;
            }
        }
        if self.fear >= FEAR_WIN {
            self.set_end(true);
        }
        let _ = p;
    }

    fn value(&self) -> i32 {
        if let Some(w) = self.pending {
            return if w { 100_000 } else { -100_000 };
        }
        let mut s = self.fear * 7;
        for l in 0..L {
            s -= 8 * i32::from(self.inv[l][0])
                + 16 * i32::from(self.inv[l][1])
                + 26 * i32::from(self.inv[l][2]);
            s += 9 * i32::from(self.locals[l]);
            s -= 35 * i32::from(self.blight[l]);
            if let Some(t) = self.ravage {
                if TERRAIN[l] == t && self.inv[l].iter().any(|x| *x > 0) {
                    let (_, kl, ba) = self.ravage_calc(l);
                    s -= 40 * i32::from(ba) + 10 * i32::from(kl);
                }
            }
            if let Some(t) = self.build {
                if TERRAIN[l] == t && self.inv[l].iter().any(|x| *x > 0) {
                    s -= 12;
                }
            }
        }
        for p in 0..2 {
            s += 2 * self.energy[p].min(8);
            s += 6 * self.presence[p].iter().filter(|x| **x).count() as i32;
            s += 6 * self.hand[p].len() as i32;
        }
        s
    }

    pub fn total_cards(&self) -> usize {
        (0..2)
            .map(|p| self.hand[p].len() + self.discard[p].len())
            .sum()
    }
}

impl Strajile {
    fn look(&self, root: Player, depth: u32) -> i32 {
        if self.over || self.pending.is_some() {
            return self.value();
        }
        match self.step {
            Step::Ravage | Step::Build => {
                let mut c = self.clone();
                let m = if self.step == Step::Ravage {
                    Move::Ravage
                } else {
                    Move::Build
                };
                c.apply(&m);
                return c.look(root, depth);
            }
            Step::Explore => {
                let mut c = self.clone();
                c.apply(&Move::Explore);
                return c.value();
            }
            _ => {}
        }
        if depth == 0 {
            return self.value();
        }
        let mut kids: Vec<(i32, Strajile)> = Game::legal_moves(self)
            .iter()
            .map(|m| {
                let mut c = self.clone();
                c.apply(m);
                (c.value(), c)
            })
            .collect();
        kids.sort_by_key(|k| std::cmp::Reverse(k.0));
        kids.truncate(if self.side == root { 3 } else { 1 });
        kids.iter()
            .map(|(_, c)| c.look(root, depth - 1))
            .max()
            .unwrap_or_else(|| self.value())
    }
}

impl Game for Strajile {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Finish];
        }
        let p = self.side.idx();
        match self.step {
            Step::Ravage => vec![Move::Ravage],
            Step::Build => vec![Move::Build],
            Step::Explore => vec![Move::Explore],
            Step::Grow => {
                let mut v = Vec::new();
                for l in 0..L {
                    if !self.presence[p][l] && self.near(p, l, 1) {
                        v.push(Move::Grow(l as u8));
                    }
                }
                v.push(Move::Gold);
                if !self.discard[p].is_empty() {
                    v.push(Move::Reclaim);
                }
                v
            }
            Step::Power => {
                let mut v = Vec::new();
                if self.plays < 2 {
                    for &c in &self.hand[p] {
                        let pw = POWERS[c as usize];
                        if pw.cost > self.energy[p] {
                            continue;
                        }
                        for l in 0..L {
                            if self.near(p, l, pw.range) {
                                v.push(Move::Play(c, l as u8));
                            }
                        }
                    }
                }
                v.push(Move::Done);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let who = format!("J{} ({})", p + 1, SPIRIT_NAMES[p]);
        match *m {
            Move::Grow(l) => {
                self.presence[p][l as usize] = true;
                self.energy[p] += 1;
                self.last = format!("{who} își extinde prezența în {}.", LAND_NAMES[l as usize]);
                self.step = Step::Power;
                self.plays = 0;
            }
            Move::Gold => {
                self.energy[p] += 2;
                self.last = format!("{who} adună energie (+2).");
                self.step = Step::Power;
                self.plays = 0;
            }
            Move::Reclaim => {
                let d = std::mem::take(&mut self.discard[p]);
                self.hand[p].extend(d);
                self.hand[p].sort_unstable();
                self.energy[p] += 1;
                self.last = format!("{who} reia cărțile jucate.");
                self.step = Step::Power;
                self.plays = 0;
            }
            Move::Play(c, l) => {
                let pw = POWERS[c as usize];
                self.energy[p] -= pw.cost;
                if let Some(i) = self.hand[p].iter().position(|x| *x == c) {
                    self.hand[p].remove(i);
                }
                self.discard[p].push(c);
                self.plays += 1;
                self.last = format!("{who} joacă «{}» în {}.", pw.name, LAND_NAMES[l as usize]);
                self.apply_power(p, c, l as usize);
            }
            Move::Done => {
                if p == 0 {
                    self.side = Player::Two;
                    self.step = Step::Grow;
                    self.energy[1] += self.income(1);
                } else {
                    self.step = Step::Ravage;
                }
                self.plays = 0;
            }
            Move::Ravage => {
                self.last = String::from("Devastare:");
                if let Some(t) = self.ravage {
                    for l in 0..L {
                        if TERRAIN[l] != t || self.inv[l].iter().all(|x| *x == 0) {
                            continue;
                        }
                        let (_, kl, ba) = self.ravage_calc(l);
                        self.locals[l] -= kl;
                        self.blight[l] += ba;
                        let budget = 2 * u32::from(self.locals[l]);
                        self.kill_invaders(l, budget, false);
                        self.last.push_str(&format!(
                            " {} (−{kl} localnici, +{ba} blestem)",
                            LAND_NAMES[l]
                        ));
                    }
                }
                self.defend = [0; L];
                self.step = Step::Build;
                if self.blight_total() >= BLIGHT_LOSE {
                    self.set_end(false);
                }
            }
            Move::Build => {
                if let Some(t) = self.build {
                    for l in 0..L {
                        if TERRAIN[l] != t {
                            continue;
                        }
                        if self.inv[l][1] > 0 {
                            if !self.terror2() {
                                self.inv[l][2] += 1;
                            }
                            self.inv[l][0] += 1;
                        } else if self.inv[l][0] > 0 {
                            self.inv[l][1] += 1;
                        }
                    }
                }
                self.last = "Construire încheiată.".into();
                self.step = Step::Explore;
            }
            Move::Explore => {
                if let Some(t) = self.explore {
                    self.explore_terrain(t);
                }
                self.ravage = self.build;
                self.build = self.explore;
                self.explore = self.deck.pop();
                self.last = "Explorare încheiată.".into();
                self.round += 1;
                self.side = Player::One;
                self.step = Step::Grow;
                self.energy[0] += self.income(0);
                if self.explore.is_none() && self.build.is_none() && self.ravage.is_none() {
                    let clean = self.invader_count() == 0;
                    self.set_end(clean);
                }
            }
            Move::Finish => {
                self.over = true;
                self.won = self.pending == Some(true);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            None
        } else if self.won {
            Some(Outcome::Win(Player::One))
        } else {
            Some(Outcome::Draw)
        }
    }

    /// Cooperativ și deterministic în afara cărții de invadatori nou trase: căutăm în lățime
    /// limitată restul rundei (tura mea, tura partenerului, apoi devastare / construire /
    /// explorare cu cărțile publice din sloturi) și evaluăm cu `value`. Nu se citește ordinea
    /// pachetului de invadatori (cartea nou trasă nu intră în `value`).
    fn evaluate(&self, p: Player) -> i32 {
        self.look(p, 9)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let slot = |s: Option<u8>| match s {
            Some(t) => TERRAIN_LET[t as usize].to_string(),
            None => "—".into(),
        };
        let mut s = format!(
            "Străjile Codrului — runda {} | frică {}/{} (teroare {}) | blesteme {}/{} | cărți invadatori rămase {}\n",
            self.round,
            self.fear,
            FEAR_WIN,
            if self.terror2() { 2 } else { 1 },
            self.blight_total(),
            BLIGHT_LOSE,
            self.deck.len()
        );
        s.push_str(&format!(
            "Invadatori: explorare {} → construire {} → devastare {}\n",
            slot(self.explore),
            slot(self.build),
            slot(self.ravage)
        ));
        for l in 0..L {
            let mut line = format!(
                "{} {:<16}[{}{}] ",
                l + 1,
                LAND_NAMES[l],
                TERRAIN_LET[TERRAIN[l] as usize],
                if COAST[l] { "~" } else { " " }
            );
            line.push_str(&format!(
                "pionieri {} colonii {} fortărețe {} | localnici {} blestem {}",
                self.inv[l][0], self.inv[l][1], self.inv[l][2], self.locals[l], self.blight[l]
            ));
            if self.defend[l] > 0 {
                line.push_str(&format!(" apărare {}", self.defend[l]));
            }
            for p in 0..2 {
                if self.presence[p][l] {
                    line.push_str(&format!(" <J{}>", p + 1));
                }
            }
            let nb: Vec<String> = self.adj[l].iter().map(|y| (y + 1).to_string()).collect();
            line.push_str(&format!("  vecini {}\n", nb.join(",")));
            s.push_str(&line);
        }
        for p in 0..2 {
            let h: Vec<String> = self.hand[p]
                .iter()
                .map(|&c| {
                    let pw = POWERS[c as usize];
                    format!("{} ({}e, raza {})", pw.id, pw.cost, pw.range)
                })
                .collect();
            s.push_str(&format!(
                "J{} {} — energie {} — mână: {}{}\n",
                p + 1,
                SPIRIT_NAMES[p],
                self.energy[p],
                h.join(", "),
                if self.discard[p].is_empty() {
                    String::new()
                } else {
                    format!(" | jucate: {}", self.discard[p].len())
                }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        if self.over {
            if self.won {
                s.push_str("VICTORIE: valea e salvată.\n");
            } else {
                s.push_str("ÎNFRÂNGERE (remiză): valea a căzut.\n");
            }
        } else if self.pending.is_some() {
            s.push_str("Jocul se încheie: sfârșit.\n");
        } else {
            match self.step {
                Step::Grow => s.push_str(&format!(
                    "La rând J{}: creștere — creste N, aduna sau reia.\n",
                    self.side.idx() + 1
                )),
                Step::Power => s.push_str(&format!(
                    "La rând J{}: cărți de putere ({} din 2 jucate) — joc <carte> <ținut> sau gata.\n",
                    self.side.idx() + 1,
                    self.plays
                )),
                Step::Ravage => s.push_str("Urmează devastarea: devastare.\n"),
                Step::Build => s.push_str("Urmează construirea: constructie.\n"),
                Step::Explore => s.push_str("Urmează explorarea: explorare.\n"),
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Grow(l) => format!("creste {}", l + 1),
            Move::Gold => "aduna".into(),
            Move::Reclaim => "reia".into(),
            Move::Play(c, l) => format!("joc {} {}", POWERS[*c as usize].id, l + 1),
            Move::Done => "gata".into(),
            Move::Ravage => "devastare".into(),
            Move::Build => "constructie".into(),
            Move::Explore => "explorare".into(),
            Move::Finish => "sfârșit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ai_game(seed: u64) -> Strajile {
        let mut g = Strajile::with_seed(seed);
        let mut rng = Rng::new(seed ^ 9);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            assert_eq!(g.total_cards(), 12);
            n += 1;
            assert!(n < 2000);
        }
        g
    }

    fn clean(g: &mut Strajile) {
        g.inv = [[0; 3]; L];
        g.locals = [0; L];
        g.blight = [0; L];
        g.defend = [0; L];
    }

    #[test]
    fn setup() {
        let g = Strajile::with_seed(1);
        assert_eq!(g.deck.len(), 12 - 2);
        assert!(g.invader_count() >= 1);
        assert_eq!(g.total_cards(), 12);
        assert!(g.build.is_some() && g.explore.is_some() && g.ravage.is_none());
    }

    #[test]
    fn growth_options() {
        let g = Strajile::with_seed(2);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Gold));
        assert!(!mv.contains(&Move::Reclaim));
        // prezență la 5: vecini 4, 6 și 1 (index 0-based 4,6,1)
        assert!(mv.contains(&Move::Grow(4)));
        assert!(!mv.contains(&Move::Grow(5)));
        assert!(!mv.contains(&Move::Grow(2)));
    }

    #[test]
    fn power_range_and_cost() {
        let mut g = Strajile::with_seed(3);
        g.apply(&Move::Gold);
        assert_eq!(g.energy[0], 4);
        let mv = g.legal_moves();
        // seva (id 2, raza 0) doar pe ținutul 5 (indice 5)
        assert!(mv.contains(&Move::Play(2, 5)));
        assert!(!mv.contains(&Move::Play(2, 4)));
        g.energy[0] = 0;
        assert!(g.legal_moves().iter().all(|m| !matches!(m, Move::Play(..))));
    }

    #[test]
    fn damage_kills_biggest_first_and_gives_fear() {
        let mut g = Strajile::with_seed(4);
        clean(&mut g);
        g.inv[4] = [2, 1, 0];
        let f = g.kill_invaders(4, 2, true);
        assert_eq!(g.inv[4], [2, 0, 0]);
        assert_eq!(f, 1);
        let f = g.kill_invaders(4, 5, true);
        assert_eq!(g.inv[4], [0, 0, 0]);
        assert_eq!(f, 0);
    }

    #[test]
    fn ravage_with_locals_and_defense() {
        let mut g = Strajile::with_seed(5);
        clean(&mut g);
        g.ravage = Some(TERRAIN[4]);
        g.inv[4] = [0, 2, 0]; // forță 4
        g.locals[4] = 1;
        g.step = Step::Ravage;
        g.side = Player::Two;
        let mut a = g.clone();
        a.apply(&Move::Ravage);
        assert_eq!(a.locals[4], 0);
        assert_eq!(a.blight[4], 1); // 4 − 2 = 2 → blestem
        let mut b = g.clone();
        b.defend[4] = 4;
        b.apply(&Move::Ravage);
        assert_eq!(b.locals[4], 1);
        assert_eq!(b.blight[4], 0);
        assert_eq!(b.defend, [0; L]);
        // localnicul rămas lovește: ucide o colonie (viață 2)
        assert_eq!(b.inv[4][1], 1);
    }

    #[test]
    fn build_and_terror() {
        let mut g = Strajile::with_seed(6);
        clean(&mut g);
        g.build = Some(TERRAIN[4]);
        g.inv[4] = [1, 0, 0];
        g.step = Step::Build;
        g.side = Player::Two;
        g.apply(&Move::Build);
        assert_eq!(g.inv[4], [1, 1, 0]);
        g.step = Step::Build;
        g.apply(&Move::Build);
        // colonia aduce un pionier și (teroare 1) o fortăreață
        assert_eq!(g.inv[4], [2, 1, 1]);
        g.fear = 14;
        g.step = Step::Build;
        g.apply(&Move::Build);
        assert_eq!(g.inv[4], [3, 1, 1]);
    }

    #[test]
    fn explore_spreads_from_coast_or_neighbors() {
        let mut g = Strajile::with_seed(7);
        clean(&mut g);
        g.inv[0][0] = 1; // Țărm
        g.explore_terrain(0); // munte: Creastă(2, interior), Piscuri(6, coastă)
        assert_eq!(g.inv[2][0], 0);
        assert_eq!(g.inv[6][0], 3);
        g.explore_terrain(0); // acum Creastă e vecină cu Piscuri
        assert_eq!(g.inv[2][0], 3);
    }

    #[test]
    fn fear_victory_through_finish() {
        let mut g = Strajile::with_seed(8);
        g.apply(&Move::Gold);
        g.fear = FEAR_WIN - 1;
        g.apply(&Move::Play(4, 5)); // strigat: frică +2, gather
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn blight_loss_is_draw() {
        let mut g = Strajile::with_seed(9);
        clean(&mut g);
        g.blight = [2, 2, 2, 2, 1, 0, 0, 0];
        g.ravage = Some(TERRAIN[4]);
        g.inv[4] = [0, 0, 2];
        g.step = Step::Ravage;
        g.side = Player::Two;
        g.apply(&Move::Ravage);
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=40u64 {
            let mut g = Strajile::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                assert_eq!(g.total_cards(), 12);
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn ai_team_often_wins() {
        let mut wins = 0;
        for seed in 1..=40u64 {
            if ai_game(seed).is_won() {
                wins += 1;
            }
        }
        eprintln!("strajile: victorii {wins}/40");
        assert!(wins >= 4, "AI ar trebui să câștige uneori: {wins}/40");
    }

    #[test]
    fn evaluate_ignores_the_invader_deck_order() {
        let g = Strajile::with_seed(3);
        let mut h = g.clone();
        h.deck.reverse();
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }
}
