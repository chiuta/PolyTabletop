//! Legiuni și Falange — variantă originală (lista B4), motor `d`; clean-room: doar mecanica generală a genului.
use crate::b4_d::*;

static SPEC: DSpec = DSpec {
    title: "Legiuni și Falange",
    rows: 7,
    cols: 9,
    cmd: 4,
    medals: 7,
    sections: false,
    turns: 30,
    depth: 1,
};

pub fn new() -> D {
    D::new(&SPEC)
}

pub fn with_seed(seed: u64) -> D {
    D::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 1500);
    }
}
