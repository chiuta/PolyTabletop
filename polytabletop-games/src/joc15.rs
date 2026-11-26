//! Jocul de 15 (15 puzzle, SUA) — cursă pe două table identice.
//!
//! Fiecare jucător are propria tablă 4×4 cu piesele 1–15 și un loc gol, amestecate la fel
//! (același amestec pentru amândoi). Pe rând, fiecare mută o piesă de lângă locul gol pe
//! tabla *lui*; câștigă primul care își aduce tabla în ordinea 1–15 (golul jos-dreapta).
//! După 300 de mutări câștigă cine e mai aproape (suma distanțelor Manhattan), egal = remiză.
//! Mutare: numărul piesei, de ex. `7`.
//!
//! Amestecul pornește din starea rezolvată prin mutări legale, deci e mereu rezolvabil.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SOLVED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
const PLY_CAP: u16 = 300;
const SCRAMBLE: usize = 70;

#[derive(Clone)]
pub struct Joc15 {
    b: [[u8; 16]; 2],
    side: Player,
    ply: u16,
}

impl Default for Joc15 {
    fn default() -> Self {
        Self::new()
    }
}

fn neighbours(blank: usize) -> Vec<usize> {
    let (r, c) = (blank / 4, blank % 4);
    let mut v = Vec::new();
    if r > 0 {
        v.push(blank - 4);
    }
    if r < 3 {
        v.push(blank + 4);
    }
    if c > 0 {
        v.push(blank - 1);
    }
    if c < 3 {
        v.push(blank + 1);
    }
    v
}

pub fn manhattan(b: &[u8; 16]) -> i32 {
    let mut d = 0;
    for (i, &t) in b.iter().enumerate() {
        if t != 0 {
            let g = t as usize - 1;
            d += ((i / 4) as i32 - (g / 4) as i32).abs() + ((i % 4) as i32 - (g % 4) as i32).abs();
        }
    }
    d
}

impl Joc15 {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut b = SOLVED;
        let mut blank = 15usize;
        let mut prev = usize::MAX;
        for _ in 0..SCRAMBLE {
            let nb: Vec<usize> = neighbours(blank)
                .into_iter()
                .filter(|&n| n != prev)
                .collect();
            let t = nb[rng.below(nb.len())];
            b.swap(blank, t);
            prev = blank;
            blank = t;
        }
        if b == SOLVED {
            b.swap(15, 14);
        }
        Joc15 {
            b: [b, b],
            side: Player::One,
            ply: 0,
        }
    }

    pub fn distance(&self, p: Player) -> i32 {
        manhattan(&self.b[p.idx()])
    }

    fn solved(&self, p: usize) -> bool {
        self.b[p] == SOLVED
    }
}

impl Game for Joc15 {
    type Move = u8;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<u8> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let b = &self.b[self.side.idx()];
        let blank = b.iter().position(|&t| t == 0).unwrap();
        let mut v: Vec<u8> = neighbours(blank).into_iter().map(|n| b[n]).collect();
        v.sort_unstable();
        v
    }
    fn apply(&mut self, m: &u8) {
        let b = &mut self.b[self.side.idx()];
        let blank = b.iter().position(|&t| t == 0).unwrap();
        let t = b.iter().position(|&x| x == *m).unwrap();
        b.swap(blank, t);
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.solved(p) {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.ply >= PLY_CAP {
            let (a, b) = (self.distance(Player::One), self.distance(Player::Two));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        100 * (self.distance(p.other()) - self.distance(p))
    }
    fn render(&self) -> String {
        let mut s = String::from("      J1                 J2\n");
        for r in 0..4 {
            for p in 0..2 {
                for c in 0..4 {
                    let t = self.b[p][r * 4 + c];
                    if t == 0 {
                        s.push_str("  . ");
                    } else {
                        s.push_str(&format!("{t:>3} "));
                    }
                }
                s.push_str("      ");
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Distanța până la ordine: J1 {}, J2 {}. La mutare: Jucătorul {}.",
            self.distance(Player::One),
            self.distance(Player::Two),
            self.side.idx() + 1
        ));
        s
    }
    fn format_move(&self, m: &u8) -> String {
        m.to_string()
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn both_boards_are_identical_and_unsolved() {
        let g = Joc15::with_seed(5);
        assert_eq!(g.b[0], g.b[1]);
        assert_ne!(g.b[0], SOLVED);
        assert!(!GameObj::legal_moves(&g).is_empty());
    }

    #[test]
    fn solving_wins() {
        let mut g = Joc15::with_seed(1);
        g.b[0] = SOLVED;
        g.b[0].swap(15, 14);
        assert!(GameObj::play(&mut g, "15").is_ok());
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Joc15::with_seed(s)), 4, 400);
        let r = ai_vs_random(|s| Box::new(Joc15::with_seed(s)), 3, Player::One, 400);
        assert!(r.is_some());
        ai_vs_random(|s| Box::new(Joc15::with_seed(s)), 4, Player::Two, 400);
    }
}
