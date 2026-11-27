//! Dinții Crocodilului — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: opt dinți ascunși, dintre care unul singur „mușcă”. Pe rând apeși un dinte (alege i, apoi arata): sigur +1 și continui; dintele rău îți ia punctele nesalvate și tura trece; cu stop salvezi. 6 runde.
use crate::b6_mask::*;

pub static SPEC: MSpec = MSpec {
    title: "Dinții Crocodilului",
    n: 8,
    ks: &[7],
    bank: true,
    ranked: false,
    rounds: 6,
};

pub fn new() -> Mask {
    Mask::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Mask::with_seed(&SPEC, s), 360);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
