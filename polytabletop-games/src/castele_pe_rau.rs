//! Castelele de pe Râu — variantă originală (lista B4), motor `g`; clean-room: doar mecanica generală a genului.
use crate::b4_g::*;

static SPEC: GSpec = GSpec {
    title: "Castelele de pe Râu",
    n: 5,
    shared: false,
    minscore: false,
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
