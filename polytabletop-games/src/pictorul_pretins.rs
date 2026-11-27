//! Pictorul Pretins — variantă originală (lista C1), motor `b6_deduce`; clean-room: doar mecanica generală a genului.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Pictorul Pretins",
    vals: &[4, 4, 2],
    caps: &[3],
    mask: 7,
    rounds: 6,
    attr_names: &["subiect", "linie", "început"],
};

pub fn new() -> Deduce {
    Deduce::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Deduce {
    Deduce::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 600);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::b6_deduce::audit(&super::SPEC);
    }
}
