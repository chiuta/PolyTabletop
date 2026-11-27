//! Conacul cu Ghicitori — variantă originală (lista C1), motor `c1_duel`; clean-room: doar mecanica generală a genului.
use crate::c1_duel::*;

static SPEC: DSpec = DSpec {
    title: "Conacul cu Ghicitori",
    deck1: "p2,p1,p2,a2,a1,d2,h2,s1,p1,d1",
    deck2: "p1,p2,p1,a2,a2,d2,h1,s1,p2,d1",
    hand: 4,
    plays: 2,
    boss_hp: 1,
    atk: &[1, 2, 3],
    scheme: &[1, 2, 2],
    prog: 14,
    doom_max: 9,
    hp: 8,
    flags: "e",
    rounds: 12,
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
