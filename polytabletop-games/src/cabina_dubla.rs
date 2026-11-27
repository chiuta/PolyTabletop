//! Cabina Dublă — variantă originală (lista C1), motor `c1_sky`; clean-room: doar mecanica generală a genului.
use crate::c1_sky::*;

static SPEC: SSpec = SSpec {
    title: "Cabina Dublă",
    lo: &[5, 5, 5, 6, 6, 6, 6],
    hi: &[9, 9, 9, 9, 9, 9, 8],
    tol: 2,
    reserve: 2,
    rerolls: 2,
};

pub fn new() -> Sky {
    Sky::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Sky {
    Sky::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 400);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::c1_util::honest(super::with_seed, |g, h, s| g.scramble(h, s), 400);
        crate::c1_util::viewer_ok(super::with_seed, 400);
    }
}
