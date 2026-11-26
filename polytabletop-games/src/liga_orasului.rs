//! Liga Orașelor — variantă originală (lista B4), motor `n`; clean-room: doar mecanica generală a genului.
use crate::b4_n::*;

static SPEC: NSpec = NSpec {
    title: "Liga Orașelor",
    edges: "0-1:2,1-2:3,2-3:2,3-4:3,4-5:2,5-0:3,0-3:4,1-4:4",
    vals: &[0, 0, 0, 0, 0, 0],
    hubs: [0, 3],
    mode: 'h',
    turns: 14,
    depth: 2,
};

pub fn new() -> N {
    N::new(&SPEC)
}

pub fn with_seed(seed: u64) -> N {
    N::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 800);
    }
}
