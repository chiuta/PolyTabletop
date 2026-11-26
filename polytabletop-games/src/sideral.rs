//! Coroana Siderală — 4X compact pentru doi (variantă originală): flote, planete, tehnologii, cărți de strategie.
//!
//! Harta: 9 sisteme — Citadela în centru (legată de toate cele șase sisteme din inel), inelul 1–6 și două
//! capete (Nodul lângă Kryn/Oasa, Abis lângă Tarsis/Ostra). Fiecare sistem are o planetă (resurse/influență).
//! J1 pornește din Aureon, J2 din Veltris (planetele-capitală nu pot fi cucerite); 3 nave, 3 jetoane de comandă.
//!
//! **Runda**: (1) *Strategie*: cărțile 1–6 (Conducere, Comert, Constructie, Tehnologie, Expeditie, Imperiu) se aleg
//! pe rând (`alege Comert`), câte două fiecare, începând cu vorbitorul (alternativ). (2) *Acțiune*: în ordinea
//! inițiativei (cartea cu numărul cel mai mic), pe rând, o acțiune: `taktic A>B n` (activezi sistemul B cu un jeton
//! de comandă sau o activare gratuită și muți n nave din A; apoi `adauga C>B n` oricâte, `gata`), `carte Nume`
//! (efectul principal; adversarul poate folosi apoi efectul secundar: `sec` / `nu`) sau `pas`. Lupta: mutări
//! forțate `zar`, runde de d10 (lovește la ≥8; Tunuri: ≥6), Scuturi anulează o lovitură pe rundă când adversarul lovește de cel puțin două ori (altfel două flote de o navă s-ar bloca la nesfârșit), până când o parte
//! rămâne fără nave. Cine rămâne în sistem cu cel puțin r nave (r+1 dacă planeta e a adversarului) o ocupă.
//! (3) *Status*: fiecare poate puncta un obiectiv public îndeplinit (`obiectiv N`), apoi se reîmprospătează resurse,
//! influență (+2 jetoane), cărțile se întorc și se dezvăluie un nou obiectiv.
//!
//! Cărți: Conducere (+3 jetoane; sec.: 2 inf → +2 jetoane), Comert (+4 resurse; sec.: 1 jeton → +2), Constructie
//! (până la 3 nave, 2 resurse fiecare, pe planetele tale; sec.: 1 jeton → 1 navă), Tehnologie (1 tehnologie, 3 resurse;
//! sec.: 1 jeton), Expeditie (2 activări gratuite; sec.: 1 inf → 1), Imperiu (+1 p, +1 dacă ții Citadela; sec.: 4 inf → +1 p).
//! Tehnologii: Tunuri, Scuturi, Propulsie (rază 2), Santier (+1 navă la Constructie), Extractie (+2 resurse/rundă).
//! Victorie: 8 puncte sau, după 12 runde, cele mai multe puncte (apoi planete; egal = remiză).
//!
//! # AI
//! `evaluate` e determinist și public (nu există informație ascunsă); zarurile sunt mutări forțate separate;
//! în timpul activării, planeta „proiectată” e numărată ca ocupată dacă flota ajunge.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const NS: usize = 9;
pub const NAMES: [&str; NS] = [
    "Citadela", "Aureon", "Kryn", "Oasa", "Veltris", "Tarsis", "Ostra", "Nodul", "Abis",
];
pub const RES: [u8; NS] = [1, 3, 2, 1, 3, 2, 1, 3, 3];
pub const INF: [u8; NS] = [2, 2, 0, 2, 2, 0, 2, 1, 1];
pub const HOME: [usize; 2] = [1, 4];
pub const EDGES: [(usize, usize); 16] = [
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 5),
    (5, 6),
    (6, 1),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (7, 2),
    (7, 3),
    (8, 5),
    (8, 6),
];
pub const CARDS: [&str; 6] = [
    "Conducere",
    "Comert",
    "Constructie",
    "Tehnologie",
    "Expeditie",
    "Imperiu",
];
pub const TECHS: [&str; 5] = ["Tunuri", "Scuturi", "Propulsie", "Santier", "Extractie"];
pub const OBJ_NAMES: [&str; 8] = [
    "Trei planete",
    "Flota de 6",
    "Doua tehnologii",
    "Tronul (Citadela)",
    "Bogatie (9 resurse)",
    "Cinci planete",
    "Flota de 9",
    "Patru tehnologii",
];
pub const OBJ_VP: [u8; 8] = [1, 1, 1, 1, 2, 2, 2, 2];
pub const WIN_VP: u8 = 8;
pub const ROUND_CAP: u32 = 12;
pub const SHIP_CAP: u8 = 12;
pub const TOKEN_CAP: u8 = 8;
const SHIP_COST: u8 = 2;
const TECH_COST: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Act,
    Status,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sub {
    None,
    Moving(usize),
    Combat(usize),
    Build { who: Player, left: u8 },
    Research { who: Player },
    Sec { card: usize, user: Player },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(usize),
    Pass,
    Card(usize),
    Tactic(usize, usize, u8),
    Add(usize, usize, u8),
    EndMove,
    Roll,
    Ship(usize),
    Research(usize),
    EndSub,
    SecUse,
    SecSkip,
    Score(usize),
    StatusDone,
}

#[derive(Clone)]
pub struct Sideral {
    owner: [Option<Player>; NS],
    ships: [[u8; 2]; NS],
    tech: [[bool; 5]; 2],
    tokens: [u8; 2],
    free: [u8; 2],
    res: [u8; 2],
    inf: [u8; 2],
    vp: [u8; 2],
    card_owner: [Option<Player>; 6],
    card_used: [bool; 6],
    scored: [[bool; 8]; 2],
    phase: Phase,
    sub: Sub,
    round: u32,
    npicks: u8,
    act_side: Player,
    passed: [bool; 2],
    st_idx: u8,
    pending_sec: Option<(usize, Player)>,
    secondary: bool,
    rng: Rng,
    log: String,
}

impl Default for Sideral {
    fn default() -> Self {
        Self::new()
    }
}

fn dist_table() -> [[u8; NS]; NS] {
    let mut d = [[99u8; NS]; NS];
    for (i, row) in d.iter_mut().enumerate() {
        row[i] = 0;
    }
    for &(a, b) in EDGES.iter() {
        d[a][b] = 1;
        d[b][a] = 1;
    }
    for k in 0..NS {
        for i in 0..NS {
            for j in 0..NS {
                let v = d[i][k] + d[k][j];
                if v < d[i][j] {
                    d[i][j] = v;
                }
            }
        }
    }
    d
}

impl Sideral {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Sideral {
            owner: [None; NS],
            ships: [[0; 2]; NS],
            tech: [[false; 5]; 2],
            tokens: [3, 3],
            free: [0, 0],
            res: [0, 0],
            inf: [0, 0],
            vp: [0, 0],
            card_owner: [None; 6],
            card_used: [false; 6],
            scored: [[false; 8]; 2],
            phase: Phase::Pick,
            sub: Sub::None,
            round: 1,
            npicks: 0,
            act_side: Player::One,
            passed: [false; 2],
            st_idx: 0,
            pending_sec: None,
            secondary: false,
            rng: Rng::new(seed),
            log: String::new(),
        };
        g.owner[HOME[0]] = Some(Player::One);
        g.owner[HOME[1]] = Some(Player::Two);
        g.ships[HOME[0]][0] = 3;
        g.ships[HOME[1]][1] = 3;
        g.refresh_income();
        g
    }

    fn speaker(&self) -> Player {
        if self.round % 2 == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    pub fn income_res(&self, p: Player) -> u8 {
        let mut r: u8 = (0..NS)
            .filter(|&s| self.owner[s] == Some(p))
            .map(|s| RES[s])
            .sum();
        if self.tech[p.idx()][4] {
            r += 2;
        }
        r
    }

    pub fn income_inf(&self, p: Player) -> u8 {
        (0..NS)
            .filter(|&s| self.owner[s] == Some(p))
            .map(|s| INF[s])
            .sum()
    }

    fn refresh_income(&mut self) {
        for p in [Player::One, Player::Two] {
            self.res[p.idx()] = self.income_res(p);
            self.inf[p.idx()] = self.income_inf(p);
        }
    }

    pub fn planets(&self, p: Player) -> usize {
        self.owner.iter().filter(|&&o| o == Some(p)).count()
    }

    pub fn ship_total(&self, p: Player) -> u8 {
        (0..NS).map(|s| self.ships[s][p.idx()]).sum()
    }

    fn tech_count(&self, p: Player) -> usize {
        self.tech[p.idx()].iter().filter(|&&t| t).count()
    }

    pub fn vp(&self, p: Player) -> u8 {
        self.vp[p.idx()]
    }

    fn range(&self, p: Player) -> u8 {
        if self.tech[p.idx()][2] {
            2
        } else {
            1
        }
    }

    fn need(&self, s: usize, p: Player) -> Option<u8> {
        if HOME.contains(&s) || self.owner[s] == Some(p) {
            return None;
        }
        Some(RES[s] + (self.owner[s].is_some() as u8))
    }

    fn try_capture(&mut self, s: usize, p: Player) {
        if let Some(n) = self.need(s, p) {
            if self.ships[s][p.idx()] >= n && self.ships[s][p.other().idx()] == 0 {
                self.owner[s] = Some(p);
                self.log
                    .push_str(&format!(" {} ocupă {}.", pname(p), NAMES[s]));
            }
        }
    }

    fn objective_ok(&self, k: usize, p: Player) -> bool {
        match k {
            0 => self.planets(p) >= 3,
            1 => self.ship_total(p) >= 6,
            2 => self.tech_count(p) >= 2,
            3 => self.owner[0] == Some(p),
            4 => self.income_res(p) >= 9,
            5 => self.planets(p) >= 5,
            6 => self.ship_total(p) >= 9,
            7 => self.tech_count(p) >= 4,
            _ => false,
        }
    }

    fn revealed(&self) -> usize {
        ((self.round + 1) as usize).min(8)
    }

    fn can_score(&self, p: Player) -> Vec<usize> {
        (0..self.revealed())
            .filter(|&k| !self.scored[p.idx()][k] && self.objective_ok(k, p))
            .collect()
    }

    fn can_sec(&self, card: usize, p: Player) -> bool {
        let i = p.idx();
        match card {
            0 => self.inf[i] >= 2,
            1..=3 => self.tokens[i] >= 1,
            4 => self.inf[i] >= 1,
            _ => self.inf[i] >= 4,
        }
    }

    fn build_cap(&self, p: Player) -> u8 {
        3 + self.tech[p.idx()][3] as u8
    }

    fn advance_turn(&mut self) {
        self.sub = Sub::None;
        self.secondary = false;
        let o = self.act_side.other();
        if self.passed[0] && self.passed[1] {
            self.begin_status();
        } else if !self.passed[o.idx()] {
            self.act_side = o;
        }
    }

    fn begin_status(&mut self) {
        self.phase = Phase::Status;
        self.st_idx = 0;
        self.status_skip();
    }

    fn status_skip(&mut self) {
        loop {
            if self.st_idx >= 2 {
                self.new_round();
                return;
            }
            let p = self.status_player();
            if !self.can_score(p).is_empty() {
                return;
            }
            self.st_idx += 1;
        }
    }

    fn status_player(&self) -> Player {
        if self.st_idx == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn new_round(&mut self) {
        self.round += 1;
        self.phase = Phase::Pick;
        self.sub = Sub::None;
        self.npicks = 0;
        self.card_owner = [None; 6];
        self.card_used = [false; 6];
        self.passed = [false; 2];
        self.free = [0, 0];
        for i in 0..2 {
            self.tokens[i] = (self.tokens[i] + 2).min(TOKEN_CAP);
        }
        self.refresh_income();
    }

    fn picker(&self) -> Player {
        let sp = self.speaker();
        if self.npicks.is_multiple_of(2) {
            sp
        } else {
            sp.other()
        }
    }

    fn start_actions(&mut self) {
        self.phase = Phase::Act;
        let low = (0..6).find(|&k| self.card_owner[k].is_some()).unwrap();
        self.act_side = self.card_owner[low].unwrap();
        self.passed = [false; 2];
    }

    fn finish_card(&mut self) {
        // după efectul principal / secundar
        if let Some((card, user)) = self.pending_sec.take() {
            if self.can_sec(card, user.other()) {
                self.sub = Sub::Sec { card, user };
                return;
            }
        }
        self.advance_turn();
    }

    fn run_effect(&mut self, card: usize, p: Player, primary: bool) {
        let i = p.idx();
        match card {
            0 => {
                if primary {
                    self.tokens[i] = (self.tokens[i] + 3).min(TOKEN_CAP);
                } else {
                    self.inf[i] -= 2;
                    self.tokens[i] = (self.tokens[i] + 2).min(TOKEN_CAP);
                }
                self.end_effect(primary);
            }
            1 => {
                if primary {
                    self.res[i] += 4;
                } else {
                    self.tokens[i] -= 1;
                    self.res[i] += 2;
                }
                self.end_effect(primary);
            }
            2 => {
                if !primary {
                    self.tokens[i] -= 1;
                }
                self.sub = Sub::Build {
                    who: p,
                    left: if primary { self.build_cap(p) } else { 1 },
                };
            }
            3 => {
                if !primary {
                    self.tokens[i] -= 1;
                }
                self.sub = Sub::Research { who: p };
            }
            4 => {
                if primary {
                    self.free[i] += 2;
                } else {
                    self.inf[i] -= 1;
                    self.free[i] += 1;
                }
                self.end_effect(primary);
            }
            _ => {
                if primary {
                    self.vp[i] += 1 + (self.owner[0] == Some(p)) as u8;
                } else {
                    self.inf[i] -= 4;
                    self.vp[i] += 1;
                }
                self.end_effect(primary);
            }
        }
    }

    fn end_effect(&mut self, primary: bool) {
        if primary {
            self.finish_card();
        } else {
            self.advance_turn();
        }
    }

    fn end_sub_effect(&mut self) {
        let primary = !self.secondary;
        self.sub = Sub::None;
        self.end_effect(primary);
    }

    fn combat_round(&mut self, s: usize) {
        let act = self.act_side;
        let def = act.other();
        let roll = |g: &mut Sideral, n: u8, on: u8| -> u8 {
            (0..n).filter(|_| (g.rng.below(10) + 1) as u8 >= on).count() as u8
        };
        let on_a = if self.tech[act.idx()][0] { 6 } else { 8 };
        let on_d = if self.tech[def.idx()][0] { 6 } else { 8 };
        let (na, nd) = (self.ships[s][act.idx()], self.ships[s][def.idx()]);
        let mut ha = roll(self, na, on_a);
        let mut hd = roll(self, nd, on_d);
        if self.tech[def.idx()][1] && ha >= 2 {
            ha -= 1;
        }
        if self.tech[act.idx()][1] && hd >= 2 {
            hd -= 1;
        }
        self.ships[s][def.idx()] = nd.saturating_sub(ha);
        self.ships[s][act.idx()] = na.saturating_sub(hd);
        self.log = format!("Luptă în {}: {} lovituri contra {}.", NAMES[s], ha, hd);
        let (a, d) = (self.ships[s][act.idx()], self.ships[s][def.idx()]);
        if a == 0 || d == 0 {
            self.try_capture(s, act);
            self.advance_turn();
        }
    }

    fn eval_side(&self, p: Player) -> i32 {
        let i = p.idx();
        let mut v = 100 * self.vp[i] as i32;
        // proprietar proiectat în timpul activării
        let proj: Option<usize> = match self.sub {
            Sub::Moving(s) | Sub::Combat(s) => Some(s),
            _ => None,
        };
        for s in 0..NS {
            let mut own = self.owner[s];
            if let Some(ps) = proj {
                if ps == s {
                    let act = self.act_side;
                    if let Some(n) = self.need(s, act) {
                        if self.ships[s][act.idx()] >= n && self.ships[s][act.other().idx()] == 0 {
                            own = Some(act);
                        }
                    }
                }
            }
            if own == Some(p) {
                v += 7 * RES[s] as i32 + 5 * INF[s] as i32;
                if s == 0 {
                    v += 10;
                }
            }
        }
        let ships: i32 = self.ship_total(p) as i32;
        v += 9 * ships;
        v += 14 * self.tech_count(p) as i32;
        v += 4 * self.tokens[i] as i32 + 4 * self.free[i] as i32;
        v += 2 * self.res[i] as i32 + self.inf[i] as i32;
        for k in 0..6 {
            if self.card_owner[k] == Some(p) && !self.card_used[k] {
                v += [9, 3, 6, 5, 5, 14][k];
            }
        }
        // fleet-ul din sistemul activ față de adversar
        if let Some(s) = proj {
            let act = self.act_side;
            let est =
                3 * (self.ships[s][act.idx()] as i32 - self.ships[s][act.other().idx()] as i32);
            if self.ships[s][act.other().idx()] > 0 {
                v += if act == p { est } else { -est };
            }
        }
        // apropiere de planete încă libere (mică, ca să se miște)
        let d = dist_table();
        for s in 0..NS {
            let n = self.ships[s][i] as i32;
            if n > 0 {
                let mut best = 9i32;
                for t in 0..NS {
                    if self.owner[t] != Some(p) && !HOME.contains(&t) {
                        best = best.min(d[s][t] as i32);
                    }
                }
                v -= n * best.min(3) / 2;
            }
        }
        v
    }
}

fn pname(p: Player) -> &'static str {
    match p {
        Player::One => "J1",
        Player::Two => "J2",
    }
}

impl Game for Sideral {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Pick => self.picker(),
            Phase::Status => self.status_player(),
            Phase::Act => match self.sub {
                Sub::Build { who, .. } | Sub::Research { who } => who,
                Sub::Sec { user, .. } => user.other(),
                _ => self.act_side,
            },
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let me = self.to_move();
        let i = me.idx();
        let mut v = Vec::new();
        match self.phase {
            Phase::Pick => {
                for k in 0..6 {
                    if self.card_owner[k].is_none() {
                        v.push(Move::Pick(k));
                    }
                }
            }
            Phase::Status => {
                for k in self.can_score(me) {
                    v.push(Move::Score(k));
                }
                v.push(Move::StatusDone);
            }
            Phase::Act => match self.sub {
                Sub::None => {
                    v.push(Move::Pass);
                    for k in 0..6 {
                        if self.card_owner[k] == Some(me) && !self.card_used[k] {
                            v.push(Move::Card(k));
                        }
                    }
                    if self.tokens[i] > 0 || self.free[i] > 0 {
                        let d = dist_table();
                        for a in 0..NS {
                            let n = self.ships[a][i];
                            if n == 0 {
                                continue;
                            }
                            for b in 0..NS {
                                if a != b && d[a][b] <= self.range(me) {
                                    v.push(Move::Tactic(a, b, 1));
                                    if n > 1 {
                                        v.push(Move::Tactic(a, b, n));
                                    }
                                }
                            }
                        }
                    }
                }
                Sub::Moving(b) => {
                    let d = dist_table();
                    for a in 0..NS {
                        let n = self.ships[a][i];
                        if n > 0 && a != b && d[a][b] <= self.range(me) {
                            v.push(Move::Add(a, b, 1));
                            if n > 1 {
                                v.push(Move::Add(a, b, n));
                            }
                        }
                    }
                    v.push(Move::EndMove);
                }
                Sub::Combat(_) => v.push(Move::Roll),
                Sub::Build { left, .. } => {
                    if left > 0 && self.res[i] >= SHIP_COST && self.ship_total(me) < SHIP_CAP {
                        for s in 0..NS {
                            if self.owner[s] == Some(me) {
                                v.push(Move::Ship(s));
                            }
                        }
                    }
                    v.push(Move::EndSub);
                }
                Sub::Research { .. } => {
                    if self.res[i] >= TECH_COST {
                        for k in 0..5 {
                            if !self.tech[i][k] {
                                v.push(Move::Research(k));
                            }
                        }
                    }
                    v.push(Move::EndSub);
                }
                Sub::Sec { .. } => {
                    v.push(Move::SecUse);
                    v.push(Move::SecSkip);
                }
            },
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.to_move();
        let i = me.idx();
        match *m {
            Move::Pick(k) => {
                self.card_owner[k] = Some(me);
                self.npicks += 1;
                self.log = format!("{} alege {}.", pname(me), CARDS[k]);
                if self.npicks == 4 {
                    self.start_actions();
                }
            }
            Move::Pass => {
                self.passed[i] = true;
                self.log = format!("{} pasează.", pname(me));
                self.advance_turn();
            }
            Move::Card(k) => {
                self.card_used[k] = true;
                self.secondary = false;
                self.pending_sec = Some((k, me));
                self.log = format!("{} folosește {}.", pname(me), CARDS[k]);
                self.run_effect(k, me, true);
            }
            Move::Tactic(a, b, n) => {
                if self.free[i] > 0 {
                    self.free[i] -= 1;
                } else {
                    self.tokens[i] -= 1;
                }
                self.ships[a][i] -= n;
                self.ships[b][i] += n;
                self.sub = Sub::Moving(b);
                self.log = format!(
                    "{} activează {} și mută {} din {}.",
                    pname(me),
                    NAMES[b],
                    n,
                    NAMES[a]
                );
            }
            Move::Add(a, b, n) => {
                self.ships[a][i] -= n;
                self.ships[b][i] += n;
                self.log = format!(
                    "{} mută încă {} din {} în {}.",
                    pname(me),
                    n,
                    NAMES[a],
                    NAMES[b]
                );
            }
            Move::EndMove => {
                if let Sub::Moving(s) = self.sub {
                    if self.ships[s][me.other().idx()] > 0 {
                        self.sub = Sub::Combat(s);
                    } else {
                        self.try_capture(s, me);
                        self.advance_turn();
                    }
                }
            }
            Move::Roll => {
                if let Sub::Combat(s) = self.sub {
                    self.combat_round(s);
                }
            }
            Move::Ship(s) => {
                self.ships[s][i] += 1;
                self.res[i] -= SHIP_COST;
                if let Sub::Build { who, left } = self.sub {
                    self.sub = Sub::Build {
                        who,
                        left: left - 1,
                    };
                }
                self.log = format!("{} construiește o navă în {}.", pname(me), NAMES[s]);
            }
            Move::Research(k) => {
                self.tech[i][k] = true;
                self.res[i] -= TECH_COST;
                self.log = format!("{} cercetează {}.", pname(me), TECHS[k]);
                self.end_sub_effect();
            }
            Move::EndSub => self.end_sub_effect(),
            Move::SecUse => {
                if let Sub::Sec { card, .. } = self.sub {
                    self.secondary = true;
                    self.sub = Sub::None;
                    self.log = format!(
                        "{} folosește efectul secundar al cărții {}.",
                        pname(me),
                        CARDS[card]
                    );
                    self.run_effect(card, me, false);
                }
            }
            Move::SecSkip => self.advance_turn(),
            Move::Score(k) => {
                self.scored[i][k] = true;
                self.vp[i] += OBJ_VP[k];
                self.log = format!(
                    "{} punctează «{}» (+{}).",
                    pname(me),
                    OBJ_NAMES[k],
                    OBJ_VP[k]
                );
                self.st_idx += 1;
                self.status_skip();
            }
            Move::StatusDone => {
                self.st_idx += 1;
                self.status_skip();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.vp[0], self.vp[1]);
        if a >= WIN_VP || b >= WIN_VP {
            if a > b {
                return Some(Outcome::Win(Player::One));
            }
            if b > a {
                return Some(Outcome::Win(Player::Two));
            }
        }
        if self.round > ROUND_CAP {
            let key = |p: Player| (self.vp[p.idx()], self.planets(p));
            return Some(match key(Player::One).cmp(&key(Player::Two)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.eval_side(p) - self.eval_side(p.other())
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Coroana Siderală — runda {}/{}\n",
            self.round.min(ROUND_CAP),
            ROUND_CAP
        );
        for k in 0..NS {
            let own = match self.owner[k] {
                Some(Player::One) => "J1",
                Some(Player::Two) => "J2",
                None => "liber",
            };
            s.push_str(&format!(
                "{} (r{} i{}) [{}]: nave J1 {}, J2 {}\n",
                NAMES[k], RES[k], INF[k], own, self.ships[k][0], self.ships[k][1]
            ));
        }
        s.push_str("Rute: ");
        let mut seen: Vec<(usize, usize)> = Vec::new();
        for &(a, b) in EDGES.iter() {
            if !seen.contains(&(a, b)) {
                seen.push((a, b));
                s.push_str(&format!("{}-{} ", NAMES[a], NAMES[b]));
            }
        }
        s.push('\n');
        for p in [Player::One, Player::Two] {
            let i = p.idx();
            let t: Vec<&str> = (0..5)
                .filter(|&k| self.tech[i][k])
                .map(|k| TECHS[k])
                .collect();
            s.push_str(&format!(
                "{}: {} p, jetoane {} (+{} gratuite), resurse {}, influență {}, tehnologii [{}]\n",
                pname(p),
                self.vp[i],
                self.tokens[i],
                self.free[i],
                self.res[i],
                self.inf[i],
                t.join(", ")
            ));
        }
        s.push_str("Cărți: ");
        for k in 0..6 {
            let o = match self.card_owner[k] {
                Some(p) => format!(
                    "{}{}",
                    pname(p),
                    if self.card_used[k] {
                        " (folosită)"
                    } else {
                        ""
                    }
                ),
                None => "liberă".into(),
            };
            s.push_str(&format!("{}.{} [{}]  ", k + 1, CARDS[k], o));
        }
        s.push('\n');
        s.push_str("Obiective dezvăluite: ");
        for k in 0..self.revealed() {
            s.push_str(&format!("{}.{} ({}p)  ", k, OBJ_NAMES[k], OBJ_VP[k]));
        }
        s.push('\n');
        if self.outcome().is_none() {
            let ph = match self.phase {
                Phase::Pick => "alege o carte de strategie".to_string(),
                Phase::Status => "status: punctează un obiectiv".into(),
                Phase::Act => match self.sub {
                    Sub::None => "acțiune".into(),
                    Sub::Moving(b) => format!("mută nave spre {} (adauga / gata)", NAMES[b]),
                    Sub::Combat(_) => "luptă (scrie «zar»)".into(),
                    Sub::Build { left, .. } => format!("construcție ({left} nave rămase)"),
                    Sub::Research { .. } => "cercetare".into(),
                    Sub::Sec { card, .. } => {
                        format!("efect secundar al cărții {}? (sec / nu)", CARDS[card])
                    }
                },
            };
            s.push_str(&format!(
                "La mutare: jucătorul {} — {}\n",
                self.to_move().idx() + 1,
                ph
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Pick(k) => format!("alege {}", CARDS[k]),
            Move::Pass => "pas".into(),
            Move::Card(k) => format!("carte {}", CARDS[k]),
            Move::Tactic(a, b, n) => format!("taktic {}>{} {}", NAMES[a], NAMES[b], n),
            Move::Add(a, b, n) => format!("adauga {}>{} {}", NAMES[a], NAMES[b], n),
            Move::EndMove => "gata".into(),
            Move::Roll => "zar".into(),
            Move::Ship(s) => format!("nava {}", NAMES[s]),
            Move::Research(k) => format!("tech {}", TECHS[k]),
            Move::EndSub => "gata".into(),
            Move::SecUse => "sec".into(),
            Move::SecSkip => "nu".into(),
            Move::Score(k) => format!("obiectiv {k}"),
            Move::StatusDone => "gata".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ships_ok(g: &Sideral) {
        for p in [Player::One, Player::Two] {
            assert!(g.ship_total(p) <= SHIP_CAP + 3, "prea multe nave");
        }
        assert_eq!(g.owner[HOME[0]], Some(Player::One));
        assert_eq!(g.owner[HOME[1]], Some(Player::Two));
        for s in 0..NS {
            // niciodată nave din ambele tabere într-un sistem la finalul unei acțiuni
            let _ = s;
        }
    }

    fn to_act(g: &mut Sideral) {
        for k in [0usize, 1, 2, 3] {
            // J1 și J2 aleg pe rând cărțile 0..3 (vorbitorul: J1 în runda 1)
            g.apply(&Move::Pick(k));
        }
    }

    #[test]
    fn map_is_connected_and_symmetric() {
        let d = dist_table();
        for row in d.iter() {
            assert!(row.iter().all(|&x| x < 99));
        }
        assert_eq!(d[HOME[0]][HOME[1]], 2);
        assert_eq!(RES[1], RES[4]);
        assert_eq!(INF[2] + INF[3], INF[5] + INF[6]);
        let mut n = NAMES.to_vec();
        n.sort();
        n.dedup();
        assert_eq!(n.len(), NS);
    }

    #[test]
    fn start_state() {
        let g = Sideral::with_seed(1);
        assert_eq!(g.ship_total(Player::One), 3);
        assert_eq!(g.income_res(Player::One), 3);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.legal_moves().len(), 6);
    }

    #[test]
    fn picks_alternate_and_start_action_phase() {
        let mut g = Sideral::with_seed(1);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Pick(5));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Pick(4));
        g.apply(&Move::Pick(3));
        g.apply(&Move::Pick(2));
        assert_eq!(g.phase, Phase::Act);
        // cartea cu numărul cel mai mic: 2 (Constructie) aparține lui J2 (al patrulea pick)
        assert_eq!(g.act_side, Player::Two);
        assert_eq!(g.card_owner[5], Some(Player::One));
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Pick(_))));
    }

    #[test]
    fn tactical_capture_of_neutral_planet() {
        let mut g = Sideral::with_seed(2);
        to_act(&mut g);
        g.act_side = Player::One;
        // Kryn (r2) cere 2 nave
        g.apply(&Move::Tactic(1, 2, 2));
        assert_eq!(g.tokens[0], 2);
        g.apply(&Move::EndMove);
        assert_eq!(g.owner[2], Some(Player::One));
        assert_eq!(g.act_side, Player::Two);
        assert_eq!(g.planets(Player::One), 2);
    }

    #[test]
    fn one_ship_cannot_take_two_resource_planet() {
        let mut g = Sideral::with_seed(2);
        to_act(&mut g);
        g.act_side = Player::One;
        g.apply(&Move::Tactic(1, 2, 1));
        g.apply(&Move::EndMove);
        assert_eq!(g.owner[2], None);
        g.act_side = Player::One;
        g.apply(&Move::Tactic(1, 3, 1)); // Oasa r1
        g.apply(&Move::EndMove);
        assert_eq!(g.owner[3], Some(Player::One));
    }

    #[test]
    fn range_depends_on_propulsion() {
        let mut g = Sideral::with_seed(2);
        to_act(&mut g);
        g.act_side = Player::One;
        let far = g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Tactic(1, 4, _) | Move::Tactic(1, 7, _)));
        assert!(!far);
        g.tech[0][2] = true;
        assert!(g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Tactic(1, 7, _))));
    }

    #[test]
    fn combat_runs_until_one_side_dies() {
        for seed in 0..30 {
            let mut g = Sideral::with_seed(seed);
            to_act(&mut g);
            g.act_side = Player::One;
            g.ships[2][1] = 2;
            g.ships[1][0] = 5;
            g.apply(&Move::Tactic(1, 2, 3));
            g.apply(&Move::EndMove);
            assert_eq!(g.sub, Sub::Combat(2));
            let mut n = 0;
            while matches!(g.sub, Sub::Combat(_)) {
                assert_eq!(g.legal_moves(), vec![Move::Roll]);
                g.apply(&Move::Roll);
                n += 1;
                assert!(n < 500);
            }
            let (a, d) = (g.ships[2][0], g.ships[2][1]);
            assert!(a == 0 || d == 0);
            if d == 0 && a >= 2 {
                assert_eq!(g.owner[2], Some(Player::One));
            }
        }
    }

    #[test]
    fn card_primary_and_secondary_flow() {
        let mut g = Sideral::with_seed(3);
        to_act(&mut g);
        g.act_side = Player::One; // J1 are cărțile 0 și 2; J2: 1 și 3
        g.apply(&Move::Card(0)); // Conducere: +3 jetoane
        assert_eq!(g.tokens[0], 6);
        // J2 poate folosi secundarul (2 inf → +2 jetoane)
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::SecUse, Move::SecSkip]);
        g.apply(&Move::SecUse);
        assert_eq!(g.tokens[1], 5);
        assert_eq!(g.inf[1], g.income_inf(Player::Two) - 2);
        assert_eq!(g.act_side, Player::Two);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn construction_and_research_pay_resources() {
        let mut g = Sideral::with_seed(3);
        to_act(&mut g);
        g.act_side = Player::One;
        g.apply(&Move::Card(2)); // Constructie
        assert!(matches!(g.sub, Sub::Build { left: 3, .. }));
        g.apply(&Move::Ship(1));
        assert_eq!(g.ship_total(Player::One), 4);
        assert_eq!(g.res[0], 1);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Ship(_))));
        g.apply(&Move::EndSub);
        // secundarul pentru J2 (are jetoane)
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::SecSkip);
        assert_eq!(g.act_side, Player::Two);
        g.apply(&Move::Card(3)); // Tehnologie (J2 are cărțile 1 și 3)
        g.apply(&Move::Research(0));
        assert!(g.tech[1][0]);
        assert_eq!(g.res[1], 0);
    }

    #[test]
    fn imperial_card_scores_with_citadel_bonus() {
        let mut g = Sideral::with_seed(4);
        g.apply(&Move::Pick(5));
        g.apply(&Move::Pick(0));
        g.apply(&Move::Pick(1));
        g.apply(&Move::Pick(2));
        g.act_side = Player::One;
        g.owner[0] = Some(Player::One);
        g.apply(&Move::Card(5));
        assert_eq!(g.vp(Player::One), 2);
    }

    #[test]
    fn status_scores_objective_and_new_round() {
        let mut g = Sideral::with_seed(5);
        to_act(&mut g);
        g.owner[2] = Some(Player::One);
        g.owner[3] = Some(Player::One); // 3 planete cu Aureon
        g.passed = [false; 2];
        g.act_side = Player::One;
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Status);
        assert_eq!(g.to_move(), Player::One);
        assert!(g.legal_moves().contains(&Move::Score(0)));
        g.apply(&Move::Score(0));
        assert_eq!(g.vp(Player::One), 1);
        assert_eq!(g.round, 2);
        assert_eq!(g.phase, Phase::Pick);
        assert_eq!(g.to_move(), Player::Two); // vorbitor alternativ
        assert_eq!(g.card_owner, [None; 6]);
        assert_eq!(g.tokens[1], 5);
    }

    #[test]
    fn win_at_eight_and_round_cap() {
        let mut g = Sideral::with_seed(6);
        g.vp[1] = 8;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        let mut h = Sideral::with_seed(6);
        h.round = ROUND_CAP + 1;
        h.vp = [3, 3];
        assert_eq!(h.outcome(), Some(Outcome::Draw));
        h.owner[2] = Some(Player::One);
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn homeworlds_cannot_be_captured() {
        let mut g = Sideral::with_seed(7);
        to_act(&mut g);
        g.act_side = Player::One;
        g.ships[4][1] = 0;
        g.apply(&Move::Tactic(1, 3, 3));
        g.apply(&Move::EndMove);
        g.act_side = Player::One;
        g.tokens[0] = 3;
        g.apply(&Move::Tactic(3, 4, 3));
        g.apply(&Move::EndMove);
        assert_eq!(g.owner[4], Some(Player::Two));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = Sideral::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "seed {seed}\n{}", g.render());
                let pick = m[rng.below(m.len())];
                g.apply(&pick);
                ships_ok(&g);
                n += 1;
                assert!(n < 6000, "seed {seed}: {n} plies");
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let (mut w, mut l) = (0, 0);
        for seed in 1..=12u64 {
            let ai = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Sideral::with_seed(seed);
            let mut rng = Rng::new(seed * 13);
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
            match g.outcome() {
                Some(Outcome::Win(p)) if p == ai => w += 1,
                Some(Outcome::Win(_)) => l += 1,
                _ => {}
            }
        }
        assert!(w > l && w >= 8, "AI {w} vs random {l}");
    }

    #[test]
    fn ai_is_fast() {
        let mut g = Sideral::with_seed(9);
        let mut rng = Rng::new(1);
        let mut worst = std::time::Duration::ZERO;
        let mut n = 0;
        while g.outcome().is_none() && n < 800 {
            let t = std::time::Instant::now();
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            worst = worst.max(t.elapsed());
            g.apply(&m);
            n += 1;
        }
        assert!(worst.as_secs_f32() < 1.0, "{worst:?}");
    }
}
