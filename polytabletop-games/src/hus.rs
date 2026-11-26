//! Hus (Namibia, khoikhoi): mancala 4 × 8, 2 semințe în fiecare groapă (64 în total).
//!
//! Reguli folosite (variantă documentată, cea mai răspândită):
//! * Fiecare jucător deține două rânduri (față și spate) unite într-un inel de 16 gropi;
//!   semănatul e mereu în același sens (vezi `mancala4`).
//! * Muți dintr-o groapă proprie cu cel puțin 2 semințe, câte una în fiecare groapă
//!   următoare. Dacă ultima sămânță cade într-o groapă deja ocupată: dacă groapa e în
//!   rândul din față și groapa din față a adversarului e ocupată, capturezi AMBELE gropi
//!   ale adversarului din acea coloană (față + spate) și le semeni mai departe, de la groapa
//!   următoare; altfel ridici groapa și continui semănatul (lanț). Dacă ultima cade într-o
//!   groapă goală, tura se termină.
//! * Pierde cine nu mai are nicio groapă cu cel puțin 2 semințe la mutare.
//!
//! Simplificări: lanțul e limitat la 3000 de semințe semănate; după 800 de mutări câștigă
//! cine are mai multe semințe pe partea lui.

use crate::mancala4::{ring_name, step, Kete, RING};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CAP: u32 = 3000;
const MAX_PLIES: u32 = 800;

#[derive(Clone)]
pub struct Hus {
    b: Kete,
    side: Player,
    plies: u32,
    winner: Option<Option<Player>>, // Some(None) = remiză
}

impl Default for Hus {
    fn default() -> Self {
        Self::new()
    }
}

impl Hus {
    pub fn new() -> Self {
        Hus {
            b: Kete {
                ring: [[2; RING]; 2],
            },
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    pub fn total_seeds(&self) -> u32 {
        self.b.total(Player::One) + self.b.total(Player::Two)
    }

    fn movable(&self, p: Player) -> Vec<usize> {
        (0..RING)
            .filter(|&r| self.b.ring[p.idx()][r] >= 2)
            .collect()
    }

    fn play(&mut self, me: Player, start: usize) {
        let (m, o) = (me.idx(), me.other().idx());
        let mut pos = start;
        let mut hand = self.b.ring[m][start];
        self.b.ring[m][start] = 0;
        let mut sown = 0u32;
        loop {
            while hand > 0 {
                pos = step(pos, 1);
                self.b.ring[m][pos] += 1;
                hand -= 1;
                sown += 1;
            }
            if self.b.ring[m][pos] <= 1 || sown > CAP {
                return;
            }
            if pos < 8 && self.b.ring[o][7 - pos] > 0 {
                let e = 7 - pos;
                hand = self.b.ring[o][e] + self.b.ring[o][15 - e];
                self.b.ring[o][e] = 0;
                self.b.ring[o][15 - e] = 0;
            } else {
                hand = self.b.ring[m][pos];
                self.b.ring[m][pos] = 0;
            }
        }
    }
}

impl Game for Hus {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.winner.is_some() {
            return Vec::new();
        }
        self.movable(self.side)
            .into_iter()
            .map(|r| r as u8)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        self.play(me, *m as usize);
        self.plies += 1;
        self.side = me.other();
        if self.movable(self.side).is_empty() {
            self.winner = Some(Some(me));
        } else if self.plies >= MAX_PLIES {
            let (a, b) = (self.b.total(Player::One), self.b.total(Player::Two));
            self.winner = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Some(Player::One),
                std::cmp::Ordering::Less => Some(Player::Two),
                _ => None,
            });
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(|w| match w {
            Some(p) => Outcome::Win(p),
            None => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let inner =
            |q: Player| -> i32 { self.b.ring[q.idx()][..8].iter().map(|&x| x as i32).sum() };
        3 * (self.b.total(p) as i32 - self.b.total(o) as i32)
            + (inner(p) - inner(o))
            + (self.movable(p).len() as i32 - self.movable(o).len() as i32)
    }

    fn render(&self) -> String {
        let mut s = self.b.render();
        s.push_str(
            "  mutare: 1–8 = rândul din față, b1–b8 = rândul din spate (coloane ale tale)\n",
        );
        s
    }

    fn format_move(&self, m: &u8) -> String {
        ring_name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mancala4::parse_ring;
    use polytabletop_core::rng::Rng;

    fn empty() -> Hus {
        Hus {
            b: Kete::empty(),
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    #[test]
    fn start_has_16_moves_and_64_seeds() {
        let g = Hus::new();
        assert_eq!(Game::legal_moves(&g).len(), 16);
        assert_eq!(g.total_seeds(), 64);
    }

    #[test]
    fn landing_in_an_empty_pit_ends_the_turn() {
        let mut g = empty();
        g.b.ring[0][0] = 2;
        g.b.ring[1][3] = 2;
        g.apply(&0);
        assert_eq!(g.b.ring[0][1], 1);
        assert_eq!(g.b.ring[0][2], 1);
        assert_eq!(g.b.ring[1][3], 2);
    }

    #[test]
    fn capture_takes_front_and_back_pits() {
        let mut g = empty();
        g.b.ring[0][0] = 2; // ultima în 2, care e ocupată
        g.b.ring[0][2] = 1;
        // coloana adversarului față în față cu 2 este 5; față 5 și spate (15-5=10)
        g.b.ring[1][5] = 3;
        g.b.ring[1][10] = 4;
        g.b.ring[1][0] = 1; // ca să nu se termine jocul
        g.apply(&0);
        // 7 semințe capturate, semănate de la groapa 3
        assert_eq!(g.b.ring[1][5], 0);
        assert_eq!(g.b.ring[1][10], 0);
        assert_eq!(g.b.total(Player::One), 2 + 1 + 7);
        assert_eq!(g.b.ring[0][3], 1);
    }

    #[test]
    fn chain_relay_on_occupied_pit_without_capture() {
        let mut g = empty();
        g.b.ring[0][0] = 2;
        g.b.ring[0][2] = 2; // devine 3, ridicată și semănată în 3,4,5
        g.b.ring[1][0] = 1;
        g.apply(&0);
        assert_eq!(g.b.ring[0][2], 0);
        assert_eq!(g.b.ring[0][5], 1);
    }

    #[test]
    fn player_without_movable_pit_loses() {
        let mut g = empty();
        g.b.ring[0][0] = 2;
        g.b.ring[1][0] = 1; // adversarul nu are groapă cu 2
        g.apply(&0);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn names_roundtrip() {
        for r in 0..RING {
            assert_eq!(parse_ring(&ring_name(r)), Some(r));
        }
    }

    #[test]
    fn seeds_conserved_and_random_games_finish() {
        for seed in 1..=30u64 {
            let mut g = Hus::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_seeds(), 64);
            }
        }
    }
}
