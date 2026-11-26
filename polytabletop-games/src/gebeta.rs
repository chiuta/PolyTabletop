//! Gebeta (Etiopia), mancala cu semănat în lanț, 2 jucători, din regulile publice (variantă răspândită).
//!
//! * 2 × 6 gropi, câte 4 semințe, câte un depozit (magazie) per jucător.
//! * Iei toate semințele dintr-o groapă proprie și le semeni câte una, în sens trigonometric
//!   (gropile tale 1–6, apoi ale adversarului 1–6).
//! * Dacă ultima sămânță cade într-o groapă **ocupată**, ridici tot ce e acolo și continui semănatul
//!   („lanț”). Dacă cade într-o groapă **goală**: dacă e a ta și cea opusă are semințe, iei opusa și
//!   sămânța ta; altfel mutarea se oprește.
//! * Când o parte rămâne goală, jocul se încheie și fiecare își pune semințele rămase în depozit.
//!   Câștigă depozitul mai mare. (Siguranță: lanțul e oprit după 300 de pași.)
//!
//! Mutări: `1`…`6`.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone)]
pub struct Gebeta {
    h: [u8; 12],
    store: [u8; 2],
    side: Player,
    over: bool,
}

impl Default for Gebeta {
    fn default() -> Self {
        Self::new()
    }
}

impl Gebeta {
    pub fn new() -> Self {
        Gebeta {
            h: [4; 12],
            store: [0; 2],
            side: Player::One,
            over: false,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.store[0], self.store[1])
    }

    fn side_sum(&self, p: usize) -> u8 {
        self.h[p * 6..p * 6 + 6].iter().sum()
    }
}

impl Game for Gebeta {
    type Move = usize;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<usize> {
        if self.over {
            return Vec::new();
        }
        let s = self.side.idx() * 6;
        (0..6)
            .filter(|&k| self.h[s + k] > 0)
            .map(|k| k + 1)
            .collect()
    }

    fn apply(&mut self, m: &usize) {
        let me = self.side.idx();
        let mut pos = me * 6 + (m - 1);
        let mut hand = self.h[pos];
        self.h[pos] = 0;
        let mut steps = 0;
        loop {
            while hand > 0 {
                pos = (pos + 1) % 12;
                self.h[pos] += 1;
                hand -= 1;
            }
            steps += 1;
            if self.h[pos] > 1 && steps < 300 {
                hand = self.h[pos];
                self.h[pos] = 0;
                continue;
            }
            if self.h[pos] == 1 && pos / 6 == me && self.h[11 - pos] > 0 {
                self.store[me] += self.h[11 - pos] + 1;
                self.h[11 - pos] = 0;
                self.h[pos] = 0;
            }
            break;
        }
        self.side = self.side.other();
        let s = self.side.idx();
        if self.side_sum(s) == 0 || self.side_sum(1 - s) == 0 {
            for p in 0..2 {
                self.store[p] += self.side_sum(p);
            }
            self.h = [0; 12];
            self.over = true;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.store[0].cmp(&self.store[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), 1 - p.idx());
        (self.store[a] as i32 - self.store[b] as i32) * 10
            + (self.side_sum(a) as i32 - self.side_sum(b) as i32)
    }

    fn render(&self) -> String {
        let mut s = String::from("Gebeta\n   ");
        for k in (6..12).rev() {
            s.push_str(&format!("{:>3} ", self.h[k]));
        }
        s.push_str(&format!("  J2 depozit {}\n   ", self.store[1]));
        for k in 0..6 {
            s.push_str(&format!("{:>3} ", self.h[k]));
        }
        s.push_str(&format!("  J1 depozit {}\n   ", self.store[0]));
        s.push_str("  1   2   3   4   5   6  (gropile lui J1)\n");
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: J{} (gropile lui J2 se numără 1–6 de la dreapta la stânga în rândul de sus)\n",
                self.side.idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &usize) -> String {
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

    #[test]
    fn seeds_are_conserved_and_chain_continues() {
        let mut g = Gebeta::new();
        g.apply(&1); // 4 semințe: gropile 2..5, ultima în groapa 5 ocupată -> lanț
        let tot: u32 = g.h.iter().map(|&x| x as u32).sum::<u32>()
            + g.store.iter().map(|&x| x as u32).sum::<u32>();
        assert_eq!(tot, 48);
        assert_ne!(g.h, Gebeta::new().h);
    }

    #[test]
    fn capture_in_own_empty_pit() {
        let mut g = Gebeta::new();
        g.h = [0; 12];
        g.h[0] = 1;
        g.h[1] = 0;
        g.h[10] = 5; // opusa lui 1 e 10
        g.h[7] = 2;
        g.apply(&1);
        assert_eq!(g.store[0], 6);
        assert_eq!(g.h[10], 0);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Gebeta::new());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly_and_is_legal() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Gebeta::new());
        let mut rng = Rng::new(3);
        while g.outcome().is_none() {
            let m = if g.to_move() == Player::One {
                g.ai_move(rng.next_u64()).unwrap()
            } else {
                let l = g.legal_moves();
                l[rng.below(l.len())].clone()
            };
            g.play(&m).unwrap();
        }
        assert_ne!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }
}
