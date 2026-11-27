//! Evadare cu Cartele — variantă originală (lista C1), motor `c1_enigma`; clean-room: doar mecanica generală a genului.
use crate::c1_enigma::*;

static SPEC: ESpec = ESpec {
    title: "Evadare cu Cartele",
    mode: 'l',
    a: 4,
    b: 3,
    c: 4,
    time: 13,
    hint: 2,
};

pub fn new() -> Enigma {
    Enigma::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Enigma {
    Enigma::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 500);
    }
}
