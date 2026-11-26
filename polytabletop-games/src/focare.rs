//! Focare — joc cooperativ de stingere a bolilor pe hartă (variantă originală, partener AI).
//!
//! Voi doi sunteți agronomi de teren într-o vale cu 24 de așezări. Patru boli ale culturilor
//! (r = rugina, m = mana, t = tăciunele, f = făinarea) se răspândesc de la o așezare la alta.
//! Mecanica „boli pe hartă, acțiuni pe tură, pachet de infectare, focare în lanț, epidemii” este
//! generică; harta, numele, bolile și textele sunt ale noastre.
//!
//! * Harta: 24 de așezări (câte 6 din fiecare boală), un inel cu 12 scurtături. Fiecare așezare
//!   are 0–3 „spori” (cuburi) din fiecare boală. Al 4-lea spor din aceeași boală produce un focar:
//!   vecinii primesc câte un spor, în lanț (o așezare focalizează o singură dată pe lanț).
//! * Tura ta: 4 acțiuni, apoi tragi 2 cărți-oraș (două epidemii în pachet fac valuri
//!   de infectare), apoi se infectează cărți din pachetul de infectare.
//!   Acțiuni: `mers X` (vecin), `zbor X` (renunți la cartea lui X), `charter X` (renunți la cartea
//!   orașului curent), `naveta X` (între stații), `statie` (cartea orașului curent), `tratez r`
//!   (−1 spor; boală vindecată: toți sporii ei din oraș), `dau` / `iau` (cartea orașului curent,
//!   doar dacă sunteți amândoi acolo), `vindec r` (la stație, 3 cărți de aceeași culoare), `pas`.
//! * Mutările forțate sunt separate (`trage`, `infecteaza`), ca AI-ul să nu „vadă” viitorul.
//! * Sfârșitul (victorie sau înfrângere) este tot o mutare forțată (`sfârșit`), ca AI-ul să nu
//!   „vadă” rezultatul înainte și să nu evite o victorie a echipei (Jucătorul 2 e partener).
//! * Limita mâinii: 7 cărți (`descarta X`).
//! * Boală vindecată și fără spori pe hartă = eradicată (nu mai infectează).
//!
//! Rezultat: toate cele 4 boli vindecate = echipa câștigă („Câștigă Jucătorul 1”).
//! Înfrângerea echipei (6 focare, fără spori în rezervă sau pachetul de cărți golit)
//! se încheie cu REMIZĂ (documentat: nu există adversar, deci nu câștigă Jucătorul 2).
//!
//! Simplificări față de jocurile de acest tip: fără roluri, fără cărți-eveniment, fără
//! carantină; stația de pornire în așezarea 1; vindecarea folosește doar mâna celui care acționează.
#![allow(clippy::needless_range_loop)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const N: usize = 24;
pub const DIS: usize = 4;
pub const CURE_N: usize = 3;
pub const MAX_OUTBREAKS: u8 = 6;
pub const SUPPLY: i32 = 14;
const HAND_LIMIT: usize = 7;
const EPIDEMICS: usize = 3;
const RATES: [usize; 6] = [2, 2, 2, 3, 3, 4];
const EPI: u8 = 255;

pub const NAMES: [&str; N] = [
    "Arin", "Bora", "Cedra", "Dalma", "Eres", "Fanar", "Gorun", "Hamal", "Ilva", "Jerna", "Kalas",
    "Lunar", "Mirta", "Norva", "Oriel", "Pelin", "Quara", "Rovan", "Salva", "Tamir", "Ulmet",
    "Vexa", "Zorel", "Yarin",
];
pub const DIS_NAMES: [&str; DIS] = ["rugina", "mana", "tăciunele", "făinarea"];
pub const DIS_LET: [char; DIS] = ['r', 'm', 't', 'f'];
const CHORDS: [(usize, usize); 12] = [
    (0, 12),
    (3, 9),
    (6, 18),
    (2, 14),
    (5, 20),
    (8, 15),
    (11, 22),
    (1, 17),
    (4, 16),
    (7, 21),
    (10, 19),
    (13, 23),
];

pub fn disease_of(city: usize) -> usize {
    city / 6
}

pub fn adjacency() -> Vec<Vec<usize>> {
    let mut a = vec![Vec::new(); N];
    for i in 0..N {
        let j = (i + 1) % N;
        a[i].push(j);
        a[j].push(i);
    }
    for (x, y) in CHORDS {
        a[x].push(y);
        a[y].push(x);
    }
    for v in a.iter_mut() {
        v.sort_unstable();
        v.dedup();
    }
    a
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Act,
    Draw,
    Discard,
    Infect,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Drive(u8),
    Fly(u8),
    Charter(u8),
    Shuttle(u8),
    Build,
    Treat(u8),
    Give,
    Take,
    Cure(u8),
    Pass,
    Draw,
    Discard(u8),
    Infect,
    Finish,
}

#[derive(Clone)]
pub struct Focare {
    adj: Vec<Vec<usize>>,
    cubes: [[u8; DIS]; N],
    supply: [i32; DIS],
    cured: [bool; DIS],
    stations: [bool; N],
    pos: [usize; 2],
    hands: [Vec<u8>; 2],
    pdeck: Vec<u8>,
    pdiscard: Vec<u8>,
    ideck: Vec<u8>,
    idiscard: Vec<u8>,
    rate_idx: usize,
    outbreaks: u8,
    side: Player,
    actions: u8,
    phase: Phase,
    over: bool,
    pending: Option<bool>,
    won: bool,
    reason: String,
    rng: Rng,
    last: String,
}

impl Default for Focare {
    fn default() -> Self {
        Self::new()
    }
}

impl Focare {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut ideck: Vec<u8> = (0..N as u8).collect();
        shuffle(&mut ideck, &mut rng);
        let mut g = Focare {
            adj: adjacency(),
            cubes: [[0; DIS]; N],
            supply: [SUPPLY; DIS],
            cured: [false; DIS],
            stations: [false; N],
            pos: [0, 0],
            hands: [Vec::new(), Vec::new()],
            pdeck: Vec::new(),
            pdiscard: Vec::new(),
            ideck: Vec::new(),
            idiscard: Vec::new(),
            rate_idx: 0,
            outbreaks: 0,
            side: Player::One,
            actions: 4,
            phase: Phase::Act,
            over: false,
            pending: None,
            won: false,
            reason: String::new(),
            rng,
            last: String::new(),
        };
        g.stations[0] = true;
        for k in 0..9 {
            let c = ideck.pop().unwrap() as usize;
            let n = 3 - k / 3;
            for _ in 0..n {
                let d = disease_of(c);
                g.cubes[c][d] += 1;
                g.supply[d] -= 1;
            }
            g.idiscard.push(c as u8);
        }
        g.ideck = ideck;
        let mut pd: Vec<u8> = (0..N as u8).collect();
        shuffle(&mut pd, &mut g.rng);
        for _ in 0..4 {
            g.hands[0].push(pd.pop().unwrap());
            g.hands[1].push(pd.pop().unwrap());
        }
        let per = pd.len() / EPICENTERS;
        let mut deck: Vec<u8> = Vec::new();
        let mut rest = pd;
        for k in 0..EPICENTERS {
            let take = if k + 1 == EPICENTERS { rest.len() } else { per };
            let mut pile: Vec<u8> = rest.drain(..take).collect();
            pile.push(EPI);
            shuffle(&mut pile, &mut g.rng);
            deck.extend(pile);
        }
        // deck[0] e fundul pachetului; pop() trage de sus.
        g.pdeck = deck;
        g.hands[0].sort_unstable();
        g.hands[1].sort_unstable();
        g
    }

    pub fn cured_count(&self) -> usize {
        self.cured.iter().filter(|x| **x).count()
    }

    pub fn outbreaks(&self) -> u8 {
        self.outbreaks
    }

    pub fn is_won(&self) -> bool {
        self.over && self.won
    }

    fn total_cubes(&self, d: usize) -> i32 {
        (0..N).map(|c| i32::from(self.cubes[c][d])).sum()
    }

    fn eradicated(&self, d: usize) -> bool {
        self.cured[d] && self.total_cubes(d) == 0
    }

    fn ended(&self) -> bool {
        self.over || self.pending.is_some()
    }

    fn lose(&mut self, why: &str) {
        if !self.ended() {
            self.pending = Some(false);
            self.reason = why.to_string();
        }
    }

    fn add_cube(&mut self, city: usize, d: usize, chain: &mut [bool; N]) {
        if self.ended() || self.eradicated(d) {
            return;
        }
        if self.cubes[city][d] >= 3 {
            if chain[city] {
                return;
            }
            chain[city] = true;
            self.outbreaks += 1;
            self.last.push_str(&format!(" Focar în {}!", NAMES[city]));
            if self.outbreaks >= MAX_OUTBREAKS {
                self.lose("prea multe focare");
                return;
            }
            let nb = self.adj[city].clone();
            for x in nb {
                self.add_cube(x, d, chain);
                if self.ended() {
                    return;
                }
            }
        } else {
            if self.supply[d] <= 0 {
                self.lose("rezerva de spori s-a epuizat");
                return;
            }
            self.cubes[city][d] += 1;
            self.supply[d] -= 1;
        }
    }

    fn draw_infection(&mut self) -> u8 {
        if self.ideck.is_empty() {
            let mut d = std::mem::take(&mut self.idiscard);
            shuffle(&mut d, &mut self.rng);
            self.ideck = d;
        }
        self.ideck.pop().expect("pachet de infectare nevid")
    }

    fn epidemic(&mut self) {
        self.rate_idx = (self.rate_idx + 1).min(RATES.len() - 1);
        if self.ideck.is_empty() {
            let mut d = std::mem::take(&mut self.idiscard);
            shuffle(&mut d, &mut self.rng);
            self.ideck = d;
        }
        let c = self.ideck.remove(0) as usize;
        self.last.push_str(&format!(" Epidemie în {}!", NAMES[c]));
        let d = disease_of(c);
        let mut chain = [false; N];
        for _ in 0..3 {
            self.add_cube(c, d, &mut chain);
        }
        self.idiscard.push(c as u8);
        let mut d = std::mem::take(&mut self.idiscard);
        shuffle(&mut d, &mut self.rng);
        self.ideck.extend(d);
    }

    fn after_action(&mut self) {
        if self.actions > 0 {
            self.actions -= 1;
        }
        if self.actions == 0 {
            self.phase = Phase::Draw;
        }
    }

    fn drop_card(&mut self, p: usize, c: u8) {
        if let Some(i) = self.hands[p].iter().position(|x| *x == c) {
            self.hands[p].remove(i);
            self.pdiscard.push(c);
        }
    }

    fn dist_from(&self, from: usize) -> [i32; N] {
        let mut d = [i32::MAX; N];
        let mut q = std::collections::VecDeque::new();
        d[from] = 0;
        q.push_back(from);
        while let Some(x) = q.pop_front() {
            for &y in &self.adj[x] {
                if d[y] == i32::MAX {
                    d[y] = d[x] + 1;
                    q.push_back(y);
                }
            }
        }
        d
    }

    /// Valoare euristică a stării (aceeași pentru ambii jucători: echipă).
    fn team_value(&self) -> i32 {
        if let Some(w) = self.pending {
            return if w { 100_000 } else { -100_000 };
        }
        if self.over {
            return if self.won { 100_000 } else { -100_000 };
        }
        const W: [i32; 4] = [0, 10, 30, 90];
        let mut s = 4000 * self.cured_count() as i32;
        s -= 400 * i32::from(self.outbreaks);
        let mut threat = [0i32; N];
        for c in 0..N {
            for d in 0..DIS {
                let k = self.cubes[c][d] as usize;
                if self.cured[d] && k == 0 {
                    continue;
                }
                s -= W[k];
                threat[c] += W[k];
                if k == 3 {
                    let bad = self.adj[c]
                        .iter()
                        .filter(|&&y| self.cubes[y][d] == 3)
                        .count() as i32;
                    s -= 40 * bad;
                }
            }
        }
        for d in 0..DIS {
            if self.supply[d] < 5 {
                s -= (5 - self.supply[d]) * 25;
            }
        }
        s += 12 * self.stations.iter().filter(|x| **x).count() as i32;
        for d in 0..DIS {
            if self.cured[d] {
                continue;
            }
            let cnt: Vec<usize> = (0..2)
                .map(|p| {
                    self.hands[p]
                        .iter()
                        .filter(|&&c| disease_of(c as usize) == d)
                        .count()
                })
                .collect();
            let (hi, lo) = if cnt[0] >= cnt[1] {
                (cnt[0], cnt[1])
            } else {
                (cnt[1], cnt[0])
            };
            s += 200 * hi.min(CURE_N) as i32 + 60 * lo.min(CURE_N) as i32;
        }
        s += self.hands[0].len() as i32 + self.hands[1].len() as i32;
        for p in 0..2 {
            let dist = self.dist_from(self.pos[p]);
            let mut best = 0;
            for c in 0..N {
                best = best.max(threat[c] - 10 * dist[c]);
            }
            s += (best * 3) / 10;
            // pregătit de vindecare: mergi spre o stație
            let ready = (0..DIS).any(|d| {
                !self.cured[d]
                    && self.hands[p]
                        .iter()
                        .filter(|&&c| disease_of(c as usize) == d)
                        .count()
                        >= CURE_N
            });
            if ready {
                let nearest = (0..N)
                    .filter(|&c| self.stations[c])
                    .map(|c| dist[c])
                    .min()
                    .unwrap_or(10);
                s += 400 - 40 * nearest.min(10);
            }
        }
        s
    }
}

impl Focare {
    /// Cea mai bună valoare de echipă atinsă prin cel mult `depth` acțiuni rămase în tură.
    fn look(&self, depth: u32) -> i32 {
        if depth == 0
            || self.over
            || self.pending.is_some()
            || self.phase != Phase::Act
            || self.actions == 0
        {
            return self.team_value();
        }
        let mut kids: Vec<(i32, Focare)> = Game::legal_moves(self)
            .iter()
            .map(|m| {
                let mut c = self.clone();
                c.apply(m);
                (c.team_value(), c)
            })
            .collect();
        kids.sort_by_key(|k| std::cmp::Reverse(k.0));
        kids.truncate(BEAM);
        kids.iter()
            .map(|(_, c)| c.look(depth - 1))
            .max()
            .unwrap_or_else(|| self.team_value())
    }
}

const BEAM: usize = 4;

impl Game for Focare {
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
        match self.phase {
            Phase::Draw => vec![Move::Draw],
            Phase::Infect => vec![Move::Infect],
            Phase::Discard => {
                let mut v: Vec<Move> = self.hands[p].iter().map(|c| Move::Discard(*c)).collect();
                v.dedup();
                v
            }
            Phase::Act => {
                let here = self.pos[p];
                let mut v = Vec::new();
                for &y in &self.adj[here] {
                    v.push(Move::Drive(y as u8));
                }
                for &c in &self.hands[p] {
                    if c as usize != here {
                        v.push(Move::Fly(c));
                    }
                }
                if self.hands[p].contains(&(here as u8)) {
                    for c in 0..N {
                        if c != here {
                            v.push(Move::Charter(c as u8));
                        }
                    }
                    if !self.stations[here] {
                        v.push(Move::Build);
                    }
                }
                if self.stations[here] {
                    for c in 0..N {
                        if c != here && self.stations[c] {
                            v.push(Move::Shuttle(c as u8));
                        }
                    }
                }
                for d in 0..DIS {
                    if self.cubes[here][d] > 0 {
                        v.push(Move::Treat(d as u8));
                    }
                }
                let q = 1 - p;
                if self.pos[q] == here {
                    if self.hands[p].contains(&(here as u8)) && self.hands[q].len() < HAND_LIMIT {
                        v.push(Move::Give);
                    }
                    if self.hands[q].contains(&(here as u8)) && self.hands[p].len() < HAND_LIMIT {
                        v.push(Move::Take);
                    }
                }
                if self.stations[here] {
                    for d in 0..DIS {
                        if !self.cured[d]
                            && self.hands[p]
                                .iter()
                                .filter(|&&c| disease_of(c as usize) == d)
                                .count()
                                >= CURE_N
                        {
                            v.push(Move::Cure(d as u8));
                        }
                    }
                }
                v.push(Move::Pass);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let who = format!("J{}", p + 1);
        self.last.clear();
        match *m {
            Move::Drive(c) => {
                self.pos[p] = c as usize;
                self.last = format!("{who} a mers în {}.", NAMES[c as usize]);
                self.after_action();
            }
            Move::Fly(c) => {
                self.drop_card(p, c);
                self.pos[p] = c as usize;
                self.last = format!("{who} a zburat în {}.", NAMES[c as usize]);
                self.after_action();
            }
            Move::Charter(c) => {
                let here = self.pos[p] as u8;
                self.drop_card(p, here);
                self.pos[p] = c as usize;
                self.last = format!("{who} a închiriat un zbor spre {}.", NAMES[c as usize]);
                self.after_action();
            }
            Move::Shuttle(c) => {
                self.pos[p] = c as usize;
                self.last = format!("{who} a folosit naveta spre {}.", NAMES[c as usize]);
                self.after_action();
            }
            Move::Build => {
                let here = self.pos[p];
                self.drop_card(p, here as u8);
                self.stations[here] = true;
                self.last = format!("{who} a ridicat o stație în {}.", NAMES[here]);
                self.after_action();
            }
            Move::Treat(d) => {
                let d = d as usize;
                let here = self.pos[p];
                let k = if self.cured[d] {
                    self.cubes[here][d]
                } else {
                    1
                };
                self.cubes[here][d] -= k;
                self.supply[d] += i32::from(k);
                self.last = format!("{who} a tratat {} în {}.", DIS_NAMES[d], NAMES[here]);
                self.after_action();
            }
            Move::Give => {
                let here = self.pos[p] as u8;
                if let Some(i) = self.hands[p].iter().position(|x| *x == here) {
                    let c = self.hands[p].remove(i);
                    self.hands[1 - p].push(c);
                    self.hands[1 - p].sort_unstable();
                }
                self.last = format!("{who} a dat cartea {}.", NAMES[here as usize]);
                self.after_action();
            }
            Move::Take => {
                let here = self.pos[p] as u8;
                if let Some(i) = self.hands[1 - p].iter().position(|x| *x == here) {
                    let c = self.hands[1 - p].remove(i);
                    self.hands[p].push(c);
                    self.hands[p].sort_unstable();
                }
                self.last = format!("{who} a luat cartea {}.", NAMES[here as usize]);
                self.after_action();
            }
            Move::Cure(d) => {
                let d = d as usize;
                let mut left = CURE_N;
                let mut i = 0;
                while i < self.hands[p].len() && left > 0 {
                    let c = self.hands[p][i];
                    if disease_of(c as usize) == d {
                        self.hands[p].remove(i);
                        self.pdiscard.push(c);
                        left -= 1;
                    } else {
                        i += 1;
                    }
                }
                self.cured[d] = true;
                self.last = format!("{who} a găsit remediul pentru {}!", DIS_NAMES[d]);
                if self.cured_count() == DIS {
                    self.pending = Some(true);
                    self.reason = "toate bolile vindecate".into();
                    return;
                }
                self.after_action();
            }
            Move::Pass => {
                self.actions = 0;
                self.phase = Phase::Draw;
                self.last = format!("{who} a dat pas.");
            }
            Move::Draw => {
                for _ in 0..2 {
                    match self.pdeck.pop() {
                        None => {
                            self.lose("pachetul de cărți s-a golit");
                            return;
                        }
                        Some(EPI) => {
                            self.epidemic();
                            if self.ended() {
                                return;
                            }
                        }
                        Some(c) => self.hands[p].push(c),
                    }
                }
                self.hands[p].sort_unstable();
                self.phase = if self.hands[p].len() > HAND_LIMIT {
                    Phase::Discard
                } else {
                    Phase::Infect
                };
                self.last = format!("{who} a tras două cărți.{}", self.last);
            }
            Move::Discard(c) => {
                self.drop_card(p, c);
                if self.hands[p].len() <= HAND_LIMIT {
                    self.phase = Phase::Infect;
                }
            }
            Move::Finish => {
                self.over = true;
                self.won = self.pending == Some(true);
            }
            Move::Infect => {
                let n = RATES[self.rate_idx];
                self.last = format!("Infectare ({n} cărți):");
                for _ in 0..n {
                    let c = self.draw_infection() as usize;
                    self.idiscard.push(c as u8);
                    self.last.push_str(&format!(" {}", NAMES[c]));
                    let d = disease_of(c);
                    let mut chain = [false; N];
                    self.add_cube(c, d, &mut chain);
                    if self.ended() {
                        return;
                    }
                }
                self.side = self.side.other();
                self.actions = 4;
                self.phase = Phase::Act;
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

    /// Cooperativ: toate mutările din faza de acțiuni sunt deterministe și publice (tragerea
    /// și infectarea sunt mutări forțate separate), deci căutăm în lățime limitată (beam)
    /// restul acțiunilor din tură și evaluăm cu `team_value`; nu citim pachetele ascunse.
    fn evaluate(&self, _p: Player) -> i32 {
        self.look(u32::from(self.actions).min(3))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Focare — vindecate {}/{} | focare {}/{} | rata infectării {} | pachet cărți {} | pachet infectare {}\n",
            self.cured_count(),
            DIS,
            self.outbreaks,
            MAX_OUTBREAKS,
            RATES[self.rate_idx],
            self.pdeck.len(),
            self.ideck.len()
        ));
        let sup: Vec<String> = (0..DIS)
            .map(|d| {
                format!(
                    "{}{}={}{}",
                    DIS_LET[d],
                    if self.cured[d] { "*" } else { "" },
                    self.supply[d],
                    if self.eradicated(d) {
                        "(eradicată)"
                    } else {
                        ""
                    }
                )
            })
            .collect();
        s.push_str(&format!(
            "Rezerva de spori: {}  (* = remediu găsit)\n",
            sup.join("  ")
        ));
        for c in 0..N {
            let mut line = format!(
                "{:>6}({}){}",
                NAMES[c],
                DIS_LET[disease_of(c)],
                if self.stations[c] { "#" } else { " " }
            );
            let cubes: Vec<String> = (0..DIS)
                .filter(|&d| self.cubes[c][d] > 0)
                .map(|d| format!("{}{}", DIS_LET[d], self.cubes[c][d]))
                .collect();
            line.push_str(&format!(" {:<10}", cubes.join(" ")));
            for q in 0..2 {
                if self.pos[q] == c {
                    line.push_str(&format!(" [J{}]", q + 1));
                }
            }
            let nb: Vec<&str> = self.adj[c].iter().map(|&y| NAMES[y]).collect();
            line.push_str(&format!("  -> {}\n", nb.join(" ")));
            s.push_str(&line);
        }
        for q in 0..2 {
            let h: Vec<String> = self.hands[q]
                .iter()
                .map(|&c| format!("{}({})", NAMES[c as usize], DIS_LET[disease_of(c as usize)]))
                .collect();
            s.push_str(&format!(
                "Mâna J{}{}: {}\n",
                q + 1,
                if q == 0 { " (tu)" } else { " (partener AI)" },
                if h.is_empty() {
                    "—".into()
                } else {
                    h.join(" ")
                }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        if self.pending.is_some() && !self.over {
            s.push_str("Jocul se încheie: sfârșit.\n");
        } else if self.over {
            if self.won {
                s.push_str("VICTORIE: toate bolile au fost vindecate.\n");
            } else {
                s.push_str(&format!("ÎNFRÂNGERE (remiză): {}.\n", self.reason));
            }
        } else {
            match self.phase {
                Phase::Act => s.push_str(&format!(
                    "La rând J{} — acțiuni rămase: {}. Mutări: mers X, zbor X, charter X, naveta X, statie, tratez r, dau, iau, vindec r, pas.\n",
                    self.side.idx() + 1,
                    self.actions
                )),
                Phase::Draw => s.push_str("Urmează tragerea a două cărți: trage.\n"),
                Phase::Discard => s.push_str("Mâna are peste 7 cărți: descarta X.\n"),
                Phase::Infect => s.push_str("Urmează infectarea: infecteaza.\n"),
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Drive(c) => format!("mers {}", NAMES[*c as usize].to_lowercase()),
            Move::Fly(c) => format!("zbor {}", NAMES[*c as usize].to_lowercase()),
            Move::Charter(c) => format!("charter {}", NAMES[*c as usize].to_lowercase()),
            Move::Shuttle(c) => format!("naveta {}", NAMES[*c as usize].to_lowercase()),
            Move::Build => "statie".into(),
            Move::Treat(d) => format!("tratez {}", DIS_LET[*d as usize]),
            Move::Give => "dau".into(),
            Move::Take => "iau".into(),
            Move::Cure(d) => format!("vindec {}", DIS_LET[*d as usize]),
            Move::Pass => "pas".into(),
            Move::Draw => "trage".into(),
            Move::Discard(c) => format!("descarta {}", NAMES[*c as usize].to_lowercase()),
            Move::Infect => "infecteaza".into(),
            Move::Finish => "sfârșit".into(),
        }
    }
}

const EPICENTERS: usize = EPIDEMICS;

#[cfg(test)]
mod tests {
    use super::*;

    fn conserved(g: &Focare) {
        let mut cities = 0;
        let mut epi = 0;
        for c in g.pdeck.iter() {
            if *c == EPI {
                epi += 1;
            } else {
                cities += 1;
            }
        }
        cities += g.hands[0].len() + g.hands[1].len() + g.pdiscard.len();
        assert_eq!(cities, N);
        assert!(epi <= EPIDEMICS);
        for d in 0..DIS {
            assert_eq!(g.total_cubes(d) + g.supply[d], SUPPLY, "spori boală {d}");
        }
        assert!(g.ideck.len() + g.idiscard.len() <= N);
    }

    fn ai_game(seed: u64) -> Focare {
        let mut g = Focare::with_seed(seed);
        let mut rng = Rng::new(seed ^ 5);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            if g.pending.is_none() {
                conserved(&g);
            }
            n += 1;
            assert!(n < 3000);
        }
        g
    }

    #[test]
    fn setup_is_correct() {
        let g = Focare::with_seed(1);
        assert_eq!(g.hands[0].len(), 4);
        assert_eq!(g.hands[1].len(), 4);
        assert_eq!(g.idiscard.len(), 9);
        assert_eq!(g.ideck.len(), N - 9);
        let total: i32 = (0..DIS).map(|d| g.total_cubes(d)).sum();
        assert_eq!(total, 18);
        assert_eq!(g.pdeck.iter().filter(|c| **c == EPI).count(), EPIDEMICS);
        conserved(&g);
        let a = adjacency();
        for i in 0..N {
            for &j in &a[i] {
                assert!(a[j].contains(&i));
            }
        }
    }

    #[test]
    fn outbreak_chains_and_counts_once() {
        let mut g = Focare::with_seed(2);
        g.cubes = [[0; DIS]; N];
        g.supply = [SUPPLY; DIS];
        for c in [0usize, 1, 23] {
            g.cubes[c][0] = 3;
            g.supply[0] -= 3;
        }
        let mut chain = [false; N];
        g.add_cube(0, 0, &mut chain);
        // 0 focalizează, 1 și 23 (3 cuburi) focalizează la rândul lor.
        assert_eq!(g.outbreaks, 3);
        assert_eq!(g.total_cubes(0) + g.supply[0], SUPPLY);
    }

    #[test]
    fn cure_needs_station_and_cards() {
        let mut g = Focare::with_seed(3);
        g.hands[0] = vec![0, 1, 2, 7];
        assert!(g.legal_moves().contains(&Move::Cure(0)));
        g.stations[0] = false;
        assert!(!g.legal_moves().contains(&Move::Cure(0)));
        g.stations[0] = true;
        g.hands[0] = vec![0, 1, 7, 8];
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Cure(_))));
    }

    #[test]
    fn cure_all_wins() {
        let mut g = Focare::with_seed(4);
        g.cured = [true, true, true, false];
        g.hands[0] = vec![18, 19, 20, 3];
        g.apply(&Move::Cure(3));
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn cured_and_empty_is_eradicated() {
        let mut g = Focare::with_seed(5);
        g.cubes = [[0; DIS]; N];
        g.supply = [SUPPLY; DIS];
        g.cured[1] = true;
        let mut ch = [false; N];
        g.add_cube(7, 1, &mut ch);
        assert_eq!(g.total_cubes(1), 0);
    }

    #[test]
    fn treat_removes_all_when_cured() {
        let mut g = Focare::with_seed(6);
        g.cubes = [[0; DIS]; N];
        g.supply = [SUPPLY; DIS];
        g.cubes[0][2] = 3;
        g.supply[2] -= 3;
        g.cured[2] = true;
        g.apply(&Move::Treat(2));
        assert_eq!(g.cubes[0][2], 0);
        assert_eq!(g.supply[2], SUPPLY);
    }

    #[test]
    fn epidemic_reshuffles_discards_on_top() {
        let mut g = Focare::with_seed(7);
        let before = g.ideck.len() + g.idiscard.len();
        g.epidemic();
        assert_eq!(g.ideck.len() + g.idiscard.len(), before);
        assert!(g.idiscard.is_empty());
        assert_eq!(g.rate_idx, 1);
    }

    #[test]
    fn turn_flow_has_forced_steps() {
        let mut g = Focare::with_seed(8);
        for _ in 0..4 {
            let m = g.legal_moves().into_iter().find(|m| *m == Move::Pass);
            if let Some(m) = m {
                g.apply(&m);
                break;
            }
        }
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
        g.apply(&Move::Draw);
        if g.phase == Phase::Discard {
            while g.phase == Phase::Discard {
                let m = g.legal_moves()[0];
                g.apply(&m);
            }
        }
        if g.pending.is_none() {
            assert_eq!(g.legal_moves(), vec![Move::Infect]);
            g.apply(&Move::Infect);
            assert_eq!(g.to_move(), Player::Two);
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Focare::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                n += 1;
                assert!(n < 5000);
                if g.pending.is_none() {
                    conserved(&g);
                }
            }
        }
    }

    #[test]
    fn ai_team_plays_and_sometimes_wins() {
        let mut wins = 0;
        let mut cured = 0;
        for seed in 1..=40u64 {
            let g = ai_game(seed);
            if g.is_won() {
                wins += 1;
            }
            cured += g.cured_count();
        }
        eprintln!(
            "focare: victorii {wins}/40, remedii medii {}",
            cured as f64 / 40.0
        );
        assert!(
            cured >= 40,
            "AI ar trebui să găsească măcar un remediu în medie"
        );
    }

    #[test]
    fn evaluate_ignores_the_hidden_deck_order() {
        let g = Focare::with_seed(3);
        let mut h = g.clone();
        h.pdeck.reverse();
        h.ideck.reverse();
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }
}
