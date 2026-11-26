//! Dog (Elveția; curse cu cărți pe inel de 64 de câmpuri), adaptat pentru 2 jucători. Titlu propriu: nu e
//! produsul comercial cu același nume al unei firme, ci regulile publice ale jocului de familie.
//!
//! * Inel de 64 de câmpuri, 4 piese pe jucător, start-uri opuse. Pachet de 110 cărți (2×52 + 6 jokeri), mâini
//!   de 6, 5, 4, 3, 2 cărți, ciclic. Fiecare mutare = o carte: A = 1, 11 sau intrare; K = 13 sau intrare;
//!   Q = 12; J = schimbi o piesă a ta cu una a adversarului; 4 = 4 pași înainte sau înapoi; 7 = 7 pași de
//!   câte 1 între piesele tale (lovești ce întâlnești, nu treci prin piesele tale); joker = orice carte;
//!   2, 3, 5, 6, 8, 9, 10 = valoarea lor.
//! * Piesa de pe câmpul de start e protejată și blochează trecerea. Nimeni nu trece peste piese din casă.
//!   Aterizarea exactă pe adversar îl trimite în kennel. Dacă nu ai nicio carte jucabilă: `pas` (arunci mâna).
//! * Câștigă primul care are toate cele 4 piese în casă.
//!
//! Mutări: `K:intra`, `A:p1`, `A:p1/11`, `5:p2`, `4:p1-` (înapoi), `7:p1` apoi `pas1:p2`, `J:p1<>o2`, `jk=5:p3`, `pas`, `imparte`.

use crate::dogkit::{DMove, DogGame, Rules};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const RULES: Rules = Rules {
    ring: 64,
    hands: &[6, 5, 4, 3, 2],
    decks: 2,
    jokers: 6,
    four_both_ways: true,
    ace_eleven: true,
    joker_wild: true,
    jack_moves: false,
    name: "Dog (curse cu cărți)",
};

#[derive(Clone)]
pub struct Dog(pub DogGame);

impl Default for Dog {
    fn default() -> Self {
        Self::new()
    }
}

impl Dog {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Dog(DogGame::with_seed(RULES, seed))
    }
}

impl Game for Dog {
    type Move = DMove;
    fn set_viewer(&mut self, w: Option<Player>) {
        self.0.set_viewer(w)
    }
    fn to_move(&self) -> Player {
        self.0.to_move()
    }
    fn legal_moves(&self) -> Vec<DMove> {
        self.0.legal_moves()
    }
    fn apply(&mut self, m: &DMove) {
        self.0.apply(m)
    }
    fn outcome(&self) -> Option<Outcome> {
        self.0.outcome()
    }
    fn evaluate(&self, p: Player) -> i32 {
        self.0.evaluate(p)
    }
    fn render(&self) -> String {
        self.0.render()
    }
    fn format_move(&self, m: &DMove) -> String {
        self.0.format_move(m)
    }
    fn ai_style(&self) -> AiStyle {
        self.0.ai_style()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Dog::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                let mut texts: Vec<String> = m.iter().map(|x| g.format_move(x)).collect();
                texts.sort();
                texts.dedup();
                assert_eq!(texts.len(), m.len(), "texte duplicate:\n{}", g.render());
                g.apply(&m[rng.below(m.len())].clone());
                n += 1;
                assert!(n < 30000, "{}", g.render());
            }
        }
    }

    #[test]
    fn ai_ignores_hidden_hand() {
        let a = Dog::with_seed(5);
        let mut b = a.clone();
        b.0 = {
            let mut t = b.0.clone();
            t.reverse_opp_for_test();
            t
        };
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
