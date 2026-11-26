//! Cortina de Influență — variantă originală (lista B4), motor `i`; clean-room: doar mecanica generală a genului.
use crate::b4_i::*;

static SPEC: ISpec = ISpec {
    title: "Cortina de Influență",
    edges: "0-1,1-2,2-3,3-4,4-5,5-6,6-7,7-8,8-9,9-10,10-11,11-0,0-6,3-9",
    vals: &[1, 2, 1, 3, 2, 1, 2, 3, 1, 2, 3, 1],
    stab: &[1, 1, 2, 1, 2, 1, 1, 2, 1, 1, 2, 1],
    starts: [&[1, 2], &[8, 9]],
    ap: 4,
    rounds: 9,
    every: 3,
    win: 15,
    depth: 2,
};

pub fn new() -> I {
    I::new(&SPEC)
}

pub fn with_seed(seed: u64) -> I {
    I::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 800);
    }
}
