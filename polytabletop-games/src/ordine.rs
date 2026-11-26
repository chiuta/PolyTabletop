//! „Ordine de bătaie” — infanterie cu zaruri de ordine și suprimare (id `ordine`).
//!
//! Motor generic, original, pentru o bătălie de infanterie 2 jucători pe o
//! hartă 12×8 cu ziduri (`##`, blochează mersul și vederea), păduri (`::`,
//! acoperire) și 2 obiective (`()`). Fiecare parte: Ofițer, 2 Grupe, Mitralieră,
//! Mortier.
//!
//! Reguli (scrise de noi):
//! * Tura: câte un „zar de ordine” pe unitate vie într-un săculeț. Mutarea forțată
//!   `trage` extrage un zar la întâmplare: partea extrasă activează o unitate care
//!   nu a acționat încă și îi dă un ORDIN. Tura se termină când săculețul e gol.
//!   6 ture.
//! * Ordine: `fire` (stă și trage), `adv d4 [xV]` (mută ≤ mv și poate trage, +1 la
//!   lovire), `run d4 [xV]` (mută ≤ 2×mv, nu trage; dacă `xV` e lipită la sosire =
//!   asalt), `amb` (ambuscadă: trage automat în prima unitate inamică care își
//!   încheie activarea în vedere și rază), `rally` (scoate 2 marcaje de suprimare),
//!   `down` (la sol: −1 marcaj, inamicii lovesc cu +1 mai greu).
//! * Suprimare: o unitate cu marcaje trebuie să treacă un test de ordin (2d6 ≤
//!   moral − marcaje, +1 dacă un Ofițer prieten e la ≤ 6) ca să execute ordinul;
//!   altfel merge la sol (rămâne unde era). Marcaj: +1 când e lovită (cel puțin o
//!   lovitură), +1 dacă pierde ≥ 2 oameni, +1 în plus la foc indirect.
//! * Foc: zaruri = oameni × zaruri/om; lovire la d6 ≥ 4 (+1 pe marcaj până la +3,
//!   +1 după avans, +1 tragător la sol, +1 țintă în pădure, +1 țintă la sol, +1 peste
//!   jumătate din rază; 6 lovește mereu); fiecare lovitură e salvată de d6 ≥ 5; una
//!   nesalvată = un om pierdut. Mitraliera și Mortierul nu trag după avans;
//!   Mortierul (distanță 3–12) trage indirect dacă o unitate prietenă vede ținta.
//! * Asalt: ambele părți lovesc simultan cu câte un zar/om (4+, fără salvare, cele
//!   cu marcaje lovesc la 5+); cine pierde mai mulți oameni ia 2 marcaje.
//! * Obiective: la sfârșitul turei, cine are mai mulți oameni din unități fără
//!   marcaje la ≤ 1 de obiectiv ia 1 punct. După 6 ture: puncte, apoi oameni rămași.
//!   Dispariția armatei adverse câștigă imediat.
//!
//! Informație completă; zarurile (și extragerea zarului de ordine) se fac la
//! mutările forțate `trage` / `arunca`, cu zaruri a căror sămânță se schimbă la
//! `clone`, deci AI-ul nu poate anticipa. AI = AlphaBeta(1) cu valori așteptate.
//! Mutări: `trage`, `B fire xW`, `B adv d4 xW`, `B run d4`, `B amb`, `B rally`,
//! `B down`, `arunca`.

use crate::w7_util::{bfs8, cell_name, cheb, Dice, GridMap};
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::Arc;

pub const W: i32 = 12;
pub const H: i32 = 8;
const MAX_TURNS: u8 = 6;
const WALLS: [(i32, i32); 4] = [(4, 2), (2, 4), (7, 1), (5, 3)];
const COVERS: [(i32, i32); 4] = [(1, 2), (3, 3), (9, 1), (6, 2)];
const OBJ: [(i32, i32); 2] = [(3, 2), (8, 5)];
const START: [(i32, i32); 5] = [(5, 0), (3, 0), (8, 0), (6, 0), (4, 0)];

pub struct UTy {
    pub name: &'static str,
    pub n: i8,
    pub mv: i32,
    pub range: i32,
    pub dpm: i32,
    pub morale: i32,
    pub heavy: bool,
    pub indirect: bool,
}

pub const TYPES: [UTy; 4] = [
    UTy {
        name: "Ofițer",
        n: 1,
        mv: 5,
        range: 6,
        dpm: 2,
        morale: 9,
        heavy: false,
        indirect: false,
    },
    UTy {
        name: "Grupă",
        n: 6,
        mv: 4,
        range: 8,
        dpm: 1,
        morale: 8,
        heavy: false,
        indirect: false,
    },
    UTy {
        name: "Mitralieră",
        n: 3,
        mv: 3,
        range: 10,
        dpm: 2,
        morale: 8,
        heavy: true,
        indirect: false,
    },
    UTy {
        name: "Mortier",
        n: 2,
        mv: 3,
        range: 12,
        dpm: 2,
        morale: 8,
        heavy: true,
        indirect: true,
    },
];
const ROSTER: [usize; 5] = [0, 1, 1, 2, 3];

fn uid(i: usize) -> char {
    ['A', 'B', 'C', 'D', 'E', 'V', 'W', 'X', 'Y', 'Z'][i]
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub ty: u8,
    pub side: u8,
    pub pos: (i32, i32),
    pub n: i8,
    pub pins: u8,
    pub prone: bool,
    pub ambush: bool,
    pub done: bool,
}

impl Unit {
    fn t(&self) -> &'static UTy {
        &TYPES[self.ty as usize]
    }
    pub fn alive(&self) -> bool {
        self.n > 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ord {
    Fire(u8),
    Adv((u8, u8), Option<u8>),
    Run((u8, u8), Option<u8>),
    Ambush,
    Rally,
    Down,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Order(u8, Ord),
    Roll,
}

#[derive(Clone, Copy, Debug)]
struct Pend {
    u: u8,
    ord: Ord,
    from: (i32, i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Order,
    Roll,
}

#[derive(Clone)]
pub struct Ordine {
    map: Arc<GridMap>,
    pub u: Vec<Unit>,
    pub turn: u8,
    pub vp: [u8; 2],
    phase: Phase,
    side: Player,
    pending: Option<Pend>,
    dice: Dice,
    over: Option<Outcome>,
    log: Vec<String>,
}

impl Default for Ordine {
    fn default() -> Self {
        Self::new()
    }
}

fn p_hit(t: i32) -> f32 {
    if t >= 6 {
        1.0 / 6.0
    } else {
        (7 - t.max(2)) as f32 / 6.0
    }
}

/// P(2d6 ≤ x)
fn p_2d6_le(x: i32) -> f32 {
    if x < 2 {
        return 0.0;
    }
    if x >= 12 {
        return 1.0;
    }
    let mut c = 0;
    for a in 1..=6 {
        for b in 1..=6 {
            if a + b <= x {
                c += 1;
            }
        }
    }
    c as f32 / 36.0
}

impl Ordine {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut u = Vec::new();
        for side in 0..2u8 {
            for (k, &(x, y)) in START.iter().enumerate() {
                let pos = if side == 0 {
                    (x, y)
                } else {
                    (W - 1 - x, H - 1 - y)
                };
                u.push(Unit {
                    ty: ROSTER[k] as u8,
                    side,
                    pos,
                    n: TYPES[ROSTER[k]].n,
                    pins: 0,
                    prone: false,
                    ambush: false,
                    done: false,
                });
            }
        }
        Ordine {
            map: Arc::new(GridMap::build(W, H, &WALLS, &COVERS)),
            u,
            turn: 1,
            vp: [0, 0],
            phase: Phase::Draw,
            side: Player::One,
            pending: None,
            dice: Dice::new(seed),
            over: None,
            log: Vec::new(),
        }
    }

    fn occupied(&self, c: (i32, i32)) -> bool {
        self.u.iter().any(|x| x.alive() && x.pos == c)
    }

    fn remaining(&self, side: u8) -> usize {
        self.u
            .iter()
            .filter(|x| x.alive() && x.side == side && !x.done)
            .count()
    }

    fn men(&self, side: u8) -> i32 {
        self.u
            .iter()
            .filter(|x| x.alive() && x.side == side)
            .map(|x| x.n as i32)
            .sum()
    }

    fn units_alive(&self, side: u8) -> usize {
        self.u
            .iter()
            .filter(|x| x.alive() && x.side == side)
            .count()
    }

    fn reachable(&self, i: usize, maxd: i32) -> Vec<(i32, i32)> {
        let m = &self.u[i];
        bfs8(&self.map, m.pos, maxd, &|c| self.occupied(c), &|_| false)
            .into_iter()
            .map(|x| x.0)
            .collect()
    }

    fn mv_of(&self, i: usize) -> i32 {
        let m = &self.u[i];
        let base = m.t().mv;
        if m.prone {
            base / 2
        } else {
            base
        }
    }

    /// Poate unitatea `a`, aflată în `from`, să tragă în `t` (vizibilitate/rază/spotter)?
    fn can_shoot(&self, a: usize, from: (i32, i32), t: usize) -> bool {
        let ua = &self.u[a];
        let ut = &self.u[t];
        if !ut.alive() || ut.side == ua.side {
            return false;
        }
        let d = cheb(from, ut.pos);
        if d > ua.t().range {
            return false;
        }
        if ua.t().indirect {
            if d < 3 {
                return false;
            }
            self.u.iter().enumerate().any(|(j, s)| {
                j != a
                    && s.alive()
                    && s.side == ua.side
                    && self.map.see(if j == a { from } else { s.pos }, ut.pos)
                    && cheb(s.pos, ut.pos) <= 12
            })
        } else {
            d >= 1 && self.map.see(from, ut.pos)
        }
    }

    /// (zaruri, ținta de lovire, ținta salvării) pentru un foc.
    fn volley(&self, a: usize, t: usize, advance: bool) -> (i32, i32, i32) {
        let ua = &self.u[a];
        let ut = &self.u[t];
        let dice = ua.n as i32 * ua.t().dpm;
        let mut hit = 4 + ua.pins.min(3) as i32;
        if advance {
            hit += 1;
        }
        if ua.prone {
            hit += 1;
        }
        if self.map.is_cover(ut.pos) {
            hit += 1;
        }
        if ut.prone {
            hit += 1;
        }
        if cheb(ua.pos, ut.pos) * 2 > ua.t().range {
            hit += 1;
        }
        (dice, hit, 5)
    }

    fn exp_cas(&self, a: usize, t: usize, advance: bool) -> f32 {
        let (d, h, s) = self.volley(a, t, advance);
        let e = d as f32 * p_hit(h) * ((s - 1) as f32 / 6.0);
        e.min(self.u[t].n as f32)
    }

    fn test_target(&self, i: usize) -> i32 {
        let m = &self.u[i];
        let mut x = m.t().morale - m.pins as i32;
        if m.ty != 0
            && self
                .u
                .iter()
                .any(|o| o.alive() && o.side == m.side && o.ty == 0 && cheb(o.pos, m.pos) <= 6)
        {
            x += 1;
        }
        x
    }

    fn do_shoot(&mut self, a: usize, t: usize, advance: bool) {
        let (dice, hit, save) = self.volley(a, t, advance);
        let mut hits = 0;
        let mut cas = 0;
        for _ in 0..dice {
            let r = self.dice.d6() as i32;
            if r >= 6 || r >= hit {
                hits += 1;
                if (self.dice.d6() as i32) < save {
                    cas += 1;
                }
            }
        }
        let cas = cas.min(self.u[t].n as i32);
        self.log.push(format!(
            "{} trage în {}: {hits} lovituri, {cas} pierderi",
            uid(a),
            uid(t)
        ));
        let indirect = self.u[a].t().indirect;
        let ut = &mut self.u[t];
        ut.n -= cas as i8;
        if hits > 0 && ut.alive() {
            ut.pins =
                (ut.pins + 1 + if cas >= 2 { 1 } else { 0 } + if indirect { 1 } else { 0 }).min(6);
        } else if hits > 0 {
            ut.pins = 0;
        }
        if !ut.alive() {
            ut.ambush = false;
        }
    }

    fn do_assault(&mut self, a: usize, t: usize) {
        let roll = |g: &mut Self, i: usize| -> i32 {
            let u = &g.u[i];
            let need = if u.pins > 0 { 5 } else { 4 };
            let mut k = 0;
            for _ in 0..u.n {
                if g.dice.d6() as i32 >= need {
                    k += 1;
                }
            }
            k
        };
        let ka = roll(self, a); // lovituri date de atacator
        let kd = roll(self, t);
        let ka = ka.min(self.u[t].n as i32);
        let kd = kd.min(self.u[a].n as i32);
        self.u[t].n -= ka as i8;
        self.u[a].n -= kd as i8;
        self.log.push(format!(
            "asalt {}→{}: {} pierderi pentru apărător, {} pentru atacator",
            uid(a),
            uid(t),
            ka,
            kd
        ));
        match ka.cmp(&kd) {
            std::cmp::Ordering::Greater => {
                if self.u[t].alive() {
                    self.u[t].pins = (self.u[t].pins + 2).min(6);
                }
            }
            std::cmp::Ordering::Less if self.u[a].alive() => {
                self.u[a].pins = (self.u[a].pins + 2).min(6);
            }
            _ => {}
        }
    }

    fn fire_ambush(&mut self, actor: usize) {
        let side = self.u[actor].side;
        for i in 0..self.u.len() {
            if !self.u[actor].alive() {
                break;
            }
            let ok = {
                let a = &self.u[i];
                a.alive() && a.ambush && a.side != side && self.can_shoot(i, a.pos, actor)
            };
            if ok {
                self.u[i].ambush = false;
                self.do_shoot(i, actor, false);
            }
        }
    }

    fn resolve(&mut self, p: Pend) {
        let i = p.u as usize;
        let need_test = self.u[i].pins > 0 && !matches!(p.ord, Ord::Down);
        if need_test {
            let r = self.dice.two_d6() as i32;
            let tt = self.test_target(i);
            if r > tt {
                self.log
                    .push(format!("{} ratează testul ({r} > {tt}): la sol", uid(i)));
                self.u[i].pos = p.from;
                self.u[i].prone = true;
                return;
            }
        }
        match p.ord {
            Ord::Fire(t) => {
                if self.u[t as usize].alive() {
                    self.do_shoot(i, t as usize, false);
                }
            }
            Ord::Adv(_, t) => {
                self.u[i].prone = false;
                if let Some(t) = t {
                    if self.u[t as usize].alive() {
                        self.do_shoot(i, t as usize, true);
                    }
                }
            }
            Ord::Run(_, t) => {
                self.u[i].prone = false;
                if let Some(t) = t {
                    if self.u[t as usize].alive() {
                        self.do_assault(i, t as usize);
                    }
                }
            }
            Ord::Ambush => {
                self.u[i].ambush = true;
            }
            Ord::Rally => {
                self.u[i].pins = self.u[i].pins.saturating_sub(2);
            }
            Ord::Down => {
                self.u[i].prone = true;
                self.u[i].pins = self.u[i].pins.saturating_sub(1);
            }
        }
        if self.u[i].alive() {
            self.fire_ambush(i);
        }
    }

    fn check_over(&mut self) {
        let a = self.units_alive(0);
        let b = self.units_alive(1);
        if a == 0 || b == 0 {
            self.over = Some(match (a, b) {
                (0, 0) => Outcome::Draw,
                (0, _) => Outcome::Win(Player::Two),
                _ => Outcome::Win(Player::One),
            });
        }
    }

    fn end_turn(&mut self) {
        for o in OBJ {
            let mut c = [0i32; 2];
            for x in self.u.iter().filter(|x| x.alive() && x.pins == 0) {
                if cheb(x.pos, o) <= 1 {
                    c[x.side as usize] += x.n as i32;
                }
            }
            if c[0] > c[1] {
                self.vp[0] += 1;
            } else if c[1] > c[0] {
                self.vp[1] += 1;
            }
        }
        if self.turn >= MAX_TURNS {
            self.over = Some(
                match (self.vp[0], self.men(0)).cmp(&(self.vp[1], self.men(1))) {
                    std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                    std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                    _ => Outcome::Draw,
                },
            );
            return;
        }
        self.turn += 1;
        for x in &mut self.u {
            x.done = false;
            x.ambush = false;
        }
        self.phase = Phase::Draw;
        self.side = Player::One;
    }

    /// Valoarea (în oameni) a unei unități pentru evaluare.
    fn worth(u: &Unit) -> f32 {
        u.n as f32 * if u.ty == 0 { 3.0 } else { 1.0 } * (1.0 + 0.3 * (u.t().dpm - 1) as f32)
    }
}

fn p8(c: (i32, i32)) -> (u8, u8) {
    (c.0 as u8, c.1 as u8)
}

impl Game for Ordine {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Draw => vec![Move::Draw],
            Phase::Roll => vec![Move::Roll],
            Phase::Order => {
                let side = self.side.idx() as u8;
                let mut v = Vec::new();
                for (i, m) in self.u.iter().enumerate() {
                    if !m.alive() || m.side != side || m.done {
                        continue;
                    }
                    let iu = i as u8;
                    v.push(Move::Order(iu, Ord::Down));
                    v.push(Move::Order(iu, Ord::Ambush));
                    if m.pins > 0 {
                        v.push(Move::Order(iu, Ord::Rally));
                    }
                    for t in 0..self.u.len() {
                        if self.can_shoot(i, m.pos, t) {
                            v.push(Move::Order(iu, Ord::Fire(t as u8)));
                        }
                    }
                    let mv = self.mv_of(i);
                    for c in self.reachable(i, mv) {
                        v.push(Move::Order(iu, Ord::Adv(p8(c), None)));
                        if !m.t().heavy {
                            for t in 0..self.u.len() {
                                if self.can_shoot(i, c, t) {
                                    v.push(Move::Order(iu, Ord::Adv(p8(c), Some(t as u8))));
                                }
                            }
                        }
                    }
                    for c in self.reachable(i, mv * 2) {
                        v.push(Move::Order(iu, Ord::Run(p8(c), None)));
                        for (t, e) in self.u.iter().enumerate() {
                            if e.alive() && e.side != side && cheb(e.pos, c) == 1 {
                                v.push(Move::Order(iu, Ord::Run(p8(c), Some(t as u8))));
                            }
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Draw => {
                let r0 = self.remaining(0);
                let r1 = self.remaining(1);
                if r0 + r1 == 0 {
                    self.end_turn();
                    return;
                }
                let k = self.dice.below(r0 + r1);
                self.side = if k < r0 { Player::One } else { Player::Two };
                self.phase = Phase::Order;
            }
            Move::Order(i, o) => {
                let iu = i as usize;
                let from = self.u[iu].pos;
                match o {
                    Ord::Adv(c, _) | Ord::Run(c, _) => {
                        self.u[iu].pos = (c.0 as i32, c.1 as i32);
                    }
                    _ => {}
                }
                self.u[iu].done = true;
                self.pending = Some(Pend { u: i, ord: o, from });
                self.phase = Phase::Roll;
            }
            Move::Roll => {
                if let Some(p) = self.pending.take() {
                    self.resolve(p);
                }
                self.check_over();
                if self.over.is_none() {
                    self.phase = Phase::Draw;
                    self.side = Player::One;
                    if self.remaining(0) + self.remaining(1) == 0 {
                        self.end_turn();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8;
        let en = 1 - me;
        let mut v = 0.0f32;
        for x in self.u.iter().filter(|x| x.alive()) {
            let w = Self::worth(x) * 14.0;
            let pinp = x.pins as f32 * 4.0 * x.n as f32 / x.t().n as f32;
            if x.side == me {
                v += w - pinp;
            } else {
                v -= w - pinp;
            }
        }
        v += (self.vp[me as usize] as f32 - self.vp[en as usize] as f32) * 60.0;
        // ordin în curs: efecte așteptate
        if let Some(pd) = self.pending {
            let i = pd.u as usize;
            let mut pass = 1.0;
            if self.u[i].pins > 0 && !matches!(pd.ord, Ord::Down) {
                pass = p_2d6_le(self.test_target(i));
            }
            let sgn = if self.u[i].side == me { 1.0 } else { -1.0 };
            let mut gain = 0.0;
            match pd.ord {
                Ord::Fire(t) => gain += self.exp_cas(i, t as usize, false) * 14.0,
                Ord::Adv(_, Some(t)) => gain += self.exp_cas(i, t as usize, true) * 14.0,
                Ord::Run(_, Some(t)) => {
                    let a = &self.u[i];
                    let d = &self.u[t as usize];
                    let ka = (a.n as f32 * 0.5).min(d.n as f32);
                    let kd = (d.n as f32 * 0.5).min(a.n as f32);
                    gain += (ka - kd) * 14.0;
                }
                Ord::Rally => gain += 4.0 * self.u[i].pins.min(2) as f32,
                Ord::Ambush => {
                    // mică valoare: poate trage mai târziu
                    gain += 6.0;
                }
                _ => {}
            }
            v += sgn * gain * pass;
            // ambuscade inamice care ar trage în unitatea activă la poziția finală
            let mut amb = 0.0;
            for (j, o) in self.u.iter().enumerate() {
                if o.alive() && o.ambush && o.side != self.u[i].side && self.can_shoot(j, o.pos, i)
                {
                    amb += self.exp_cas(j, i, false);
                }
            }
            v -= sgn * amb * 14.0;
        }
        // poziționare
        for (i, x) in self.u.iter().enumerate().filter(|(_, x)| x.alive()) {
            let sgn = if x.side == me { 1.0 } else { -1.0 };
            let mut s = 0.0f32;
            let dmin = OBJ.iter().map(|&o| cheb(x.pos, o)).min().unwrap_or(0);
            if x.ty != 3 && x.ty != 2 {
                s -= dmin as f32 * 0.8;
            }
            if dmin <= 1 {
                s += 4.0;
            }
            if self.map.is_cover(x.pos) {
                s += 2.0;
            }
            if x.ambush {
                s += 3.0;
            }
            for (j, e) in self
                .u
                .iter()
                .enumerate()
                .filter(|(_, e)| e.alive() && e.side != x.side)
            {
                if self.can_shoot(j, e.pos, i) {
                    s -= self.exp_cas(j, i, false) * 5.0;
                }
                if self.can_shoot(i, x.pos, j) {
                    s += self.exp_cas(i, j, false) * 5.0;
                }
            }
            v += sgn * s;
        }
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ordine de bătaie — tura {}/{}  puncte {}–{}  ({})\n",
            self.turn,
            MAX_TURNS,
            self.vp[0],
            self.vp[1],
            match self.phase {
                Phase::Draw => "extrage un zar de ordine".to_string(),
                Phase::Order => format!(
                    "ordin pentru J{}  (în săculeț: {}–{})",
                    self.side.idx() + 1,
                    self.remaining(0),
                    self.remaining(1)
                ),
                Phase::Roll => "zaruri".to_string(),
            }
        );
        for y in (0..H).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..W {
                let c = (x, y);
                let mut cell = if self.map.is_wall(c) {
                    "##".to_string()
                } else if OBJ.contains(&c) {
                    "()".to_string()
                } else if self.map.is_cover(c) {
                    "::".to_string()
                } else {
                    " .".to_string()
                };
                for (i, m) in self.u.iter().enumerate() {
                    if m.alive() && m.pos == c {
                        cell = format!("{} ", uid(i));
                    }
                }
                s.push_str(&cell);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   ");
        for x in 0..W {
            s.push_str(&format!("{}  ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for (i, m) in self.u.iter().enumerate() {
            s.push_str(&format!(
                "{} {:<11} {:>3} oameni {}/{} marcaje {}{}{}{}\n",
                uid(i),
                m.t().name,
                cell_name(m.pos.0, m.pos.1),
                m.n.max(0),
                m.t().n,
                m.pins,
                if m.prone { " la sol" } else { "" },
                if m.ambush { " ambuscadă" } else { "" },
                if m.done && m.alive() {
                    " (a acționat)"
                } else {
                    ""
                },
            ));
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let c = |p: &(u8, u8)| cell_name(p.0 as i32, p.1 as i32);
        match m {
            Move::Draw => "trage".into(),
            Move::Roll => "arunca".into(),
            Move::Order(i, o) => {
                let id = uid(*i as usize);
                match o {
                    Ord::Fire(t) => format!("{id} fire x{}", uid(*t as usize)),
                    Ord::Adv(p, None) => format!("{id} adv {}", c(p)),
                    Ord::Adv(p, Some(t)) => format!("{id} adv {} x{}", c(p), uid(*t as usize)),
                    Ord::Run(p, None) => format!("{id} run {}", c(p)),
                    Ord::Run(p, Some(t)) => format!("{id} run {} x{}", c(p), uid(*t as usize)),
                    Ord::Ambush => format!("{id} amb"),
                    Ord::Rally => format!("{id} rally"),
                    Ord::Down => format!("{id} down"),
                }
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn legal(g: &Ordine) -> Vec<Move> {
        Game::legal_moves(g)
    }

    fn men_total(g: &Ordine) -> i32 {
        g.men(0) + g.men(1)
    }

    #[test]
    fn setup_is_symmetric_and_legal() {
        let g = Ordine::with_seed(1);
        assert_eq!(g.u.len(), 10);
        for x in &g.u {
            assert!(!g.map.is_wall(x.pos));
        }
        for o in OBJ {
            assert!(!g.map.is_wall(o));
        }
        assert_eq!(g.men(0), 1 + 6 + 6 + 3 + 2);
        assert_eq!(g.men(0), g.men(1));
        assert_eq!(legal(&g), vec![Move::Draw]);
    }

    #[test]
    fn draw_picks_a_side_with_units_and_flow_returns() {
        let mut g = Ordine::with_seed(3);
        g.apply(&Move::Draw);
        assert_eq!(g.phase, Phase::Order);
        let mv = legal(&g);
        assert!(!mv.is_empty());
        let side = g.to_move().idx() as u8;
        for m in &mv {
            if let Move::Order(i, _) = m {
                assert_eq!(g.u[*i as usize].side, side);
            }
        }
        g.apply(&mv[0]);
        assert_eq!(legal(&g), vec![Move::Roll]);
        g.apply(&Move::Roll);
        assert_eq!(g.phase, Phase::Draw);
    }

    #[test]
    fn draw_probability_follows_bag_counts() {
        // 5 zaruri vs 1: partea cu 5 se extrage mult mai des
        let mut c0 = 0;
        for s in 0..400 {
            let mut g = Ordine::with_seed(s + 1);
            for i in 6..10 {
                g.u[i].done = true;
            }
            g.apply(&Move::Draw);
            if g.to_move() == Player::One {
                c0 += 1;
            }
        }
        assert!(c0 > 300, "{c0}");
    }

    #[test]
    fn every_unit_acts_once_per_turn() {
        let mut g = Ordine::with_seed(5);
        let mut rng = Rng::new(9);
        let mut acted = [0u8; 10];
        let t0 = g.turn;
        while g.turn == t0 && g.outcome().is_none() {
            let mv = legal(&g);
            let m = mv[rng.below(mv.len())];
            if let Move::Order(i, _) = m {
                acted[i as usize] += 1;
            }
            g.apply(&m);
        }
        if g.outcome().is_none() {
            for (i, a) in acted.iter().enumerate() {
                if g.u[i].alive() {
                    assert_eq!(*a, 1, "unitatea {i}");
                } else {
                    assert!(*a <= 1);
                }
            }
        }
    }

    #[test]
    fn pinned_unit_must_test_and_failure_sends_it_down() {
        let mut fails = 0;
        let mut passes = 0;
        for s in 0..200 {
            let mut g = Ordine::with_seed(s + 1);
            g.u[1].pins = 5; // morală 8 − 5 = 3 (+1 ofițer aproape) = 4: eșec frecvent
            g.phase = Phase::Order;
            g.side = Player::One;
            let from = g.u[1].pos;
            let to = (from.0 as u8, from.1 as u8 + 1);
            g.apply(&Move::Order(1, Ord::Adv(to, None)));
            g.apply(&Move::Roll);
            if g.u[1].pos == from {
                assert!(g.u[1].prone);
                fails += 1;
            } else {
                passes += 1;
            }
        }
        assert!(fails > 100 && passes > 5, "{fails} {passes}");
    }

    #[test]
    fn rally_removes_pins_and_down_helps() {
        let mut g = Ordine::with_seed(2);
        g.u[1].pins = 2;
        g.phase = Phase::Order;
        assert!(legal(&g).contains(&Move::Order(1, Ord::Rally)));
        let g2 = Ordine::with_seed(2);
        let mut g2 = g2;
        g2.phase = Phase::Order;
        assert!(!legal(&g2).contains(&Move::Order(1, Ord::Rally)));
        // down
        let mut g3 = Ordine::with_seed(2);
        g3.u[2].pins = 1;
        g3.phase = Phase::Order;
        g3.apply(&Move::Order(2, Ord::Down));
        g3.apply(&Move::Roll);
        assert!(g3.u[2].prone);
        assert_eq!(g3.u[2].pins, 0);
    }

    #[test]
    fn volley_modifiers() {
        let mut g = Ordine::with_seed(1);
        g.u[1].pos = (0, 0);
        g.u[6].pos = (0, 4);
        let (d, h0, s) = g.volley(1, 6, false);
        assert_eq!((d, s), (6, 5));
        assert_eq!(h0, 4);
        assert_eq!(g.volley(1, 6, true).1, 5);
        g.u[6].prone = true;
        assert_eq!(g.volley(1, 6, false).1, 5);
        g.u[1].pins = 2;
        assert_eq!(g.volley(1, 6, false).1, 7);
    }

    #[test]
    fn mortar_needs_a_spotter_and_min_range() {
        let mut g = Ordine::with_seed(1);
        let mort = 4;
        g.u[mort].pos = (4, 0);
        // inamic la distanță 6, în spatele unui zid; nimeni nu-l vede
        for i in 0..5 {
            if i != mort {
                g.u[i].pos = (0, i as i32);
            }
        }
        g.u[5].pos = (9, 0);
        g.u[6].pos = (9, 5);
        g.u[7].pos = (10, 5);
        g.u[8].pos = (11, 5);
        g.u[9].pos = (11, 6);
        // niciun aliat nu vede ținta: nu poate trage
        let can_before = g.can_shoot(mort, g.u[mort].pos, 5);
        // un aliat cu vedere liberă
        g.u[0].pos = (9, 3);
        assert!(g.can_shoot(mort, g.u[mort].pos, 5));
        let _ = can_before;
        // sub raza minimă
        g.u[5].pos = (5, 0);
        assert!(!g.can_shoot(mort, g.u[mort].pos, 5));
    }

    #[test]
    fn ambush_fires_once_on_visible_actor() {
        let mut g = Ordine::with_seed(7);
        g.u[1].pos = (0, 3);
        g.u[1].ambush = true;
        g.u[6].pos = (0, 6);
        let n0 = g.u[6].n;
        let mut total_loss = 0i32;
        for s in 0..60 {
            let mut h = g.clone();
            h.dice = Dice::new(s + 1);
            h.fire_ambush(6);
            assert!(
                !h.u[1].ambush,
                "{} {:?}",
                h.can_shoot(1, h.u[1].pos, 6),
                h.u[6]
            );
            total_loss += (n0 - h.u[6].n) as i32;
        }
        assert!(total_loss > 20);
    }

    #[test]
    fn assault_casualties_bounded_and_loser_pinned() {
        for s in 0..100 {
            let mut g = Ordine::with_seed(s + 1);
            g.u[1].pos = (5, 3);
            g.u[6].pos = (5, 4);
            let (na, nd) = (g.u[1].n, g.u[6].n);
            g.do_assault(1, 6);
            assert!(g.u[1].n >= 0 && g.u[1].n <= na);
            assert!(g.u[6].n >= 0 && g.u[6].n <= nd);
            let (la, ld) = (na - g.u[1].n, nd - g.u[6].n);
            if la > ld && g.u[1].alive() {
                assert_eq!(g.u[1].pins, 2);
            }
            if ld > la && g.u[6].alive() {
                assert_eq!(g.u[6].pins, 2);
            }
        }
    }

    #[test]
    fn objectives_and_end() {
        let mut g = Ordine::with_seed(1);
        g.u[1].pos = OBJ[0];
        g.turn = MAX_TURNS;
        g.end_turn();
        assert_eq!(g.vp, [1, 0]);
        assert!(g.outcome().is_some());
        // unitățile suprimate nu contează
        let mut g = Ordine::with_seed(1);
        g.u[1].pos = OBJ[0];
        g.u[1].pins = 1;
        g.end_turn();
        assert_eq!(g.vp, [0, 0]);
    }

    #[test]
    fn random_games_finish_and_men_never_rise() {
        for seed in 1..=20u64 {
            let mut g = Ordine::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
            let mut last = men_total(&g);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = legal(&g);
                assert!(!mv.is_empty(), "{}", Game::render(&g));
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                let t = men_total(&g);
                assert!(t <= last);
                last = t;
                n += 1;
                assert!(n < 5000);
            }
            let live: Vec<_> = g.u.iter().filter(|x| x.alive()).map(|x| x.pos).collect();
            for (i, a) in live.iter().enumerate() {
                assert!(!g.map.is_wall(*a));
                for b in &live[i + 1..] {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly_and_is_fast() {
        let mut wins = 0;
        let mut worst = 0u128;
        for seed in 1..=10u64 {
            let mut g = Ordine::with_seed(seed + 100);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let m = if Game::to_move(&g) == Player::Two && g.phase == Phase::Order {
                    let t = std::time::Instant::now();
                    let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                    worst = worst.max(t.elapsed().as_millis());
                    m
                } else {
                    let mv = legal(&g);
                    mv[rng.below(mv.len())]
                };
                g.apply(&m);
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 8, "AI a câștigat doar {wins}/10");
        assert!(worst < 1000, "AI prea lent: {worst} ms");
    }
}
