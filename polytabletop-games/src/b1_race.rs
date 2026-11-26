//! Motor comun pentru cinci curse pe traseu circular (variante proprii, 2 jucători), cu zar sau
//! cărți și reguli de configurare: intrare, bonus, sărituri, scurtătură, blocaje, cărți.
//! Jocurile concrete (cai_de_curse, zbor, marmure, blocaje, scuze) sunt configurații ale
//! aceluiași motor; vezi `Cfg`.
//!
//! Traseu de 40 de celule; intrarea lui J1 e celula 0, a lui J2 celula 20 (celule sigure).
//! O piesă face 40 de pași pe traseu (progres 0–39), apoi intră pe culoarul propriu de 4 celule
//! (progres 40–43; 43 = țintă). Zarul/cartea se aruncă automat în starea jocului.
#![allow(clippy::all)]

use crate::b1_util::sq;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const L: i32 = 40;
const H: i32 = 4;
const GOAL: i32 = L + H - 1;

#[derive(Clone, Copy, Debug)]
pub struct Cfg {
    pub pieces: usize,
    /// rezultate de zar cu care o piesă intră din curte (gol = orice)
    pub enter_on: &'static [i8],
    pub extra_on_six: bool,
    /// salt de culoare: aterizare pe celulă cu `abs % 4 == (0 | 2)` (după jucător) duce 4 înainte
    pub color_jump: bool,
    /// progres de unde „zbori” 12 celule înainte
    pub fly_from: i32,
    /// scurtătură opțională: (de la, la)
    pub hole: Option<(i32, i32)>,
    /// blocaje neutre pe celule absolute
    pub blockers: &'static [i32],
    pub cards: bool,
}

pub const CAI: Cfg = Cfg {
    pieces: 4,
    enter_on: &[1, 6],
    extra_on_six: true,
    color_jump: false,
    fly_from: -1,
    hole: None,
    blockers: &[],
    cards: false,
};
pub const ZBOR: Cfg = Cfg {
    pieces: 4,
    enter_on: &[5, 6],
    extra_on_six: true,
    color_jump: true,
    fly_from: 16,
    hole: None,
    blockers: &[],
    cards: false,
};
pub const MARMURE: Cfg = Cfg {
    pieces: 4,
    enter_on: &[1, 6],
    extra_on_six: true,
    color_jump: false,
    fly_from: -1,
    hole: Some((12, 30)),
    blockers: &[],
    cards: false,
};
pub const BLOCAJE: Cfg = Cfg {
    pieces: 3,
    enter_on: &[],
    extra_on_six: false,
    color_jump: false,
    fly_from: -1,
    hole: None,
    blockers: &[7, 13, 27, 33, 37],
    cards: false,
};
pub const SCUZE: Cfg = Cfg {
    pieces: 3,
    enter_on: &[],
    extra_on_six: false,
    color_jump: false,
    fly_from: -1,
    hole: None,
    blockers: &[],
    cards: true,
};

const DECK: [i8; 11] = [1, 2, 3, -4, 5, 7, 8, 10, 11, 12, 99];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Go(u8),        // piesă
    Hole(u8),      // piesă, scurtătură
    Swap(u8, u8),  // piesă proprie, piesă adversă
    Sorry(u8, u8), // piesă din curte, piesă adversă
    Block(u8),     // celulă absolută
    Pass,
}

#[derive(Clone)]
pub struct Race {
    cfg: Cfg,
    pos: [[i32; 4]; 2], // -1 curte, 0..=43
    blockers: Vec<i32>,
    side: Player,
    roll: i8,
    pending: bool,
    rng: Rng,
    ply: u32,
}

fn abs(p: usize, pr: i32) -> i32 {
    (p as i32 * L / 2 + pr) % L
}

impl Race {
    pub fn new(cfg: Cfg) -> Self {
        Self::with_seed(cfg, Rng::from_clock().next_u64())
    }
    pub fn with_seed(cfg: Cfg, seed: u64) -> Self {
        let mut r = Race {
            cfg,
            pos: [[-1; 4]; 2],
            blockers: cfg.blockers.to_vec(),
            side: Player::One,
            roll: 0,
            pending: false,
            rng: Rng::new(seed),
            ply: 0,
        };
        r.draw();
        r
    }
    fn draw(&mut self) {
        self.roll = if self.cfg.cards {
            DECK[self.rng.below(DECK.len())]
        } else {
            self.rng.d6() as i8
        };
    }
    fn n(&self) -> usize {
        self.cfg.pieces
    }
    fn done(&self, p: usize) -> bool {
        (0..self.n()).all(|i| self.pos[p][i] == GOAL)
    }
    fn at_track(&self, p: usize, a: i32) -> Option<usize> {
        (0..self.n())
            .find(|&i| self.pos[p][i] >= 0 && self.pos[p][i] < L && abs(p, self.pos[p][i]) == a)
    }
    fn is_safe(a: i32) -> bool {
        a == 0 || a == L / 2
    }
    /// Țintă și validitate pentru o deplasare de `r` pași a piesei `i` a lui `p`.
    fn target(&self, p: usize, i: usize, r: i32) -> Option<i32> {
        let pr = self.pos[p][i];
        if pr < 0 || r == 0 {
            return None;
        }
        let t = pr + r;
        if t < 0 || t > GOAL {
            return None;
        }
        // blocaje pe drum (strict între) pe traseu
        if r > 0 {
            for s in pr + 1..t {
                if s < L && self.blockers.contains(&abs(p, s)) {
                    return None;
                }
            }
        }
        if t < L {
            let a = abs(p, t);
            if self.at_track(p, a).is_some() {
                return None;
            }
            if let Some(_o) = self.at_track(1 - p, a) {
                if Self::is_safe(a) {
                    return None;
                }
            }
        } else if t < GOAL && (0..self.n()).any(|j| j != i && self.pos[p][j] == t) {
            return None;
        }
        Some(t)
    }
    fn can_enter(&self, p: usize) -> bool {
        if self.at_track(p, abs(p, 0)).is_some() {
            return false;
        }
        if let Some(o) = self.at_track(1 - p, abs(p, 0)) {
            let _ = o;
            return false;
        }
        true
    }
    fn enter_ok(&self) -> bool {
        if self.cfg.cards {
            self.roll == 1 || self.roll == 2
        } else if self.cfg.enter_on.is_empty() {
            true
        } else {
            self.cfg.enter_on.contains(&self.roll)
        }
    }
    fn gen(&self) -> Vec<Mv> {
        let p = self.side.idx();
        let n = self.n();
        let mut v = vec![];
        if self.pending {
            for a in 0..L {
                if !Self::is_safe(a)
                    && !self.blockers.contains(&a)
                    && self.at_track(0, a).is_none()
                    && self.at_track(1, a).is_none()
                {
                    v.push(Mv::Block(a as u8));
                }
            }
            return v;
        }
        let r = self.roll as i32;
        if self.cfg.cards && r == 99 {
            for i in 0..n {
                if self.pos[p][i] == -1 {
                    for j in 0..n {
                        let o = self.pos[1 - p][j];
                        if o >= 0 && o < L && !Self::is_safe(abs(1 - p, o)) {
                            v.push(Mv::Sorry(i as u8, j as u8));
                        }
                    }
                    break;
                }
            }
            return if v.is_empty() { vec![Mv::Pass] } else { v };
        }
        let mut seen_yard = false;
        for i in 0..n {
            let pr = self.pos[p][i];
            if pr == -1 {
                if !seen_yard && self.enter_ok() && self.can_enter(p) {
                    v.push(Mv::Go(i as u8));
                }
                seen_yard = true;
                continue;
            }
            if pr == GOAL {
                continue;
            }
            if let Some(t) = self.target(p, i, r) {
                v.push(Mv::Go(i as u8));
                if let Some((from, to)) = self.cfg.hole {
                    if t == from && self.at_track(p, abs(p, to)).is_none() {
                        v.push(Mv::Hole(i as u8));
                    }
                }
            }
            if self.cfg.cards && r == 11 && pr < L {
                for j in 0..n {
                    let o = self.pos[1 - p][j];
                    if o >= 0 && o < L && !Self::is_safe(abs(1 - p, o)) {
                        v.push(Mv::Swap(i as u8, j as u8));
                    }
                }
            }
        }
        if v.is_empty() {
            v.push(Mv::Pass);
        }
        v
    }
    fn land(&mut self, p: usize, i: usize, mut t: i32) {
        // salturi automate
        if t < L && self.cfg.color_jump && abs(p, t) % 4 == (p as i32) * 2 {
            let t2 = t + 4;
            if self.free_for(p, t2) {
                t = t2;
            }
        }
        if t == self.cfg.fly_from {
            let t2 = t + 12;
            if self.free_for(p, t2) {
                t = t2;
            }
        }
        self.pos[p][i] = t;
        if t < L {
            let a = abs(p, t);
            if let Some(o) = self.at_track(1 - p, a) {
                self.pos[1 - p][o] = -1;
            }
            if let Some(k) = self.blockers.iter().position(|&b| b == a) {
                self.blockers.remove(k);
                self.pending = true;
            }
        }
    }
    fn free_for(&self, p: usize, t: i32) -> bool {
        if t >= L {
            return t < GOAL;
        }
        let a = abs(p, t);
        self.at_track(p, a).is_none()
            && !(self.at_track(1 - p, a).is_some() && Self::is_safe(a))
            && !self.blockers.contains(&a)
    }
    fn end_turn(&mut self, bonus: bool) {
        self.ply += 1;
        if !self.done(0) && !self.done(1) && !(bonus && self.cfg.extra_on_six && self.roll == 6) {
            self.side = self.side.other();
        }
        self.draw();
    }
    fn progress(&self, p: usize) -> i32 {
        (0..self.n())
            .map(|i| {
                if self.pos[p][i] < 0 {
                    0
                } else {
                    self.pos[p][i] + 1
                }
            })
            .sum()
    }
}

impl Game for Race {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        self.gen()
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match m {
            Mv::Pass => self.end_turn(false),
            Mv::Go(i) => {
                let i = *i as usize;
                let r = self.roll as i32;
                if self.pos[p][i] == -1 {
                    self.land(p, i, 0);
                } else {
                    let t = self.target(p, i, r).unwrap();
                    self.land(p, i, t);
                }
                if !self.pending {
                    self.end_turn(true);
                }
            }
            Mv::Hole(i) => {
                let (_, to) = self.cfg.hole.unwrap();
                self.land(p, *i as usize, to);
                if !self.pending {
                    self.end_turn(true);
                }
            }
            Mv::Swap(i, j) => {
                let (i, j) = (*i as usize, *j as usize);
                let (a, b) = (self.pos[p][i], self.pos[1 - p][j]);
                let (xa, xb) = (abs(p, a), abs(1 - p, b));
                // piesa mea ia celula adversarului și invers (progresele se recalculează)
                self.pos[p][i] = (xb - p as i32 * L / 2).rem_euclid(L);
                self.pos[1 - p][j] = (xa - (1 - p) as i32 * L / 2).rem_euclid(L);
                self.end_turn(false);
            }
            Mv::Sorry(i, j) => {
                let (i, j) = (*i as usize, *j as usize);
                let a = abs(1 - p, self.pos[1 - p][j]);
                self.pos[1 - p][j] = -1;
                self.pos[p][i] = (a - p as i32 * L / 2).rem_euclid(L);
                self.end_turn(false);
            }
            Mv::Block(a) => {
                self.blockers.push(*a as i32);
                self.blockers.sort();
                self.pending = false;
                self.end_turn(true);
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.done(0) {
            return Some(Outcome::Win(Player::One));
        }
        if self.done(1) {
            return Some(Outcome::Win(Player::Two));
        }
        if self.ply >= 800 {
            return Some(match self.progress(0).cmp(&self.progress(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = self.progress(me) - self.progress(1 - me);
        for q in 0..2 {
            let v = (0..self.n()).filter(|&i| self.pos[q][i] == GOAL).count() as i32 * 30;
            s += if q == me { v } else { -v };
        }
        s
    }
    fn render(&self) -> String {
        let mut s = String::new();
        let line = |a0: i32, a1: i32, s: &mut String| {
            for a in a0..a1 {
                let c = if let Some(i) = self.at_track(0, a) {
                    char::from(b'1' + i as u8)
                } else if let Some(i) = self.at_track(1, a) {
                    char::from(b'a' + i as u8)
                } else if self.blockers.contains(&a) {
                    '#'
                } else if Self::is_safe(a) {
                    '+'
                } else {
                    '.'
                };
                s.push(c);
            }
        };
        s.push_str("Traseu 0–19: ");
        line(0, 20, &mut s);
        s.push_str("\nTraseu 20–39: ");
        line(20, 40, &mut s);
        s.push('\n');
        s.push_str(
            "J1 = cifre 1–n, J2 = litere a–; # blocaj; + celulă sigură (intrări: 0 și 20).\n",
        );
        for p in 0..2 {
            let name = if p == 0 { "J1" } else { "J2" };
            let txt: Vec<String> = (0..self.n())
                .map(|i| {
                    let pr = self.pos[p][i];
                    let id = if p == 0 {
                        (b'1' + i as u8) as char
                    } else {
                        (b'a' + i as u8) as char
                    };
                    match pr {
                        -1 => format!("{id}:curte"),
                        x if x == GOAL => format!("{id}:țintă"),
                        x if x >= L => format!("{id}:culoar {}", x - L + 1),
                        x => format!("{id}:celula {}", abs(p, x)),
                    }
                })
                .collect();
            s.push_str(&format!("{name}: {}\n", txt.join(", ")));
        }
        if self.cfg.cards {
            let t = match self.roll {
                99 => "Scuze (iei locul unei piese adverse)".to_string(),
                -4 => "înapoi 4".to_string(),
                11 => "11 (înainte 11 sau schimb de locuri)".to_string(),
                x => format!("{x}"),
            };
            s.push_str(&format!("Cartea trasă: {t}\n"));
        } else {
            s.push_str(&format!("Zarul: {}\n", self.roll));
        }
        if self.pending {
            s.push_str("Ai luat un blocaj: așază-l pe o celulă liberă (bloc <celulă>).\n");
        }
        let _ = sq;
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        let p = self.side.idx();
        let id = |i: u8, p: usize| {
            if p == 0 {
                (b'1' + i) as char
            } else {
                (b'a' + i) as char
            }
        };
        match m {
            Mv::Go(i) => format!("{}", id(*i, p)),
            Mv::Hole(i) => format!("{}!", id(*i, p)),
            Mv::Swap(i, j) => format!("{}x{}", id(*i, p), id(*j, 1 - p)),
            Mv::Sorry(i, j) => format!("{}s{}", id(*i, p), id(*j, 1 - p)),
            Mv::Block(a) => format!("bloc {a}"),
            Mv::Pass => "pas".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;

    #[test]
    fn enter_roll_and_extra_turn() {
        let mut g = Race::with_seed(CAI, 1);
        g.roll = 3;
        assert_eq!(g.legal_moves(), vec![Mv::Pass]);
        g.roll = 6;
        assert_eq!(g.legal_moves(), vec![Mv::Go(0)]);
        g.apply(&Mv::Go(0));
        assert_eq!(g.pos[0][0], 0);
        // 6 dă tură în plus
        assert_eq!(g.to_move(), Player::One);
    }
    #[test]
    fn capture_sends_home() {
        let mut g = Race::with_seed(CAI, 2);
        g.pos[0][0] = 5;
        g.pos[1][0] = 5 + 20 - 20; // J2 progres 5 = celula 25
        g.pos[1][1] = -1;
        // piesă J1 la celula 22 (progres 22) lovește J2 aflat la progres 3 (celula 23)
        g.pos[0][0] = 22;
        g.pos[1][0] = 3;
        g.roll = 1;
        assert!(g.legal_moves().contains(&Mv::Go(0)));
        g.apply(&Mv::Go(0));
        assert_eq!(g.pos[1][0], -1);
    }
    #[test]
    fn blocker_flow() {
        let mut g = Race::with_seed(BLOCAJE, 3);
        g.pos[0][0] = 5;
        g.roll = 2; // celula 7 = blocaj
        assert!(g.legal_moves().contains(&Mv::Go(0)));
        g.apply(&Mv::Go(0));
        assert!(g.pending);
        let m = g.legal_moves();
        assert!(m.iter().all(|x| matches!(x, Mv::Block(_))));
        g.apply(&m[0]);
        assert!(!g.pending);
        assert_eq!(g.blockers.len(), 5);
    }
    #[test]
    fn cards_swap_and_sorry() {
        let mut g = Race::with_seed(SCUZE, 4);
        g.pos[0][0] = 3;
        g.pos[1][0] = 5; // celula 25
        g.roll = 99;
        g.pos[0][1] = -1;
        assert!(g.legal_moves().iter().any(|m| matches!(m, Mv::Sorry(1, 0))));
        g.roll = 11;
        assert!(g.legal_moves().iter().any(|m| matches!(m, Mv::Swap(0, 0))));
        g.apply(&Mv::Swap(0, 0));
        assert_eq!(abs(0, g.pos[0][0]), 25);
        assert_eq!(abs(1, g.pos[1][0]), 3);
    }
    #[test]
    fn all_variants_play() {
        for cfg in [CAI, ZBOR, MARMURE, BLOCAJE, SCUZE] {
            check(|s| Race::with_seed(cfg, s), 1500);
        }
    }
}
