//! Focare și Leacuri — variantă originală (lista C1), motor `c1_graf`; clean-room: doar mecanica generală a genului.
use crate::c1_graf::*;

static SPEC: GSpec = GSpec {
    title: "Focare și Leacuri",
    n: 12,
    edges: "0-1,1-2,2-3,3-4,4-5,5-6,6-7,7-8,8-9,9-10,10-11,11-0,0-6,3-9",
    start: [1, 9],
    exit: 0,
    items: &[3, 5, 8, 10],
    need: 4,
    cap: 3,
    per: 4,
    spread: 'o',
    doom_max: 3,
    rounds: 16,
    pursuer: 99,
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
