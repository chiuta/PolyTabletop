//! Lamele din Pivniță — variantă originală (lista C1), motor `c1_duel`; clean-room: doar mecanica generală a genului.
use crate::c1_duel::*;

static SPEC: DSpec = DSpec {
    title: "Lamele din Pivniță",
    deck1: "a4,a3,a3,a2,d3,d2,h2,a2,s1,a1",
    deck2: "a3,a3,a2,a2,d3,d2,h2,a4,s1,d1",
    hand: 4,
    plays: 2,
    boss_hp: 28,
    atk: &[2, 3, 4, 5],
    scheme: &[],
    prog: 0,
    doom_max: 99,
    hp: 10,
    flags: "e",
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
