//! Crima din Port: Ancheta — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: criminalul (dintre 36 de suspecți, cu patru atribute: armă, indiciu, loc, motiv) e cunoscut doar de expert, care poate lăsa cel mult 3 indicii; anchetatorul ghicește. 6 runde, rolurile alternează.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Crima din Port: Ancheta",
    vals: &[3, 3, 2, 2],
    caps: &[3],
    mask: 15,
    rounds: 6,
    attr_names: &["armă", "indiciu", "loc", "motiv"],
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
