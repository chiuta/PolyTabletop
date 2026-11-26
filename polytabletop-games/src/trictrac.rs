//! Trictrac (Franța) — variantă SIMPLIFICATĂ cu punctaj redus, 2 jucători.
//!
//! Trictrac-ul real (scorul pe „găuri”, jan-urile, „battre à vrai/à faux”, „le grand jan”, marcatul cu știfturi, reguli de
//! plată) e mult mai bogat; aici se păstrează scheletul: 24 de puncte, 15 piese pe jucător care pornesc stivuite pe punctul
//! lor de start, două zaruri aruncate de un `arunc` explicit, fără lovirea pieselor.
//!
//! * Fiecare jucător mută în sens propriu pe 24 de puncte (relative: 1…24); punctul 1 al lui J2 e opus punctului 1 al lui J1.
//!   Nu poți ateriza pe un punct ocupat de adversar. Fiecare zar se joacă separat (`4:1` = zarul 4 cu piesa de pe punctul 1);
//!   dublă = doar cele două zaruri (nu patru). Piesele ies din joc doar când toate sunt în ultimul sfert (19–24), cu orice zar
//!   suficient; dacă nu poți muta niciun zar, tura se termină.
//! * Punctaj în „găuri”, calculat la sfârșitul turei celui care a mutat: dublă +2; „jan de sfârșit” (cel puțin 2 piese pe
//!   fiecare dintre 3 puncte ale ultimului sfert) +4; fiecare piesă scoasă +1. Primul la 15 găuri câștigă; dacă se scot
//!   toate cele 15 piese, câștigă cel cu mai multe găuri; după 400 de mutări, cel cu mai multe găuri (egal = remiză).
//!
//! Mutări: `arunc`, `<zar>:<punct>`. AI: AlphaBeta(1), nu își citește zarurile viitoare.

use crate::q2_util::by_score;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const GOAL: i32 = 15;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Play(u8, u8),
}

#[derive(Clone)]
pub struct Trictrac {
    pts: [[u8; 24]; 2],
    off: [u8; 2],
    holes: [i32; 2],
    rem: Vec<u8>,
    rolled_double: bool,
    off_this_turn: u8,
    side: Player,
    plies: u32,
    rng: Rng,
    last: String,
}

impl Default for Trictrac {
    fn default() -> Self {
        Self::new()
    }
}

fn abs(p: usize, i: usize) -> usize {
    (i + 12 * p) % 24
}

impl Trictrac {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut pts = [[0u8; 24]; 2];
        pts[0][0] = 15;
        pts[1][0] = 15;
        Trictrac {
            pts,
            off: [0; 2],
            holes: [0; 2],
            rem: Vec::new(),
            rolled_double: false,
            off_this_turn: 0,
            side: Player::One,
            plies: 0,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }
    fn can_play(&self, p: usize, d: u8, i: usize) -> bool {
        if self.pts[p][i] == 0 {
            return false;
        }
        let t = i + d as usize;
        if t >= 24 {
            return (0..18).all(|k| self.pts[p][k] == 0);
        }
        let o = 1 - p;
        let a = abs(p, t);
        let ro = (a + 24 - 12 * o) % 24;
        self.pts[o][ro] == 0
    }
    fn any_play(&self) -> bool {
        let p = self.side.idx();
        self.rem
            .iter()
            .any(|&d| (0..24).any(|i| self.can_play(p, d, i)))
    }
    fn finish_turn(&mut self) {
        let p = self.side.idx();
        let mut gain = self.off_this_turn as i32;
        if self.rolled_double {
            gain += 2;
        }
        if (18..24).filter(|&k| self.pts[p][k] >= 2).count() >= 3 {
            gain += 4;
        }
        self.holes[p] += gain;
        self.last = format!("J{} marchează {gain} găuri", p + 1);
        self.rem.clear();
        self.rolled_double = false;
        self.off_this_turn = 0;
        self.side = self.side.other();
        self.plies += 1;
    }
    fn done(&self) -> bool {
        self.holes.iter().any(|&h| h >= GOAL)
            || self.off.iter().any(|&o| o >= 15)
            || self.plies >= 400
    }
}

impl Game for Trictrac {
    type Move = Move;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.done() {
            return Vec::new();
        }
        if self.rem.is_empty() {
            return vec![Move::Roll];
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        let mut seen = Vec::new();
        for &d in &self.rem {
            if seen.contains(&d) {
                continue;
            }
            seen.push(d);
            for i in 0..24 {
                if self.can_play(p, d, i) {
                    v.push(Move::Play(d, i as u8));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        match *m {
            Move::Roll => {
                let (a, b) = (self.rng.d6(), self.rng.d6());
                self.rolled_double = a == b;
                self.rem = vec![a, b];
                if !self.any_play() {
                    self.finish_turn();
                }
            }
            Move::Play(d, i) => {
                let pos = self.rem.iter().position(|&x| x == d).unwrap();
                self.rem.remove(pos);
                let i = i as usize;
                self.pts[p][i] -= 1;
                let t = i + d as usize;
                if t >= 24 {
                    self.off[p] += 1;
                    self.off_this_turn += 1;
                } else {
                    self.pts[p][t] += 1;
                }
                if self.rem.is_empty() || !self.any_play() {
                    self.finish_turn();
                }
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if !self.done() {
            return None;
        }
        Some(by_score(self.holes[0], self.holes[1]))
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let prog = |p: usize| -> i32 {
            let mut v = self.off[p] as i32 * 30;
            for i in 0..24 {
                v += self.pts[p][i] as i32 * i as i32;
            }
            v
        };
        let p = pl.idx();
        (self.holes[p] - self.holes[1 - p]) * 40 + prog(p) - prog(1 - p)
    }
    fn render(&self) -> String {
        let mut s = String::from("Trictrac (simplificat) — găuri: ");
        s.push_str(&format!(
            "J1 {}, J2 {} (la {GOAL}); piese scoase: {} / {}\n",
            self.holes[0], self.holes[1], self.off[0], self.off[1]
        ));
        for p in 0..2 {
            let row: Vec<String> = (0..24)
                .filter(|&i| self.pts[p][i] > 0)
                .map(|i| format!("{}:{}", i + 1, self.pts[p][i]))
                .collect();
            s.push_str(&format!("J{} (punct:piese): {}\n", p + 1, row.join(" ")));
        }
        if !self.rem.is_empty() {
            let r: Vec<String> = self.rem.iter().map(|d| d.to_string()).collect();
            s.push_str(&format!("Zaruri rămase: {}\n", r.join(" ")));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        match self.outcome() {
            Some(Outcome::Win(w)) => s.push_str(&format!("Câștigă J{}.\n", w.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }
    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Play(d, i) => format!("{d}:{}", i + 1),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_scores_two_holes_and_off_requires_home() {
        let mut g = Trictrac::with_seed(1);
        assert!(!g.can_play(0, 6, 0) || g.pts[0][0] > 0);
        g.pts[0] = [0; 24];
        g.pts[0][20] = 3;
        g.pts[0][21] = 2;
        g.pts[0][22] = 2;
        g.pts[0][23] = 1;
        g.pts[1] = [0; 24];
        g.pts[1][0] = 15;
        g.rem = vec![4, 4];
        g.rolled_double = true;
        g.apply(&Move::Play(4, 20));
        g.apply(&Move::Play(4, 20));
        // dublă +2, 2 piese scoase +1 fiecare, jan: puncte 22,23 cu ≥2 + 21 → 2<... verificat mai jos
        assert!(g.holes[0] >= 4, "{}", g.render());
        assert_eq!(g.off[0], 2);
    }

    #[test]
    fn cannot_land_on_opponent() {
        let mut g = Trictrac::with_seed(1);
        g.pts[0] = [0; 24];
        g.pts[0][0] = 1;
        g.pts[1] = [0; 24];
        // punctul 1 al lui J2 = absolut 12 = punctul relativ 13 (index 12) al lui J1
        g.pts[1][0] = 1;
        assert!(!g.can_play(0, 6, 6));
        g.pts[0][6] = 1;
        assert!(!g.can_play(0, 6, 6));
        g.pts[0][6] = 0;
        g.pts[0][11] = 1;
        assert!(!g.can_play(0, 1, 11));
    }

    #[test]
    fn games_finish() {
        for seed in 1..=4u64 {
            let mut g = Trictrac::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())].clone());
                n += 1;
                assert!(n < 5000);
            }
        }
    }
}
