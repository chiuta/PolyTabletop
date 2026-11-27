//! Ger peste Avanpost — variantă originală (lista C1), motor `c1_duel`; clean-room: doar mecanica generală a genului.
use crate::c1_duel::*;

static SPEC: DSpec = DSpec {
    title: "Ger peste Avanpost",
    deck1: "a4,a3,a3,a3,a2,d3,d2,d2,h2,h2,s1,a2",
    deck2: "a3,a4,a3,a2,a2,d3,d3,d2,h3,h1,s1,a3",
    hand: 4,
    plays: 2,
    boss_hp: 34,
    atk: &[3, 4, 5],
    scheme: &[2],
    prog: 0,
    doom_max: 8,
    hp: 14,
    flags: "e",
    rounds: 16,
};

pub fn new() -> Duel {
    Duel::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Duel {
    Duel::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 900);
    }
}
