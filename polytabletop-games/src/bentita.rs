//! Banderola pe Frunte — variantă originală B6 (motor comun `b6_ask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: fiecare are un obiect (dintre 18) pe care îl vede doar adversarul. Pe rând puneți întrebări da/nu despre al vostru (intreb X=v, cel mult 5 pe jucător) sau ghiciți: corect +2, greșit +1 pentru celălalt. 6 runde.
use crate::b6_ask::*;

static SPEC: ASpec = ASpec {
    title: "Banderola pe Frunte",
    vals: &[3, 3, 2],
    mode: AMode::Own,
    rounds: 6,
    qmax: 5,
    attr_names: &["formă", "mărime", "culoare"],
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
