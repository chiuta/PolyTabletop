//! Marșul Umbrelor — variantă originală (lista B4), motor `c`; clean-room: doar mecanica generală a genului.
use crate::b4_c::*;

static SPEC: CSpec = CSpec {
    title: "Marșul Umbrelor",
    edges: "0-1,1-2,2-3,3-4,4-5,5-6,6-7,7-8,8-9,9-10,2-6,4-8,3-7",
    vals: &[1, 1, 2, 2, 3, 3, 3, 2, 2, 1, 1],
    forts: &[3, 5, 7],
    caps: [0, 10],
    ap: 3,
    turns: 16,
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
