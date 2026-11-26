#![allow(clippy::doc_lazy_continuation)]
//! Omweso (Uganda), joc de semințe din familia mancala, pentru doi jucători.
//!
//! * Tablă cu 4 rânduri × 8 gropi; fiecare jucător are două rânduri: **interior** (cel dinspre
//!   centrul mesei) și **exterior**. 64 de semințe: la început câte 2 în fiecare groapă (aranjarea
//!   inițială se alege liber în regulile tradiționale; aici e fixată).
//! * Mutarea: alegi o groapă proprie cu cel puțin 2 semințe și le semeni, câte una pe groapă, în sens
//!   trigonometric prin cele 16 gropi ale tale (rândul interior de la stânga la dreapta, rândul
//!   exterior de la dreapta la stânga). Nu semeni niciodată pe partea adversarului.
//! * Dacă ultima sămânță cade într-o groapă nevidă:
//!   - dacă e o groapă din rândul interior, iar gropile adversarului din aceeași coloană (interioară
//!     și exterioară) sunt ambele nevide, **capturezi** semințele din ele și le semeni de la groapa
//!     următoare celei din care a început ture în curs;
//!   - altfel iei toate semințele din groapa respectivă și continui să semeni (releu).
//!   Tura se termină când ultima sămânță cade într-o groapă goală.
//! * Pierde cine nu mai are nicio groapă cu cel puțin 2 semințe la rândul său. Regulile cu semănat
//!   în sens invers (clockwise) și bonusurile de turneu nu sunt implementate. Un releu infinit se
//!   oprește după 400 de ture-releu; după 400 de mutări câștigă cine are mai multe semințe.
//! Mutări: `i1`…`i8` (groapa din rândul interior, numărată de la stânga), `o1`…`o8` (rândul exterior).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COLS: usize = 8;
const MAX_MOVES: u16 = 400;
const MAX_LAPS: usize = 400;

/// Poziția pe cerc (0..15): 0–7 interior stânga→dreapta, 8–15 exterior dreapta→stânga.
fn pit_index(label: usize) -> usize {
    label
}

fn col_of(pos: usize) -> usize {
    if pos < COLS {
        pos
    } else {
        15 - pos
    }
}

#[derive(Clone)]
pub struct Omweso {
    /// pits[p][pos], pos conform `pit_index`.
    pits: [[u8; 16]; 2],
    side: Player,
    moves: u16,
}

impl Default for Omweso {
    fn default() -> Self {
        Self::new()
    }
}

impl Omweso {
    pub fn new() -> Self {
        Omweso {
            pits: [[2; 16]; 2],
            side: Player::One,
            moves: 0,
        }
    }

    fn total(&self, p: usize) -> u32 {
        self.pits[p].iter().map(|&x| x as u32).sum()
    }

    fn movable(&self, p: usize) -> Vec<usize> {
        (0..16).filter(|&i| self.pits[p][i] >= 2).collect()
    }

    fn pit_name(pos: usize) -> String {
        if pos < COLS {
            format!("i{}", pos + 1)
        } else {
            format!("o{}", 8 - (pos - COLS))
        }
    }

    /// Joacă o mutare completă din groapa `start` pentru jucătorul `me`.
    fn sow(&mut self, me: usize, start: usize) {
        let you = 1 - me;
        let mut hand = self.pits[me][start] as u32;
        self.pits[me][start] = 0;
        let mut pos = start;
        let mut lap_start = start;
        for _ in 0..MAX_LAPS {
            // semănare
            let mut last = pos;
            while hand > 0 {
                last = (last + 1) % 16;
                self.pits[me][last] += 1;
                hand -= 1;
            }
            if self.pits[me][last] == 1 {
                return; // groapă goală
            }
            // captură?
            if last < COLS {
                let c = col_of(last);
                let (oi, oo) = (c, 15 - c);
                let (a, b) = (self.pits[you][oi], self.pits[you][oo]);
                if a > 0 && b > 0 {
                    hand = a as u32 + b as u32;
                    self.pits[you][oi] = 0;
                    self.pits[you][oo] = 0;
                    pos = lap_start;
                    continue;
                }
            }
            // releu
            hand = self.pits[me][last] as u32;
            self.pits[me][last] = 0;
            pos = last;
            lap_start = last;
        }
    }
}

impl Game for Omweso {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.movable(self.side.idx())
            .into_iter()
            .map(|i| i as u8)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        self.sow(self.side.idx(), *m as usize);
        self.moves += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let me = self.side.idx();
        if self.movable(me).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        if self.moves >= MAX_MOVES {
            return Some(match self.total(0).cmp(&self.total(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), 1 - p.idx());
        let mob = |x: usize| self.movable(x).len() as i32;
        (self.total(a) as i32 - self.total(b) as i32) * 10 + (mob(a) - mob(b)) * 3
    }

    fn render(&self) -> String {
        let row = |p: usize, inner: bool| -> String {
            let v: Vec<String> = if inner {
                (0..COLS)
                    .map(|c| format!("{:2}", self.pits[p][c]))
                    .collect()
            } else {
                (0..COLS)
                    .map(|c| format!("{:2}", self.pits[p][15 - c]))
                    .collect()
            };
            v.join(" ")
        };
        let mut s = String::from(
            "Omweso — gropi interior/exterior (coloanele 1–8 din perspectiva lui J1)\n",
        );
        // J2 e „sus”: coloanele sale sunt oglindite față de J1
        let rev = |t: String| -> String {
            let mut parts: Vec<&str> = t.split(' ').filter(|x| !x.is_empty()).collect();
            parts.reverse();
            parts
                .iter()
                .map(|x| format!("{x:>2}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        s.push_str(&format!("J2 exterior: {}\n", rev(row(1, false))));
        s.push_str(&format!("J2 interior: {}\n", rev(row(1, true))));
        s.push_str(&format!("J1 interior: {}\n", row(0, true)));
        s.push_str(&format!("J1 exterior: {}\n", row(0, false)));
        s.push_str(&format!(
            "Semințe: J1 {}, J2 {}. La mutare: J{}.\n",
            self.total(0),
            self.total(1),
            self.side.idx() + 1
        ));
        s
    }

    fn format_move(&self, m: &u8) -> String {
        Self::pit_name(pit_index(*m as usize))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn seeds_are_conserved_and_names_unique() {
        let g = Omweso::new();
        let names: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert_eq!(names.len(), 16);
        let mut s = names.clone();
        s.sort();
        s.dedup();
        assert_eq!(s.len(), 16);
        let mut h = Omweso::new();
        let mut rng = Rng::new(3);
        for _ in 0..60 {
            if h.outcome().is_some() {
                break;
            }
            let mv = h.legal_moves();
            h.apply(&mv[rng.below(mv.len())]);
            assert_eq!(h.total(0) + h.total(1), 64);
        }
    }

    #[test]
    fn capture_takes_both_opposing_pits() {
        let mut g = Omweso::new();
        g.pits = [[0; 16]; 2];
        g.pits[0][0] = 2; // i1: 2 semințe -> groapele 1 și 2
        g.pits[0][2] = 1; // ultima cade în groapa 2, nevidă: captură în coloana 2
        g.pits[1][2] = 3; // J2 interior, coloana 2
        g.pits[1][13] = 2; // J2 exterior, aceeași coloană (15 - 2)
        g.pits[0][8] = 1;
        let before = g.total(0) + g.total(1);
        g.sow(0, 0);
        assert_eq!(g.pits[1][2], 0);
        assert_eq!(g.pits[1][13], 0);
        assert_eq!(g.total(0) + g.total(1), before);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Omweso::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 1000);
            }
        }
    }
}
