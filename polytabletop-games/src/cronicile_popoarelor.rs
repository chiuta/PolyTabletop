//! Cronicile Popoarelor — variantă originală (lista B4), motor `c`; clean-room: doar mecanica generală a genului.
use crate::b4_c::*;

static SPEC: CSpec = CSpec {
    title: "Cronicile Popoarelor",
    edges: "0-1,1-2,2-3,3-4,4-5,5-0,0-6,2-7,4-8,6-7,7-8,8-6",
    vals: &[1, 2, 1, 2, 1, 2, 3, 3, 3],
    forts: &[],
    caps: [1, 4],
    ap: 3,
    turns: 14,
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
