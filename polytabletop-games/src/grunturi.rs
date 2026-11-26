//! Grunțuri Alese — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: doar „cuvinte scurte” permise: din cele trei atribute ale țintei (sunet, mărime, culoare) poți indica doar primele două, cel mult 2 indicii. Ținta e ascunsă; ghicitorul alege dintre 18 elemente. 6 runde.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Grunțuri Alese",
    vals: &[3, 3, 2],
    caps: &[2],
    mask: 3,
    rounds: 6,
    attr_names: &["sunet", "mărime", "culoare"],
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
