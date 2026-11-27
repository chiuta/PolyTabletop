//! Expediția Levatelor — variantă originală (lista C1), motor `c1_crew`; clean-room: doar mecanica generală a genului.
use crate::c1_crew::*;

static SPEC: CSpec = CSpec {
    title: "Expediția Levatelor",
    tasks: 3,
};

pub fn new() -> Crew {
    Crew::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Crew {
    Crew::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 100);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::c1_util::honest(super::with_seed, |g, h, s| g.scramble(h, s), 100);
        crate::c1_util::viewer_ok(super::with_seed, 100);
    }
}
