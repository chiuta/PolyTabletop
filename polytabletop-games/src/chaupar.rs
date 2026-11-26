//! Chaupar (India) — jocul în cruce cu cauri, varianta cu 2 jucători (rudă cu Pachisi).
//!
//! Reguli folosite (variantă documentată, simplificată):
//! * Fiecare jucător are 4 piese. Circuitul are 48 de pătrate (0..47); J1 pornește de la 0,
//!   J2 de la 24, ambii în sens crescător. După un tur întreg piesa urcă pe „coloana de
//!   acasă” (6 pătrate) și ajunge în centru **exact** (progres 54).
//! * Se aruncă 4 cauri (fiecare 50%): numărul de cauri cu gura în sus = pașii (1, 2, 3, 4),
//!   iar niciunul în sus valorează 8. **4 și 8 dau încă o aruncare.**
//! * O piesă din „curte” intră doar cu 1 sau 8 (se așază pe pătratul ei de start).
//! * Pătratele 0, 6, 12, … 42 (multipli de 6) sunt „castele”: acolo piesele nu pot fi
//!   capturate. Aterizarea pe un pătrat obișnuit cu piese adverse le trimite în curte și
//!   dă o aruncare în plus (nu și pe coloana de acasă, care e privată).
//! * Dacă nicio piesă nu poate muta, mutarea forțată este `pas`.
//! * Câștigă cine aduce primul toate cele 4 piese în centru.

//!
//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Chaupar): jocul real folosește 7 cauri
//! (valori 7/10/2/3/4/25/30/14, aruncări „înalte” 10/25/30 care dau altă aruncare), 8 pătrate
//! sigure „floare”, nevoia de cel puțin o captură înainte de a intra acasă și intrarea
//! doar cu aruncări mari. Varianta de aici (4 cauri, 1/8 pentru intrare, pătrate multiplu de
//! 6) este o simplificare documentată, apropiată de Pachisi, păstrată nemodificată.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const YARD: i8 = -1;
const HOME: i8 = 54;
const RING: i32 = 48;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Throw,
    Enter,
    /// progresul piesei care mută
    Step(i8),
    Pass,
}

#[derive(Clone)]
pub struct Chaupar {
    pieces: [[i8; 4]; 2],
    side: Player,
    val: Option<u8>,
    rng: Rng,
}

impl Default for Chaupar {
    fn default() -> Self {
        Self::new()
    }
}

fn ring_pos(p: usize, prog: i8) -> Option<i32> {
    if (0..48).contains(&prog) {
        Some((24 * p as i32 + prog as i32) % RING)
    } else {
        None
    }
}

fn safe(sq: i32) -> bool {
    sq % 6 == 0
}

impl Chaupar {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Chaupar {
            pieces: [[YARD; 4]; 2],
            side: Player::One,
            val: None,
            rng: Rng::new(seed),
        }
    }

    fn step_ok(&self, prog: i8, v: u8) -> bool {
        (0..HOME).contains(&prog) && prog + v as i8 <= HOME
    }
}

impl Game for Chaupar {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let Some(v) = self.val else {
            return vec![Move::Throw];
        };
        let mut out = Vec::new();
        let me = &self.pieces[self.side.idx()];
        if (v == 1 || v == 8) && me.contains(&YARD) {
            out.push(Move::Enter);
        }
        let mut progs: Vec<i8> = me.iter().copied().filter(|&p| self.step_ok(p, v)).collect();
        progs.sort_unstable();
        progs.dedup();
        out.extend(progs.into_iter().map(Move::Step));
        if out.is_empty() {
            out.push(Move::Pass);
        }
        out
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        match *m {
            Move::Throw => {
                let ups = self.rng.below(16).count_ones() as u8;
                self.val = Some(if ups == 0 { 8 } else { ups });
                return;
            }
            Move::Pass => {}
            Move::Enter | Move::Step(_) => {
                let from = match *m {
                    Move::Step(p) => p,
                    _ => YARD,
                };
                let v = self.val.unwrap() as i8;
                let to = if from == YARD { 0 } else { from + v };
                let i = self.pieces[me].iter().position(|&p| p == from).unwrap();
                self.pieces[me][i] = to;
                if let Some(sq) = ring_pos(me, to) {
                    if !safe(sq) {
                        let mut cap = false;
                        for q in self.pieces[1 - me].iter_mut() {
                            if ring_pos(1 - me, *q) == Some(sq) {
                                *q = YARD;
                                cap = true;
                            }
                        }
                        if cap {
                            // aruncare în plus prin captură
                            self.val = None;
                            return;
                        }
                    }
                }
            }
        }
        let extra = matches!(self.val, Some(4) | Some(8));
        self.val = None;
        if self.outcome().is_none() && !extra {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.pieces[p.idx()].iter().all(|&x| x == HOME) {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let prog = |q: usize| -> i32 {
            self.pieces[q]
                .iter()
                .map(|&x| if x == YARD { -3 } else { x as i32 })
                .sum()
        };
        let me = p.idx();
        let mut s = 5 * (prog(me) - prog(1 - me));
        // amenințări: piese neprotejate aflate la 1..4 pași în fața unei piese adverse
        for &a in &self.pieces[me] {
            let Some(sa) = ring_pos(me, a) else { continue };
            if safe(sa) {
                continue;
            }
            for &b in &self.pieces[1 - me] {
                if let Some(sb) = ring_pos(1 - me, b) {
                    let d = (sa - sb).rem_euclid(RING);
                    if (1..=4).contains(&d) || d == 8 {
                        s -= 6;
                    }
                }
            }
        }
        for &b in &self.pieces[1 - me] {
            let Some(sb) = ring_pos(1 - me, b) else {
                continue;
            };
            if safe(sb) {
                continue;
            }
            for &a in &self.pieces[me] {
                if let Some(sa) = ring_pos(me, a) {
                    let d = (sb - sa).rem_euclid(RING);
                    if (1..=4).contains(&d) || d == 8 {
                        s += 6;
                    }
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Chaupar — circuit 0..47 (J1 pornește de la 0, J2 de la 24), castele = multipli de 6\n",
        );
        for p in [Player::One, Player::Two] {
            let pc = &self.pieces[p.idx()];
            let yard = pc.iter().filter(|&&x| x == YARD).count();
            let home = pc.iter().filter(|&&x| x == HOME).count();
            let mut on: Vec<String> = pc
                .iter()
                .filter(|&&x| x != YARD && x != HOME)
                .map(|&x| match ring_pos(p.idx(), x) {
                    Some(sq) => sq.to_string(),
                    None => format!("casa{}", x - 47),
                })
                .collect();
            on.sort();
            s.push_str(&format!(
                "J{}: curte {}, pe drum [{}], în centru {}\n",
                p.idx() + 1,
                yard,
                on.join(" "),
                home
            ));
        }
        if let Some(v) = self.val {
            s.push_str(&format!("Aruncare: {v}\n"));
        }
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Throw => "arunc".into(),
            Move::Enter => "intra".into(),
            Move::Pass => "pas".into(),
            Move::Step(p) => match ring_pos(self.side.idx(), *p) {
                Some(sq) => format!("mut:{sq}"),
                None => format!("mut:casa{}", *p - 47),
            },
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g() -> Chaupar {
        Chaupar::with_seed(2)
    }

    #[test]
    fn entering_needs_one_or_eight() {
        let mut c = g();
        c.val = Some(3);
        assert_eq!(c.legal_moves(), vec![Move::Pass]);
        c.val = Some(1);
        assert_eq!(c.legal_moves(), vec![Move::Enter]);
        c.val = Some(8);
        assert!(c.legal_moves().contains(&Move::Enter));
    }

    #[test]
    fn extra_throw_on_4_and_8() {
        let mut c = g();
        c.pieces[0] = [2, YARD, YARD, YARD];
        c.val = Some(4);
        c.apply(&Move::Step(2));
        assert_eq!(c.side, Player::One);
        assert_eq!(c.val, None);
        c.pieces[0] = [2, YARD, YARD, YARD];
        c.val = Some(2);
        c.apply(&Move::Step(2));
        assert_eq!(c.side, Player::Two);
    }

    #[test]
    fn capture_on_normal_square_not_on_castle() {
        let mut c = g();
        // J1 la progres 1 (pătrat 1), J2 la progres 25 (pătrat (24+25)%48 = 1)
        c.pieces[0] = [YARD, YARD, YARD, 0];
        c.pieces[1] = [25, YARD, YARD, YARD];
        c.val = Some(1);
        c.apply(&Move::Step(0));
        assert_eq!(c.pieces[1][0], YARD);
        assert_eq!(c.side, Player::One); // captura dă aruncare în plus
                                         // castel: 6
        let mut c = g();
        c.pieces[0] = [2, YARD, YARD, YARD];
        c.pieces[1] = [30, YARD, YARD, YARD]; // (24+30)%48 = 6
        c.val = Some(4);
        c.apply(&Move::Step(2));
        assert_eq!(c.pieces[1][0], 30);
    }

    #[test]
    fn exact_finish_required() {
        let mut c = g();
        c.pieces[0] = [52, HOME, HOME, HOME];
        c.val = Some(3);
        assert_eq!(c.legal_moves(), vec![Move::Pass]);
        c.val = Some(2);
        assert_eq!(c.legal_moves(), vec![Move::Step(52)]);
        c.apply(&Move::Step(52));
        assert_eq!(c.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..40u64 {
            let mut c = Chaupar::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
            let mut n = 0;
            while c.outcome().is_none() {
                let mv = c.legal_moves();
                assert!(!mv.is_empty());
                c.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 20000, "seed {seed}");
            }
        }
    }
}
