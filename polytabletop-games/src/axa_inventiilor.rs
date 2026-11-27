//! Axa Invențiilor — variantă originală (lista C1), motor `c1_cronol`; clean-room: doar mecanica generală a genului.
use crate::c1_cronol::*;

static SPEC: TSpec = TSpec {
    title: "Axa Invențiilor",
    goal: 7,
    cap: 28,
    era: 0,
};

pub fn new() -> Cronol {
    Cronol::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Cronol {
    Cronol::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 600);
    }
}
