//! Linia Frontului — variantă originală (lista B4), motor `c`; clean-room: doar mecanica generală a genului.
use crate::b4_c::*;

static SPEC: CSpec = CSpec {
    title: "Linia Frontului",
    edges: "0-1,1-2,2-3,3-4,4-5,6-7,7-8,8-9,9-10,10-11,0-6,1-7,2-8,3-9,4-10,5-11,1-8,3-10",
    vals: &[1, 2, 2, 3, 2, 1, 1, 2, 3, 2, 2, 1],
    forts: &[2, 9],
    caps: [0, 11],
    ap: 4,
    turns: 12,
    fl: "",
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
