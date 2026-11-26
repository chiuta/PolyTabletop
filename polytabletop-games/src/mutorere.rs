//! Mū tōrere (Noua Zeelandă, maori), 2 jucători, din regulile publice.
//!
//! * Tabla: opt puncte `kewai` (1–8, în cerc) și un punct central `pūtahi` (0).
//!   Fiecare jucător are patru piese; la start ocupă patru `kewai` vecine
//!   (Jucătorul 1: 1–4, Jucătorul 2: 5–8), iar centrul e gol. Începe Jucătorul 1.
//! * O piesă se mută pe un punct **vecin gol**: de pe un `kewai` pe `kewai`-ul
//!   vecin (i±1) sau în centru; din centru pe orice `kewai` gol (toate sunt vecine).
//! * **Intrarea în centru** e permisă doar dacă piesa mutată se învecinează cu cel
//!   puțin o piesă a adversarului.
//! * Pierde jucătorul care nu mai are nicio mutare (e blocat).
//!
//! Alegeri de implementare (sursele nu spun): cine începe (Jucătorul 1) și
//! remiza – poziția inițială e remiză teoretic –, declarată la a treia repetare
//! a aceleiași poziții (cu același jucător la mutare) sau după 200 de mutări.
//!
//! Mutări: `3-4` (de pe kewai 3 pe kewai 4), `2-0` (în centru), `0-6` (din centru).
//! Informație perfectă, fără ascuns.

use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

const MAX_PLIES: u16 = 200;

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct MuTorere {
    /// Indexul 0 = centru, 1..=8 = kewai. Conținut: 0 gol, 1 = J1, 2 = J2.
    cell: [u8; 9],
    side: Player,
    plies: u16,
    history: Vec<u64>,
    drawn: bool,
}

impl Default for MuTorere {
    fn default() -> Self {
        Self::new()
    }
}

fn next(i: u8) -> u8 {
    i % 8 + 1
}
fn prev(i: u8) -> u8 {
    (i + 6) % 8 + 1
}

impl MuTorere {
    pub fn new() -> Self {
        let mut cell = [0u8; 9];
        for (i, c) in cell.iter_mut().enumerate().skip(1) {
            *c = if i <= 4 { 1 } else { 2 };
        }
        let mut g = MuTorere {
            cell,
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

    fn key(&self) -> u64 {
        let mut h = self.side.idx() as u64;
        for (i, &c) in self.cell.iter().enumerate() {
            h ^= hash_mix(((i as u64) << 8) | c as u64);
        }
        h
    }

    fn tag(p: Player) -> u8 {
        p.idx() as u8 + 1
    }

    fn moves_for(&self, p: Player) -> Vec<Move> {
        let me = Self::tag(p);
        let foe = 3 - me;
        let mut v = Vec::new();
        for from in 1..=8u8 {
            if self.cell[from as usize] != me {
                continue;
            }
            for to in [next(from), prev(from)] {
                if self.cell[to as usize] == 0 {
                    v.push(Move { from, to });
                }
            }
            if self.cell[0] == 0
                && (self.cell[next(from) as usize] == foe || self.cell[prev(from) as usize] == foe)
            {
                v.push(Move { from, to: 0 });
            }
        }
        if self.cell[0] == me {
            for to in 1..=8u8 {
                if self.cell[to as usize] == 0 {
                    v.push(Move { from: 0, to });
                }
            }
        }
        v
    }

    pub fn cells(&self) -> [u8; 9] {
        self.cell
    }
}

impl Game for MuTorere {
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
        let t = Self::tag(self.side);
        self.cell[m.from as usize] = 0;
        self.cell[m.to as usize] = t;
        self.side = self.side.other();
        self.plies += 1;
        let k = self.key();
        self.history.push(k);
        let reps = self.history.iter().filter(|&&x| x == k).count();
        if reps >= 3 || self.plies >= MAX_PLIES {
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
        let mine = self.moves_for(p).len() as i32;
        let theirs = self.moves_for(p.other()).len() as i32;
        (mine - theirs) * 10
    }

    fn render(&self) -> String {
        let c = |i: usize| match self.cell[i] {
            1 => "X",
            2 => "O",
            _ => ".",
        };
        let mut s = String::from("Mū tōrere — X = Jucător 1, O = Jucător 2\n");
        s.push_str(&format!("      {}2   {}3\n", c(2), c(3)));
        s.push_str(&format!("   {}1   ({})   {}4\n", c(1), c(0), c(4)));
        s.push_str(&format!(
            "      {}8   {}7   {}6   {}5\n",
            c(8),
            c(7),
            c(6),
            c(5)
        ));
        s.push_str("Kewai 1–8 în cerc (1-2-3-4-5-6-7-8-1), pūtahi = 0 (centru).\n");
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Jucător {} câștigă (adversarul e blocat).\n",
                p.idx() + 1
            )),
            Some(Outcome::Draw) => s.push_str("Remiză (poziție repetată sau limita de mutări).\n"),
            None => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!("{}-{}", m.from, m.to)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(8)
    }

    fn hash_key(&self) -> Option<u64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_position_moves() {
        let g = MuTorere::new();
        // J1 pe 1..4: 4 se poate muta pe centru (vecin cu 5 al adversarului) și 1 (vecin 8),
        let mv = g.legal_moves();
        let txt: Vec<String> = mv.iter().map(|m| g.format_move(m)).collect();
        assert!(txt.contains(&"4-0".to_string()));
        assert!(txt.contains(&"1-0".to_string()));
        assert!(!txt.contains(&"2-0".to_string()));
        assert!(!txt.contains(&"3-0".to_string()));
        assert_eq!(mv.len(), 2);
    }

    #[test]
    fn centre_needs_adjacent_enemy_and_centre_moves_anywhere_empty() {
        let mut g = MuTorere::new();
        g.apply(&Move { from: 4, to: 0 });
        // J2: kewai 4 e gol; 5 poate merge pe 4; centrul e ocupat
        let txt: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(txt.contains(&"5-4".to_string()));
        assert!(!txt.iter().any(|t| t.ends_with("-0")));
    }

    #[test]
    fn blocked_player_loses() {
        let mut g = MuTorere::new();
        g.cell = [0; 9];
        // J2 blocat: o piesă pe 1 înconjurată de J1 pe 2 și 8, centrul ocupat de J1
        g.cell[1] = 2;
        g.cell[2] = 1;
        g.cell[8] = 1;
        g.cell[0] = 1;
        g.side = Player::Two;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn pieces_are_conserved_and_games_end() {
        for seed in 1..=30u64 {
            let mut g = MuTorere::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.cell.iter().filter(|&&c| c == 1).count(), 4);
                assert_eq!(g.cell.iter().filter(|&&c| c == 2).count(), 4);
                n += 1;
                assert!(n <= 250);
            }
        }
    }

    #[test]
    fn repetition_is_a_draw() {
        let mut g = MuTorere::new();
        let m = g.legal_moves()[0].clone();
        let mut after = g.clone();
        after.apply(&m);
        let k = after.key();
        g.history.push(k);
        g.history.push(k);
        assert_eq!(Game::outcome(&g), None);
        g.apply(&m);
        assert_eq!(Game::outcome(&g), Some(Outcome::Draw));
    }

    #[test]
    fn ai_does_not_lose_to_random() {
        use polytabletop_core::GameObj;
        for seed in 1..=8u64 {
            let mut g: Box<dyn GameObj> = Box::new(MuTorere::new());
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 300 {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
            }
            assert_ne!(g.outcome(), Some(Outcome::Win(Player::Two)));
        }
    }
}
