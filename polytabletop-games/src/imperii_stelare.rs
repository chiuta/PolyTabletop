//! Imperii Stelare — variantă originală (lista B4), motor `c`; clean-room: doar mecanica generală a genului.
use crate::b4_c::*;

static SPEC: CSpec = CSpec {
    title: "Imperii Stelare",
    edges: "0-1,1-2,2-3,3-4,4-5,5-6,6-7,7-8,8-9,9-0,0-5,2-7",
    vals: &[2, 1, 2, 1, 2, 1, 2, 1, 2, 1],
    forts: &[],
    caps: [3, 8],
    ap: 3,
    turns: 12,
    fl: "t",
    depth: 2,
};

pub fn new() -> C {
    C::new(&SPEC)
}

pub fn with_seed(seed: u64) -> C {
    C::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 1500);
    }
}
