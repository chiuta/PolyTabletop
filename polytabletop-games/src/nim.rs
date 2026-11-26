//! Nim cu grămezi: patru grămezi (3, 4, 5, 6). La rând, iei oricâți bețe vrei dintr-o
//! singură grămadă (cel puțin unul). Cine ia ultimul băț câștigă (joc normal).
//! Mutări: litera grămezii + număr, de exemplu «b3» = trei bețe din grămada b.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const HEAPS: usize = 4;
const START: [u8; HEAPS] = [3, 4, 5, 6];

/// Mutare: (grămadă, câte bețe se iau).
pub type NimMove = (u8, u8);

#[derive(Clone)]
pub struct Nim {
    heaps: [u8; HEAPS],
    side: Player,
}

impl Default for Nim {
    fn default() -> Self {
        Self::new()
    }
}

impl Nim {
    pub fn new() -> Self {
        Nim {
            heaps: START,
            side: Player::One,
        }
    }

    fn nim_sum(&self) -> u8 {
        self.heaps.iter().fold(0, |a, &h| a ^ h)
    }
}

impl Game for Nim {
    type Move = NimMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<NimMove> {
        let mut out = Vec::new();
        for (i, &h) in self.heaps.iter().enumerate() {
            for k in 1..=h {
                out.push((i as u8, k));
            }
        }
        out
    }

    fn apply(&mut self, m: &NimMove) {
        self.heaps[m.0 as usize] -= m.1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.heaps.iter().all(|&h| h == 0) {
            // The side to move has no stick to take: the previous mover took the last one.
            Some(Outcome::Win(self.side.other()))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Exact theory: the side to move wins iff the nim-sum is non-zero.
        let mover_wins = self.nim_sum() != 0;
        if mover_wins == (self.side == p) {
            1000
        } else {
            -1000
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for (i, &h) in self.heaps.iter().enumerate() {
            s.push_str(&format!(
                "{}: {} ({})\n",
                (b'a' + i as u8) as char,
                "| ".repeat(h as usize).trim_end(),
                h
            ));
        }
        s.push_str("Ia oricâte bețe dintr-o grămadă; cine ia ultimul băț câștigă. Ex.: b3\n");
        s
    }

    fn format_move(&self, m: &NimMove) -> String {
        format!("{}{}", (b'a' + m.0) as char, m.1)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 200,
            max_depth: 4,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.heaps.iter().enumerate() {
            h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
        }
        Some(h)
    }

    fn move_key(&self, m: &NimMove) -> Option<u32> {
        Some(m.0 as u32 * 16 + m.1 as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn opening_has_18_moves() {
        let g = Nim::new();
        assert_eq!(g.legal_moves().len(), 3 + 4 + 5 + 6);
        assert_eq!(g.format_move(&(1, 3)), "b3");
    }

    #[test]
    fn last_stick_wins() {
        let mut g = Nim::new();
        g.heaps = [0, 0, 0, 1];
        assert_eq!(g.outcome(), None);
        g.apply(&(3, 1));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn random_playouts_end() {
        for seed in 1..=20u64 {
            let mut g = Nim::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n <= 18);
            }
        }
    }

    #[test]
    fn ai_plays_perfectly() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(20));
        // Starting position has nim-sum 4 != 0, so One wins with perfect play.
        let mut g = Nim::new();
        let mut rng = Rng::new(7);
        while g.outcome().is_none() {
            let m = if g.side == Player::One {
                polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
            } else {
                let ms = g.legal_moves();
                ms[rng.below(ms.len())]
            };
            g.apply(&m);
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
}
