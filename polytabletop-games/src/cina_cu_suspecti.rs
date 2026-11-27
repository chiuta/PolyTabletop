//! Cina cu Suspecți — variantă originală (lista C1), motor `b6_ask`; clean-room: doar mecanica generală a genului.
use crate::b6_ask::*;

static SPEC: ASpec = ASpec {
    title: "Cina cu Suspecți",
    vals: &[4, 3, 3, 2],
    mode: AMode::Shared,
    rounds: 5,
    qmax: 5,
    attr_names: &["armă", "loc", "motiv", "alibi"],
};

pub fn new() -> Ask {
    Ask::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Ask {
    Ask::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 400);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::b6_ask::audit(&super::SPEC);
    }
}
