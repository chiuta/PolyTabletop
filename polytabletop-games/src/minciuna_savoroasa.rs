//! Minciuna Savuroasă — variantă originală (lista C1), motor `b6_pick`; clean-room: doar mecanica generală a genului.
use crate::b6_pick::*;

static SPEC: PSpec = PSpec {
    title: "Minciuna Savuroasă",
    n: 5,
    stake: true,
    rounds: 6,
    label: "varianta",
};

pub fn new() -> Pick {
    Pick::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Pick {
    Pick::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 300);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::b6_pick::audit(&super::SPEC);
    }
}
