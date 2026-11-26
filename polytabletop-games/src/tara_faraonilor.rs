//! Țara Faraonilor — variantă originală (lista B4), motor `c`; clean-room: doar mecanica generală a genului.
use crate::b4_c::*;

static SPEC: CSpec = CSpec {
    title: "Țara Faraonilor",
    edges: "0-1,1-2,3-4,4-5,6-7,7-8,0-3,3-6,1-4,4-7,2-5,5-8",
    vals: &[1, 2, 1, 2, 3, 2, 1, 2, 1],
    forts: &[4],
    caps: [6, 2],
    ap: 3,
    turns: 14,
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
