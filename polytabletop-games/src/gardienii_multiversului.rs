//! Gardienii Multiversului — variantă originală (lista C1), motor `c1_duel`; clean-room: doar mecanica generală a genului.
use crate::c1_duel::*;

static SPEC: DSpec = DSpec {
    title: "Gardienii Multiversului",
    deck1: "a4,a3,a3,a2,a2,a1,d3,d2,d2,h2,s1,a3,d1,a2",
    deck2: "a3,a3,a2,a2,a4,d2,d2,d3,h3,s1,a1,a2,h1,d1",
    hand: 5,
    plays: 2,
    boss_hp: 80,
    atk: &[3, 4, 4, 5],
    scheme: &[1, 1, 2],
    prog: 0,
    doom_max: 6,
    hp: 11,
    flags: "",
    rounds: 14,
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
