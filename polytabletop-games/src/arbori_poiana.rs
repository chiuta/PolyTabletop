//! Arborii din Poiană — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: fiecare își plantează propria poiană de 5×4 cu răsaduri luate pe rând din oferta comună (patru specii). Răsadurile se plantează lângă unul propriu dacă se poate. Se punctează grupurile de cel puțin trei copaci de aceeași specie și rândurile sau coloanele cu toate cele patru specii.
#![allow(clippy::manual_is_multiple_of)]
use crate::brand_grid::{GridGame, Place, Spec, Term::*};

pub static SPEC: Spec = Spec {
    id: "arbori_poiana",
    w: 5,
    h: 4,
    kinds: 4,
    offer: 3,
    place: Place::Adjacent,
    terms: &[Groups { min: 3, per: 2 }, Variety(4)],
};

pub fn new() -> GridGame {
    GridGame::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;
    use polytabletop_core::rng::Rng;
    use polytabletop_core::Game;

    #[test]
    fn plays() {
        for seed in 1..=4u64 {
            let mut g = GridGame::with_seed(&SPEC, seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n <= 2 * SPEC.w * SPEC.h);
            }
        }
    }

    #[test]
    fn ai_is_legal() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let g = GridGame::with_seed(&SPEC, 3);
        let m = polytabletop_core::GameObj::ai_move(&g, 7).unwrap();
        assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&m));
    }
}
