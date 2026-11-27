//! Vrăjitorii din Ruine — variantă originală (lista C1), motor `c1_duel`; clean-room: doar mecanica generală a genului.
use crate::c1_duel::*;

static SPEC: DSpec = DSpec {
    title: "Vrăjitorii din Ruine",
    deck1: "c1,c2,c1,a2,a2,a3,a1,d2,d2,h2,c1,a2,s1,a3",
    deck2: "c2,c1,c1,a3,a2,a2,a1,d2,d3,h2,c1,a2,s1,a2",
    hand: 4,
    plays: 3,
    boss_hp: 55,
    atk: &[3, 4, 5],
    scheme: &[],
    prog: 0,
    doom_max: 99,
    hp: 12,
    flags: "m",
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
