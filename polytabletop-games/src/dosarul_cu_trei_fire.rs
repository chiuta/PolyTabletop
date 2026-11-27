//! Dosarul cu Trei Fire — variantă originală (lista C1), motor `c1_enigma`; clean-room: doar mecanica generală a genului.
use crate::c1_enigma::*;

static SPEC: ESpec = ESpec {
    title: "Dosarul cu Trei Fire",
    mode: 'k',
    a: 12,
    b: 5,
    c: 3,
    time: 5,
    hint: 0,
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
