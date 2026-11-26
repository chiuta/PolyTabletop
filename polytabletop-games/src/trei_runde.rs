//! Aceleași Cărți, Trei Runde — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: o țintă ascunsă dintre 18 elemente; limita de indicii scade pe parcurs: 3, 3, 2, 2, 1, 1 în cele 6 runde (ca la „aceleași cărți, reguli tot mai stricte”). Cine descrie și cine ghicește alternează.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Aceleași Cărți, Trei Runde",
    vals: &[3, 3, 2],
    caps: &[3, 3, 2, 2, 1, 1],
    mask: 7,
    rounds: 6,
    attr_names: &["epocă", "domeniu", "ton"],
};

pub fn new() -> Deduce {
    Deduce::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Deduce::with_seed(&SPEC, s), 300);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
