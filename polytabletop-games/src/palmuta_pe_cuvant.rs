//! Pălmuța pe Cuvânt — variantă originală B6 (motor comun `b6_slap`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: se numără în ritm cinci cuvinte (pâine, sare, vin, miere, lapte), ciclic. Joci un simbol (joc s) pe masă; dacă e cel numărat acum, toți dau palma (prind), cel mai lent ia masa. Câștigă cine rămâne primul fără cărți în mână (8 cărți, 5 simboluri).
use crate::b6_slap::*;

static SPEC: SSpec = SSpec {
    title: "Pălmuța pe Cuvânt",
    syms: 5,
    hand: 8,
    mode: SMode::Call,
    cap: 160,
    names: &["pâine", "sare", "vin", "miere", "lapte"],
};

pub fn new() -> Slap {
    Slap::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Slap::with_seed(&SPEC, s), 400);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
