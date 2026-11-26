//! Treptele Civilizației — variantă originală (lista B4), motor `tb`; clean-room: doar mecanica generală a genului.
use crate::b4_tb::*;

static SPEC: TbSpec = TbSpec {
    title: "Treptele Civilizației",
    market: 4,
    deck: 36,
    fl: "",
    turns: 12,
    setb: 2,
    inc_mul: 2,
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
