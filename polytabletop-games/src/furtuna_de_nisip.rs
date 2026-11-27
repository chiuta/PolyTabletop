//! Furtuna de Nisip — variantă originală (lista C1), motor `c1_graf`; clean-room: doar mecanica generală a genului.
use crate::c1_graf::*;

static SPEC: GSpec = GSpec {
    title: "Furtuna de Nisip",
    n: 16,
    edges: "0-1,0-4,1-2,1-5,2-3,2-6,3-7,4-5,4-8,5-6,5-9,6-7,6-10,7-11,8-9,8-12,9-10,9-13,10-11,10-14,11-15,12-13,13-14,14-15",
    start: [5, 6],
    exit: 5,
    items: &[0, 3, 12, 15, 9],
    need: 5,
    cap: 2,
    per: 5,
    spread: 'w',
    doom_max: 12,
    rounds: 12,
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
