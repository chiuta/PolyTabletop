//! Codrul Nesupus — război asimetric într-o pădure cu poieni și drumuri (variantă originală), 2 jucători.
//!
//! Două facțiuni cu reguli complet diferite (la pornire se atribuie la sorți; J1 = om):
//! * **Regența** (ocupantul): gărzi, trei feluri de clădiri (Cherestea, Atelier, Cazarmă), lemn, fabricat.
//! * **Frunzișul** (insurgența): luptători, simpatie, suporteri (cărți ascunse) și baze (câte una pe fel de poiană).
//!
//! Harta: 8 poieni (fiecare cu un fel — Stejar, Ferigă, Ciupercă — și 2 locuri de clădit) legate prin drumuri.
//! Cărți: 24 (3 feluri × 7 + 3 Păsări joker). Mâna maximă la final de tură: 5.
//!
//! **Tura Regenței**: `trag`; 3 acțiuni (`mars A>B n`, `lupta P`, `recruteaza`, `construieste c|a|k P` — costă 1 lemn,
//! lemnul/tură = nr. de Cherestele —, poiana trebuie *stăpânită*: mai multe piese ale Regenței decât ale Frunzișului);
//! apoi seara `fabrica P` (card de felul poienii sau Pasăre pe fiecare Atelier: +1 p); `gata`.
//! Construirea a n‑a clădire de un fel dă [0,1,2,3] p.
//! **Tura Frunzișului**: `trag`; 3 + (nr. de baze) acțiuni: `mobilizeaza fel` (card din mână → suporteri),
//! `propaga P` (simpatie: 1 suporter de felul poienii, 2 dacă acolo sunt ≥3 gărzi; adiacentă simpatiei existente),
//! `revolta P` (2 suporteri, poiană cu simpatie și fără bază de acel fel: bază + luptători, distruge clădirile Regenței),
//! `recruteaza` (1 luptător pe bază), `mars`, `lupta`. Simpatia dă 1 p (2 p după a patra).
//!
//! **Lupta**: apărătorul poate juca o ambuscadă (`ambuscada`: card de felul poienii sau Pasăre; atacatorul pierde 2
//! luptători; atacatorul o poate anula cu `contra`), apoi mutarea forțată `zar`: două zaruri, înalt/jos. Luptătorii
//! Frunzișului apărând („gherilă”) lovesc cu zarul înalt. Apărătorul fără luptători: +1 lovitură pentru atacator.
//! Loviturile scot întâi războinici, apoi clădiri/simpatii/baze (+1 p pentru cel care le-a scos).
//! Câștigă primul la 18 p; după 30 de ture câștigă cine are mai multe puncte.
//!
//! # AI și informație
//! Câte un `evaluate` separat pe facțiune. Se citesc doar piesele de pe masă, propria mână și numărul de cărți al
//! adversarului. Tragerea și zarurile sunt mutări forțate separate; AI-ul caută 1 ply.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const NP: usize = 8;
pub const NAMES: [&str; NP] = [
    "Stejari", "Ferigi", "Ciuperci", "Izvor", "Cenusa", "Maracini", "Hat", "Tufis",
];
pub const SUIT: [u8; NP] = [0, 1, 2, 0, 1, 2, 0, 1];
pub const SUIT_NAMES: [&str; 4] = ["stejar", "ferig", "ciuperca", "pasare"];
pub const EDGES: [(usize, usize); 11] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 0),
    (1, 5),
    (3, 7),
    (2, 6),
];
pub const TOTAL_GUARDS: u8 = 14;
pub const TOTAL_FIGHTERS: u8 = 12;
pub const TOTAL_SYM: u8 = 8;
pub const BLD_MAX: u8 = 4;
pub const WIN_VP: u8 = 18;
pub const TURN_CAP: u32 = 30;
pub const DECK_SIZE: usize = 24;
const TRACK: [u8; 4] = [0, 1, 2, 3];
const BLD_NAMES: [&str; 3] = ["c", "a", "k"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fac {
    Reg,
    Ins,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Act,
    Craft,
    Discard,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stage {
    DefReact,
    AttReact,
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Bat {
    at: usize,
    att: Player,
    stage: Stage,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    March(usize, usize, u8),
    Battle(usize),
    Recruit,
    Build(usize, usize),
    Craft(usize),
    Done,
    Discard(usize),
    Mobilize(usize),
    Spread(usize),
    Revolt(usize),
    Ambush,
    NoAmbush,
    Counter,
    NoCounter,
    Roll,
}

#[derive(Clone)]
pub struct Codru {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    fac: [Fac; 2],
    guards: [u8; NP],
    fighters: [u8; NP],
    bld: [[u8; 3]; NP],
    base: [Option<usize>; 3],
    sym: [bool; NP],
    hand: [[u8; 4]; 2],
    sup: [u8; 4],
    deck: Vec<usize>,
    discard: Vec<usize>,
    vp: [u8; 2],
    side: Player,
    phase: Phase,
    left: u8,
    wood: u8,
    recruited: bool,
    crafted: [u8; NP],
    bat: Option<Bat>,
    turns: u32,
    rng: Rng,
    log: String,
}

impl Default for Codru {
    fn default() -> Self {
        Self::new()
    }
}

fn adjacent(a: usize, b: usize) -> bool {
    EDGES
        .iter()
        .any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
}

fn neighbors(a: usize) -> impl Iterator<Item = usize> {
    (0..NP).filter(move |&b| adjacent(a, b))
}

fn dist_table() -> [[u8; NP]; NP] {
    let mut d = [[99u8; NP]; NP];
    for (i, row) in d.iter_mut().enumerate() {
        row[i] = 0;
    }
    for &(a, b) in EDGES.iter() {
        d[a][b] = 1;
        d[b][a] = 1;
    }
    for k in 0..NP {
        for i in 0..NP {
            for j in 0..NP {
                let v = d[i][k] + d[k][j];
                if v < d[i][j] {
                    d[i][j] = v;
                }
            }
        }
    }
    d
}

impl Codru {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    /// Facțiunile se atribuie după seed (par: J1 = Regența).
    pub fn with_seed(seed: u64) -> Self {
        let swapped = (Rng::new(seed).next_u64() >> 20) & 1 == 1;
        Self::with_factions(seed, swapped)
    }

    pub fn with_factions(seed: u64, swapped: bool) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<usize> = Vec::new();
        for s in 0..3 {
            for _ in 0..7 {
                deck.push(s);
            }
        }
        deck.extend([3usize; 3]);
        for i in (1..deck.len()).rev() {
            let j = rng.below(i + 1);
            deck.swap(i, j);
        }
        let fac = if swapped {
            [Fac::Ins, Fac::Reg]
        } else {
            [Fac::Reg, Fac::Ins]
        };
        let mut g = Codru {
            viewer: None,
            fac,
            guards: [0; NP],
            fighters: [0; NP],
            bld: [[0; 3]; NP],
            base: [None; 3],
            sym: [false; NP],
            hand: [[0; 4]; 2],
            sup: [0; 4],
            deck,
            discard: Vec::new(),
            vp: [0; 2],
            side: Player::One,
            phase: Phase::Draw,
            left: 0,
            wood: 0,
            recruited: false,
            crafted: [0; NP],
            bat: None,
            turns: 0,
            rng,
            log: String::new(),
        };
        g.guards[0] = 4;
        g.guards[1] = 2;
        g.bld[0][0] = 1;
        g.bld[0][2] = 1;
        g.fighters[4] = 3;
        g.sym[5] = true;
        for _ in 0..3 {
            for p in 0..2 {
                let c = g.deck.pop().unwrap();
                g.hand[p][c] += 1;
            }
        }
        for _ in 0..2 {
            let c = g.deck.pop().unwrap();
            g.sup[c] += 1;
        }
        // Frunzișul pornește cu cărțile de pe masă în „suporteri”; Regența începe.
        g.side = g.player_of(Fac::Reg);
        g
    }

    pub fn player_of(&self, f: Fac) -> Player {
        if self.fac[0] == f {
            Player::One
        } else {
            Player::Two
        }
    }

    pub fn faction(&self, p: Player) -> Fac {
        self.fac[p.idx()]
    }

    fn war(&self, f: Fac, c: usize) -> u8 {
        match f {
            Fac::Reg => self.guards[c],
            Fac::Ins => self.fighters[c],
        }
    }

    fn war_total(&self, f: Fac) -> u8 {
        (0..NP).map(|c| self.war(f, c)).sum()
    }

    fn bases_on_map(&self) -> u8 {
        self.base.iter().filter(|b| b.is_some()).count() as u8
    }

    fn sym_on_map(&self) -> u8 {
        self.sym.iter().filter(|&&s| s).count() as u8
    }

    fn bld_on_map(&self, t: usize) -> u8 {
        (0..NP).map(|c| self.bld[c][t]).sum()
    }

    fn base_at(&self, c: usize) -> bool {
        self.base.contains(&Some(c))
    }

    fn pieces(&self, f: Fac, c: usize) -> u8 {
        match f {
            Fac::Reg => self.guards[c] + self.bld[c].iter().sum::<u8>(),
            Fac::Ins => self.fighters[c] + self.base_at(c) as u8 + self.sym[c] as u8,
        }
    }

    pub fn rules(&self, c: usize) -> bool {
        self.pieces(Fac::Reg, c) > self.pieces(Fac::Ins, c)
    }

    fn slots_used(&self, c: usize) -> u8 {
        self.bld[c].iter().sum::<u8>() + self.base_at(c) as u8
    }

    fn hand_total(&self, p: Player) -> u8 {
        self.hand[p.idx()].iter().sum()
    }

    pub fn vp(&self, p: Player) -> u8 {
        self.vp[p.idx()]
    }

    fn card_for(&self, p: Player, c: usize) -> Option<usize> {
        let s = SUIT[c] as usize;
        let h = &self.hand[p.idx()];
        if h[s] > 0 {
            Some(s)
        } else if h[3] > 0 {
            Some(3)
        } else {
            None
        }
    }

    fn discard_card(&mut self, p: Player, k: usize) {
        self.hand[p.idx()][k] -= 1;
        self.discard.push(k);
    }

    fn sup_for(&self, c: usize) -> u8 {
        self.sup[SUIT[c] as usize] + self.sup[3]
    }

    fn pay_sup(&mut self, c: usize, n: u8) {
        for _ in 0..n {
            let s = SUIT[c] as usize;
            let k = if self.sup[s] > 0 { s } else { 3 };
            self.sup[k] -= 1;
            self.discard.push(k);
        }
    }

    /// Scoate `hits` piese ale lui `f` din poiana `c`: întâi războinici, apoi piese. Întoarce punctele pentru adversar.
    fn remove_hits(&mut self, f: Fac, c: usize, hits: u8) -> u8 {
        let mut h = hits;
        let mut pts = 0;
        match f {
            Fac::Reg => {
                let k = h.min(self.guards[c]);
                self.guards[c] -= k;
                h -= k;
                for t in 0..3 {
                    while h > 0 && self.bld[c][t] > 0 {
                        self.bld[c][t] -= 1;
                        h -= 1;
                        pts += 1;
                    }
                }
            }
            Fac::Ins => {
                let k = h.min(self.fighters[c]);
                self.fighters[c] -= k;
                h -= k;
                if h > 0 && self.sym[c] {
                    self.sym[c] = false;
                    h -= 1;
                    pts += 1;
                }
                if h > 0 && self.base_at(c) {
                    for b in self.base.iter_mut() {
                        if *b == Some(c) {
                            *b = None;
                        }
                    }
                    pts += 1;
                }
            }
        }
        pts
    }

    fn draw_card(&mut self, p: Player) {
        if self.deck.is_empty() {
            let mut d = std::mem::take(&mut self.discard);
            for i in (1..d.len()).rev() {
                let j = self.rng.below(i + 1);
                d.swap(i, j);
            }
            self.deck = d;
        }
        if let Some(c) = self.deck.pop() {
            self.hand[p.idx()][c] += 1;
        }
    }

    fn actions_budget(&self, f: Fac) -> u8 {
        match f {
            Fac::Reg => 3,
            Fac::Ins => 3 + self.bases_on_map(),
        }
    }

    fn finish_phase(&mut self) {
        // după ultima acțiune
        let f = self.faction(self.side);
        if self.phase == Phase::Act && self.left == 0 {
            if f == Fac::Reg {
                self.phase = Phase::Craft;
                return;
            }
            self.end_turn_checks();
        }
    }

    fn end_turn_checks(&mut self) {
        if self.hand_total(self.side) > 5 {
            self.phase = Phase::Discard;
        } else {
            self.next_turn();
        }
    }

    fn next_turn(&mut self) {
        self.turns += 1;
        self.side = self.side.other();
        self.phase = Phase::Draw;
        self.recruited = false;
        self.crafted = [0; NP];
        self.bat = None;
    }

    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + self.hand[0].iter().map(|&x| x as usize).sum::<usize>()
            + self.hand[1].iter().map(|&x| x as usize).sum::<usize>()
            + self.sup.iter().map(|&x| x as usize).sum::<usize>()
    }

    fn start_battle(&mut self, at: usize) {
        let att = self.side;
        let def = att.other();
        let stage = if self.card_for(def, at).is_some() {
            Stage::DefReact
        } else {
            Stage::Roll
        };
        self.bat = Some(Bat { at, att, stage });
    }

    fn ambush_hits(&mut self, b: Bat) {
        let af = self.faction(b.att);
        let k = 2.min(self.war(af, b.at));
        match af {
            Fac::Reg => self.guards[b.at] -= k,
            Fac::Ins => self.fighters[b.at] -= k,
        }
        if self.war(af, b.at) == 0 {
            self.bat = None;
            self.log.push_str(" Atacatorul rămâne fără luptători.");
        } else {
            self.bat = Some(Bat {
                stage: Stage::Roll,
                ..b
            });
        }
    }

    fn opp_hidden_safe_eval(&self, f: Fac, me: Player) -> i32 {
        let o = me.other();
        let vp = self.vp[me.idx()] as i32;
        let dist = dist_table();
        match f {
            Fac::Reg => {
                let mut v = 90 * vp;
                let mut towns = 0;
                for c in 0..NP {
                    v += 5 * self.guards[c] as i32;
                    let b: i32 = self.bld[c].iter().map(|&x| x as i32).sum();
                    v += 9 * b;
                    if self.rules(c) {
                        towns += 1;
                    }
                    // apropierea de piese ale Frunzișului
                    if self.guards[c] > 0 {
                        let mut best = 9i32;
                        for e in 0..NP {
                            if self.pieces(Fac::Ins, e) > 0 {
                                best = best.min(dist[c][e] as i32);
                            }
                        }
                        v -= self.guards[c] as i32 * best.min(4);
                    }
                }
                v += 3 * towns;
                v += 2 * self.bld_on_map(0) as i32;
                v += 2 * self.hand_total(me) as i32;
                // amenințarea Frunzișului
                v -= 6 * self.sym_on_map() as i32;
                v -= 12 * self.bases_on_map() as i32;
                v -= 3 * self.war_total(Fac::Ins) as i32;
                v -= 2 * self.sup.iter().map(|&x| x as i32).sum::<i32>();
                v
            }
            Fac::Ins => {
                let mut v = 90 * vp;
                for c in 0..NP {
                    v += 4 * self.fighters[c] as i32;
                    if self.sym[c] {
                        v += 12;
                    }
                    if self.base_at(c) {
                        v += 16;
                    }
                    if self.fighters[c] > 0 && self.pieces(Fac::Reg, c) > 0 {
                        v += 2;
                    }
                }
                v += 5 * self.sup.iter().map(|&x| x as i32).sum::<i32>();
                v += 2 * self.hand_total(me) as i32;
                v -= 2 * self.war_total(Fac::Reg) as i32;
                v -= 6 * (0..3).map(|t| self.bld_on_map(t) as i32).sum::<i32>();
                let _ = o;
                v
            }
        }
    }
}

impl Game for Codru {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        if let Some(b) = self.bat {
            if b.stage == Stage::DefReact {
                return b.att.other();
            }
        }
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if let Some(b) = self.bat {
            return match b.stage {
                Stage::DefReact => vec![Move::Ambush, Move::NoAmbush],
                Stage::AttReact => vec![Move::Counter, Move::NoCounter],
                Stage::Roll => vec![Move::Roll],
            };
        }
        let me = self.side;
        let f = self.faction(me);
        let mut v = Vec::new();
        match self.phase {
            Phase::Draw => v.push(Move::Draw),
            Phase::Discard => {
                for k in 0..4 {
                    if self.hand[me.idx()][k] > 0 {
                        v.push(Move::Discard(k));
                    }
                }
            }
            Phase::Craft => {
                for c in 0..NP {
                    if self.crafted[c] < self.bld[c][1] && self.card_for(me, c).is_some() {
                        v.push(Move::Craft(c));
                    }
                }
                v.push(Move::Done);
            }
            Phase::Act => {
                // marș
                for a in 0..NP {
                    let w = self.war(f, a);
                    if w == 0 {
                        continue;
                    }
                    for b in neighbors(a) {
                        if f == Fac::Reg && !(self.rules(a) || self.rules(b)) {
                            continue;
                        }
                        let mut ns = vec![1u8, w];
                        if w >= 4 {
                            ns.push(w / 2);
                        }
                        ns.sort_unstable();
                        ns.dedup();
                        for n in ns {
                            v.push(Move::March(a, b, n));
                        }
                    }
                    let of = if f == Fac::Reg { Fac::Ins } else { Fac::Reg };
                    if self.pieces(of, a) > 0 {
                        v.push(Move::Battle(a));
                    }
                }
                match f {
                    Fac::Reg => {
                        if !self.recruited
                            && self.war_total(Fac::Reg) < TOTAL_GUARDS
                            && self.bld_on_map(2) > 0
                        {
                            v.push(Move::Recruit);
                        }
                        if self.wood > 0 {
                            for t in 0..3 {
                                if self.bld_on_map(t) >= BLD_MAX {
                                    continue;
                                }
                                for c in 0..NP {
                                    if self.rules(c) && self.slots_used(c) < 2 {
                                        v.push(Move::Build(t, c));
                                    }
                                }
                            }
                        }
                    }
                    Fac::Ins => {
                        for k in 0..4 {
                            if self.hand[me.idx()][k] > 0 {
                                v.push(Move::Mobilize(k));
                            }
                        }
                        if !self.recruited
                            && self.bases_on_map() > 0
                            && self.war_total(Fac::Ins) < TOTAL_FIGHTERS
                        {
                            v.push(Move::Recruit);
                        }
                        let any_sym = self.sym_on_map() > 0;
                        for c in 0..NP {
                            if !self.sym[c] && self.sym_on_map() < TOTAL_SYM {
                                let ok = !any_sym || neighbors(c).any(|n| self.sym[n]);
                                let cost = if self.guards[c] >= 3 { 2 } else { 1 };
                                if ok && self.sup_for(c) >= cost {
                                    v.push(Move::Spread(c));
                                }
                            }
                            let s = SUIT[c] as usize;
                            if self.sym[c] && self.base[s].is_none() && self.sup_for(c) >= 2 {
                                v.push(Move::Revolt(c));
                            }
                        }
                    }
                }
                v.push(Move::Done);
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let f = self.faction(me);
        let tag = format!("J{}", me.idx() + 1);
        match *m {
            Move::Draw => {
                self.draw_card(me);
                self.phase = Phase::Act;
                self.left = self.actions_budget(f);
                self.wood = self.bld_on_map(0);
                self.log = format!("{tag} trage o carte.");
            }
            Move::March(a, b, n) => {
                match f {
                    Fac::Reg => {
                        self.guards[a] -= n;
                        self.guards[b] += n;
                    }
                    Fac::Ins => {
                        self.fighters[a] -= n;
                        self.fighters[b] += n;
                    }
                }
                self.left -= 1;
                self.log = format!("{tag} mută {n} din {} în {}.", NAMES[a], NAMES[b]);
                self.finish_phase();
            }
            Move::Battle(c) => {
                self.left -= 1;
                self.log = format!("{tag} atacă în {}.", NAMES[c]);
                self.start_battle(c);
            }
            Move::Recruit => {
                self.recruited = true;
                match f {
                    Fac::Reg => {
                        for c in 0..NP {
                            for _ in 0..self.bld[c][2] {
                                if self.war_total(Fac::Reg) < TOTAL_GUARDS {
                                    self.guards[c] += 1;
                                }
                            }
                        }
                    }
                    Fac::Ins => {
                        for s in 0..3 {
                            if let Some(c) = self.base[s] {
                                if self.war_total(Fac::Ins) < TOTAL_FIGHTERS {
                                    self.fighters[c] += 1;
                                }
                            }
                        }
                    }
                }
                self.left -= 1;
                self.log = format!("{tag} recrutează.");
                self.finish_phase();
            }
            Move::Build(t, c) => {
                let n = self.bld_on_map(t) as usize;
                self.vp[me.idx()] += TRACK[n];
                self.bld[c][t] += 1;
                self.wood -= 1;
                self.left -= 1;
                self.log = format!("{tag} construiește în {} (+{} p).", NAMES[c], TRACK[n]);
                self.finish_phase();
            }
            Move::Craft(c) => {
                let k = self.card_for(me, c).unwrap();
                self.discard_card(me, k);
                self.crafted[c] += 1;
                self.vp[me.idx()] += 1;
                self.log = format!("{tag} fabrică în {} (+1 p).", NAMES[c]);
            }
            Move::Done => {
                match self.phase {
                    Phase::Act if f == Fac::Reg => self.phase = Phase::Craft,
                    _ => self.end_turn_checks(),
                }
                self.log = format!("{tag} termină.");
            }
            Move::Discard(k) => {
                self.discard_card(me, k);
                if self.hand_total(me) <= 5 {
                    self.next_turn();
                }
            }
            Move::Mobilize(k) => {
                self.hand[me.idx()][k] -= 1;
                self.sup[k] += 1;
                self.left -= 1;
                self.log = format!("{tag} mobilizează un suporter.");
                self.finish_phase();
            }
            Move::Spread(c) => {
                let cost = if self.guards[c] >= 3 { 2 } else { 1 };
                self.pay_sup(c, cost);
                let n = self.sym_on_map();
                self.sym[c] = true;
                let pts = if n >= 4 { 2 } else { 1 };
                self.vp[me.idx()] += pts;
                self.left -= 1;
                self.log = format!("{tag} propagă simpatia în {} (+{pts} p).", NAMES[c]);
                self.finish_phase();
            }
            Move::Revolt(c) => {
                self.pay_sup(c, 2);
                let s = SUIT[c] as usize;
                let destroyed: u8 = self.bld[c].iter().sum();
                self.bld[c] = [0; 3];
                self.vp[me.idx()] += destroyed;
                self.base[s] = Some(c);
                let same = (0..NP)
                    .filter(|&e| self.sym[e] && SUIT[e] as usize == s)
                    .count() as u8;
                let room = TOTAL_FIGHTERS - self.war_total(Fac::Ins);
                self.fighters[c] += same.max(1).min(room);
                self.left -= 1;
                self.log = format!(
                    "{tag} ridică o revoltă în {} ({destroyed} clădiri).",
                    NAMES[c]
                );
                self.finish_phase();
            }
            Move::Ambush => {
                let b = self.bat.unwrap();
                let def = self.to_move();
                let k = self.card_for(def, b.at).unwrap();
                self.discard_card(def, k);
                self.log = format!("J{} întinde o ambuscadă!", def.idx() + 1);
                if self.card_for(b.att, b.at).is_some() {
                    self.bat = Some(Bat {
                        stage: Stage::AttReact,
                        ..b
                    });
                } else {
                    self.ambush_hits(b);
                    self.finish_after_battle();
                }
            }
            Move::NoAmbush => {
                let b = self.bat.unwrap();
                self.bat = Some(Bat {
                    stage: Stage::Roll,
                    ..b
                });
            }
            Move::Counter => {
                let b = self.bat.unwrap();
                let k = self.card_for(b.att, b.at).unwrap();
                self.discard_card(b.att, k);
                self.log.push_str(" Ambuscada e anulată.");
                self.bat = Some(Bat {
                    stage: Stage::Roll,
                    ..b
                });
            }
            Move::NoCounter => {
                let b = self.bat.unwrap();
                self.ambush_hits(b);
                self.finish_after_battle();
            }
            Move::Roll => {
                let b = self.bat.unwrap();
                let (d1, d2) = (self.rng.d6(), self.rng.d6());
                let (hi, lo) = (d1.max(d2), d1.min(d2));
                let af = self.faction(b.att);
                let df = self.faction(b.att.other());
                let (ar, dr) = if df == Fac::Ins { (lo, hi) } else { (hi, lo) };
                let aw = self.war(af, b.at);
                let dw = self.war(df, b.at);
                let mut ah = ar.min(aw);
                let dh = dr.min(dw);
                if dw == 0 {
                    ah += 1;
                }
                let pa = self.remove_hits(df, b.at, ah);
                let pd = self.remove_hits(af, b.at, dh);
                self.vp[b.att.idx()] += pa;
                self.vp[b.att.other().idx()] += pd;
                self.log = format!(
                    "Luptă în {}: zaruri {hi}/{lo} → atacatorul dă {ah}, apărătorul {dh} lovituri.",
                    NAMES[b.at]
                );
                self.bat = None;
                self.finish_after_battle();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.vp[0], self.vp[1]);
        if a >= WIN_VP && a > b {
            return Some(Outcome::Win(Player::One));
        }
        if b >= WIN_VP && b > a {
            return Some(Outcome::Win(Player::Two));
        }
        if a >= WIN_VP && b >= WIN_VP && a == b {
            return Some(Outcome::Draw);
        }
        if self.turns >= TURN_CAP {
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mut v = self.opp_hidden_safe_eval(self.faction(p), p)
            - self.opp_hidden_safe_eval(self.faction(o), o);
        // luptă în așteptare: estimare după numere (public)
        if let Some(b) = self.bat {
            let af = self.faction(b.att);
            let df = self.faction(b.att.other());
            let est = 4 * (self.war(af, b.at) as i32 - self.war(df, b.at) as i32);
            v += if b.att == p { est } else { -est };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("Codrul Nesupus\n");
        s.push_str(&format!(
            "J1 = {}, J2 = {}\n",
            if self.fac[0] == Fac::Reg {
                "Regența"
            } else {
                "Frunzișul"
            },
            if self.fac[1] == Fac::Reg {
                "Regența"
            } else {
                "Frunzișul"
            }
        ));
        for c in 0..NP {
            let mut line = format!("{} [{}]: ", NAMES[c], SUIT_NAMES[SUIT[c] as usize]);
            if self.guards[c] > 0 {
                line.push_str(&format!("gărzi {} ", self.guards[c]));
            }
            for t in 0..3 {
                for _ in 0..self.bld[c][t] {
                    line.push_str(&format!("[{}] ", ["Cherestea", "Atelier", "Cazarma"][t]));
                }
            }
            if self.fighters[c] > 0 {
                line.push_str(&format!("luptători {} ", self.fighters[c]));
            }
            if self.sym[c] {
                line.push_str("(simpatie) ");
            }
            if self.base_at(c) {
                line.push_str("[Baza] ");
            }
            if self.rules(c) {
                line.push_str("«stăpânită» ");
            }
            s.push_str(line.trim_end());
            s.push('\n');
        }
        s.push_str("Drumuri: ");
        for (a, b) in EDGES {
            s.push_str(&format!("{}-{} ", NAMES[a], NAMES[b]));
        }
        s.push('\n');
        let reg = self.player_of(Fac::Reg);
        s.push_str(&format!(
            "Puncte: J1 {}, J2 {} (la {WIN_VP}); tura {}/{}; Regența: lemn {}, clădiri C{} A{} K{}; Frunzișul: simpatii {}/{}, baze {}\n",
            self.vp[0],
            self.vp[1],
            self.turns + 1,
            TURN_CAP,
            self.wood,
            self.bld_on_map(0),
            self.bld_on_map(1),
            self.bld_on_map(2),
            self.sym_on_map(),
            TOTAL_SYM,
            self.bases_on_map()
        ));
        let vw = self.viewer.unwrap_or_else(|| self.to_move());
        let h = &self.hand[vw.idx()];
        s.push_str(&format!(
            "Mâna ta (J{}): stejar {}, ferig {}, ciuperca {}, pasare {}; J{} are {} cărți\n",
            vw.idx() + 1,
            h[0],
            h[1],
            h[2],
            h[3],
            vw.other().idx() + 1,
            self.hand_total(vw.other())
        ));
        let sup_total: u8 = self.sup.iter().sum();
        if self.fac[0] == Fac::Ins {
            s.push_str(&format!(
                "Suporterii tăi: stejar {}, ferig {}, ciuperca {}, pasare {}\n",
                self.sup[0], self.sup[1], self.sup[2], self.sup[3]
            ));
        } else {
            s.push_str(&format!("Suporteri adversari: {sup_total}\n"));
        }
        let _ = reg;
        if self.outcome().is_none() {
            let who = self.to_move();
            let ph = if let Some(b) = self.bat {
                match b.stage {
                    Stage::DefReact => "ambuscadă? (ambuscada / fara)".to_string(),
                    Stage::AttReact => "contra? (contra / lasa)".to_string(),
                    Stage::Roll => "zaruri (scrie «zar»)".to_string(),
                }
            } else {
                match self.phase {
                    Phase::Draw => "trage (scrie «trag»)".into(),
                    Phase::Act => format!("{} acțiuni rămase", self.left),
                    Phase::Craft => "seara: fabrica / gata".into(),
                    Phase::Discard => "aruncă până la 5 cărți".into(),
                }
            };
            s.push_str(&format!(
                "La mutare: jucătorul {} — {}\n",
                who.idx() + 1,
                ph
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Draw => "trag".into(),
            Move::March(a, b, n) => format!("mars {}>{} {}", NAMES[a], NAMES[b], n),
            Move::Battle(c) => format!("lupta {}", NAMES[c]),
            Move::Recruit => "recruteaza".into(),
            Move::Build(t, c) => format!("construieste {} {}", BLD_NAMES[t], NAMES[c]),
            Move::Craft(c) => format!("fabrica {}", NAMES[c]),
            Move::Done => "gata".into(),
            Move::Discard(k) => format!("arunca {}", SUIT_NAMES[k]),
            Move::Mobilize(k) => format!("mobilizeaza {}", SUIT_NAMES[k]),
            Move::Spread(c) => format!("propaga {}", NAMES[c]),
            Move::Revolt(c) => format!("revolta {}", NAMES[c]),
            Move::Ambush => "ambuscada".into(),
            Move::NoAmbush => "fara".into(),
            Move::Counter => "contra".into(),
            Move::NoCounter => "lasa".into(),
            Move::Roll => "zar".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

impl Codru {
    fn finish_after_battle(&mut self) {
        if self.bat.is_none() {
            self.finish_phase();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_game(seed: u64, swapped: bool) -> Codru {
        Codru::with_factions(seed, swapped)
    }

    fn assert_invariants(g: &Codru) {
        assert!(g.war_total(Fac::Reg) <= TOTAL_GUARDS);
        assert!(g.war_total(Fac::Ins) <= TOTAL_FIGHTERS);
        assert!(g.sym_on_map() <= TOTAL_SYM);
        for t in 0..3 {
            assert!(g.bld_on_map(t) <= BLD_MAX);
        }
        for c in 0..NP {
            assert!(g.slots_used(c) <= 2, "slots in {}", NAMES[c]);
        }
        assert_eq!(g.total_cards(), DECK_SIZE);
    }

    #[test]
    fn map_connected() {
        let d = dist_table();
        for row in d.iter() {
            assert!(row.iter().all(|&x| x < 99));
        }
        let mut names = NAMES.to_vec();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), NP);
    }

    #[test]
    fn setup_is_consistent() {
        for seed in 1..=8 {
            let g = rand_game(seed, seed % 2 == 0);
            assert_invariants(&g);
            assert_eq!(g.war_total(Fac::Reg), 6);
            assert_eq!(g.war_total(Fac::Ins), 3);
            assert_eq!(g.side, g.player_of(Fac::Reg));
            assert!(g.rules(0));
            assert_eq!(g.hand_total(Player::One), 3);
        }
    }

    #[test]
    fn turn_flow_regency() {
        let mut g = rand_game(1, false);
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
        g.apply(&Move::Draw);
        assert_eq!(g.left, 3);
        assert_eq!(g.wood, 1);
        // construiește Cherestea (a doua dă 1 p)
        let before = g.vp(Player::One);
        g.apply(&Move::Build(0, 1));
        assert_eq!(g.vp(Player::One), before + 1);
        assert_eq!(g.wood, 0);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Build(..))));
        g.apply(&Move::Recruit);
        assert_eq!(g.guards[0], 5);
        assert!(!g.legal_moves().contains(&Move::Recruit));
        g.apply(&Move::March(0, 1, 2));
        assert_eq!(g.phase, Phase::Craft);
        g.apply(&Move::Done);
        if g.hand_total(Player::One) <= 5 {
            assert_eq!(g.side, Player::Two);
            assert_eq!(g.phase, Phase::Draw);
        }
    }

    #[test]
    fn rule_requires_majority() {
        let mut g = rand_game(2, false);
        g.guards = [0; NP];
        g.guards[2] = 1;
        g.fighters = [0; NP];
        g.fighters[2] = 1;
        assert!(!g.rules(2));
        g.guards[2] = 2;
        assert!(g.rules(2));
        g.base[SUIT[2] as usize] = Some(2);
        assert!(!g.rules(2));
    }

    #[test]
    fn insurgent_spread_revolt_and_cost() {
        let mut g = rand_game(3, false);
        g.side = g.player_of(Fac::Ins);
        g.phase = Phase::Act;
        g.left = 3;
        g.sup = [2, 0, 0, 0];
        g.hand = [[0; 4]; 2];
        g.guards = [0; NP];
        // simpatie la 5 (Maracini, ciupercă); vecini: 4, 6, 1
        let ms = g.legal_moves();
        assert!(!ms.contains(&Move::Spread(0)), "nu e adiacentă");
        g.sup = [0, 2, 0, 0];
        let ms = g.legal_moves();
        assert!(ms.contains(&Move::Spread(1)), "{ms:?}");
        let v0 = g.vp(g.side);
        g.apply(&Move::Spread(1));
        assert_eq!(g.vp(g.side), v0 + 1);
        assert_eq!(g.sup[1], 1);
        // revoltă în 1 (ferigă): 2 suporteri
        g.sup = [0, 2, 0, 0];
        g.bld[1][1] = 1;
        let ms = g.legal_moves();
        assert!(ms.contains(&Move::Revolt(1)));
        let v1 = g.vp(g.side);
        g.apply(&Move::Revolt(1));
        assert_eq!(g.base[1], Some(1));
        assert_eq!(g.bld[1][1], 0);
        assert_eq!(g.vp(g.side), v1 + 1);
        assert!(g.fighters[1] >= 1);
        assert_eq!(g.bases_on_map(), 1);
    }

    #[test]
    fn spread_costs_two_under_martial_law() {
        let mut g = rand_game(3, false);
        g.side = g.player_of(Fac::Ins);
        g.phase = Phase::Act;
        g.left = 3;
        g.sup = [0, 1, 0, 0];
        g.guards[1] = 3;
        assert!(!g.legal_moves().contains(&Move::Spread(1)));
        g.sup = [0, 1, 0, 1];
        assert!(g.legal_moves().contains(&Move::Spread(1)));
    }

    #[test]
    fn ambush_flow_and_counter() {
        let mut g = rand_game(4, false);
        g.guards = [0; NP];
        g.fighters = [0; NP];
        g.guards[0] = 4;
        g.fighters[0] = 3;
        g.hand = [[0; 4]; 2];
        let ins = g.player_of(Fac::Ins);
        g.hand[ins.idx()][SUIT[0] as usize] = 1;
        g.phase = Phase::Act;
        g.left = 3;
        g.apply(&Move::Battle(0));
        assert_eq!(g.to_move(), ins);
        assert_eq!(g.legal_moves(), vec![Move::Ambush, Move::NoAmbush]);
        g.apply(&Move::Ambush);
        // atacatorul nu are carte: ambuscada lovește imediat
        assert_eq!(g.guards[0], 2);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        // același, cu contra
        let mut h = rand_game(4, false);
        h.guards = [0; NP];
        h.fighters = [0; NP];
        h.guards[0] = 4;
        h.fighters[0] = 3;
        h.hand = [[0; 4]; 2];
        h.hand[ins.idx()][SUIT[0] as usize] = 1;
        h.hand[h.player_of(Fac::Reg).idx()][3] = 1;
        h.phase = Phase::Act;
        h.left = 3;
        h.apply(&Move::Battle(0));
        h.apply(&Move::Ambush);
        assert_eq!(h.legal_moves(), vec![Move::Counter, Move::NoCounter]);
        h.apply(&Move::Counter);
        assert_eq!(h.guards[0], 4);
        assert_eq!(h.legal_moves(), vec![Move::Roll]);
        assert_eq!(h.discard.len(), 2);
    }

    #[test]
    fn guerrilla_defender_uses_high_die() {
        // Frunzișul apără cu 6 luptători contra 6 gărzi: sumă pierderi se verifică statistic
        let mut reg_loss = 0i32;
        let mut ins_loss = 0i32;
        for seed in 0..400u64 {
            let mut g = rand_game(seed, false);
            g.guards = [0; NP];
            g.fighters = [0; NP];
            g.guards[0] = 6;
            g.fighters[0] = 6;
            g.hand = [[0; 4]; 2];
            g.phase = Phase::Act;
            g.left = 3;
            g.apply(&Move::Battle(0));
            g.apply(&Move::Roll);
            reg_loss += 6 - g.guards[0] as i32;
            ins_loss += 6 - g.fighters[0] as i32;
        }
        assert!(ins_loss < reg_loss, "gherila: {ins_loss} vs {reg_loss}");
    }

    #[test]
    fn battle_removes_buildings_and_scores() {
        let mut g = rand_game(5, false);
        g.guards = [0; NP];
        g.fighters = [0; NP];
        g.fighters[0] = 6;
        g.bld = [[0; 3]; NP];
        g.bld[0][1] = 2;
        g.hand = [[0; 4]; 2];
        g.side = g.player_of(Fac::Ins);
        g.phase = Phase::Act;
        g.left = 3;
        g.apply(&Move::Battle(0));
        g.apply(&Move::Roll);
        // apărătorul n-are războinici: ≥ 1 lovitură + zarul → cel puțin 1 clădire a căzut
        assert!(g.bld[0][1] <= 1);
        assert!(g.vp(g.player_of(Fac::Ins)) >= 1);
    }

    #[test]
    fn win_at_threshold_and_cap() {
        let mut g = rand_game(6, false);
        g.vp[0] = WIN_VP;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut h = rand_game(6, false);
        h.turns = TURN_CAP;
        h.vp = [3, 5];
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::Two)));
        h.vp = [5, 5];
        assert_eq!(h.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn hand_limit_forces_discard() {
        let mut g = rand_game(7, false);
        g.hand[0] = [3, 3, 0, 0];
        g.phase = Phase::Act;
        g.left = 1;
        g.apply(&Move::Done);
        assert_eq!(g.phase, Phase::Craft);
        g.apply(&Move::Done);
        assert_eq!(g.phase, Phase::Discard);
        assert!(g
            .legal_moves()
            .iter()
            .all(|m| matches!(m, Move::Discard(_))));
        g.apply(&Move::Discard(0));
        assert_eq!(g.side, Player::Two);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..=24u64 {
            let mut g = rand_game(seed, seed % 2 == 0);
            let mut rng = Rng::new(seed * 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "seed {seed}\n{}", g.render());
                let pick = m[rng.below(m.len())];
                g.apply(&pick);
                assert_invariants(&g);
                n += 1;
                assert!(n < 6000, "seed {seed}: {n} plies");
            }
        }
    }

    fn duel(seed: u64, ai_fac: Fac) -> (bool, bool) {
        let swapped = ai_fac != Fac::Reg; // J2 = AI
        let mut g = Codru::with_factions(seed, swapped);
        let ai = g.player_of(ai_fac);
        let mut rng = Rng::new(seed * 11);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = if g.to_move() == ai {
                polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())]
            };
            g.apply(&mv);
            n += 1;
            assert!(n < 6000);
        }
        (
            g.outcome() == Some(Outcome::Win(ai)),
            g.outcome() == Some(Outcome::Win(ai.other())),
        )
    }

    #[test]
    fn ai_beats_random_with_each_faction() {
        for f in [Fac::Reg, Fac::Ins] {
            let (mut w, mut l) = (0, 0);
            for seed in 1..=12u64 {
                let (a, b) = duel(seed, f);
                w += a as i32;
                l += b as i32;
            }
            assert!(w > l && w >= 8, "{f:?}: AI {w} vs random {l}");
        }
    }

    #[test]
    fn ai_is_fast() {
        let mut g = rand_game(9, false);
        let mut rng = Rng::new(1);
        let mut worst = std::time::Duration::ZERO;
        let mut n = 0;
        while g.outcome().is_none() && n < 600 {
            let t = std::time::Instant::now();
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            worst = worst.max(t.elapsed());
            g.apply(&m);
            n += 1;
        }
        assert!(worst.as_secs_f32() < 1.0, "{worst:?}");
    }
}
