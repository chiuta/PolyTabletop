//! „Gridiron” — sport fantastic pe ture cu pase și tăvăleli (id `gridiron`).
//!
//! Motor generic, original: două echipe de 5 jucători pe un teren 12×7 (zone de
//! punct pe coloanele `a` și `l`). Fiecare echipă joacă 6 ture pe repriză, 2 reprize.
//! Pe tura ta fiecare jucător poate face O acțiune; tura se termină cu `fin` sau
//! la „pierderea turei” (turnover).
//!
//! Jucători (original): Pasator (A/V: mv 6, forță 3, agilitate 3+, armură 8),
//! Prinzător (B/W: mv 7, forță 2, 3+, 7, prinde cu +1), Alergător (C/X: mv 7, forță 2,
//! 3+, 7), Zid (D/Y: mv 5, forță 3, 4+, 8), Trântor (E/Z: mv 4, forță 4, 4+, 9).
//!
//! Reguli (scrise de noi):
//! * `B>d4`: mută (și se ridică, dacă e la pământ: costă 3 din mv). Până la mv
//!   pătrate + 2 pătrate „în plus” (zar 2+). Părăsirea unui pătrat vecin cu un inamic în
//!   picioare cere zar de ESCAPARE (agilitate −1, +1 pe fiecare inamic vecin cu
//!   pătratul nou); intrarea pe mingea liberă cere zar de PRINDERE (agilitate, +1 pe
//!   inamic vecin). Zarul d6: 1 eșuează mereu, 6 reușește mereu. Un eșec = căzut
//!   (zar de armură) și pierderea turei.
//! * `B xK`: tăvăleală cu un inamic în picioare vecin. Zaruri de tăvăleală: forță
//!   proprie (+1 pe coechipier în picioare vecin cu ținta) față de forța ținta (+1
//!   pe inamic vecin cu atacatorul): mai mare = 2 zaruri (3 la dublu) alese de atacator,
//!   mai mică = 2 (3) alese de apărător, egal = 1. Față: 1 atacator jos (pierde tura),
//!   2 ambii jos (pierde tura), 3–4 împins, 5–6 apărător împins și doborât. Împins în
//!   afara terenului = scos (pleacă până la următoarea repriză/punct).
//! * Doborât: zar de armură 2d6 > armura = rănit și scos din joc până la următorul
//!   punct. Purtătorul mingii doborât scapă mingea (revine aleator în jur).
//! * `B>d4 xK`: asalt (o dată pe tură): mută și apoi tăvălește.
//! * `B pass K`: pasă (o dată pe tură) de către purtător, către un coechipier în
//!   picioare la ≤ 8: zar 3+ la ≤ 3, 4+ la ≤ 6, 5+ la ≤ 8 (agilitate mai slabă = +1; −1
//!   pe inamic vecin cu pasatorul). Apoi prinderea (agilitate, −1 prinzător). Orice
//!   eșec = mingea scapă și pierzi tura. `B hand K`: predare la vecin (prindere −1).
//! * `B sus`: ridică-te. Punct: purtătorul în picioare ajunge în zona de punct adversă
//!   = 1 punct; după punct, echipa adversă primește (minge liberă pe jumătatea ei).
//! * Final: după 2 reprize, mai multe puncte câștigă; egal = remiză.
//!
//! Simplificări: fără interceptări, fără urmărire după împingere, împingerea în
//! lanț = doborâre pe loc. Zarurile se aruncă la mutarea forțată `arunca`; la
//! începutul fiecărui punct `lovitura` pune mingea.
//! Mutări: `A>d4`, `A>d4 xV`, `A xV`, `A pass B`, `A hand B`, `A sus`, `fin`,
//! `lovitura`, `arunca`.

use crate::w7_util::{cell_name, cheb, Dice};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const W: i32 = 12;
pub const H: i32 = 7;
const TURNS: u8 = 6;

pub struct PTy {
    pub name: &'static str,
    pub ma: i32,
    pub st: i32,
    pub ag: i32,
    pub av: i32,
    pub catch_bonus: i32,
}

pub const TYPES: [PTy; 5] = [
    PTy {
        name: "Pasator",
        ma: 6,
        st: 3,
        ag: 3,
        av: 8,
        catch_bonus: 0,
    },
    PTy {
        name: "Prinzător",
        ma: 7,
        st: 2,
        ag: 3,
        av: 7,
        catch_bonus: 1,
    },
    PTy {
        name: "Alergător",
        ma: 7,
        st: 2,
        ag: 3,
        av: 7,
        catch_bonus: 0,
    },
    PTy {
        name: "Zid",
        ma: 5,
        st: 3,
        ag: 4,
        av: 8,
        catch_bonus: 0,
    },
    PTy {
        name: "Trântor",
        ma: 4,
        st: 4,
        ag: 4,
        av: 9,
        catch_bonus: 0,
    },
];
const FORM: [(i32, i32); 5] = [(2, 3), (4, 1), (4, 5), (3, 2), (3, 4)];

fn pid(i: usize) -> char {
    ['A', 'B', 'C', 'D', 'E', 'V', 'W', 'X', 'Y', 'Z'][i]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum St {
    Up,
    Down,
    Out,
}

#[derive(Clone, Debug)]
pub struct Pl {
    pub ty: u8,
    pub side: u8,
    pub pos: (i32, i32),
    pub st: St,
    pub acted: bool,
}

impl Pl {
    fn t(&self) -> &'static PTy {
        &TYPES[self.ty as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ball {
    Carried(u8),
    Loose((i32, i32)),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Go(u8, (u8, u8), Option<u8>),
    Block(u8, u8),
    Stand(u8),
    Pass(u8, u8),
    Hand(u8, u8),
    End,
    Kick,
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Kick,
    Play,
    Roll,
}

/// Un pas al unui traseu cu zarurile lui.
#[derive(Clone, Copy, Debug)]
struct Step {
    to: (i32, i32),
    dodge: Option<i32>,
    gfi: bool,
    pick: Option<i32>,
}

#[derive(Clone)]
pub struct Gridiron {
    pub pl: Vec<Pl>,
    pub ball: Ball,
    pub score: [u8; 2],
    pub half: u8,
    turns_left: [u8; 2],
    phase: Phase,
    side: Player,
    pending: Option<Move>,
    blitz_used: bool,
    pass_used: bool,
    hand_used: bool,
    turnover: bool,
    dice: Dice,
    over: Option<Outcome>,
    log: Vec<String>,
    /// Cine a primit în prima repriză (celălalt primește în a doua).
    first_receiver: u8,
}

impl Default for Gridiron {
    fn default() -> Self {
        Self::new()
    }
}

fn p_ok(t: i32) -> f32 {
    if t <= 2 {
        5.0 / 6.0
    } else if t >= 6 {
        1.0 / 6.0
    } else {
        (7 - t) as f32 / 6.0
    }
}

fn inside(c: (i32, i32)) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < W && c.1 < H
}

fn cat_of(face: u8) -> usize {
    match face {
        5 | 6 => 0,
        3 | 4 => 1,
        2 => 2,
        _ => 3,
    }
}

/// Probabilități (apărător doborât, împins, ambii jos, atacator jos) pentru n zaruri.
pub fn block_probs(n: u32, attacker_chooses: bool) -> [f32; 4] {
    let mut out = [0.0f32; 4];
    let total = 6u32.pow(n);
    for k in 0..total {
        let mut x = k;
        let mut best = if attacker_chooses { 3 } else { 0 };
        for _ in 0..n {
            let c = cat_of((x % 6) as u8 + 1);
            x /= 6;
            best = if attacker_chooses {
                best.min(c)
            } else {
                best.max(c)
            };
        }
        out[best] += 1.0 / total as f32;
    }
    out
}

impl Gridiron {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut pl = Vec::new();
        for side in 0..2u8 {
            for (k, &(x, y)) in FORM.iter().enumerate() {
                let pos = if side == 0 {
                    (x, y)
                } else {
                    (W - 1 - x, H - 1 - y)
                };
                pl.push(Pl {
                    ty: k as u8,
                    side,
                    pos,
                    st: St::Up,
                    acted: false,
                });
            }
        }
        Gridiron {
            pl,
            ball: Ball::Loose((W / 2, H / 2)),
            score: [0, 0],
            half: 1,
            turns_left: [TURNS, TURNS],
            phase: Phase::Kick,
            side: Player::One,
            pending: None,
            blitz_used: false,
            pass_used: false,
            hand_used: false,
            turnover: false,
            dice: Dice::new(seed),
            over: None,
            log: Vec::new(),
            first_receiver: 0,
        }
    }

    // ---- geometrie ----

    fn at(&self, c: (i32, i32)) -> Option<usize> {
        self.pl.iter().position(|p| p.st != St::Out && p.pos == c)
    }

    fn tz(&self, side: u8, c: (i32, i32)) -> i32 {
        // inamici în picioare vecini cu c
        self.pl
            .iter()
            .filter(|p| p.st == St::Up && p.side != side && cheb(p.pos, c) == 1)
            .count() as i32
    }

    fn carrier(&self) -> Option<usize> {
        match self.ball {
            Ball::Carried(c) => Some(c as usize),
            _ => None,
        }
    }

    fn end_col(side: u8) -> i32 {
        if side == 0 {
            W - 1
        } else {
            0
        }
    }

    fn ma_eff(&self, i: usize) -> i32 {
        let p = &self.pl[i];
        p.t().ma - if p.st == St::Down { 3 } else { 0 }
    }

    /// Trasee cu probabilitate maximă către fiecare pătrat accesibil.
    fn reach(&self, i: usize) -> Vec<((i32, i32), f32, Vec<Step>)> {
        let p = &self.pl[i];
        let maxs = (self.ma_eff(i) + 2).max(0) as usize;
        let n = (W * H) as usize;
        let ix = |c: (i32, i32)| (c.1 * W + c.0) as usize;
        let mut best: Vec<Vec<Option<(f32, usize)>>> = vec![vec![None; n]; maxs + 1];
        best[0][ix(p.pos)] = Some((1.0, usize::MAX));
        let carrying = self.carrier() == Some(i);
        for s in 0..maxs {
            for sq in 0..n {
                let Some((pr, _)) = best[s][sq] else { continue };
                let c = (sq as i32 % W, sq as i32 / W);
                let leaving_tz = self.tz(p.side, c) > 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nb = (c.0 + dx, c.1 + dy);
                        if !inside(nb) {
                            continue;
                        }
                        if let Some(o) = self.at(nb) {
                            if o != i {
                                continue;
                            }
                        }
                        let mut q = pr;
                        if leaving_tz {
                            q *= p_ok(self.dodge_target(i, nb));
                        }
                        if (s as i32 + 1) > self.ma_eff(i) {
                            q *= p_ok(2);
                        }
                        if !carrying {
                            if let Ball::Loose(b) = self.ball {
                                if b == nb {
                                    q *= p_ok(p.t().ag + self.tz(p.side, nb));
                                }
                            }
                        }
                        let cur = best[s + 1][ix(nb)];
                        if cur.map(|x| q > x.0).unwrap_or(true) {
                            best[s + 1][ix(nb)] = Some((q, sq));
                        }
                    }
                }
            }
        }
        let mut out = Vec::new();
        for sq in 0..n {
            let c = (sq as i32 % W, sq as i32 / W);
            if c == p.pos {
                continue;
            }
            // cel mai bun strat (probabilitate max, apoi pași puțini)
            let mut pick: Option<(usize, f32)> = None;
            for (s, layer) in best.iter().enumerate().skip(1) {
                if let Some((pr, _)) = layer[sq] {
                    if pick.map(|x| pr > x.1 + 1e-6).unwrap_or(true) {
                        pick = Some((s, pr));
                    }
                }
            }
            let Some((s0, pr)) = pick else { continue };
            // reconstituire
            let mut sqs = vec![sq];
            let mut s = s0;
            let mut cur = sq;
            while s > 0 {
                let (_, par) = best[s][cur].unwrap();
                sqs.push(par);
                cur = par;
                s -= 1;
            }
            sqs.reverse();
            let mut steps = Vec::new();
            for w in 1..sqs.len() {
                let a = (sqs[w - 1] as i32 % W, sqs[w - 1] as i32 / W);
                let b = (sqs[w] as i32 % W, sqs[w] as i32 / W);
                steps.push(Step {
                    to: b,
                    dodge: if self.tz(p.side, a) > 0 {
                        Some(self.dodge_target(i, b))
                    } else {
                        None
                    },
                    gfi: w as i32 > self.ma_eff(i),
                    pick: match self.ball {
                        Ball::Loose(bq) if bq == b && !carrying => {
                            Some(p.t().ag + self.tz(p.side, b))
                        }
                        _ => None,
                    },
                });
            }
            out.push((c, pr, steps));
        }
        out
    }

    fn dodge_target(&self, i: usize, to: (i32, i32)) -> i32 {
        let p = &self.pl[i];
        (p.t().ag + self.tz(p.side, to) - 1).clamp(2, 6)
    }

    fn catch_target(&self, i: usize, bonus: i32) -> i32 {
        let p = &self.pl[i];
        (p.t().ag + self.tz(p.side, p.pos) - p.t().catch_bonus - bonus).clamp(2, 6)
    }

    fn pass_target(&self, a: usize, b: usize) -> Option<i32> {
        let d = cheb(self.pl[a].pos, self.pl[b].pos);
        if !(1..=8).contains(&d) {
            return None;
        }
        let base = if d <= 3 {
            3
        } else if d <= 6 {
            4
        } else {
            5
        };
        let p = &self.pl[a];
        let t = base + (p.t().ag - 3) + self.tz(p.side, p.pos);
        Some(t.clamp(2, 6))
    }

    fn strengths(&self, a: usize, d: usize) -> (i32, i32) {
        let pa = &self.pl[a];
        let pd = &self.pl[d];
        let ast = self
            .pl
            .iter()
            .enumerate()
            .filter(|(j, p)| {
                *j != a && p.st == St::Up && p.side == pa.side && cheb(p.pos, pd.pos) == 1
            })
            .count() as i32;
        let dst = self
            .pl
            .iter()
            .enumerate()
            .filter(|(j, p)| {
                *j != d && p.st == St::Up && p.side == pd.side && cheb(p.pos, pa.pos) == 1
            })
            .count() as i32;
        (pa.t().st + ast, pd.t().st + dst)
    }

    /// (zaruri, atacatorul alege?)
    fn block_dice(&self, a: usize, d: usize) -> (u32, bool) {
        let (sa, sd) = self.strengths(a, d);
        if sa == sd {
            (1, true)
        } else if sa > sd {
            (if sa >= 2 * sd { 3 } else { 2 }, true)
        } else {
            (if sd >= 2 * sa { 3 } else { 2 }, false)
        }
    }

    // ---- zaruri ----

    fn roll_ok(&mut self, target: i32) -> bool {
        let r = self.dice.d6() as i32;
        r != 1 && (r == 6 || r >= target)
    }

    fn bounce(&mut self, from: (i32, i32), depth: u8) {
        let dirs = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let d = dirs[self.dice.below(8)];
        let mut to = (from.0 + d.0, from.1 + d.1);
        if !inside(to) {
            to = from;
        }
        if depth < 4 {
            if let Some(o) = self.at(to) {
                if self.pl[o].st == St::Up {
                    let t = self.catch_target(o, 0);
                    if self.roll_ok(t) {
                        self.ball = Ball::Carried(o as u8);
                        self.log.push(format!("{} prinde mingea", pid(o)));
                        return;
                    }
                    self.bounce(to, depth + 1);
                    return;
                }
                if self.pl[o].st == St::Down {
                    self.bounce(to, depth + 1);
                    return;
                }
            }
        }
        self.ball = Ball::Loose(to);
    }

    fn knock_down(&mut self, i: usize, mine_turn: bool) {
        let pos = self.pl[i].pos;
        self.pl[i].st = St::Down;
        if self.carrier() == Some(i) {
            self.ball = Ball::Loose(pos);
            self.bounce(pos, 0);
        }
        let r = self.dice.two_d6() as i32;
        if r > self.pl[i].t().av {
            self.log.push(format!("{} e rănit (armură {r})", pid(i)));
            self.pl[i].st = St::Out;
            self.pl[i].pos = (-1, -1);
        } else {
            self.log.push(format!("{} la pământ", pid(i)));
        }
        if mine_turn {
            self.turnover = true;
        }
    }

    fn active(&self) -> u8 {
        self.side.idx() as u8
    }

    fn do_block(&mut self, a: usize, d: usize) {
        let (n, att) = self.block_dice(a, d);
        let mut faces = Vec::new();
        for _ in 0..n {
            faces.push(self.dice.d6());
        }
        let cats: Vec<usize> = faces.iter().map(|&f| cat_of(f)).collect();
        let c = if att {
            *cats.iter().min().unwrap()
        } else {
            *cats.iter().max().unwrap()
        };
        let mine = self.pl[a].side == self.active();
        self.log.push(format!(
            "{} tăvălește {} ({} zaruri: {:?}) → {}",
            pid(a),
            pid(d),
            n,
            faces,
            ["apărător doborât", "împins", "ambii jos", "atacator jos"][c]
        ));
        match c {
            3 => self.knock_down(a, mine),
            2 => {
                self.knock_down(d, false);
                self.knock_down(a, mine);
            }
            1 => self.push(a, d, false),
            _ => self.push(a, d, true),
        }
    }

    fn push(&mut self, a: usize, d: usize, then_down: bool) {
        let pa = self.pl[a].pos;
        let pd = self.pl[d].pos;
        let sx = (pd.0 - pa.0).signum();
        let sy = (pd.1 - pa.1).signum();
        let cands: Vec<(i32, i32)> = if sx != 0 && sy != 0 {
            vec![(sx, sy), (sx, 0), (0, sy)]
        } else if sx != 0 {
            vec![(sx, 0), (sx, 1), (sx, -1)]
        } else {
            vec![(0, sy), (1, sy), (-1, sy)]
        };
        let mut dest: Option<(i32, i32)> = None;
        let mut crowd = false;
        for c in cands {
            let t = (pd.0 + c.0, pd.1 + c.1);
            if !inside(t) {
                crowd = true;
                continue;
            }
            if self.at(t).is_none() {
                dest = Some(t);
                break;
            }
        }
        match dest {
            Some(t) => {
                self.pl[d].pos = t;
                if let Ball::Carried(c) = self.ball {
                    if c as usize == d {
                        // rămâne purtător
                    }
                }
                if then_down {
                    self.knock_down(d, false);
                }
            }
            None => {
                if crowd {
                    self.log.push(format!("{} împins în public: scos", pid(d)));
                    let had = self.carrier() == Some(d);
                    self.pl[d].st = St::Out;
                    self.pl[d].pos = (-1, -1);
                    if had {
                        self.ball = Ball::Loose(pd);
                        self.bounce(pd, 0);
                    }
                } else if then_down {
                    self.knock_down(d, false);
                }
            }
        }
    }

    fn check_td(&mut self) -> bool {
        if let Some(c) = self.carrier() {
            let p = &self.pl[c];
            if p.st == St::Up && p.pos.0 == Self::end_col(p.side) {
                let s = p.side;
                self.score[s as usize] += 1;
                self.log.push(format!("PUNCT pentru J{}!", s + 1));
                return true;
            }
        }
        false
    }

    fn exec_go(&mut self, i: usize, to: (i32, i32)) -> bool {
        // întoarce true dacă acțiunea a mers fără pierderea turei
        let (steps, _) = {
            let r = self.reach(i);
            match r.into_iter().find(|x| x.0 == to) {
                Some((_, pr, st)) => (st, pr),
                None => return true,
            }
        };
        if self.pl[i].st == St::Down {
            self.pl[i].st = St::Up;
        }
        for s in steps {
            if let Some(t) = s.dodge {
                if !self.roll_ok(t) {
                    self.pl[i].pos = s.to;
                    self.log.push(format!("{} cade la escapare", pid(i)));
                    self.knock_down(i, true);
                    return false;
                }
            }
            if s.gfi && !self.roll_ok(2) {
                self.pl[i].pos = s.to;
                self.log.push(format!("{} cade în plus", pid(i)));
                self.knock_down(i, true);
                return false;
            }
            self.pl[i].pos = s.to;
            if let Some(t) = s.pick {
                if self.roll_ok(t) {
                    self.ball = Ball::Carried(i as u8);
                    self.log.push(format!("{} ridică mingea", pid(i)));
                } else {
                    self.log.push(format!("{} scapă mingea", pid(i)));
                    self.bounce(s.to, 0);
                    self.turnover = true;
                    return false;
                }
            }
        }
        true
    }

    fn resolve(&mut self, m: Move) {
        match m {
            Move::Go(p, to, blk) => {
                let i = p as usize;
                let ok = self.exec_go(i, (to.0 as i32, to.1 as i32));
                self.pl[i].acted = true;
                if ok {
                    if let Some(d) = blk {
                        self.blitz_used = true;
                        if self.pl[d as usize].st == St::Up {
                            self.do_block(i, d as usize);
                        }
                    }
                }
            }
            Move::Block(p, d) => {
                self.pl[p as usize].acted = true;
                self.do_block(p as usize, d as usize);
            }
            Move::Pass(p, t) => {
                let (a, b) = (p as usize, t as usize);
                self.pl[a].acted = true;
                self.pass_used = true;
                let tg = self.pass_target(a, b).unwrap_or(6);
                let r = self.dice.d6() as i32;
                let pos_a = self.pl[a].pos;
                if r == 1 {
                    self.log.push(format!("{} lasă mingea să-i scape", pid(a)));
                    self.ball = Ball::Loose(pos_a);
                    self.bounce(pos_a, 0);
                    self.turnover = true;
                } else if r == 6 || r >= tg {
                    let ct = self.catch_target(b, 0);
                    if self.roll_ok(ct) {
                        self.ball = Ball::Carried(t);
                        self.log
                            .push(format!("{} → {}: pasă prinsă", pid(a), pid(b)));
                    } else {
                        self.log.push(format!("{} scapă pasa", pid(b)));
                        let pb = self.pl[b].pos;
                        self.ball = Ball::Loose(pb);
                        self.bounce(pb, 0);
                        self.turnover = true;
                    }
                } else {
                    self.log.push(format!("pasa lui {} e proastă", pid(a)));
                    let pb = self.pl[b].pos;
                    self.ball = Ball::Loose(pb);
                    self.bounce(pb, 0);
                    self.turnover = true;
                }
            }
            Move::Hand(p, t) => {
                let (a, b) = (p as usize, t as usize);
                self.pl[a].acted = true;
                self.hand_used = true;
                let ct = self.catch_target(b, 1);
                if self.roll_ok(ct) {
                    self.ball = Ball::Carried(t);
                    self.log.push(format!("{} predă lui {}", pid(a), pid(b)));
                } else {
                    let pb = self.pl[b].pos;
                    self.ball = Ball::Loose(pb);
                    self.bounce(pb, 0);
                    self.turnover = true;
                }
            }
            _ => {}
        }
    }

    // ---- ture ----

    fn reset_formation(&mut self) {
        for (i, p) in self.pl.iter_mut().enumerate() {
            let k = i % 5;
            let (x, y) = FORM[k];
            p.pos = if p.side == 0 {
                (x, y)
            } else {
                (W - 1 - x, H - 1 - y)
            };
            p.st = St::Up;
            p.acted = false;
        }
    }

    fn finish_turn(&mut self, td: bool) {
        let me = self.active() as usize;
        self.turns_left[me] = self.turns_left[me].saturating_sub(1);
        for p in &mut self.pl {
            p.acted = false;
        }
        self.blitz_used = false;
        self.pass_used = false;
        self.hand_used = false;
        self.turnover = false;
        let other = 1 - me;
        let next = if self.turns_left[other] > 0 {
            Some(other)
        } else if self.turns_left[me] > 0 {
            Some(me)
        } else {
            None
        };
        match next {
            Some(s) => {
                self.side = if s == 0 { Player::One } else { Player::Two };
                if td {
                    self.phase = Phase::Kick;
                } else {
                    self.phase = Phase::Play;
                }
            }
            None => {
                if self.half == 1 {
                    self.half = 2;
                    self.turns_left = [TURNS, TURNS];
                    let rec = 1 - self.first_receiver as usize;
                    self.side = if rec == 0 { Player::One } else { Player::Two };
                    self.phase = Phase::Kick;
                } else {
                    self.over = Some(match self.score[0].cmp(&self.score[1]) {
                        std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                        std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                        _ => Outcome::Draw,
                    });
                }
            }
        }
    }

    // ---- evaluare ----

    fn board_value(&self, me: u8) -> f32 {
        let en = 1 - me;
        let mut v = (self.score[me as usize] as f32 - self.score[en as usize] as f32) * 1000.0;
        for p in &self.pl {
            let s = if p.side == me { 1.0 } else { -1.0 };
            let w = match p.st {
                St::Up => 30.0,
                St::Down => 12.0,
                St::Out => 0.0,
            };
            v += s * w;
        }
        let ballpos = match self.ball {
            Ball::Carried(c) => self.pl[c as usize].pos,
            Ball::Loose(b) => b,
        };
        match self.ball {
            Ball::Carried(c) => {
                let c = c as usize;
                let p = &self.pl[c];
                let s = if p.side == me { 1.0 } else { -1.0 };
                let prog = if p.side == 0 {
                    p.pos.0
                } else {
                    W - 1 - p.pos.0
                } as f32;
                let tzs = self.tz(p.side, p.pos) as f32;
                let sup = self
                    .pl
                    .iter()
                    .enumerate()
                    .filter(|(j, q)| {
                        *j != c && q.st == St::Up && q.side == p.side && cheb(q.pos, p.pos) == 1
                    })
                    .count() as f32;
                v += s * (45.0 + prog * 16.0 - tzs * 9.0 + sup * 3.0);
            }
            Ball::Loose(b) => {
                let near = |side: u8| {
                    self.pl
                        .iter()
                        .filter(|q| q.st == St::Up && q.side == side)
                        .map(|q| cheb(q.pos, b))
                        .min()
                        .unwrap_or(20) as f32
                };
                v += (near(en) - near(me)) * 8.0;
            }
        }
        for p in self.pl.iter().filter(|p| p.st == St::Up) {
            let s = if p.side == me { 1.0 } else { -1.0 };
            let d = cheb(p.pos, ballpos) as f32;
            v -= s * d * 0.4;
        }
        v
    }

    fn down_value(&self, i: usize) -> f32 {
        let p = &self.pl[i];
        let injury = (1.0 - Self::p_2d6_le(p.t().av)) * 30.0;
        18.0 + injury + if self.carrier() == Some(i) { 60.0 } else { 0.0 }
    }

    fn p_2d6_le(x: i32) -> f32 {
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

    fn block_ev(&self, a: usize, d: usize) -> f32 {
        let (n, att) = self.block_dice(a, d);
        let pr = block_probs(n, att);
        pr[0] * self.down_value(d)
            + pr[1] * 4.0
            + pr[2] * (self.down_value(d) - self.down_value(a) - 45.0)
            - pr[3] * (self.down_value(a) + 45.0)
    }
}

fn p8(c: (i32, i32)) -> (u8, u8) {
    (c.0 as u8, c.1 as u8)
}

impl Game for Gridiron {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Kick => vec![Move::Kick],
            Phase::Roll => vec![Move::Roll],
            Phase::Play => {
                let side = self.active();
                let mut v = vec![Move::End];
                for (i, p) in self.pl.iter().enumerate() {
                    if p.side != side || p.st == St::Out || p.acted {
                        continue;
                    }
                    let iu = i as u8;
                    if p.st == St::Down {
                        v.push(Move::Stand(iu));
                    }
                    for (c, _, _) in self.reach(i) {
                        v.push(Move::Go(iu, p8(c), None));
                        if !self.blitz_used {
                            for (j, e) in self.pl.iter().enumerate() {
                                if e.side != side && e.st == St::Up && cheb(e.pos, c) == 1 {
                                    v.push(Move::Go(iu, p8(c), Some(j as u8)));
                                }
                            }
                        }
                    }
                    if p.st == St::Up {
                        for (j, e) in self.pl.iter().enumerate() {
                            if e.side != side && e.st == St::Up && cheb(e.pos, p.pos) == 1 {
                                v.push(Move::Block(iu, j as u8));
                            }
                        }
                        if self.carrier() == Some(i) {
                            for (j, q) in self.pl.iter().enumerate() {
                                if j == i || q.side != side || q.st != St::Up {
                                    continue;
                                }
                                if !self.pass_used && self.pass_target(i, j).is_some() {
                                    v.push(Move::Pass(iu, j as u8));
                                }
                                if !self.hand_used && cheb(q.pos, p.pos) == 1 {
                                    v.push(Move::Hand(iu, j as u8));
                                }
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
            Move::End => self.finish_turn(false),
            Move::Kick => {
                self.reset_formation();
                // mingea liberă, pe jumătatea echipei care primește
                let rec = self.active();
                let (lo, hi) = if rec == 0 {
                    (0, W / 2 - 1)
                } else {
                    (W / 2, W - 1)
                };
                loop {
                    let x = lo + self.dice.below((hi - lo + 1) as usize) as i32;
                    let y = self.dice.below(H as usize) as i32;
                    if self.at((x, y)).is_none() {
                        self.ball = Ball::Loose((x, y));
                        break;
                    }
                }
                self.phase = Phase::Play;
            }
            Move::Stand(i) => {
                self.pl[i as usize].st = St::Up;
                self.pl[i as usize].acted = true;
            }
            Move::Roll => {
                if let Some(mv) = self.pending.take() {
                    self.resolve(mv);
                }
                let td = self.check_td();
                if td {
                    self.finish_turn(true);
                } else if self.turnover {
                    self.finish_turn(false);
                } else {
                    self.phase = Phase::Play;
                }
            }
            Move::Go(..) | Move::Block(..) | Move::Pass(..) | Move::Hand(..) => {
                self.pending = Some(*m);
                self.phase = Phase::Roll;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8;
        let base = self.board_value(me);
        let Some(m) = self.pending else {
            return base as i32;
        };
        let sgn = |side: u8| if side == me { 1.0 } else { -1.0 };
        let turn_pen = 60.0;
        let v = match m {
            Move::Go(pi, to, blk) => {
                let i = pi as usize;
                let to = (to.0 as i32, to.1 as i32);
                let r = self.reach(i);
                let Some((_, pr, steps)) = r.into_iter().find(|x| x.0 == to) else {
                    return base as i32;
                };
                let mut s = self.clone();
                s.pending = None;
                s.pl[i].st = St::Up;
                for st in &steps {
                    if st.pick.is_some() {
                        s.ball = Ball::Carried(pi);
                    }
                }
                s.pl[i].pos = to;
                let mut vs = s.board_value(me);
                if let Some(d) = blk {
                    vs += sgn(self.pl[i].side) * s.block_ev(i, d as usize);
                }
                // reușită cu TD
                if s.carrier() == Some(i) && to.0 == Self::end_col(self.pl[i].side) {
                    vs += sgn(self.pl[i].side) * 1000.0;
                }
                let mut f = self.clone();
                f.pending = None;
                f.pl[i].st = St::Down;
                if f.carrier() == Some(i) {
                    f.ball = Ball::Loose(f.pl[i].pos);
                }
                let vf = f.board_value(me) - sgn(self.pl[i].side) * turn_pen;
                pr * vs + (1.0 - pr) * vf
            }
            Move::Block(a, d) => {
                base + sgn(self.pl[a as usize].side) * self.block_ev(a as usize, d as usize)
            }
            Move::Pass(a, t) | Move::Hand(a, t) => {
                let ia = a as usize;
                let it = t as usize;
                let hand = matches!(m, Move::Hand(..));
                let pthrow = if hand {
                    1.0
                } else {
                    self.pass_target(ia, it).map(p_ok).unwrap_or(0.0)
                };
                let pc = p_ok(self.catch_target(it, if hand { 1 } else { 0 }));
                let mut s = self.clone();
                s.pending = None;
                s.ball = Ball::Carried(t);
                let mut vs = s.board_value(me);
                if self.pl[it].pos.0 == Self::end_col(self.pl[it].side) {
                    vs += sgn(self.pl[it].side) * 1000.0;
                }
                let mut f = self.clone();
                f.pending = None;
                f.ball = Ball::Loose(self.pl[it].pos);
                let vf = f.board_value(me) - sgn(self.pl[ia].side) * turn_pen;
                let ps = pthrow * pc;
                ps * vs + (1.0 - ps) * vf
            }
            _ => base,
        };
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Gridiron — repriza {}  ture rămase {}–{}  scor {}–{}  ({})\n",
            self.half,
            self.turns_left[0],
            self.turns_left[1],
            self.score[0],
            self.score[1],
            match self.phase {
                Phase::Kick => "lovitura",
                Phase::Play => "joc",
                Phase::Roll => "zaruri",
            }
        );
        for y in (0..H).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..W {
                let c = (x, y);
                let mut cell = if x == 0 || x == W - 1 {
                    " : ".to_string()
                } else {
                    " . ".to_string()
                };
                if self.ball == Ball::Loose(c) {
                    cell = " o ".to_string();
                }
                for (i, p) in self.pl.iter().enumerate() {
                    if p.st != St::Out && p.pos == c {
                        let mark = if self.carrier() == Some(i) {
                            '@'
                        } else if p.st == St::Down {
                            '_'
                        } else {
                            ' '
                        };
                        cell = format!("{}{} ", pid(i), mark);
                    }
                }
                s.push_str(&cell);
            }
            s.push('\n');
        }
        s.push_str("   ");
        for x in 0..W {
            s.push_str(&format!(" {} ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for (i, p) in self.pl.iter().enumerate() {
            s.push_str(&format!(
                "{} {:<10} {:>3} {}{}\n",
                pid(i),
                p.t().name,
                if p.st == St::Out {
                    "-".to_string()
                } else {
                    cell_name(p.pos.0, p.pos.1)
                },
                match p.st {
                    St::Up => "în picioare",
                    St::Down => "la pământ",
                    St::Out => "scos",
                },
                if p.acted { " (a acționat)" } else { "" }
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
            Move::End => "fin".into(),
            Move::Kick => "lovitura".into(),
            Move::Roll => "arunca".into(),
            Move::Stand(i) => format!("{} sus", pid(*i as usize)),
            Move::Go(i, to, None) => format!("{}>{}", pid(*i as usize), c(to)),
            Move::Go(i, to, Some(d)) => {
                format!("{}>{} x{}", pid(*i as usize), c(to), pid(*d as usize))
            }
            Move::Block(i, d) => format!("{} x{}", pid(*i as usize), pid(*d as usize)),
            Move::Pass(i, t) => format!("{} pass {}", pid(*i as usize), pid(*t as usize)),
            Move::Hand(i, t) => format!("{} hand {}", pid(*i as usize), pid(*t as usize)),
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

    fn legal(g: &Gridiron) -> Vec<Move> {
        Game::legal_moves(g)
    }

    fn kicked(seed: u64) -> Gridiron {
        let mut g = Gridiron::with_seed(seed);
        g.apply(&Move::Kick);
        g
    }

    fn clear_field(g: &mut Gridiron) {
        for p in g.pl.iter_mut() {
            p.st = St::Out;
            p.pos = (-1, -1);
        }
    }

    fn place(g: &mut Gridiron, i: usize, c: (i32, i32)) {
        g.pl[i].st = St::Up;
        g.pl[i].pos = c;
    }

    #[test]
    fn formation_and_kick() {
        let g = kicked(1);
        let mut seen = std::collections::HashSet::new();
        for p in &g.pl {
            assert!(seen.insert(p.pos));
            assert!(inside(p.pos));
        }
        match g.ball {
            Ball::Loose(b) => {
                assert!(b.0 < W / 2);
                assert!(g.at(b).is_none());
            }
            _ => panic!(),
        }
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn block_probabilities_sum_and_order() {
        for n in 1..=3 {
            for att in [true, false] {
                let p = block_probs(n, att);
                assert!((p.iter().sum::<f32>() - 1.0).abs() < 1e-5);
            }
        }
        let one = block_probs(1, true);
        assert!((one[0] - 1.0 / 3.0).abs() < 1e-5);
        assert!((one[1] - 1.0 / 3.0).abs() < 1e-5);
        // 2 zaruri alese de atacator: mai bine decât 1; alese de apărător: mai rău
        assert!(block_probs(2, true)[0] > one[0]);
        assert!(block_probs(2, false)[0] < one[0]);
        assert!(block_probs(3, true)[0] > block_probs(2, true)[0]);
    }

    #[test]
    fn block_dice_and_assists() {
        let mut g = kicked(2);
        clear_field(&mut g);
        place(&mut g, 4, (5, 3)); // Trântor ST4
        place(&mut g, 6, (6, 3)); // Prinzător inamic ST2
        assert_eq!(g.block_dice(4, 6), (3, true)); // 4 vs 2: dublu
        place(&mut g, 7, (7, 3)); // un coechipier inamic nu e vecin cu atacatorul
        assert_eq!(g.strengths(4, 6), (4, 2));
        place(&mut g, 8, (5, 2)); // inamic vecin cu atacatorul: +1 apărător
        assert_eq!(g.strengths(4, 6), (4, 3));
        assert_eq!(g.block_dice(4, 6), (2, true));
        place(&mut g, 3, (6, 4)); // coechipier vecin cu ținta: +1 atacator
        assert_eq!(g.strengths(4, 6), (5, 3));
        // egal → 1 zar
        let mut h = kicked(2);
        clear_field(&mut h);
        place(&mut h, 3, (5, 3));
        place(&mut h, 8, (6, 3)); // Zid vs Zid
        assert_eq!(h.block_dice(3, 8), (1, true));
        // mai slab: apărătorul alege
        place(&mut h, 1, (5, 4));
        let mut k = kicked(2);
        clear_field(&mut k);
        place(&mut k, 1, (5, 3));
        place(&mut k, 9, (6, 3));
        assert_eq!(k.block_dice(1, 9), (3, false));
    }

    #[test]
    fn reach_probabilities_and_limits() {
        let mut g = kicked(3);
        clear_field(&mut g);
        place(&mut g, 2, (2, 3)); // Alergător ma 7
        g.ball = Ball::Loose((11, 6));
        let r = g.reach(2);
        let get = |c: (i32, i32)| r.iter().find(|x| x.0 == c).map(|x| x.1);
        assert!((get((9, 3)).unwrap() - 1.0).abs() < 1e-5); // 7 pași
        let gfi = get((10, 3)).unwrap();
        assert!((gfi - 5.0 / 6.0).abs() < 1e-4, "{gfi}");
        let gfi2 = get((11, 3)).unwrap();
        assert!((gfi2 - 25.0 / 36.0).abs() < 1e-4);
        assert!(get((2, 3)).is_none());
        // niciun traseu nu trece prin jucători
        place(&mut g, 7, (3, 3));
        let r2 = g.reach(2);
        assert!(r2.iter().all(|x| x.0 != (3, 3)));
    }

    #[test]
    fn dodge_needed_next_to_enemy() {
        let mut g = kicked(3);
        clear_field(&mut g);
        place(&mut g, 2, (2, 3));
        place(&mut g, 7, (3, 3)); // inamic vecin
        g.ball = Ball::Loose((11, 6));
        let r = g.reach(2);
        let p = r.iter().find(|x| x.0 == (2, 4)).unwrap().1;
        // ocolind prin (1,4) se plătește o singură escapare (agilitate 3 − 1 = 2 → 5/6)
        assert!((p - 5.0 / 6.0).abs() < 1e-4, "{p}");
        // spre spate: ținta fără inamic vecin: AG 3 − 1 = 2 → 5/6
        let q = r.iter().find(|x| x.0 == (1, 3)).unwrap().1;
        assert!((q - 5.0 / 6.0).abs() < 1e-4, "{q}");
    }

    #[test]
    fn pickup_probability_on_ball() {
        let mut g = kicked(3);
        clear_field(&mut g);
        place(&mut g, 2, (2, 3));
        g.ball = Ball::Loose((4, 3));
        let r = g.reach(2);
        let p = r.iter().find(|x| x.0 == (4, 3)).unwrap().1;
        assert!((p - 4.0 / 6.0).abs() < 1e-4, "{p}"); // 3+ = 4/6
    }

    #[test]
    fn push_directions_and_crowd() {
        let mut g = kicked(4);
        clear_field(&mut g);
        place(&mut g, 4, (5, 3));
        place(&mut g, 6, (6, 3));
        g.push(4, 6, false);
        assert_eq!(g.pl[6].pos, (7, 3));
        // blocat drept: ia o diagonală
        let mut g = kicked(4);
        clear_field(&mut g);
        place(&mut g, 4, (5, 3));
        place(&mut g, 6, (6, 3));
        place(&mut g, 7, (7, 3));
        g.push(4, 6, false);
        assert!(g.pl[6].pos == (7, 4) || g.pl[6].pos == (7, 2));
        // lângă margine: scos
        let mut g = kicked(4);
        clear_field(&mut g);
        place(&mut g, 4, (5, 0));
        place(&mut g, 6, (5, 1));
        place(&mut g, 7, (4, 2));
        place(&mut g, 8, (5, 2));
        place(&mut g, 9, (6, 2));
        // împins în sus (+y) e blocat; împinge spre rândul 0 din 1: atacator la (5,2), țintă (5,1)
        let mut h = kicked(4);
        clear_field(&mut h);
        place(&mut h, 4, (5, 1));
        place(&mut h, 6, (5, 0));
        h.push(4, 6, false);
        assert_eq!(h.pl[6].st, St::Out);
    }

    #[test]
    fn knockdown_armor_statistics() {
        let mut out = 0;
        let n = 4000;
        for s in 0..n {
            let mut g = kicked(s + 1);
            clear_field(&mut g);
            place(&mut g, 2, (5, 3)); // AV 7
            g.knock_down(2, false);
            if g.pl[2].st == St::Out {
                out += 1;
            }
        }
        // P(2d6 > 7) = 15/36 ≈ 0.4167
        let f = out as f32 / n as f32;
        assert!((f - 15.0 / 36.0).abs() < 0.03, "{f}");
    }

    #[test]
    fn carrier_knocked_down_loses_ball() {
        let mut g = kicked(5);
        clear_field(&mut g);
        place(&mut g, 4, (5, 3));
        g.ball = Ball::Carried(4);
        g.side = Player::One;
        g.knock_down(4, true);
        assert!(g.carrier() != Some(4));
        assert!(g.turnover);
    }

    #[test]
    fn touchdown_scores_and_switches() {
        let mut g = kicked(6);
        clear_field(&mut g);
        place(&mut g, 2, (11, 3));
        g.ball = Ball::Carried(2);
        assert!(g.check_td());
        assert_eq!(g.score, [1, 0]);
        g.finish_turn(true);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(legal(&g), vec![Move::Kick]);
        // purtătorul de partea proprie nu marchează
        let mut h = kicked(6);
        clear_field(&mut h);
        place(&mut h, 2, (0, 3));
        h.ball = Ball::Carried(2);
        assert!(!h.check_td());
    }

    #[test]
    fn pass_target_grows_with_distance() {
        let mut g = kicked(7);
        clear_field(&mut g);
        place(&mut g, 0, (1, 3));
        place(&mut g, 1, (3, 3));
        place(&mut g, 2, (6, 3));
        place(&mut g, 3, (9, 3));
        place(&mut g, 4, (11, 6));
        let t1 = g.pass_target(0, 1).unwrap();
        let t2 = g.pass_target(0, 2).unwrap();
        let t3 = g.pass_target(0, 3).unwrap();
        assert!(t1 < t2 && t2 < t3, "{t1} {t2} {t3}");
        assert!(g.pass_target(0, 4).is_none()); // peste 8
    }

    #[test]
    fn turns_and_halves_count() {
        let mut g = Gridiron::with_seed(8);
        g.apply(&Move::Kick);
        let mut turns = [0u32; 2];
        let mut halves_seen = [false; 2];
        while g.outcome().is_none() {
            halves_seen[(g.half - 1) as usize] = true;
            match g.phase {
                Phase::Kick => g.apply(&Move::Kick),
                Phase::Play => {
                    turns[g.to_move().idx()] += 1;
                    g.apply(&Move::End);
                }
                Phase::Roll => g.apply(&Move::Roll),
            }
        }
        assert_eq!(turns, [12, 12]);
        assert_eq!(halves_seen, [true, true]);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        // a doua repriză începe cu cealaltă echipă
    }

    #[test]
    fn second_half_swaps_receiver() {
        let mut g = Gridiron::with_seed(8);
        g.apply(&Move::Kick);
        for _ in 0..12 {
            g.apply(&Move::End);
        }
        assert_eq!(g.half, 2);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(legal(&g), vec![Move::Kick]);
    }

    #[test]
    fn random_games_finish_and_pieces_conserved() {
        for seed in 1..=30u64 {
            let mut g = Gridiron::with_seed(seed);
            let mut rng = Rng::new(seed * 3 + 1);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = legal(&g);
                assert!(!mv.is_empty(), "{}", Game::render(&g));
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                n += 1;
                assert!(n < 6000, "prea lung");
                assert_eq!(g.pl.len(), 10);
                let mut seen = std::collections::HashSet::new();
                for p in &g.pl {
                    if p.st != St::Out {
                        assert!(inside(p.pos), "{:?}", p);
                        assert!(seen.insert(p.pos), "suprapunere\n{}", Game::render(&g));
                    }
                }
                match g.ball {
                    Ball::Carried(c) => assert!(g.pl[c as usize].st == St::Up),
                    Ball::Loose(b) => assert!(inside(b)),
                }
            }
            assert!(g.turns_left == [0, 0] || g.half == 2);
        }
    }

    #[test]
    fn ai_scores_and_beats_random() {
        let mut wins = 0;
        let mut worst = 0u128;
        let mut ai_goals = 0;
        for seed in 1..=10u64 {
            let mut g = Gridiron::with_seed(seed + 300);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 6000 {
                let m = if Game::to_move(&g) == Player::Two {
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
            ai_goals += g.score[1] as u32;
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(
            wins >= 6,
            "AI a câștigat doar {wins}/10 (puncte {ai_goals})"
        );
        assert!(ai_goals >= 6, "AI a marcat doar {ai_goals}");
        assert!(worst < 1000, "AI prea lent: {worst} ms");
    }
}
