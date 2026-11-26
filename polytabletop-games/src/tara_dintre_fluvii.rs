//! Țara dintre Fluvii — variantă originală (lista B4), motor `g`; clean-room: doar mecanica generală a genului.
use crate::b4_g::*;

static SPEC: GSpec = GSpec {
    title: "Țara dintre Fluvii",
    n: 6,
    shared: true,
    minscore: true,
    tiles: 48,
    depth: 1,
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
        crate::brand_util::check(super::with_seed, 800);
    }
}
