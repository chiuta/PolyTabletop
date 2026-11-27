//! Sala de Comandă Stelară — variantă originală (lista C1), motor `c1_alert`; clean-room: doar mecanica generală a genului.
use crate::c1_alert::*;

static SPEC: ASpec = ASpec {
    title: "Sala de Comandă Stelară",
    rounds: 12,
    hull: 4,
    spawn: 260,
    start_dist: 2,
};

pub fn new() -> Alert {
    Alert::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Alert {
    Alert::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 600);
    }
}
