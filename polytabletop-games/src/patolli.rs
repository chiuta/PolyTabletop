//! Patolli (Mexic precolumbian), 2 jucători — reconstituire modernă din regulile publice.
//!
//! Regulile originale nu s-au păstrat integral; implementăm varianta reconstituită cel mai des:
//! * Cărare circulară de 52 de pătrate (în forma tradițională de cruce); fiecare jucător are 6 piese.
//! * Cinci boabe, fiecare cu o față marcată; numărul de fețe marcate = pași (0 = nicio mutare, 5 = 10 pași).
//! * Piesele intră pe cale la pătratul de start al jucătorului (J1 pe 1, J2 pe 27), parcurg exact 52 de
//!   pătrate și ies când ating exact pătratul 52 al drumului lor (nu se poate depăși).
//! * Pe un pătrat ocupat de o piesă proprie nu se poate ateriza; aterizând pe o piesă adversă o trimiți
//!   înapoi (afară), în afara pătratelor sigure (cele patru colțuri ale crucii: 1, 14, 27, 40), unde
//!   piesele coexistă.
//! * Câștigă primul care scoate toate cele 6 piese; după 800 de mutări câștigă cine e mai avansat.
//!
//! Mutări: `piesa 0` (piesa de pe poziția 0 = intră pe cale), `piesa 17` (piesa aflată la pasul 17 din drumul
//! tău), `pas` (când zarul dă 0 sau nu ai mutări). Zarul se vede în starea jocului.
#![allow(clippy::manual_is_multiple_of)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RING: usize = 52;
const PIECES: usize = 6;
const MAX_PLIES: u32 = 800;

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Piece(u8),
    Pass,
}

#[derive(Clone)]
pub struct Patolli {
    rng: Rng,
    /// poziția în drumul propriu: 0 = neintrată, 1..=51 pe cale, 52 = ieșită
    pos: [[u8; PIECES]; 2],
    side: Player,
    roll: u8,
    plies: u32,
    log: String,
}

impl Default for Patolli {
    fn default() -> Self {
        Self::new()
    }
}

fn square(p: usize, step: u8) -> usize {
    let start = if p == 0 { 0 } else { 26 };
    (start + step as usize - 1) % RING
}

fn safe(sq: usize) -> bool {
    sq % 13 == 0
}

impl Patolli {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Patolli {
            rng: Rng::new(seed),
            pos: [[0; PIECES]; 2],
            side: Player::One,
            roll: 0,
            plies: 0,
            log: String::new(),
        };
        g.throw();
        g
    }

    fn throw(&mut self) {
        let marked = (0..5).filter(|_| self.rng.below(2) == 1).count() as u8;
        self.roll = if marked == 5 { 10 } else { marked };
    }

    fn done(&self, p: usize) -> usize {
        self.pos[p].iter().filter(|&&x| x == 52).count()
    }

    fn progress(&self, p: usize) -> i32 {
        self.pos[p].iter().map(|&x| x as i32).sum()
    }

    fn own_at(&self, p: usize, sq: usize) -> bool {
        self.pos[p]
            .iter()
            .any(|&x| x > 0 && x < 52 && square(p, x) == sq)
    }

    fn moves(&self) -> Vec<Move> {
        let p = self.side.idx();
        let mut v = Vec::new();
        if self.roll == 0 {
            return vec![Move::Pass];
        }
        let mut seen = Vec::new();
        for i in 0..PIECES {
            let x = self.pos[p][i];
            if x == 52 || seen.contains(&x) {
                continue;
            }
            let t = x as usize + self.roll as usize;
            if t > 52 {
                continue;
            }
            if t < 52 && self.own_at(p, square(p, t as u8)) {
                continue;
            }
            seen.push(x);
            v.push(Move::Piece(x));
        }
        if v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }
}

impl Game for Patolli {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.moves()
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let q = 1 - p;
        match m {
            Move::Pass => {
                self.log = format!("J{}: pas (zar {})", p + 1, self.roll);
            }
            Move::Piece(x) => {
                let i = self.pos[p].iter().position(|&y| y == *x).unwrap();
                let t = (*x as usize + self.roll as usize) as u8;
                self.pos[p][i] = t;
                self.log = format!("J{}: piesa {} → {}", p + 1, x, t);
                if t < 52 {
                    let sq = square(p, t);
                    if !safe(sq) {
                        for j in 0..PIECES {
                            let y = self.pos[q][j];
                            if y > 0 && y < 52 && square(q, y) == sq {
                                self.pos[q][j] = 0;
                                self.log.push_str(" (trimite înapoi o piesă)");
                            }
                        }
                    }
                }
            }
        }
        self.plies += 1;
        self.side = self.side.other();
        self.throw();
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.done(p) == PIECES {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.plies >= MAX_PLIES {
            return Some(match self.progress(0).cmp(&self.progress(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.progress(p.idx()) - self.progress(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::from("Patolli — 52 de pătrate; X = J1, O = J2; * = pătrat sigur\n");
        let mut line = String::new();
        for sq in 0..RING {
            let a = self.own_at(0, sq);
            let b = self.own_at(1, sq);
            let c = match (a, b) {
                (true, true) => '#',
                (true, false) => 'X',
                (false, true) => 'O',
                _ => {
                    if safe(sq) {
                        '*'
                    } else {
                        '.'
                    }
                }
            };
            line.push(c);
            if sq % 13 == 12 {
                line.push('\n');
            }
        }
        s.push_str(&line);
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: pași {:?}, ieșite {}\n",
                p + 1,
                self.pos[p],
                self.done(p)
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {} (zarul a dat {}).\n",
                self.side.idx() + 1,
                self.roll
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Piece(x) => format!("piesa {x}"),
            Move::Pass => "pas".into(),
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
    fn random_games_end() {
        for seed in 1..=5u64 {
            let mut g = Patolli::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
            }
        }
    }

    #[test]
    fn capture_and_safe_squares() {
        let mut g = Patolli::with_seed(1);
        g.pos = [[0; PIECES]; 2];
        g.roll = 3;
        g.side = Player::One;
        // J1 la pasul 10 -> pătrat 9; J2 pe pătratul 12 = pasul 13 al lui J2? square(1,13)=(26+12)%52=38
        g.pos[0][0] = 10; // pătrat 9
        g.pos[1][0] = (12 + 52 - 26 + 1) as u8; // pe pătratul 12
        g.apply(&Move::Piece(10)); // J1 ajunge pe 12 -> pasul 13
        assert_eq!(g.pos[1][0], 0);
        // pătrat sigur: pătratul 13 (pasul 14)
        g.pos[0][0] = 10;
        g.pos[1][0] = (13 + 52 - 26 + 1) as u8;
        g.side = Player::One;
        g.roll = 4;
        g.apply(&Move::Piece(10));
        assert_ne!(g.pos[1][0], 0);
    }
}
