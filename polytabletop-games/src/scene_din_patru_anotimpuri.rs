//! Scene din Patru Anotimpuri — variantă originală (lista C1), motor `c1_enigma`; clean-room: doar mecanica generală a genului.
use crate::c1_enigma::*;

static SPEC: ESpec = ESpec {
    title: "Scene din Patru Anotimpuri",
    mode: 'k',
    a: 14,
    b: 6,
    c: 3,
    time: 6,
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
