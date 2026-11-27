//! Tranșee Tăcute — variantă originală (lista C1), motor `c1_resist`; clean-room: doar mecanica generală a genului.
use crate::c1_resist::*;

static SPEC: RSpec = RSpec {
    title: "Tranșee Tăcute",
    rounds: 18,
    cap: 2,
    hand: 5,
    wild: 4,
    per_type: 6,
};

pub fn new() -> Resist {
    Resist::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Resist {
    Resist::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 600);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::c1_util::honest(super::with_seed, |g, h, s| g.scramble(h, s), 400);
        crate::c1_util::viewer_ok(super::with_seed, 400);
    }
}
