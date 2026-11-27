//! Tema în Doi — variantă originală (lista C1), motor `c1_duet`; clean-room: doar mecanica generală a genului.
use crate::c1_duet::*;

static SPEC: USpec = USpec {
    title: "Tema în Doi",
    timer: 20,
};

pub fn new() -> Duet {
    Duet::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Duet {
    Duet::with_seed(&SPEC, seed)
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
