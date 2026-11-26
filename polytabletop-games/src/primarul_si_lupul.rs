//! Primarul și Lupul — variantă originală B6 (motor comun `b6_ask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: primarul știe ținta (12 elemente cu atributele tip, mărime, culoare) și răspunde adevărat; celălalt jucător are cel mult 5 întrebări (intreb X=v) și apoi ghicește. Rolurile alternează, 6 runde.
use crate::b6_ask::*;

static SPEC: ASpec = ASpec {
    title: "Primarul și Lupul",
    vals: &[3, 2, 2],
    mode: AMode::Mayor,
    rounds: 6,
    qmax: 5,
    attr_names: &["tip", "mărime", "culoare"],
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
