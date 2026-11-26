//! Categoria Îndrăzneață — variantă originală B6 (motor comun `b6_mask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: opt căsuțe pe o mască ascunsă; categoria (cat j: 2, 3, 4 sau 5 căsuțe bune) o alege ADVERSARUL celui la rând. Apoi alegi căsuțe (alege i, arata) ca la „zece uși”: bună +1 și continui; rea: pierzi banca, tura trece; stop salvează.
use crate::b6_mask::*;

static SPEC: MSpec = MSpec {
    title: "Categoria Îndrăzneață",
    n: 8,
    ks: &[2, 3, 4, 5],
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
