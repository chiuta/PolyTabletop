//! Toc (Franța), jocul de curse cu cărți și popice pe inel (variantă „Tock”), 2 jucători.
//!
//! * Inel de 80 de câmpuri (20 pe fiecare latură), 4 piese pe jucător, start-uri opuse. Pachet de 54 de cărți
//!   (52 + 2 jokeri), mâini de 5, 4, 4, 4, … cărți, ciclic. A = 1 sau intrare; K = 13 sau intrare; Q = 12;
//!   J = 11 sau schimb; 4 = 4 pași **numai înapoi**; 7 = 7 pași de câte 1; joker = 15 pași sau intrare;
//!   celelalte = valoarea lor.
//! * Piesa de pe câmpul de start e protejată și blochează trecerea; aterizarea exactă pe adversar îl trimite în
//!   kennel; casa (4 câmpuri) e protejată și se umple cu număr exact, fără a sări peste piese.
//! * Fără carte jucabilă: `pas`. Primul cu toate cele 4 piese în casă câștigă.
//!
//! Mutări: `K:intra`, `A:p1`, `5:p2`, `4:p1`, `7:p1` apoi `pas1:p2`, `J:p1` (11 pași), `J:p1<>o2` (schimb), `jk:p3`, `pas`, `imparte`.

use crate::dogkit::{DMove, DogGame, Rules};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const RULES: Rules = Rules {
    ring: 80,
    hands: &[5, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    decks: 1,
    jokers: 2,
    four_both_ways: false,
    ace_eleven: false,
    joker_wild: false,
    jack_moves: true,
    name: "Toc",
};

#[derive(Clone)]
pub struct Toc(pub DogGame);

impl Default for Toc {
    fn default() -> Self {
        Self::new()
    }
}

impl Toc {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Toc(DogGame::with_seed(RULES, seed))
    }
}

impl Game for Toc {
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
    fn random_games_end_with_unique_move_texts() {
        for seed in 1..=4u64 {
            let mut g = Toc::with_seed(seed);
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
    fn four_goes_backwards_and_start_square_blocks() {
        let mut g = DogGame::with_seed(RULES, 3);
        g.test_setup(&[6, 4], &[3], &[-1; 4], [-1, 5, 0, -1]);
        let m = g.legal_moves();
        let t: Vec<String> = m.iter().map(|x| g.format_move(x)).collect();
        assert!(t.iter().any(|x| x == "4:p2"), "{t:?}");
    }
}
