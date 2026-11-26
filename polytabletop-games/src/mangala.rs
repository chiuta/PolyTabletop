//! Mangala (mancala turcească): 2 × 6 gropi, 4 semințe în fiecare, câte un depozit.
//!
//! Reguli folosite (descrierile publice ale jocului turcesc):
//! * Alegi o groapă proprie nevidă. Dacă are mai mult de o sămânță, prima sămânță se
//!   lasă în ACEEAȘI groapă, apoi se seamănă mai departe; dacă are o singură sămânță,
//!   aceasta trece în groapa următoare.
//! * Sensul e trigonometric; semănatul trece prin depozitul propriu, dar sare peste cel
//!   al adversarului.
//! * Ultima sămânță în depozitul propriu: mai muți o dată.
//! * Ultima sămânță într-o groapă a adversarului care ajunge astfel la număr PAR:
//!   iei toată groapa.
//! * Ultima sămânță într-o groapă proprie goală: iei sămânța și groapa opusă (dacă
//!   are semințe).
//! * Când un jucător nu mai are semințe în gropi, celălalt își pune restul în depozit;
//!   câștigă depozitul mai mare (24–24 = remiză).
//!
//! Simplificare: după 600 de mutări fiecare își ia semințele de pe partea proprie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const MAX_PLIES: u32 = 600;

#[derive(Clone)]
pub struct Mangala {
    pits: [u8; 12], // 0..=5 Unu (stânga→dreapta), 6..=11 Doi (dreapta→stânga)
    store: [u8; 2],
    side: Player,
    plies: u32,
    over: bool,
}

impl Default for Mangala {
    fn default() -> Self {
        Self::new()
    }
}

/// Poziție în inelul de 13 locuri al jucătorului `p`: 0..=5 gropile proprii, 6 depozitul,
/// 7..=12 gropile adversarului. `Ok(i)` = groapă, `Err(())` = depozit.
fn ring(p: Player, r: usize) -> Result<usize, ()> {
    let (own, opp) = match p {
        Player::One => (0, 6),
        Player::Two => (6, 0),
    };
    match r {
        0..=5 => Ok(own + r),
        6 => Err(()),
        _ => Ok(opp + r - 7),
    }
}

impl Mangala {
    pub fn new() -> Self {
        Mangala {
            pits: [4; 12],
            store: [0, 0],
            side: Player::One,
            plies: 0,
            over: false,
        }
    }

    fn range(p: Player) -> std::ops::Range<usize> {
        match p {
            Player::One => 0..6,
            Player::Two => 6..12,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.store[0], self.store[1])
    }

    pub fn total_seeds(&self) -> u32 {
        self.pits.iter().map(|&x| x as u32).sum::<u32>()
            + self.store.iter().map(|&x| x as u32).sum::<u32>()
    }

    fn side_empty(&self, p: Player) -> bool {
        self.pits[Self::range(p)].iter().all(|&x| x == 0)
    }

    /// Semănat; întoarce `true` dacă mai muți o dată.
    fn sow(&mut self, m: u8) -> bool {
        let me = self.side;
        let start = (m - 1) as usize;
        let idx = Self::range(me).start + start;
        let n = self.pits[idx];
        self.pits[idx] = 0;
        let mut r = start;
        let mut left = n;
        if n > 1 {
            self.pits[idx] += 1; // prima sămânță rămâne în groapa de start
            left -= 1;
        } else {
            r += 1; // singura sămânță trece mai departe
            match ring(me, r) {
                Ok(i) => self.pits[i] += 1,
                Err(()) => self.store[me.idx()] += 1,
            }
            left = 0;
        }
        while left > 0 {
            r = (r + 1) % 13;
            match ring(me, r) {
                Ok(i) => self.pits[i] += 1,
                Err(()) => self.store[me.idx()] += 1,
            }
            left -= 1;
        }
        match ring(me, r) {
            Err(()) => true,
            Ok(i) => {
                if Self::range(me.other()).contains(&i) {
                    if self.pits[i].is_multiple_of(2) {
                        self.store[me.idx()] += self.pits[i];
                        self.pits[i] = 0;
                    }
                } else if self.pits[i] == 1 {
                    let opp = 11 - i;
                    if self.pits[opp] > 0 {
                        self.store[me.idx()] += self.pits[opp] + 1;
                        self.pits[opp] = 0;
                        self.pits[i] = 0;
                    }
                }
                false
            }
        }
    }

    fn finish(&mut self) {
        for p in [Player::One, Player::Two] {
            let rest: u8 = self.pits[Self::range(p)].iter().sum();
            for i in Self::range(p) {
                self.pits[i] = 0;
            }
            self.store[p.idx()] += rest;
        }
        self.over = true;
    }
}

impl Game for Mangala {
    type Move = u8;

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
        let again = self.sow(*m);
        self.plies += 1;
        if !again {
            self.side = self.side.other();
        }
        if self.plies >= MAX_PLIES || self.side_empty(Player::One) || self.side_empty(Player::Two) {
            self.finish();
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
        let sd = |q: Player| -> i32 { self.pits[Self::range(q)].iter().map(|&x| x as i32).sum() };
        let bonus = if self.side == p { 2 } else { -2 };
        12 * (self.store[p.idx()] as i32 - self.store[p.other().idx()] as i32)
            + (sd(p) - sd(p.other()))
            + bonus
    }

    fn render(&self) -> String {
        let p = &self.pits;
        let mut s = String::new();
        s.push_str("  Jucător 2 (O):   6   5   4   3   2   1\n");
        s.push_str(&format!(
            "       [{:>2}]  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}\n",
            self.store[1], p[11], p[10], p[9], p[8], p[7], p[6]
        ));
        s.push_str(&format!(
            "              {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}   [{:>2}]\n",
            p[0], p[1], p[2], p[3], p[4], p[5], self.store[0]
        ));
        s.push_str("  Jucător 1 (X):   1   2   3   4   5   6\n");
        s.push_str("  [..] = depozitul jucătorului\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        m.to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn custom(pits: [u8; 12], side: Player) -> Mangala {
        Mangala {
            pits,
            store: [0, 0],
            side,
            plies: 0,
            over: false,
        }
    }

    #[test]
    fn first_seed_stays_in_the_start_pit() {
        let mut g = Mangala::new();
        g.apply(&1); // 4 semințe: 1 rămâne, 3 în gropile 2,3,4
        assert_eq!(g.pits[0], 1);
        assert_eq!(g.pits[1], 5);
        assert_eq!(g.pits[3], 5);
        assert_eq!(g.pits[4], 4);
    }

    #[test]
    fn single_seed_moves_on() {
        let mut p = [0u8; 12];
        p[0] = 1;
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.pits[0], 0);
        assert_eq!(g.pits[1], 1);
    }

    #[test]
    fn last_seed_in_store_gives_another_turn() {
        let mut p = [0u8; 12];
        p[3] = 3; // 1 rămâne în 3, apoi 4, 5 … ultima în depozit? 3 semințe: pit3,pit4,pit5
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&4);
        assert_eq!(g.to_move(), Player::Two); // ultima în groapa 6 (goală), nu în depozit
        let mut p = [0u8; 12];
        p[3] = 4; // pit3, pit4, pit5, depozit
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&4);
        assert_eq!(g.stores().0, 1);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn even_opponent_pit_is_captured() {
        let mut p = [0u8; 12];
        p[5] = 3; // 5 rămâne, depozit, groapa 6 (idx 6)
        p[6] = 1; // devine 2 → par → capturat
        p[11] = 2;
        let mut g = custom(p, Player::One);
        g.apply(&6);
        // 3 semințe: idx5 (1), depozit (+1), idx6 (+1 → 2)
        assert_eq!(g.stores().0, 1 + 2);
        assert_eq!(g.pits[6], 0);
    }

    #[test]
    fn empty_own_pit_captures_opposite() {
        let mut p = [0u8; 12];
        p[0] = 2; // 1 rămâne în 0, 1 merge în 1 (goală)
        p[10] = 5; // opusa lui 1 este 11-1 = 10
        p[8] = 1; // adversarul păstrează o sămânță, jocul continuă
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.stores().0, 6);
        assert_eq!(g.pits[10], 0);
        assert_eq!(g.pits[1], 0);
    }

    #[test]
    fn seeds_conserved_and_random_games_finish() {
        for seed in 1..=40u64 {
            let mut g = Mangala::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_seeds(), 48);
            }
        }
    }
}
