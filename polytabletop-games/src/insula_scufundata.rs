//! Insula care se Scufundă — variantă originală (lista C1), motor `c1_graf`; clean-room: doar mecanica generală a genului.
use crate::c1_graf::*;

static SPEC: GSpec = GSpec {
    title: "Insula care se Scufundă",
    n: 12,
    edges: "0-1,0-4,1-2,1-5,2-3,2-6,3-7,4-5,4-8,5-6,5-9,6-7,6-10,7-11,8-9,9-10,10-11",
    start: [5, 6],
    exit: 0,
    items: &[3, 8, 11],
    need: 3,
    cap: 1,
    per: 3,
    spread: 's',
    doom_max: 4,
    rounds: 15,
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
