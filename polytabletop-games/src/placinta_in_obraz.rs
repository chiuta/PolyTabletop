//! Plăcinta în Obraz — variantă originală (lista C1), motor `b6_mask`; clean-room: doar mecanica generală a genului.
use crate::b6_mask::*;

static SPEC: MSpec = MSpec {
    title: "Plăcinta în Obraz",
    n: 6,
    ks: &[5],
    bank: false,
    ranked: false,
    rounds: 6,
};

pub fn new() -> Mask {
    Mask::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Mask {
    Mask::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 300);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::b6_mask::audit(&super::SPEC);
    }
}
