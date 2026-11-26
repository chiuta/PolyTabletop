//! Totemul Grăbit — variantă originală B6 (motor comun `b6_slap`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: 8 cărți-simbol fiecare (4 simboluri). Joci pe masă un simbol (joc s); dacă e la fel ca simbolul de deasupra stivei adversarului, are loc un duel de reflexe (prind), câștigat cu șansa 50% ± 8% pe serie; pierzătorul ia toate cărțile de pe masă. Câștigă cine rămâne fără cărți în mână.
use crate::b6_slap::*;

static SPEC: SSpec = SSpec {
    title: "Totemul Grăbit",
    syms: 4,
    hand: 8,
    mode: SMode::Top,
    cap: 160,
    names: &["cerc", "pătrat", "triunghi", "stea"],
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
