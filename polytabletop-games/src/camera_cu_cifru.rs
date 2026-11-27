//! Camera cu Cifru — variantă originală (lista C1), motor `c1_enigma`; clean-room: doar mecanica generală a genului.
use crate::c1_enigma::*;

static SPEC: ESpec = ESpec {
    title: "Camera cu Cifru",
    mode: 'l',
    a: 3,
    b: 3,
    c: 5,
    time: 11,
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
