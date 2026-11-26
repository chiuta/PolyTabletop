//! Așezările Imperiului — variantă originală (lista B4), motor `tb`; clean-room: doar mecanica generală a genului.
use crate::b4_tb::*;

static SPEC: TbSpec = TbSpec {
    title: "Așezările Imperiului",
    market: 5,
    deck: 40,
    fl: "z",
    turns: 14,
    setb: 2,
    inc_mul: 1,
    depth: 2,
};

pub fn new() -> Tb {
    Tb::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Tb {
    Tb::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 800);
    }
}
