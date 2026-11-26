//! Zece Uși, Câteva Adevărate — variantă originală B6 (motor comun `b6_mask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: zece uși, dintre care 4 (număr cunoscut) sunt bune, pe o mască ascunsă. Pe rând alegi o ușă (alege i, apoi arata): bună +1 și continui; rea: pierzi punctele nesalvate și tura trece. Cu stop salvezi. 6 întrebări.
use crate::b6_mask::*;

static SPEC: MSpec = MSpec {
    title: "Zece Uși, Câteva Adevărate",
    n: 10,
    ks: &[4],
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
        crate::brand_util::check(|s| Mask::with_seed(&SPEC, s), 300);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
