//! Jocul gâștei (Game of the Goose, Italia), 2 jucători, din regulile publice.
//!
//! * Cărare de 63 de căsuțe, două zaruri; pornești din 0 și ajungi **exact** pe 63 (dacă treci peste, te
//!   întorci cu surplusul).
//! * Căsuțe speciale: **gâscă** (5, 9, 14, 18, 23, 27, 32, 36, 41, 45, 50, 54, 59): mai mergi o dată cu
//!   aceeași sumă; **pod** (6): sari pe 12; **han** (19): pierzi o tură; **fântână** (31): pierzi trei ture;
//!   **labirint** (42): te întorci pe 30; **închisoare** (52): pierzi trei ture; **moartea** (58): te
//!   întorci pe 1. Prima aruncare 3+6 duce pe 26, 4+5 pe 53.
//! * Variante alese de noi: în loc de „stai până vine altul” (care poate bloca doi jucători), fântâna și
//!   închisoarea costă trei ture; nu se schimbă locurile când ajungi pe căsuța adversarului.
//!
//! Mutări: `arunc` (zarurile se aruncă în starea jocului), `astept` (tură pierdută).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const GOOSE: [u8; 13] = [5, 9, 14, 18, 23, 27, 32, 36, 41, 45, 50, 54, 59];

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Roll,
    Wait,
}

#[derive(Clone)]
pub struct Goose {
    rng: Rng,
    pos: [u8; 2],
    skip: [u8; 2],
    side: Player,
    first: [bool; 2],
    winner: Option<Player>,
    log: String,
}

impl Default for Goose {
    fn default() -> Self {
        Self::new()
    }
}

impl Goose {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Goose {
            rng: Rng::new(seed),
            pos: [0, 0],
            skip: [0, 0],
            side: Player::One,
            first: [true, true],
            winner: None,
            log: String::new(),
        }
    }

    pub fn positions(&self) -> [u8; 2] {
        self.pos
    }

    /// Aplică efectele căsuței; întoarce `true` dacă se mai aruncă (gâscă).
    fn land(&mut self, who: usize, sum: u8, log: &mut String) -> bool {
        let mut p = self.pos[who];
        if p > 63 {
            p = 63 - (p - 63);
            log.push_str(&format!(" întoarcere pe {p}"));
        }
        self.pos[who] = p;
        match p {
            6 => {
                self.pos[who] = 12;
                log.push_str(" podul: sare pe 12");
            }
            19 => {
                self.skip[who] = 1;
                log.push_str(" hanul: pierde o tură");
            }
            31 => {
                self.skip[who] = 3;
                log.push_str(" fântâna: pierde trei ture");
            }
            42 => {
                self.pos[who] = 30;
                log.push_str(" labirintul: înapoi pe 30");
            }
            52 => {
                self.skip[who] = 3;
                log.push_str(" închisoarea: pierde trei ture");
            }
            58 => {
                self.pos[who] = 1;
                log.push_str(" moartea: înapoi pe 1");
            }
            63 => {
                self.winner = Some(self.side);
            }
            x if GOOSE.contains(&x) => {
                self.pos[who] = (x + sum).min(126);
                log.push_str(&format!(" gâscă: încă {sum}"));
                return true;
            }
            _ => {}
        }
        false
    }
}

impl Game for Goose {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        if self.skip[self.side.idx()] > 0 {
            vec![Move::Wait]
        } else {
            vec![Move::Roll]
        }
    }

    fn apply(&mut self, m: &Move) {
        let who = self.side.idx();
        let mut log = format!("J{}:", who + 1);
        match m {
            Move::Wait => {
                self.skip[who] -= 1;
                log.push_str(" așteaptă");
            }
            Move::Roll => {
                let a = self.rng.d6();
                let b = self.rng.d6();
                let sum = a + b;
                log.push_str(&format!(" {a}+{b}"));
                if self.first[who] && ((a == 3 && b == 6) || (a == 6 && b == 3)) {
                    self.pos[who] = 26;
                    log.push_str(" → 26");
                } else if self.first[who] && ((a == 4 && b == 5) || (a == 5 && b == 4)) {
                    self.pos[who] = 53;
                    log.push_str(" → 53");
                } else {
                    self.pos[who] += sum;
                    log.push_str(&format!(" → {}", self.pos[who].min(126)));
                    let mut guard = 0;
                    while self.winner.is_none() && self.land(who, sum, &mut log) && guard < 10 {
                        guard += 1;
                        log.push_str(&format!(" → {}", self.pos[who].min(126)));
                    }
                }
                self.first[who] = false;
            }
        }
        self.log = log;
        if self.winner.is_none() {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.pos[p.idx()] as i32 - self.pos[p.other().idx()] as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Jocul gâștei — 63 de căsuțe (G = gâscă)\n");
        let mut row = String::new();
        for i in 0..=63u8 {
            let mark = match i {
                x if GOOSE.contains(&x) => 'G',
                6 => 'P',
                19 => 'H',
                31 => 'F',
                42 => 'L',
                52 => 'Î',
                58 => 'M',
                _ => '.',
            };
            let (a, b) = (self.pos[0] == i, self.pos[1] == i);
            let c = if a && b {
                '*'
            } else if a {
                'X'
            } else if b {
                'O'
            } else {
                mark
            };
            row.push(c);
            if i % 16 == 15 {
                row.push('\n');
            }
        }
        s.push_str(&row);
        s.push_str(
            "\n(P pod, H han, F fântână, L labirint, Î închisoare, M moarte; X = J1, O = J2)\n",
        );
        s.push_str(&format!(
            "J1 pe {}{}, J2 pe {}{}\n",
            self.pos[0],
            if self.skip[0] > 0 {
                format!(" (pierde {} ture)", self.skip[0])
            } else {
                String::new()
            },
            self.pos[1],
            if self.skip[1] > 0 {
                format!(" (pierde {} ture)", self.skip[1])
            } else {
                String::new()
            },
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.winner {
            Some(p) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Wait => "astept".into(),
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
        for seed in 1..=30u64 {
            let mut g = Goose::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[0]);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn bounce_and_special_squares() {
        let mut g = Goose::with_seed(1);
        let mut log = String::new();
        g.pos[0] = 65;
        g.land(0, 5, &mut log);
        assert_eq!(g.pos[0], 61);
        g.pos[0] = 42;
        g.land(0, 5, &mut log);
        assert_eq!(g.pos[0], 30);
        g.pos[0] = 58;
        g.land(0, 5, &mut log);
        assert_eq!(g.pos[0], 1);
        g.pos[0] = 6;
        g.land(0, 5, &mut log);
        assert_eq!(g.pos[0], 12);
        g.pos[0] = 9;
        assert!(g.land(0, 5, &mut log));
        assert_eq!(g.pos[0], 14);
        g.pos[0] = 52;
        g.land(0, 5, &mut log);
        assert_eq!(g.skip[0], 3);
    }

    #[test]
    fn exact_finish() {
        let mut g = Goose::with_seed(1);
        let mut log = String::new();
        g.pos[0] = 63;
        g.land(0, 3, &mut log);
        assert_eq!(g.winner, Some(Player::One));
    }
}
