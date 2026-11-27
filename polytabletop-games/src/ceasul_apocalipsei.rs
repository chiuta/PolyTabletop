//! Ceasul Apocalipsei — variantă originală (lista C1), motor `c1_graf`; clean-room: doar mecanica generală a genului.
use crate::c1_graf::*;

static SPEC: GSpec = GSpec {
    title: "Ceasul Apocalipsei",
    n: 12,
    edges: "0-1,0-4,1-2,1-5,2-3,2-6,3-7,4-5,4-8,5-6,5-9,6-7,6-10,7-11,8-9,9-10,10-11,0-11",
    start: [4, 7],
    exit: 6,
    items: &[2, 9, 11, 0],
    need: 4,
    cap: 2,
    per: 4,
    spread: 'o',
    doom_max: 3,
    rounds: 14,
    pursuer: 11,
    rst: false,
    hp: 3,
};

pub fn new() -> G {
    G::new(&SPEC)
}

pub fn with_seed(seed: u64) -> G {
    G::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 900);
    }
}
