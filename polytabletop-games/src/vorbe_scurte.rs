//! Descrierea în trei vorbe — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: o țintă ascunsă dintre 12 elemente (A tip 1–3, B mărime 1–2, C mișcare 1–2). Cine descrie dă cel mult 3 indicii adevărate, apoi celălalt ghicește; rolurile alternează, 6 runde. Informație ascunsă: ținta.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Descrierea în trei vorbe",
    vals: &[3, 2, 2],
    caps: &[3],
    mask: 7,
    rounds: 6,
    attr_names: &["tip", "mărime", "mișcare"],
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
