//! Itinerar — joc original de turism pe tablă („cumperi destinații”, 2–4 locuri).
//!
//! Mecanica „rezervi destinații, construiești cazare, încasezi taxe de la turiști” e generică;
//! tabla (28 de câmpuri: 7 regiuni × 3 destinații + 7 câmpuri speciale), numele, prețurile,
//! cărțile Tombolei și regulile de mai jos sunt ale noastre. Fără licitații, ipoteci sau schimburi.
//!
//! Tura (fiecare pas este o mutare textuală):
//! 1. `arunc` — zarurile 2d6 (aruncate automat); `mut` — pionul avansează și se rezolvă câmpul.
//!    (Pasul dublu există ca IA-ul să nu „vadă” zarurile înainte de a alege.)
//! 2. Destinație liberă pe care ai aterizat: `rezerv` (plătești taxa de intrare = prețul) sau `las`.
//!    Destinație cu proprietar: plătești **taxa de cazare**. Cu colecția completă a regiunii
//!    (toate cele 3 destinații ale aceluiași jucător) taxele se dublează.
//! 3. La sfârșit poți: `construiesc N` (un nivel de cazare pe destinația N, maximum 3 niveluri;
//!    costul depinde de regiune), `rezerv N` (rezervare la distanță, o dată pe tură, preț ×1,5),
//!    `zbor` (de pe un aeroport către celălalt, 60; ține locul zarurilor și închide tura) sau `gata`.
//! 4. Dublă = încă o aruncare; a treia dublă la rând trimite la **vamă** (pierzi tura următoare).
//!
//! Câmpuri speciale: Aeroport Nord (start; la fiecare trecere încasezi 100), Tombolă (carte de
//! eveniment procedurală), Vamă, Bac (plătești 15 și treci 3 câmpuri înainte), Aeroport Sud
//! (zonă de odihnă și aeroport), Hotel de lux (`lux`: 60 pentru un nivel gratuit pe una din
//! unitățile tale, sau `las`), Taxa de turist (10% din bani, între 30 și 120).
//! Faliment = eliminare (proprietățile revin băncii); ultimul rămas câștigă, iar după 40 de
//! runde câștigă cea mai mare avere (bani + prețuri + niveluri); egalitate = remiză.
//!
//! Informație publică (fără mâini ascunse). Registrul joacă în doi; `with_players(3..=4)` există
//! pentru teste (locurile impare/pare se mapează pe Player::Two/One).

#![allow(clippy::needless_range_loop)]

use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

pub const SQ: usize = 28;
pub const MAX_ROUNDS: u32 = 40;
const START_CASH: i32 = 1200;
const SALARY: i32 = 100;
const FLIGHT: i32 = 60;
const FERRY: i32 = 15;
const LUXURY: i32 = 60;
const MAX_LEVEL: u8 = 3;
const PLY_CAP: u32 = 8000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sq {
    AirportN,
    Dest(u8, i32),
    Lottery,
    Customs,
    Ferry,
    AirportS,
    Luxury,
    Tax,
}

const BOARD: [(Sq, &str); SQ] = [
    (Sq::AirportN, "Aeroport Nord"),
    (Sq::Dest(0, 60), "Golful Albastru"),
    (Sq::Dest(0, 60), "Plaja de Aur"),
    (Sq::Dest(0, 70), "Portul Pescarilor"),
    (Sq::Lottery, "Tombolă"),
    (Sq::Dest(1, 80), "Pasul Înalt"),
    (Sq::Dest(1, 80), "Valea Verde"),
    (Sq::Dest(1, 90), "Cabana din Stâncă"),
    (Sq::Customs, "Vamă"),
    (Sq::Dest(2, 100), "Lacul Oglindă"),
    (Sq::Dest(2, 100), "Insula Nuferilor"),
    (Sq::Dest(2, 110), "Izvorul Cald"),
    (Sq::Ferry, "Bac"),
    (Sq::Dest(3, 120), "Poarta Soarelui"),
    (Sq::Dest(3, 130), "Strada Olarilor"),
    (Sq::Dest(3, 130), "Turnul cu Ceas"),
    (Sq::AirportS, "Aeroport Sud (odihnă)"),
    (Sq::Dest(4, 150), "Dealul Viilor"),
    (Sq::Dest(4, 160), "Crama Veche"),
    (Sq::Dest(4, 160), "Conacul Roșu"),
    (Sq::Luxury, "Hotel de lux"),
    (Sq::Dest(5, 180), "Oaza Palmierilor"),
    (Sq::Dest(5, 190), "Dunele Aurii"),
    (Sq::Dest(5, 190), "Caravanseraiul"),
    (Sq::Tax, "Taxa de turist"),
    (Sq::Dest(6, 220), "Insula Coralilor"),
    (Sq::Dest(6, 230), "Laguna Secretă"),
    (Sq::Dest(6, 240), "Farul de Argint"),
];
const REGION_NAMES: [&str; 7] = [
    "Litoral",
    "Munții",
    "Lacurile",
    "Orașul vechi",
    "Podgoriile",
    "Deșertul",
    "Arhipelagul",
];
const BUILD_COST: [i32; 7] = [40, 50, 60, 70, 80, 90, 100];
const LEVEL_MULT: [i32; 4] = [1, 3, 5, 8];
const CUSTOMS_POS: usize = 8;
const AIRPORTS: [usize; 2] = [0, 16];

fn price(sq: usize) -> i32 {
    match BOARD[sq].0 {
        Sq::Dest(_, p) => p,
        _ => 0,
    }
}
fn region(sq: usize) -> Option<usize> {
    match BOARD[sq].0 {
        Sq::Dest(r, _) => Some(r as usize),
        _ => None,
    }
}
fn region_squares(r: usize) -> Vec<usize> {
    (0..SQ).filter(|&i| region(i) == Some(r)).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Step,
    ReserveHere,
    ReserveAt(usize),
    Decline,
    Build(usize),
    Fly,
    Luxury,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Roll,
    Step,
    Buy(usize),
    Lux,
    Manage,
}

#[derive(Clone)]
pub struct Itinerar {
    n: usize,
    pos: [usize; 4],
    cash: [i32; 4],
    alive: [bool; 4],
    skip: [u8; 4],
    owner: [Option<u8>; SQ],
    lvl: [u8; SQ],
    seat: usize,
    phase: Phase,
    doubles: u8,
    dice: (u8, u8),
    remote_done: bool,
    round: u32,
    plies: u32,
    winner: Option<Outcome>,
    log: String,
    rng: Rng,
}

impl Default for Itinerar {
    fn default() -> Self {
        Self::new()
    }
}

fn player_of(seat: usize) -> Player {
    if seat & 1 == 0 {
        Player::One
    } else {
        Player::Two
    }
}

/// Probabilitatea (în 36-imi) ca 2d6 să dea `d`.
fn p2d6(d: usize) -> i32 {
    match d {
        2..=7 => d as i32 - 1,
        8..=12 => 13 - d as i32,
        _ => 0,
    }
}

impl Itinerar {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::with_players(2, seed)
    }

    pub fn with_players(n: usize, seed: u64) -> Self {
        let n = n.clamp(2, 4);
        let mut alive = [false; 4];
        for a in alive.iter_mut().take(n) {
            *a = true;
        }
        Itinerar {
            n,
            pos: [0; 4],
            cash: [START_CASH; 4],
            alive,
            skip: [0; 4],
            owner: [None; SQ],
            lvl: [0; SQ],
            seat: 0,
            phase: Phase::Roll,
            doubles: 0,
            dice: (0, 0),
            remote_done: false,
            round: 1,
            plies: 0,
            winner: None,
            log: String::new(),
            rng: Rng::new(seed),
        }
    }

    pub fn players(&self) -> usize {
        self.n
    }
    pub fn cash(&self, seat: usize) -> i32 {
        self.cash[seat]
    }
    pub fn position(&self, seat: usize) -> usize {
        self.pos[seat]
    }
    pub fn owner_of(&self, sq: usize) -> Option<usize> {
        self.owner[sq].map(|o| o as usize)
    }
    pub fn level_of(&self, sq: usize) -> u8 {
        self.lvl[sq]
    }
    pub fn is_alive(&self, seat: usize) -> bool {
        self.alive[seat]
    }
    pub fn round(&self) -> u32 {
        self.round
    }
    pub fn seat_to_move(&self) -> usize {
        self.seat
    }

    fn owns_region(&self, s: usize, r: usize) -> bool {
        region_squares(r)
            .iter()
            .all(|&q| self.owner[q] == Some(s as u8))
    }

    /// Taxa de cazare pe `sq` (0 dacă nu are proprietar).
    pub fn stay_tax(&self, sq: usize) -> i32 {
        let (Some(o), Some(r)) = (self.owner[sq], region(sq)) else {
            return 0;
        };
        let base = price(sq) / 5 * LEVEL_MULT[self.lvl[sq] as usize];
        if self.owns_region(o as usize, r) {
            base * 2
        } else {
            base
        }
    }

    pub fn net_worth(&self, s: usize) -> i32 {
        let mut w = self.cash[s];
        for q in 0..SQ {
            if self.owner[q] == Some(s as u8) {
                w += price(q) + self.lvl[q] as i32 * BUILD_COST[region(q).unwrap()];
            }
        }
        w
    }

    fn alive_count(&self) -> usize {
        self.alive.iter().filter(|&&a| a).count()
    }

    fn can_build(&self, q: usize) -> bool {
        self.owner[q] == Some(self.seat as u8)
            && self.lvl[q] < MAX_LEVEL
            && self.cash[self.seat] >= BUILD_COST[region(q).unwrap()]
    }

    fn manage_moves(&self) -> Vec<Move> {
        let s = self.seat;
        let mut v = Vec::new();
        for q in 0..SQ {
            if region(q).is_some() {
                if self.can_build(q) {
                    v.push(Move::Build(q));
                }
                if !self.remote_done && self.owner[q].is_none() && self.cash[s] >= price(q) * 3 / 2
                {
                    v.push(Move::ReserveAt(q));
                }
            }
        }
        if AIRPORTS.contains(&self.pos[s]) && self.doubles == 0 && self.cash[s] >= FLIGHT {
            v.push(Move::Fly);
        }
        v.push(Move::Done);
        v
    }

    /// Plătește; vinde niveluri dacă trebuie; la faliment elimină locul. Întoarce true dacă a plătit.
    fn pay(&mut self, s: usize, amount: i32, to: Option<usize>) -> bool {
        if amount <= 0 {
            return true;
        }
        while self.cash[s] < amount {
            let best = (0..SQ)
                .filter(|&q| self.owner[q] == Some(s as u8) && self.lvl[q] > 0)
                .max_by_key(|&q| (self.lvl[q], std::cmp::Reverse(q)));
            match best {
                Some(q) => {
                    self.lvl[q] -= 1;
                    self.cash[s] += BUILD_COST[region(q).unwrap()] / 2;
                }
                None => break,
            }
        }
        if self.cash[s] < amount {
            let rest = self.cash[s].max(0);
            self.cash[s] = 0;
            if let Some(t) = to {
                self.cash[t] += rest;
            }
            self.eliminate(s);
            return false;
        }
        self.cash[s] -= amount;
        if let Some(t) = to {
            self.cash[t] += amount;
        }
        true
    }

    fn eliminate(&mut self, s: usize) {
        self.alive[s] = false;
        self.cash[s] = 0;
        for q in 0..SQ {
            if self.owner[q] == Some(s as u8) {
                self.owner[q] = None;
                self.lvl[q] = 0;
            }
        }
        self.log.push_str(&format!(" J{} dă faliment!", s + 1));
        if self.alive_count() == 1 {
            let w = (0..self.n).find(|&i| self.alive[i]).unwrap();
            self.winner = Some(Outcome::Win(player_of(w)));
        }
    }

    fn finish_by_wealth(&mut self) {
        let mut best = i32::MIN;
        let mut who: Vec<usize> = Vec::new();
        for s in 0..self.n {
            if !self.alive[s] {
                continue;
            }
            let w = self.net_worth(s);
            if w > best {
                best = w;
                who.clear();
                who.push(s);
            } else if w == best {
                who.push(s);
            }
        }
        // egalitate între locuri ale aceluiași Player = victorie pentru acel Player
        let first = player_of(who[0]);
        self.winner = Some(if who.iter().all(|&s| player_of(s) == first) {
            Outcome::Win(first)
        } else {
            Outcome::Draw
        });
    }

    fn move_forward(&mut self, s: usize, steps: usize) {
        let old = self.pos[s];
        self.pos[s] = (old + steps) % SQ;
        if old + steps >= SQ {
            self.cash[s] += SALARY;
        }
    }

    fn start_turn(&mut self) {
        loop {
            if self.winner.is_some() {
                return;
            }
            if self.alive[self.seat] && self.skip[self.seat] > 0 {
                self.skip[self.seat] -= 1;
                self.log
                    .push_str(&format!(" J{} e reținut la vamă.", self.seat + 1));
                self.next_seat();
                continue;
            }
            if self.alive[self.seat] {
                break;
            }
            self.next_seat();
        }
        self.phase = Phase::Roll;
        self.doubles = 0;
        self.remote_done = false;
    }

    fn next_seat(&mut self) {
        let mut s = self.seat;
        loop {
            s = (s + 1) % self.n;
            if self.alive[s] {
                break;
            }
        }
        if s <= self.seat {
            self.round += 1;
            if self.round > MAX_ROUNDS && self.winner.is_none() {
                self.finish_by_wealth();
            }
        }
        self.seat = s;
    }

    fn end_turn(&mut self) {
        if self.winner.is_some() {
            return;
        }
        self.next_seat();
        self.start_turn();
    }

    /// După aterizare/decizii: ori încă o aruncare (dublă), ori sfârșitul turei.
    fn after_actions(&mut self) {
        if self.winner.is_some() {
            return;
        }
        if !self.alive[self.seat] {
            self.end_turn();
            return;
        }
        if self.manage_moves().len() > 1 {
            self.phase = Phase::Manage;
        } else {
            self.finish_manage();
        }
    }

    fn finish_manage(&mut self) {
        if self.winner.is_some() {
            return;
        }
        if self.alive[self.seat] && self.dice.0 == self.dice.1 && self.doubles > 0 {
            self.phase = Phase::Roll;
        } else {
            self.end_turn();
        }
    }

    fn land(&mut self) {
        let s = self.seat;
        let sq = self.pos[s];
        match BOARD[sq].0 {
            Sq::Dest(_, p) => match self.owner[sq] {
                None => {
                    if self.cash[s] >= p {
                        self.phase = Phase::Buy(sq);
                        return;
                    }
                }
                Some(o) if o as usize != s => {
                    let t = self.stay_tax(sq);
                    self.log
                        .push_str(&format!(" taxă de cazare {} către J{}.", t, o as usize + 1));
                    self.pay(s, t, Some(o as usize));
                }
                _ => {}
            },
            Sq::Tax => {
                let t = (self.cash[s] / 10).clamp(30, 120);
                self.log.push_str(&format!(" taxa de turist: {t}."));
                self.pay(s, t, None);
            }
            Sq::Ferry => {
                self.log.push_str(" bacul (15) te duce 3 câmpuri înainte.");
                if self.pay(s, FERRY, None) {
                    self.move_forward(s, 3);
                    self.land_simple();
                    if self.phase != Phase::Step {
                        return;
                    }
                }
            }
            Sq::Lottery => {
                self.lottery();
                if matches!(self.phase, Phase::Buy(_)) {
                    return;
                }
            }
            Sq::Luxury => {
                let upg =
                    (0..SQ).any(|q| self.owner[q] == Some(s as u8) && self.lvl[q] < MAX_LEVEL);
                if upg && self.cash[s] >= LUXURY {
                    self.phase = Phase::Lux;
                    return;
                }
            }
            Sq::AirportN | Sq::AirportS | Sq::Customs => {}
        }
        self.after_actions();
    }

    /// Rezolvare după mutarea cu bacul (fără lanțuri: bacul duce pe o destinație).
    fn land_simple(&mut self) {
        let s = self.seat;
        let sq = self.pos[s];
        self.phase = Phase::Step;
        if let Sq::Dest(_, p) = BOARD[sq].0 {
            match self.owner[sq] {
                None => {
                    if self.cash[s] >= p {
                        self.phase = Phase::Buy(sq);
                    }
                }
                Some(o) if o as usize != s => {
                    let t = self.stay_tax(sq);
                    self.pay(s, t, Some(o as usize));
                }
                _ => {}
            }
        }
    }

    fn lottery(&mut self) {
        let s = self.seat;
        let k = self.rng.below(8);
        match k {
            0 => {
                self.cash[s] += 100;
                self.log.push_str(" tombolă: premiu +100.");
            }
            1 => {
                self.cash[s] += 50;
                self.log.push_str(" tombolă: cupon de călătorie +50.");
            }
            2 => {
                self.log.push_str(" tombolă: reparații −60.");
                self.pay(s, 60, None);
            }
            3 => {
                self.log.push_str(" tombolă: asigurare −40.");
                self.pay(s, 40, None);
            }
            4 => {
                self.log
                    .push_str(" tombolă: zbor gratuit până la următorul aeroport.");
                let next = (1..=SQ)
                    .find(|d| AIRPORTS.contains(&((self.pos[s] + d) % SQ)))
                    .unwrap();
                self.move_forward(s, next);
            }
            5 => {
                self.log
                    .push_str(" tombolă: festival, încasezi 25 de la fiecare.");
                for o in 0..self.n {
                    if o != s && self.alive[o] && self.alive[s] {
                        self.pay(o, 25, Some(s));
                    }
                }
            }
            6 => {
                self.log
                    .push_str(" tombolă: cină pentru toți, plătești 25 fiecăruia.");
                for o in 0..self.n {
                    if o != s && self.alive[o] && self.alive[s] {
                        self.pay(s, 25, Some(o));
                    }
                }
            }
            _ => {
                self.log.push_str(" tombolă: ocol, te întorci 3 câmpuri.");
                self.pos[s] = (self.pos[s] + SQ - 3) % SQ;
                let sq = self.pos[s];
                if let Sq::Dest(_, p) = BOARD[sq].0 {
                    if self.owner[sq].is_none() && self.cash[s] >= p {
                        self.phase = Phase::Buy(sq);
                    } else if let Some(o) = self.owner[sq] {
                        if o as usize != s {
                            let t = self.stay_tax(sq);
                            self.pay(s, t, Some(o as usize));
                        }
                    }
                }
            }
        }
    }
}

impl Game for Itinerar {
    type Move = Move;

    fn to_move(&self) -> Player {
        player_of(self.seat)
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Roll => vec![Move::Roll],
            Phase::Step => vec![Move::Step],
            Phase::Buy(_) => vec![Move::ReserveHere, Move::Decline],
            Phase::Lux => vec![Move::Luxury, Move::Decline],
            Phase::Manage => self.manage_moves(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.plies += 1;
        let s = self.seat;
        self.log.clear();
        match *m {
            Move::Roll => {
                let d1 = self.rng.d6();
                let d2 = self.rng.d6();
                self.dice = (d1, d2);
                if d1 == d2 {
                    self.doubles += 1;
                } else {
                    self.doubles = 0;
                }
                self.log = format!("J{} aruncă {}+{}.", s + 1, d1, d2);
                self.phase = Phase::Step;
            }
            Move::Step => {
                if self.doubles >= 3 {
                    self.pos[s] = CUSTOMS_POS;
                    self.skip[s] = 1;
                    self.log = format!("J{} a dat trei duble: vamă, pierde tura următoare.", s + 1);
                    self.doubles = 0;
                    self.end_turn();
                } else {
                    let st = (self.dice.0 + self.dice.1) as usize;
                    self.move_forward(s, st);
                    self.log = format!(
                        "J{} aruncă {}+{} și ajunge la {}.",
                        s + 1,
                        self.dice.0,
                        self.dice.1,
                        BOARD[self.pos[s]].1
                    );
                    self.land();
                }
            }
            Move::ReserveHere => {
                if let Phase::Buy(q) = self.phase {
                    self.cash[s] -= price(q);
                    self.owner[q] = Some(s as u8);
                    self.log = format!("J{} rezervă {}.", s + 1, BOARD[q].1);
                }
                self.after_actions();
            }
            Move::Decline => {
                self.after_actions();
            }
            Move::Luxury => {
                self.cash[s] -= LUXURY;
                let q = (0..SQ)
                    .filter(|&q| self.owner[q] == Some(s as u8) && self.lvl[q] < MAX_LEVEL)
                    .max_by_key(|&q| (BUILD_COST[region(q).unwrap()], std::cmp::Reverse(q)))
                    .unwrap();
                self.lvl[q] += 1;
                self.log = format!("J{} primește un nivel gratuit la {}.", s + 1, BOARD[q].1);
                self.after_actions();
            }
            Move::ReserveAt(q) => {
                self.cash[s] -= price(q) * 3 / 2;
                self.owner[q] = Some(s as u8);
                self.remote_done = true;
                self.log = format!("J{} rezervă la distanță {}.", s + 1, BOARD[q].1);
                self.phase = Phase::Manage;
                if self.manage_moves().len() <= 1 {
                    self.finish_manage();
                }
            }
            Move::Build(q) => {
                self.cash[s] -= BUILD_COST[region(q).unwrap()];
                self.lvl[q] += 1;
                self.log = format!(
                    "J{} construiește la {} (nivel {}).",
                    s + 1,
                    BOARD[q].1,
                    self.lvl[q]
                );
                if self.manage_moves().len() <= 1 {
                    self.finish_manage();
                }
            }
            Move::Fly => {
                self.cash[s] -= FLIGHT;
                let to = if self.pos[s] == AIRPORTS[0] {
                    AIRPORTS[1]
                } else {
                    AIRPORTS[0]
                };
                let steps = (to + SQ - self.pos[s]) % SQ;
                self.move_forward(s, steps);
                self.log = format!("J{} zboară la {}.", s + 1, BOARD[to].1);
                self.doubles = 0;
                self.end_turn();
            }
            Move::Done => self.finish_manage(),
        }
        if self.winner.is_none() && self.plies >= PLY_CAP {
            self.finish_by_wealth();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        let lv = level();
        let seat_value = |s: usize| -> i32 {
            if !self.alive[s] {
                return -5000;
            }
            if lv == Level::Easy {
                // Easy: alegere aproape aleatoare, determinată de poziție (fără zaruri viitoare).
                let h = hash_mix(
                    (self.cash[s] as u64) << 20
                        ^ (self.pos[s] as u64) << 8
                        ^ self.plies as u64
                        ^ (s as u64) << 40
                        ^ (self.owner.iter().flatten().count() as u64) << 50,
                );
                return (h % 1000) as i32;
            }
            let mut v = self.cash[s];
            let opp = (0..self.n).filter(|&o| o != s && self.alive[o]).count() as i32;
            let left = (MAX_ROUNDS as i32 + 1 - self.round as i32).max(0);
            for q in 0..SQ {
                if self.owner[q] != Some(s as u8) {
                    continue;
                }
                let r = region(q).unwrap();
                v += price(q) + self.lvl[q] as i32 * BUILD_COST[r];
                if self.owns_region(s, r) {
                    v += price(q) / 2;
                }
                if lv >= Level::Hard {
                    // randament: taxă așteptată × șansa de aterizare pe rundă × runde rămase
                    v += self.stay_tax(q) * opp * left * 29 / 100 / 2;
                }
            }
            if lv >= Level::Hard {
                // risc: taxe așteptate în următoarele 12 câmpuri + pericol de faliment
                let mut risk = 0;
                let mut worst = 0;
                for d in 2..=12usize {
                    let q = (self.pos[s] + d) % SQ;
                    if let Some(o) = self.owner[q] {
                        if o as usize != s {
                            let t = self.stay_tax(q);
                            risk += t * p2d6(d) / 36;
                            worst = worst.max(t);
                        }
                    }
                }
                v -= risk * 2;
                if self.cash[s] < worst {
                    v -= (worst - self.cash[s]) * 2;
                }
            } else if self.cash[s] < 100 {
                v -= 100 - self.cash[s];
            }
            v
        };
        let mut total = 0;
        for s in 0..self.n {
            let v = seat_value(s);
            if player_of(s) == p {
                total += v;
            } else {
                total -= v;
            }
        }
        total
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Itinerar — runda {}/{} (tabla: câmp, nume, proprietar și niveluri)\n",
            self.round.min(MAX_ROUNDS),
            MAX_ROUNDS
        );
        const MARK: [&str; 4] = ["①", "②", "③", "④"];
        for row in 0..7 {
            let mut line = String::new();
            for col in 0..4 {
                let i = col * 7 + row;
                let mut cell = format!("{:>2} {}", i, BOARD[i].1);
                if let Some(o) = self.owner[i] {
                    cell.push_str(&format!(" [J{}", o + 1));
                    if self.lvl[i] > 0 {
                        cell.push_str(&format!(" n{}", self.lvl[i]));
                    }
                    cell.push(']');
                }
                for k in 0..self.n {
                    if self.alive[k] && self.pos[k] == i {
                        cell.push(' ');
                        cell.push_str(MARK[k]);
                    }
                }
                let pad = 38usize.saturating_sub(cell.chars().count());
                line.push_str(&cell);
                line.push_str(&" ".repeat(pad));
            }
            s.push_str(line.trim_end());
            s.push('\n');
        }
        for k in 0..self.n {
            if self.alive[k] {
                s.push_str(&format!(
                    "J{}: {} lei (avere {}){}\n",
                    k + 1,
                    self.cash[k],
                    self.net_worth(k),
                    if self.skip[k] > 0 {
                        ", reținut la vamă"
                    } else {
                        ""
                    }
                ));
            } else {
                s.push_str(&format!("J{}: eliminat\n", k + 1));
            }
        }
        for r in 0..7 {
            let sq = region_squares(r);
            let owners: Vec<Option<u8>> = sq.iter().map(|&q| self.owner[q]).collect();
            if owners[0].is_some() && owners.iter().all(|o| *o == owners[0]) {
                s.push_str(&format!(
                    "Colecție completă: {} (J{}), taxe ×2\n",
                    REGION_NAMES[r],
                    owners[0].unwrap() + 1
                ));
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!(
                "Ultima acțiune:{}{}\n",
                if self.log.starts_with(' ') { "" } else { " " },
                self.log
            ));
        }
        if self.winner.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let who = format!("J{}", self.seat + 1);
        match self.phase {
            Phase::Roll => s.push_str(&format!("{who}: arunc\n")),
            Phase::Step => s.push_str(&format!(
                "{who} a aruncat {}+{}: mut\n",
                self.dice.0, self.dice.1
            )),
            Phase::Buy(q) => s.push_str(&format!(
                "{who} poate rezerva {} ({}, regiunea {}) cu {} lei: rezerv / las\n",
                BOARD[q].1,
                q,
                REGION_NAMES[region(q).unwrap()],
                price(q)
            )),
            Phase::Lux => s.push_str(&format!(
                "{who} la Hotel de lux: lux ({LUXURY} lei pentru un nivel gratuit) / las\n"
            )),
            Phase::Manage => s.push_str(&format!(
                "{who}: construiesc N / rezerv N (preț ×1,5) / zbor / gata\n"
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Step => "mut".into(),
            Move::ReserveHere => "rezerv".into(),
            Move::ReserveAt(q) => format!("rezerv {q}"),
            Move::Decline => "las".into(),
            Move::Build(q) => format!("construiesc {q}"),
            Move::Fly => "zbor".into(),
            Move::Luxury => "lux".into(),
            Move::Done => "gata".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::ai;

    fn play_random(mut g: Itinerar, seed: u64) -> Itinerar {
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            let m = mv[rng.below(mv.len())];
            g.apply(&m);
            n += 1;
            assert!(n < 20_000);
        }
        g
    }

    fn at_manage(g: &mut Itinerar) {
        g.phase = Phase::Manage;
    }

    #[test]
    fn board_is_consistent() {
        assert_eq!(BOARD.len(), 28);
        for r in 0..7 {
            assert_eq!(region_squares(r).len(), 3, "regiunea {r}");
        }
        let specials = BOARD.iter().filter(|b| region_is_none(b.0)).count();
        assert_eq!(specials, 7);
        assert_eq!(BOARD[AIRPORTS[0]].0, Sq::AirportN);
        assert_eq!(BOARD[AIRPORTS[1]].0, Sq::AirportS);
        assert_eq!(BOARD[CUSTOMS_POS].0, Sq::Customs);
    }

    fn region_is_none(s: Sq) -> bool {
        !matches!(s, Sq::Dest(..))
    }

    #[test]
    fn reserve_and_pay_tax() {
        let mut g = Itinerar::with_seed(1);
        g.pos[0] = 1;
        g.phase = Phase::Step;
        g.dice = (1, 2);
        g.pos[0] = 0;
        g.apply(&Move::Step); // 3 pași: 0 -> 3
        assert_eq!(g.pos[0], 3);
        assert_eq!(g.legal_moves(), vec![Move::ReserveHere, Move::Decline]);
        g.apply(&Move::ReserveHere);
        assert_eq!(g.owner[3], Some(0));
        assert_eq!(g.cash[0], START_CASH - 70);
        // J2 aterizează pe 3
        g.seat = 1;
        g.phase = Phase::Step;
        g.dice = (1, 2);
        g.pos[1] = 0;
        let before = g.cash[1];
        g.apply(&Move::Step);
        assert_eq!(before - g.cash[1], 70 / 5);
        assert_eq!(g.cash[0], START_CASH - 70 + 14);
    }

    #[test]
    fn complete_region_doubles_and_levels_scale() {
        let mut g = Itinerar::with_seed(2);
        g.owner[1] = Some(0);
        g.owner[2] = Some(0);
        assert_eq!(g.stay_tax(1), 12);
        g.owner[3] = Some(0);
        assert_eq!(g.stay_tax(1), 24);
        g.lvl[1] = 3;
        assert_eq!(g.stay_tax(1), 12 * 8 * 2);
        g.owner[3] = Some(1);
        assert_eq!(g.stay_tax(1), 12 * 8);
    }

    #[test]
    fn build_caps_at_three_levels_and_costs_by_region() {
        let mut g = Itinerar::with_seed(3);
        g.owner[5] = Some(0);
        at_manage(&mut g);
        for k in 1..=3 {
            assert!(g.legal_moves().contains(&Move::Build(5)));
            let c = g.cash[0];
            g.apply(&Move::Build(5));
            assert_eq!(c - g.cash[0], 50);
            assert_eq!(g.lvl[5], k);
            g.phase = Phase::Manage;
        }
        assert!(!g.legal_moves().contains(&Move::Build(5)));
    }

    #[test]
    fn passing_airport_pays_salary() {
        let mut g = Itinerar::with_seed(4);
        g.pos[0] = 26;
        g.phase = Phase::Step;
        g.dice = (2, 3);
        let c = g.cash[0];
        g.apply(&Move::Step); // 26 -> 3 (propriu: 31 mod 28)
        assert_eq!(g.pos[0], 3);
        assert!(g.cash[0] >= c + SALARY);
    }

    #[test]
    fn three_doubles_go_to_customs() {
        let mut g = Itinerar::with_seed(5);
        g.doubles = 2;
        g.phase = Phase::Roll;
        // forțează dublă
        loop {
            let mut c = g.clone();
            c.apply(&Move::Roll);
            if c.dice.0 == c.dice.1 {
                g = c;
                break;
            }
            g.rng.next_u64();
        }
        assert_eq!(g.doubles, 3);
        g.apply(&Move::Step);
        assert_eq!(g.pos[0], CUSTOMS_POS);
        assert_eq!(g.skip[0], 1);
        assert_eq!(g.seat, 1);
        // J2 își termină tura; J1 e sărit o dată
        g.apply(&Move::Roll);
        g.apply(&Move::Step);
        while g.seat == 1 && g.winner.is_none() {
            let m = *g.legal_moves().last().unwrap();
            g.apply(&m);
        }
        assert_eq!(g.skip[0], 0);
    }

    #[test]
    fn airport_flight_to_other_airport() {
        let mut g = Itinerar::with_seed(6);
        g.pos[0] = 16;
        g.phase = Phase::Manage;
        g.doubles = 0;
        assert!(g.legal_moves().contains(&Move::Fly));
        let c = g.cash[0];
        g.apply(&Move::Fly);
        assert_eq!(g.pos[0], 0);
        assert_eq!(g.cash[0], c - FLIGHT + SALARY);
        assert_eq!(g.seat, 1);
        g.pos[1] = 5;
        g.seat = 1;
        g.phase = Phase::Manage;
        assert!(!g.legal_moves().contains(&Move::Fly));
    }

    #[test]
    fn remote_reservation_once_per_turn_at_premium() {
        let mut g = Itinerar::with_seed(7);
        g.phase = Phase::Manage;
        g.apply(&Move::ReserveAt(1));
        assert_eq!(g.cash[0], START_CASH - 90);
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::ReserveAt(_))));
    }

    #[test]
    fn bankruptcy_eliminates_and_ends_two_player_game() {
        let mut g = Itinerar::with_seed(8);
        g.owner[26] = Some(1);
        g.owner[25] = Some(1);
        g.owner[27] = Some(1);
        g.lvl[26] = 3;
        g.cash[0] = 10;
        g.pos[0] = 24;
        g.phase = Phase::Step;
        g.dice = (1, 1);
        g.apply(&Move::Step);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert!(!g.alive[0]);
        assert_eq!(g.owner_of(26), Some(1));
    }

    #[test]
    fn bankrupt_properties_return_to_bank_in_multiplayer() {
        let mut g = Itinerar::with_players(3, 9);
        g.owner[1] = Some(2);
        g.owner[2] = Some(2);
        g.owner[3] = Some(2);
        g.lvl[3] = 3;
        g.cash[2] = 0;
        g.eliminate(2);
        assert_eq!(g.owner[1], None);
        assert_eq!(g.lvl[3], 0);
        assert_eq!(g.alive_count(), 2);
        assert!(g.outcome().is_none());
    }

    #[test]
    fn round_cap_gives_richest_the_win() {
        let mut g = Itinerar::with_seed(10);
        g.round = MAX_ROUNDS;
        g.seat = 1;
        g.cash[0] = 2000;
        g.phase = Phase::Manage;
        g.pos[1] = 5;
        g.apply(&Move::Done);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut h = Itinerar::with_seed(10);
        h.round = MAX_ROUNDS;
        h.seat = 1;
        h.phase = Phase::Manage;
        h.apply(&Move::Done);
        assert_eq!(h.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn luxury_hotel_upgrades_a_unit() {
        let mut g = Itinerar::with_seed(11);
        g.owner[13] = Some(0);
        g.pos[0] = 17;
        g.phase = Phase::Step;
        g.dice = (1, 2);
        g.apply(&Move::Step);
        assert_eq!(g.pos[0], 20);
        assert_eq!(g.legal_moves(), vec![Move::Luxury, Move::Decline]);
        g.apply(&Move::Luxury);
        assert_eq!(g.lvl[13], 1);
    }

    #[test]
    fn move_texts_roundtrip_via_gameobj() {
        use polytabletop_core::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(Itinerar::with_seed(12));
        assert_eq!(g.legal_moves(), vec!["arunc".to_string()]);
        assert!(g.play("ARUNC").is_ok());
        assert_eq!(g.legal_moves(), vec!["mut".to_string()]);
        assert!(g.play("zbor").is_err());
    }

    #[test]
    fn deterministic_with_seed() {
        let a = play_random(Itinerar::with_seed(77), 5);
        let b = play_random(Itinerar::with_seed(77), 5);
        assert_eq!(a.render(), b.render());
        assert_eq!(a.outcome(), b.outcome());
    }

    #[test]
    fn random_games_end_for_2_3_4_players() {
        for n in 2..=4 {
            for seed in 1..=25u64 {
                let g = play_random(Itinerar::with_players(n, seed), seed ^ 9);
                assert!(g.outcome().is_some());
                let tot_owned = g.owner.iter().flatten().count();
                assert!(tot_owned <= 21);
            }
        }
    }

    #[test]
    fn money_is_never_negative() {
        let mut g = Itinerar::with_players(3, 31);
        let mut rng = Rng::new(3);
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            let m = mv[rng.below(mv.len())];
            g.apply(&m);
            for s in 0..3 {
                assert!(g.cash[s] >= 0);
            }
        }
    }

    #[test]
    fn ai_does_not_see_future_dice() {
        // alegerea IA-ului nu depinde de seminția zarurilor: aceeași poziție publică,
        // alt rng intern -> aceeași mutare la Normal.
        let mut a = Itinerar::with_seed(100);
        let mut b = Itinerar::with_seed(100);
        b.rng = Rng::new(999_999);
        for _ in 0..30 {
            if a.outcome().is_some() {
                break;
            }
            // aduce ambele la aceeași poziție publică: aplică aceleași mutări, dar zarurile diferă
            let ma = ai::best_move(&a, 1).unwrap();
            let mb = ai::best_move(&b, 1).unwrap();
            if matches!(a.phase, Phase::Manage | Phase::Roll) && a.phase == b.phase {
                assert_eq!(ma, mb);
            }
            a.apply(&ma);
            b.apply(&mb);
            b.pos = a.pos;
            b.cash = a.cash;
            b.owner = a.owner;
            b.lvl = a.lvl;
            b.alive = a.alive;
            b.skip = a.skip;
            b.seat = a.seat;
            b.phase = a.phase;
            b.doubles = a.doubles;
            b.dice = a.dice;
            b.round = a.round;
            b.winner = a.winner;
        }
    }
}
