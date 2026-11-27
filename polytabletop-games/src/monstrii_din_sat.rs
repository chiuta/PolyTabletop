//! Monștrii din Sat — variantă originală (lista C1), motor `c1_graf`; clean-room: doar mecanica generală a genului.
use crate::c1_graf::*;

static SPEC: GSpec = GSpec {
    title: "Monștrii din Sat",
    n: 9,
    edges: "0-1,0-3,1-2,1-4,2-5,3-4,3-6,4-5,4-7,5-8,6-7,7-8,0-8",
    start: [4, 6],
    exit: 0,
    items: &[2, 5, 8],
    need: 3,
    cap: 1,
    per: 2,
    spread: 'f',
    doom_max: 99,
    rounds: 12,
    pursuer: 2,
    rst: false,
    hp: 4,
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
