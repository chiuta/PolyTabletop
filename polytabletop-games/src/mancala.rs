//! Mancala (Kalah variant): 2 rows of 6 pits, a store each, 4 seeds per pit.
//! Sowing is counter-clockwise; ending in your store gives another turn;
//! ending in an empty pit of yours captures the opposite pit.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone)]
pub struct Mancala {
    pits: [u8; 14], // 0..=5 One, 6 One's store, 7..=12 Two, 13 Two's store
    side: Player,
    over: bool,
}

impl Default for Mancala {
    fn default() -> Self {
        Self::new()
    }
}

impl Mancala {
    pub fn new() -> Self {
        let mut pits = [4u8; 14];
        pits[6] = 0;
        pits[13] = 0;
        Mancala {
            pits,
            side: Player::One,
            over: false,
        }
    }

    fn range(p: Player) -> std::ops::Range<usize> {
        match p {
            Player::One => 0..6,
            Player::Two => 7..13,
        }
    }

    fn store(p: Player) -> usize {
        match p {
            Player::One => 6,
            Player::Two => 13,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.pits[6], self.pits[13])
    }
}

impl Game for Mancala {
    type Move = u8; // pit number 1..=6 from the mover's point of view

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            return Vec::new();
        }
        let start = Self::range(self.side).start;
        (0..6u8)
            .filter(|&k| self.pits[start + k as usize] > 0)
            .map(|k| k + 1)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        let start = Self::range(me).start + (*m as usize - 1);
        let own_store = Self::store(me);
        let opp_store = Self::store(me.other());
        let mut seeds = self.pits[start];
        self.pits[start] = 0;
        let mut pos = start;
        while seeds > 0 {
            pos = (pos + 1) % 14;
            if pos == opp_store {
                continue;
            }
            self.pits[pos] += 1;
            seeds -= 1;
        }
        let mut extra = false;
        if pos == own_store {
            extra = true;
        } else if Self::range(me).contains(&pos) && self.pits[pos] == 1 {
            let opp = 12 - pos;
            if self.pits[opp] > 0 {
                self.pits[own_store] += self.pits[opp] + 1;
                self.pits[opp] = 0;
                self.pits[pos] = 0;
            }
        }
        let one_empty = self.pits[0..6].iter().all(|&x| x == 0);
        let two_empty = self.pits[7..13].iter().all(|&x| x == 0);
        if one_empty || two_empty {
            for i in 0..6 {
                self.pits[6] += self.pits[i];
                self.pits[i] = 0;
            }
            for i in 7..13 {
                self.pits[13] += self.pits[i];
                self.pits[i] = 0;
            }
            self.over = true;
        }
        if !extra {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = self.stores();
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mine = self.pits[Self::store(p)] as i32;
        let theirs = self.pits[Self::store(p.other())] as i32;
        let side_mine: i32 = self.pits[Self::range(p)].iter().map(|&x| x as i32).sum();
        let side_theirs: i32 = self.pits[Self::range(p.other())]
            .iter()
            .map(|&x| x as i32)
            .sum();
        10 * (mine - theirs) + (side_mine - side_theirs)
    }

    fn render(&self) -> String {
        let p = &self.pits;
        let mut s = String::new();
        s.push_str("  Jucător 2 (O):  6   5   4   3   2   1\n");
        s.push_str(&format!(
            "       [{:>2}]   {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}\n",
            p[13], p[12], p[11], p[10], p[9], p[8], p[7]
        ));
        s.push_str(&format!(
            "              {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}   [{:>2}]\n",
            p[0], p[1], p[2], p[3], p[4], p[5], p[6]
        ));
        s.push_str("  Jucător 1 (X):   1   2   3   4   5   6\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        m.to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_turn_when_ending_in_store() {
        let mut g = Mancala::new();
        g.apply(&3); // pit 3 has 4 seeds -> ends in the store
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.stores().0, 1);
    }

    #[test]
    fn seeds_are_conserved() {
        let mut g = Mancala::new();
        let mut rng = polytabletop_core::rng::Rng::new(7);
        while g.outcome().is_none() {
            let m = g.legal_moves();
            let pick = m[rng.below(m.len())];
            g.apply(&pick);
            let total: u32 = g.pits.iter().map(|&x| x as u32).sum();
            assert_eq!(total, 48);
        }
    }
}
