//! Limbajul Icoanelor — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: o țintă ascunsă dintre 32 de elemente cu patru atribute (icoană, culoare, număr, formă). Cine „vorbește” doar prin icoane poate pune până la 4 indicii adevărate; celălalt ghicește. 6 runde, rolurile alternează.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Limbajul Icoanelor",
    vals: &[4, 2, 2, 2],
    caps: &[4],
    mask: 15,
    rounds: 6,
    attr_names: &["icoană", "culoare", "număr", "formă"],
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
