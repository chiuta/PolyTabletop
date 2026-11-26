//! Narde scurtă (Short Nardy), jocul persan/rusesc din familia „tavla”, 2 jucători.
//!
//! Varianta scurtă standard: aceeași tablă de 24 de puncte, 15 piese pe jucător, poziția de start ca în table,
//! piesele singure se lovesc și intră de pe bară, zarurile se joacă pe rând (dubla de patru ori), apoi scoaterea
//! pieselor; câștigă primul care scoate toate cele 15 piese. Fără cub de dublare.
//!
//! Implementare: peste motorul `backgammon` (același regulament, aceeași interfață de mutări `13/7`, `bar/22`,
//! `3/off`); este un înveliș subțire cu titlu și documentație proprii. Variantele regionale ale numărătorii
//! („mars”, „oyn”) nu sunt modelate: se joacă o singură partidă.

use crate::backgammon::{Backgammon, Mv};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone)]
pub struct Narde(pub Backgammon);

impl Default for Narde {
    fn default() -> Self {
        Self::new()
    }
}

impl Narde {
    pub fn new() -> Self {
        Narde(Backgammon::with_seed(Rng::from_clock().next_u64()))
    }
    pub fn with_seed(seed: u64) -> Self {
        Narde(Backgammon::with_seed(seed))
    }
}

impl Game for Narde {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.0.to_move()
    }
    fn legal_moves(&self) -> Vec<Mv> {
        self.0.legal_moves()
    }
    fn apply(&mut self, m: &Mv) {
        self.0.apply(m)
    }
    fn outcome(&self) -> Option<Outcome> {
        self.0.outcome()
    }
    fn evaluate(&self, p: Player) -> i32 {
        self.0.evaluate(p)
    }
    fn render(&self) -> String {
        format!("Narde scurtă (Short Nardy)\n{}", self.0.render())
    }
    fn format_move(&self, m: &Mv) -> String {
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
    fn plays_like_the_underlying_engine() {
        let mut g = Narde::with_seed(4);
        let mut rng = Rng::new(4);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            assert!(!m.is_empty());
            g.apply(&m[rng.below(m.len())].clone());
            n += 1;
            assert!(n < 6000);
        }
        assert!(g.render().starts_with("Narde scurtă"));
    }

    #[test]
    fn ai_is_deterministic_and_legal() {
        let g = Narde::with_seed(9);
        let a = polytabletop_core::GameObj::ai_move(&g, 3).unwrap();
        assert_eq!(Some(a.clone()), polytabletop_core::GameObj::ai_move(&g, 3));
        assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&a));
    }
}
