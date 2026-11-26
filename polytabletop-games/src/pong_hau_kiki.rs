//! Pong hau k'i (China), 2 jucători, din regulile publice.
//!
//! * Tabla are cinci puncte: două sus (1, 2), centrul (3) și două jos (4, 5); liniile unesc 1–2, 4–5
//!   și cele două diagonale prin centru (1–3–5 și 2–3–4).
//! * Fiecare jucător are două piese: J1 (X) pe 1 și 2, J2 (O) pe 4 și 5; centrul e gol. J1 începe.
//! * Muți o piesă pe un punct vecin liber (pe o linie). Pierde cine nu mai poate muta (e blocat).
//! * Remiză la a treia repetare a aceleiași poziții sau după 100 de mutări.
//!
//! Mutări: `1-3` (de pe punctul 1 pe punctul 3).
#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ADJ: [&[usize]; 5] = [&[1, 2], &[0, 2], &[0, 1, 3, 4], &[2, 4], &[2, 3]];
const MAX_PLIES: u16 = 100;

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct PongHauKiki {
    cell: [u8; 5],
    side: Player,
    plies: u16,
    history: Vec<u32>,
    drawn: bool,
}

impl Default for PongHauKiki {
    fn default() -> Self {
        Self::new()
    }
}

impl PongHauKiki {
    pub fn new() -> Self {
        let mut g = PongHauKiki {
            cell: [1, 1, 0, 2, 2],
            side: Player::One,
            plies: 0,
            history: Vec::new(),
            drawn: false,
        };
        g.history.push(g.key());
        g
    }

    pub fn with_seed(_seed: u64) -> Self {
        Self::new()
    }

    fn key(&self) -> u32 {
        let mut k = self.side.idx() as u32;
        for &c in &self.cell {
            k = k * 3 + c as u32;
        }
        k
    }

    fn moves_for(&self, p: Player) -> Vec<Move> {
        let t = p.idx() as u8 + 1;
        let mut v = Vec::new();
        for from in 0..5 {
            if self.cell[from] == t {
                for &to in ADJ[from] {
                    if self.cell[to] == 0 {
                        v.push(Move {
                            from: from as u8 + 1,
                            to: to as u8 + 1,
                        });
                    }
                }
            }
        }
        v
    }
}

impl Game for PongHauKiki {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.moves_for(self.side)
    }

    fn apply(&mut self, m: &Move) {
        let t = self.side.idx() as u8 + 1;
        self.cell[m.from as usize - 1] = 0;
        self.cell[m.to as usize - 1] = t;
        self.side = self.side.other();
        self.plies += 1;
        let k = self.key();
        self.history.push(k);
        if self.history.iter().filter(|&&x| x == k).count() >= 3 || self.plies >= MAX_PLIES {
            self.drawn = true;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.moves_for(self.side).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        if self.drawn {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.moves_for(p).len() as i32 - self.moves_for(p.other()).len() as i32) * 10
    }

    fn render(&self) -> String {
        let c = |i: usize| match self.cell[i] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        };
        let mut s = String::from("Pong hau k'i — X = J1, O = J2\n");
        s.push_str(&format!("  {}(1)-----{}(2)\n", c(0), c(1)));
        s.push_str("     \\   /\n");
        s.push_str(&format!("      {}(3)\n", c(2)));
        s.push_str("     /   \\\n");
        s.push_str(&format!("  {}(4)-----{}(5)\n", c(3), c(4)));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Jucătorul {} câștigă (adversarul e blocat).\n",
                p.idx() + 1
            )),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!("{}-{}", m.from, m.to)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_moves() {
        let g = PongHauKiki::new();
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert_eq!(t.len(), 2);
        assert!(t.contains(&"1-3".to_string()) && t.contains(&"2-3".to_string()));
    }

    #[test]
    fn blocked_loses() {
        let mut g = PongHauKiki::new();
        g.cell = [2, 1, 2, 2, 1];
        // J1 pe 2 (vecini 1:O, 3:O) și 5 (vecini 3:O, 4:O) -> blocat
        g.side = Player::One;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = PongHauKiki::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
            }
        }
    }
}
