//! Beetle (Marea Britanie) — joc de zaruri pentru copii: construiești un gândac.
//!
//! Aruncă un zar pe rând (`arunc`). Piesele: 1 = corp, 2 = cap, 3 = picior (6), 4 = antenă (2),
//! 5 = ochi (2), 6 = coadă. Ordinea contează: corpul se pune primul; capul, picioarele și coada
//! cer corpul; antenele și ochii cer capul. Un zar care nu se potrivește se pierde.
//! Câștigă primul gândac complet (13 piese).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// corp, cap, picioare, antene, ochi, coadă
const NEED: [u8; 6] = [1, 1, 6, 2, 2, 1];
const NAMES: [&str; 6] = ["corp", "cap", "picioare", "antene", "ochi", "coadă"];

#[derive(Clone)]
pub struct Beetle {
    parts: [[u8; 6]; 2],
    side: Player,
    last: Option<(u8, bool)>,
    rng: Rng,
}

impl Default for Beetle {
    fn default() -> Self {
        Self::new()
    }
}

impl Beetle {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Beetle {
            parts: [[0; 6]; 2],
            side: Player::One,
            last: None,
            rng: Rng::new(seed),
        }
    }
    pub fn count(&self, p: Player) -> u8 {
        self.parts[p.idx()].iter().sum()
    }
    fn can_add(&self, p: usize, face: usize) -> bool {
        let h = &self.parts[p];
        if h[face] >= NEED[face] {
            return false;
        }
        match face {
            0 => true,
            1 | 2 | 5 => h[0] > 0,
            _ => h[1] > 0,
        }
    }
}

impl Game for Beetle {
    type Move = ();
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<()> {
        if self.outcome().is_some() {
            Vec::new()
        } else {
            vec![()]
        }
    }
    fn apply(&mut self, _m: &()) {
        let d = self.rng.d6();
        let p = self.side.idx();
        let ok = self.can_add(p, d as usize - 1);
        if ok {
            self.parts[p][d as usize - 1] += 1;
        }
        self.last = Some((d, ok));
        if self.outcome().is_none() {
            self.side = self.side.other();
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.count(p) == 13 {
                return Some(Outcome::Win(p));
            }
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        self.count(p) as i32 - self.count(p.other()) as i32
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for p in 0..2 {
            let h = &self.parts[p];
            s.push_str(&format!("J{}: ", p + 1));
            for k in 0..6 {
                s.push_str(&format!("{} {}/{}  ", NAMES[k], h[k], NEED[k]));
            }
            s.push_str(&format!(
                "({}/13)\n",
                self.count(if p == 0 { Player::One } else { Player::Two })
            ));
        }
        if let Some((d, ok)) = self.last {
            s.push_str(&format!(
                "Ultimul zar: {d} ({}).\n",
                if ok {
                    "pus pe gândac"
                } else {
                    "nu se potrivește"
                }
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}.", self.side.idx() + 1));
        s
    }
    fn format_move(&self, _m: &()) -> String {
        "arunc".into()
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn body_is_needed_first() {
        let g = Beetle::with_seed(1);
        assert!(g.can_add(0, 0));
        assert!(!g.can_add(0, 1));
        assert!(!g.can_add(0, 3));
        let mut h = g.clone();
        h.parts[0][0] = 1;
        h.parts[0][1] = 1;
        assert!(h.can_add(0, 3));
        assert!(!h.can_add(0, 1));
        assert_eq!(GameObj::legal_moves(&h), vec!["arunc"]);
    }

    #[test]
    fn random_games_finish() {
        random_finishes(|s| Box::new(Beetle::with_seed(s)), 8, 3000);
        ai_vs_random(|s| Box::new(Beetle::with_seed(s)), 2, Player::One, 3000);
    }
}
