//! Marile Puteri — variantă originală (lista B4), motor `i`; clean-room: doar mecanica generală a genului.
use crate::b4_i::*;

static SPEC: ISpec = ISpec {
    title: "Marile Puteri",
    edges: "0-1,1-2,3-4,4-5,6-7,7-8,0-3,3-6,1-4,4-7,2-5,5-8",
    vals: &[2, 1, 2, 1, 3, 1, 2, 1, 2],
    stab: &[1, 1, 1, 1, 2, 1, 1, 1, 1],
    starts: [&[0, 3], &[8, 5]],
    ap: 3,
    rounds: 10,
    every: 2,
    win: 12,
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
