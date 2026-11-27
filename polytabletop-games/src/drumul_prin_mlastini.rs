//! Drumul prin Mlaștini — variantă originală (lista C1), motor `c1_duel`; clean-room: doar mecanica generală a genului.
use crate::c1_duel::*;

static SPEC: DSpec = DSpec {
    title: "Drumul prin Mlaștini",
    deck1: "p2,p1,p1,a2,a3,d2,d2,h2,p2,s1,a1,p1,d1,a2",
    deck2: "p1,p2,p1,a2,a2,d3,d2,h2,p1,s1,a3,p2,h1,d1",
    hand: 5,
    plays: 2,
    boss_hp: 1,
    atk: &[2, 3],
    scheme: &[1, 2],
    prog: 28,
    doom_max: 8,
    hp: 10,
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
