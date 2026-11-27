//! Asediul Turnului — variantă originală (lista C1), motor `c1_graf`; clean-room: doar mecanica generală a genului.
use crate::c1_graf::*;

static SPEC: GSpec = GSpec {
    title: "Asediul Turnului",
    n: 10,
    edges: "0-1,0-2,0-3,1-4,1-5,2-6,2-7,3-8,3-9,4-5,5-6,6-7,7-8,8-9",
    start: [1, 3],
    exit: 0,
    items: &[],
    need: 12,
    cap: 3,
    per: 4,
    spread: 'a',
    doom_max: 3,
    rounds: 18,
    pursuer: 99,
    rst: false,
    hp: 99,
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
