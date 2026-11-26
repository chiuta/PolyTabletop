//! Navigatorii Oceanului — variantă originală (lista B4), motor `n`; clean-room: doar mecanica generală a genului.
use crate::b4_n::*;

static SPEC: NSpec = NSpec {
    title: "Navigatorii Oceanului",
    edges: "0-1:2,1-2:2,2-3:2,3-4:2,4-5:2,5-6:2,6-7:2,7-0:2,1-5:3,3-7:3",
    vals: &[0, 2, 3, 4, 2, 3, 4, 2],
    hubs: [0, 4],
    mode: 'n',
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
