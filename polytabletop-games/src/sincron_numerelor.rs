//! Sincron de Numere — variantă originală (lista C1), motor `c1_mind`; clean-room: doar mecanica generală a genului.
use crate::c1_mind::*;

static SPEC: MSpec = MSpec {
    title: "Sincron de Numere",
    levels: 6,
    lives: 3,
    top: 100,
};

pub fn new() -> Mind {
    Mind::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Mind {
    Mind::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 6000);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::c1_util::honest(super::with_seed, |g, h, s| g.scramble(h, s), 400);
        crate::c1_util::viewer_ok(super::with_seed, 400);
    }
}
