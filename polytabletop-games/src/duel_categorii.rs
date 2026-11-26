//! Duel de Categorii — variantă originală B6 (motor comun `b6_slap`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: 10 cărți-simbol fiecare (6 simboluri). Joci un simbol (joc s); dacă se potrivește cu cel de deasupra stivei adversarului, începe un duel de categorii (prind), câștigat cu șansa 50% ± 8% pe serie; pierzătorul ia masa. Câștigă primul fără cărți în mână.
use crate::b6_slap::*;

static SPEC: SSpec = SSpec {
    title: "Duel de Categorii",
    syms: 6,
    hand: 10,
    mode: SMode::Top,
    cap: 160,
    names: &["frunză", "val", "flacără", "piatră", "vânt", "lună"],
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
