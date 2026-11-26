//! Valul Ideilor — variantă originală B6 (motor comun `b6_mask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: o listă de 12 idei, 6 dintre ele corecte, pe o mască ascunsă; cele de sus valorează 3, 2 sau 1 punct. Pe rând alegi o idee (alege i, apoi arata): corectă = puncte și continui; greșită = tura trece. 6 liste.
use crate::b6_mask::*;

static SPEC: MSpec = MSpec {
    title: "Valul Ideilor",
    n: 12,
    ks: &[6],
    bank: false,
    ranked: true,
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
