//! Vampirii din Ceață — variantă originală (lista C1), motor `c1_noapte`; clean-room: doar mecanica generală a genului.
use crate::c1_noapte::*;

static SPEC: NSpec = NSpec {
    title: "Vampirii din Ceață",
    deck: ['r', 's', 'b', 'm', 'v', 't'],
    bad: "rm",
    hands: 3,
};

pub fn new() -> Noapte {
    Noapte::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Noapte {
    Noapte::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 300);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::c1_util::honest(super::with_seed, |g, h, s| g.scramble(h, s), 300);
        crate::c1_util::viewer_ok(super::with_seed, 300);
    }
}
