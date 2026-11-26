#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Ashta Chamma (India) — joc de cursă cu cauri pe tabla 5×5, 2 jucători, 4 piese fiecare.
//!
//! Reguli folosite (varianta cea mai răspândită, în adaptare pentru doi):
//! * Tabla are 25 de pătrate. Jucătorii pornesc de la mijlocul laturilor opuse (J1 jos, J2 sus).
//!   Se aruncă 4 cauri: numărul de cauri cu deschiderea în sus dă mutarea (1, 2, 3, 4), iar
//!   nicio gură în sus = 8. **4 și 8 dau o aruncare în plus** (la fel o captură sau o piesă
//!   ajunsă în centru).
//! * Piesele merg în sens trigonometric pe inelul exterior (16 pătrate), iar o piesă poate
//!   intra în inelul interior (8 pătrate, sens orar) și apoi în centru **doar dacă a capturat
//!   cel puțin o piesă adversă** („kill”); până atunci se învârte pe inelul exterior. Centrul se
//!   atinge cu aruncare exactă.
//! * Pătratele sigure (X): cele patru mijloace de latură și cele patru colțuri, plus centrul.
//!   Pe un pătrat nesigur, o piesă singură a adversarului e capturată (se întoarce la start,
//!   pierzându-și „kill”-ul); două sau mai multe piese ale aceluiași jucător pe același pătrat
//!   formează un bloc și nu pot fi capturate.
//! * Câștigă cine aduce primul toate cele 4 piese în centru.
//! * Mutări: `arunc`, apoi numărul pătratului de pe cale al piesei mutate (0 = start,
//!   1–15 = inelul exterior, 16–23 = inelul interior, 24 = centru); un `*` după număr = piesa
//!   a capturat deja (ex.: `7`, `7*`). `pas` dacă nu ai nicio mutare.
//! * Simplificări: fără varianta cu patru jucători, fără reguli speciale de dublet.
//! * AI: nu vede aruncarea următoare (se aruncă la mutarea forțată `arunc`); la Hard/Expert
//!   evaluează în așteptare peste distribuția aruncărilor adversarului.

use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const OUTER: [(u8, u8); 16] = [
    (4, 2),
    (4, 3),
    (4, 4),
    (3, 4),
    (2, 4),
    (1, 4),
    (0, 4),
    (0, 3),
    (0, 2),
    (0, 1),
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (4, 1),
];
const INNER: [(u8, u8); 9] = [
    (3, 1),
    (2, 1),
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 3),
    (3, 3),
    (3, 2),
    (2, 2),
];
const GOAL: u8 = 24;

/// Coordonatele (rând, coloană) ale pătratului `idx` pe calea jucătorului `p`.
pub fn coord(p: usize, idx: u8) -> (u8, u8) {
    let (r, c) = if idx < 16 {
        OUTER[idx as usize]
    } else {
        INNER[(idx - 16) as usize]
    };
    if p == 0 {
        (r, c)
    } else {
        (4 - r, 4 - c)
    }
}

pub fn is_safe(rc: (u8, u8)) -> bool {
    let (r, c) = rc;
    r % 2 == 0 && c % 2 == 0
}

/// P(aruncare) pentru 1, 2, 3, 4, 8 (din 16): C(4,k)/16, cu 0 în sus = 8.
pub const ROLLS: [(u8, u32); 5] = [(1, 4), (2, 6), (3, 4), (4, 1), (8, 1)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Go(u8, bool),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub idx: u8,
    pub hit: bool,
}

#[derive(Clone)]
pub struct AshtaChamma {
    pieces: [[Piece; 4]; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    last: String,
}

impl Default for AshtaChamma {
    fn default() -> Self {
        Self::new()
    }
}

fn step(p: Piece, r: u8) -> Option<u8> {
    if p.idx >= GOAL {
        return None;
    }
    if p.hit {
        let t = p.idx + r;
        (t <= GOAL).then_some(t)
    } else {
        Some((p.idx + r) % 16)
    }
}

impl AshtaChamma {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        AshtaChamma {
            pieces: [[Piece { idx: 0, hit: false }; 4]; 2],
            side: Player::One,
            roll: 0,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }
    pub fn pieces(&self) -> &[[Piece; 4]; 2] {
        &self.pieces
    }
    pub fn set_piece_for_test(&mut self, p: usize, i: usize, idx: u8, hit: bool) {
        self.pieces[p][i] = Piece { idx, hit };
    }
    pub fn set_roll_for_test(&mut self, side: Player, roll: u8) {
        self.side = side;
        self.roll = roll;
    }

    fn at(&self, p: usize, rc: (u8, u8)) -> usize {
        self.pieces[p]
            .iter()
            .filter(|q| q.idx < GOAL && coord(p, q.idx) == rc)
            .count()
    }

    fn targets(&self, r: u8) -> Vec<(Piece, u8)> {
        let me = self.side.idx();
        let mut v: Vec<(Piece, u8)> = Vec::new();
        for &pc in &self.pieces[me] {
            if let Some(t) = step(pc, r) {
                if !v.iter().any(|(q, _)| *q == pc) {
                    v.push((pc, t));
                }
            }
        }
        v
    }

    fn roll_value(r: u8) -> i32 {
        r as i32
    }

    /// Valoarea statică din punctul de vedere al lui `p`.
    fn static_eval(&self, p: Player) -> i32 {
        let score = |q: usize| -> i32 {
            let mut s = 0i32;
            for pc in &self.pieces[q] {
                s += if pc.idx == GOAL {
                    260
                } else if pc.hit {
                    pc.idx as i32 * 6 + 70
                } else {
                    pc.idx as i32 * 2
                };
                // pericol: o piesă singură pe un pătrat nesigur la distanță 1–4 / 8 în fața unei piese adverse
                if pc.idx < GOAL {
                    let rc = coord(q, pc.idx);
                    if !is_safe(rc) && self.at(q, rc) == 1 {
                        let o = 1 - q;
                        let mut danger = 0;
                        for op in &self.pieces[o] {
                            for (r, w) in ROLLS {
                                if let Some(t) = step(*op, r) {
                                    if op.idx < GOAL && coord(o, t) == rc {
                                        danger += w as i32;
                                    }
                                }
                            }
                        }
                        s -= danger * (20 + pc.idx as i32 * 2) / 16 * 4;
                    }
                }
            }
            s
        };
        score(p.idx()) - score(p.other().idx())
    }
}

impl Game for AshtaChamma {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.roll == 0 {
            return vec![Move::Roll];
        }
        let v: Vec<Move> = self
            .targets(self.roll)
            .into_iter()
            .map(|(p, _)| Move::Go(p.idx, p.hit))
            .collect();
        if v.is_empty() {
            vec![Move::Pass]
        } else {
            v
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        match *m {
            Move::Roll => {
                let ups = (0..4).filter(|_| self.rng.below(2) == 1).count() as u8;
                self.roll = if ups == 0 { 8 } else { ups };
                self.last = format!("J{} aruncă {}", me + 1, self.roll);
            }
            Move::Pass => {
                let extra = self.roll == 4 || self.roll == 8;
                self.roll = 0;
                if !extra {
                    self.side = self.side.other();
                }
            }
            Move::Go(idx, hit) => {
                let r = self.roll;
                let from = Piece { idx, hit };
                let t = step(from, r).expect("mutare legală");
                let k = self.pieces[me]
                    .iter()
                    .position(|&q| q == from)
                    .expect("piesa există");
                let mut extra = r == 4 || r == 8;
                let mut newhit = hit;
                let rc = coord(me, t);
                if t < GOAL && !is_safe(rc) {
                    let o = 1 - me;
                    let opp: Vec<usize> = (0..4)
                        .filter(|&j| {
                            self.pieces[o][j].idx < GOAL && coord(o, self.pieces[o][j].idx) == rc
                        })
                        .collect();
                    if opp.len() == 1 {
                        self.pieces[o][opp[0]] = Piece { idx: 0, hit: false };
                        newhit = true;
                        extra = true;
                        self.last = format!("J{} capturează o piesă", me + 1);
                    }
                }
                self.pieces[me][k] = Piece {
                    idx: t,
                    hit: newhit,
                };
                if t == GOAL {
                    extra = true;
                }
                self.roll = 0;
                if self.outcome().is_none() && !extra {
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.pieces[p].iter().all(|q| q.idx == GOAL) {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let base = self.static_eval(p) * 10;
        let lv = level();
        if self.roll != 0 || lv < Level::Hard || self.outcome().is_some() {
            return base
                + crate::p2_util::noise(
                    crate::p2_util::sig_of(&[base as u64, self.roll as u64]),
                    if lv == Level::Easy { 120 } else { 0 },
                );
        }
        // așteptare peste aruncarea celui care urmează
        let mut tot = 0i64;
        for (r, w) in ROLLS {
            let mut c = self.clone();
            c.roll = r;
            let mv = c.legal_moves();
            let mut best: Option<i32> = None;
            for m in mv {
                let mut d = c.clone();
                d.apply(&m);
                let v = d.static_eval(p) * 10;
                best = Some(match best {
                    None => v,
                    Some(b) => {
                        if c.side == p {
                            b.max(v)
                        } else {
                            b.min(v)
                        }
                    }
                });
            }
            tot += best.unwrap_or(base) as i64 * w as i64;
        }
        (tot / 16) as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Ashta Chamma — J1 (a) pornește jos, J2 (b) sus\n");
        for r in 0..5u8 {
            let mut line = String::new();
            for c in 0..5u8 {
                let a = self.at(0, (r, c));
                let b = self.at(1, (r, c));
                let mark = if is_safe((r, c)) { 'X' } else { '.' };
                let cell = match (a, b) {
                    (0, 0) => format!("{mark}  "),
                    (a, 0) => format!("a{a} "),
                    (0, b) => format!("b{b} "),
                    (a, b) => format!("a{a}b{b}"),
                };
                line.push_str(&format!("{cell:<4}"));
            }
            s.push_str(&line);
            s.push('\n');
        }
        for p in 0..2 {
            let t: Vec<String> = self.pieces[p]
                .iter()
                .map(|q| format!("{}{}", q.idx, if q.hit { "*" } else { "" }))
                .collect();
            s.push_str(&format!("Piesele J{} (cale): {}\n", p + 1, t.join(" ")));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.roll != 0 {
            s.push_str(&format!("Aruncare: {}\n", Self::roll_value(self.roll)));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Pass => "pas".into(),
            Move::Go(i, h) => format!("{}{}", i, if *h { "*" } else { "" }),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_is_consistent() {
        // 16 + 8 pătrate + centru, toate distincte
        let mut seen = std::collections::HashSet::new();
        for i in 0..=GOAL {
            assert!(seen.insert(coord(0, i)));
        }
        assert_eq!(seen.len(), 25);
        assert_eq!(coord(0, 0), (4, 2));
        assert_eq!(coord(1, 0), (0, 2));
        assert_eq!(coord(0, GOAL), (2, 2));
        assert_eq!(coord(1, GOAL), (2, 2));
        // pașii consecutivi sunt vecini
        for i in 0..GOAL {
            let (a, b) = (coord(0, i), coord(0, i + 1));
            let d = (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs();
            assert_eq!(d, 1, "{i}");
        }
        assert_eq!((0..25u8).filter(|&i| is_safe(coord(0, i))).count(), 9);
        assert_eq!(ROLLS.iter().map(|r| r.1).sum::<u32>(), 16);
    }

    #[test]
    fn unhit_piece_circles_and_cannot_enter_inner() {
        let mut g = AshtaChamma::with_seed(1);
        g.set_piece_for_test(0, 0, 15, false);
        g.set_roll_for_test(Player::One, 3);
        // 15 + 3 → 2 (a doua tură pe inelul exterior)
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Go(15, false)));
        g.apply(&Move::Go(15, false));
        assert!(g.pieces()[0].iter().any(|q| q.idx == 2 && !q.hit));
    }

    #[test]
    fn hit_piece_enters_inner_and_needs_exact_center() {
        let mut g = AshtaChamma::with_seed(1);
        g.set_piece_for_test(0, 0, 15, true);
        g.set_roll_for_test(Player::One, 2);
        g.apply(&Move::Go(15, true));
        assert!(g.pieces()[0].iter().any(|q| q.idx == 17 && q.hit));
        // la 23 doar 1 duce în centru; 2 depășește
        g.set_piece_for_test(0, 1, 23, true);
        g.set_roll_for_test(Player::One, 2);
        assert!(!g.legal_moves().contains(&Move::Go(23, true)));
        g.set_roll_for_test(Player::One, 1);
        assert!(g.legal_moves().contains(&Move::Go(23, true)));
    }

    #[test]
    fn capture_sends_home_grants_hit_and_extra_roll() {
        let mut g = AshtaChamma::with_seed(1);
        // J1 idx 4 = (2,4) sigur; folosim idx 3 = (3,4) nesigur. J2 piesă pe (3,4) = idx cale J2: coord(1,i)==(3,4) → coord(0,i)==(1,0) → i=11
        g.set_piece_for_test(1, 0, 11, false);
        g.set_piece_for_test(0, 0, 1, false);
        g.set_roll_for_test(Player::One, 2);
        g.apply(&Move::Go(1, false));
        assert_eq!(g.pieces()[1][0], Piece { idx: 0, hit: false });
        assert!(g.pieces()[0].iter().any(|q| q.idx == 3 && q.hit));
        assert_eq!(g.to_move(), Player::One); // aruncare în plus
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
    }

    #[test]
    fn safe_squares_and_blocks_protect() {
        let mut g = AshtaChamma::with_seed(1);
        // piesă J2 pe pătrat sigur (colț): coord(0,2)=(4,4) → J2 idx cu coord(1,i)==(4,4) → (0,0) → i=10
        g.set_piece_for_test(1, 0, 10, false);
        g.set_piece_for_test(0, 0, 0, false);
        g.set_roll_for_test(Player::One, 2);
        g.apply(&Move::Go(0, false));
        assert_eq!(g.pieces()[1][0].idx, 10);
        // bloc: două piese J2 pe (3,4) → idx 11
        let mut g = AshtaChamma::with_seed(1);
        g.set_piece_for_test(1, 0, 11, false);
        g.set_piece_for_test(1, 1, 11, false);
        g.set_piece_for_test(0, 0, 1, false);
        g.set_roll_for_test(Player::One, 2);
        g.apply(&Move::Go(1, false));
        assert_eq!(g.pieces()[1][0].idx, 11);
        assert_eq!(g.pieces()[1][1].idx, 11);
        assert!(!g.pieces()[0].iter().any(|q| q.hit));
    }

    #[test]
    fn extra_roll_on_4_and_8_and_win() {
        let mut g = AshtaChamma::with_seed(1);
        g.set_roll_for_test(Player::One, 4);
        g.apply(&Move::Go(0, false));
        assert_eq!(g.to_move(), Player::One);
        let mut g = AshtaChamma::with_seed(1);
        g.set_roll_for_test(Player::One, 2);
        g.apply(&Move::Go(0, false));
        assert_eq!(g.to_move(), Player::Two);
        let mut g = AshtaChamma::with_seed(1);
        for i in 0..3 {
            g.set_piece_for_test(0, i, GOAL, true);
        }
        g.set_piece_for_test(0, 3, 23, true);
        g.set_roll_for_test(Player::One, 1);
        g.apply(&Move::Go(23, true));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_and_ai_games_finish() {
        for seed in 1..6u64 {
            let mut g = AshtaChamma::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = Game::legal_moves(&g);
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
        let mut g = AshtaChamma::with_seed(9);
        let mut n = 0;
        while g.outcome().is_none() && n < 20000 {
            let m = polytabletop_core::ai::best_move(&g, n).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(g.outcome().is_some());
    }
}
