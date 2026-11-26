//! Sungka (Filipine): 2 × 7 gropi și câte un depozit („casă”) pe jucător,
//! 7 semințe în fiecare groapă (98 în total).
//!
//! Semănat în sens trigonometric (invers acelor de ceasornic): pe partea
//! proprie de la stânga la dreapta, depozitul propriu, apoi partea adversă,
//! sărind depozitul adversarului.
//! * Ultima sămânță în depozitul propriu: încă o mutare.
//! * Ultima sămânță într-o groapă ocupată (de oricare parte): se ridică toate
//!   semințele ei și semănatul continuă.
//! * Ultima sămânță într-o groapă goală proprie: se capturează groapa opusă
//!   (dacă are semințe) împreună cu sămânța ultimă, în depozit.
//! * Ultima sămânță într-o groapă goală adversă: tura se termină.
//!
//! Simplificări (documentate): o singură rundă (fără gropi „arse” / burnt
//! houses și fără runda a doua); jucătorii mută pe rând (Jucătorul 1 începe),
//! nu simultan; jocul se termină când jucătorul la mutare nu mai are semințe
//! (adversarul își adaugă semințele rămase pe partea lui) sau când cineva are
//! peste 49 de semințe în depozit. Un semănat în lanț e limitat la 2000 de
//! semințe puse, ca să nu poată cicla la infinit.

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Sungka): regulile de semănat, ultima
//! sămânță în depozit / casă ocupată / casă goală proprie (captura casei opuse) / casă goală
//! adversă corespund. Diferențe documentate: Wikipedia are start simultan, runda a doua cu
//! reumplere și „case arse” (implementate doar în `congkak.rs`).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const STORE: [usize; 2] = [7, 15];
const CAP: u32 = 2000;

#[derive(Clone)]
pub struct Sungka {
    pits: [u8; 16], // 0..=6 One, 7 One's store, 8..=14 Two, 15 Two's store
    side: Player,
    over: bool,
}

impl Default for Sungka {
    fn default() -> Self {
        Self::new()
    }
}

impl Sungka {
    pub fn new() -> Self {
        let mut pits = [7u8; 16];
        pits[7] = 0;
        pits[15] = 0;
        Sungka {
            pits,
            side: Player::One,
            over: false,
        }
    }

    fn range(p: Player) -> std::ops::Range<usize> {
        match p {
            Player::One => 0..7,
            Player::Two => 8..15,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.pits[7], self.pits[15])
    }

    fn side_empty(&self, p: Player) -> bool {
        self.pits[Self::range(p)].iter().all(|&x| x == 0)
    }

    fn finish(&mut self) {
        for p in [Player::One, Player::Two] {
            let rest: u8 = self.pits[Self::range(p)].iter().sum();
            for i in Self::range(p) {
                self.pits[i] = 0;
            }
            self.pits[STORE[p.idx()]] += rest;
        }
        self.over = true;
    }
}

impl Game for Sungka {
    type Move = u8; // groapa 1..=7 din punctul de vedere al jucătorului la mutare

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            return Vec::new();
        }
        let start = Self::range(self.side).start;
        (0..7u8)
            .filter(|&k| self.pits[start + k as usize] > 0)
            .map(|k| k + 1)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        let own = STORE[me.idx()];
        let opp_store = STORE[me.other().idx()];
        let start = Self::range(me).start + (*m as usize - 1);
        let mut seeds = self.pits[start];
        self.pits[start] = 0;
        let mut pos = start;
        let mut sown = 0u32;
        let mut extra = false;
        loop {
            while seeds > 0 {
                pos = (pos + 1) % 16;
                if pos == opp_store {
                    continue;
                }
                self.pits[pos] += 1;
                seeds -= 1;
                sown += 1;
            }
            if pos == own {
                extra = true;
                break;
            }
            if self.pits[pos] > 1 {
                if sown > CAP {
                    break;
                }
                seeds = self.pits[pos];
                self.pits[pos] = 0;
                continue;
            }
            if Self::range(me).contains(&pos) {
                let opp = 14 - pos;
                if self.pits[opp] > 0 {
                    self.pits[own] += self.pits[opp] + 1;
                    self.pits[opp] = 0;
                    self.pits[pos] = 0;
                }
            }
            break;
        }
        let next = if extra { me } else { me.other() };
        if self.pits[7] > 49 || self.pits[15] > 49 || self.side_empty(next) {
            self.finish();
        }
        if !extra {
            self.side = me.other();
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
        let mine = self.pits[STORE[p.idx()]] as i32;
        let theirs = self.pits[STORE[p.other().idx()]] as i32;
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
        s.push_str("  Jucător 2 (O):   7   6   5   4   3   2   1\n");
        s.push_str(&format!(
            "       [{:>2}]   {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}\n",
            p[15], p[14], p[13], p[12], p[11], p[10], p[9], p[8]
        ));
        s.push_str(&format!(
            "              {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}   [{:>2}]\n",
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]
        ));
        s.push_str("  Jucător 1 (X):   1   2   3   4   5   6   7\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        m.to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn custom(pits: [u8; 16], side: Player) -> Sungka {
        Sungka {
            pits,
            side,
            over: false,
        }
    }

    #[test]
    fn first_pit_ends_in_store_for_another_turn() {
        let mut g = Sungka::new();
        g.apply(&1); // 7 semințe: groapele 2..7 și depozitul
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.stores().0, 1);
        assert_eq!(g.pits[0], 0);
        assert_eq!(g.pits[1], 8);
    }

    #[test]
    fn sowing_skips_opponent_store() {
        let mut p = [0u8; 16];
        p[6] = 10; // groapa 7: depozit, 7 adverse, (sare 15), apoi 0, 1
        p[3] = 1;
        p[9] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&7);
        assert_eq!(g.pits[15], 0);
        // ultima sămânță cade în groapa goală 1 și captură opusa (13) cu 1 sămânță
        assert_eq!(g.pits[7], 3);
        let total: u32 = g.pits.iter().map(|&x| x as u32).sum();
        assert_eq!(total, 12);
    }

    #[test]
    fn capture_opposite_pit() {
        let mut p = [0u8; 16];
        p[0] = 1; // -> groapa 1 (index 1), goală
        p[13] = 5; // opusă lui index 1 este 13
        p[4] = 1;
        p[10] = 2;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.pits[7], 6);
        assert_eq!(g.pits[13], 0);
        assert_eq!(g.pits[1], 0);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn no_capture_when_opposite_empty() {
        let mut p = [0u8; 16];
        p[0] = 1;
        p[4] = 1;
        p[10] = 2;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.pits[1], 1);
        assert_eq!(g.pits[7], 0);
    }

    #[test]
    fn relay_continues_from_occupied_pit() {
        let mut p = [0u8; 16];
        p[0] = 2; // sare în 1 și 2; 2 are 3 -> ridică 4 (1+3)... vezi jos
        p[2] = 2;
        p[10] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        // Index 2 devine 3, se ridică; sămânță în 3, 4, 5 (goale) -> se termină în 5 cu captură opus (9: gol).
        assert_eq!(g.pits[2], 0);
        assert_eq!(g.pits[3], 1);
        assert_eq!(g.pits[4], 1);
        assert_eq!(g.pits[5], 1);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn game_ends_when_mover_has_no_seeds() {
        let mut p = [0u8; 16];
        p[6] = 1; // ultima sămânță intră în depozit; apoi Jucătorul 1 nu mai are semințe
        p[8] = 3;
        p[15] = 10;
        let mut g = custom(p, Player::One);
        g.apply(&7);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert_eq!(g.stores(), (1, 13));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn seeds_are_conserved_in_random_games() {
        for seed in 1..=8u64 {
            let mut g = Sungka::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                let total: u32 = g.pits.iter().map(|&x| x as u32).sum();
                assert_eq!(total, 98);
                n += 1;
                assert!(n < 5000);
            }
            let (a, b) = g.stores();
            assert_eq!(a as u32 + b as u32, 98);
        }
    }

    #[test]
    fn ai_is_fast() {
        let mut g = Sungka::new();
        let mut rng = Rng::new(9);
        while g.outcome().is_none() {
            let t = std::time::Instant::now();
            let m = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
            if !cfg!(debug_assertions) {
                assert!(
                    t.elapsed().as_secs_f64() < 1.5,
                    "slow AI: {:?}",
                    t.elapsed()
                );
            }
            polytabletop_core::GameObj::play(&mut g, &m).unwrap();
        }
    }
}
