//! Schițe Grăbite — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: schițatorul „desenează” țintei (dintre 20 de elemente) trăsături adevărate, cel mult 3 (A formă 1–5, B contur 1–2, C umbră 1–2); celălalt ghicește. 6 runde, rolurile alternează.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Schițe Grăbite",
    vals: &[5, 2, 2],
    caps: &[3],
    mask: 7,
    rounds: 6,
    attr_names: &["formă", "contur", "umbră"],
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
