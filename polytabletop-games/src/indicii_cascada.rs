//! Indicii în Cascadă — variantă originală B6 (motor comun `b6_ask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: un cuvânt-țintă ascuns dintre 18; la fiecare mutare ori ceri un indiciu nou (mai mult, cel mult 3), ori ghicești. Primul ghicit corect ia 1 + indiciile nefolosite; greșeala dă 1 adversarului. 6 runde.
use crate::b6_ask::*;

static SPEC: ASpec = ASpec {
    title: "Indicii în Cascadă",
    vals: &[3, 3, 2],
    mode: AMode::Ladder,
    rounds: 6,
    qmax: 3,
    attr_names: &["categorie", "lungime", "început"],
};

pub fn new() -> Ask {
    Ask::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Ask::with_seed(&SPEC, s), 400);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
