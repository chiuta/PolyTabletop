//! Mimica Mută — variantă originală B6 (motor comun `b6_deduce`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: mimul nu spune cuvinte, doar arată doi dintre cei trei „gesturi” (atributele B și C ale țintei dintre 18 elemente), cel mult 2 indicii; ghicitorul alege ținta. 6 runde, rolurile alternează.
use crate::b6_deduce::*;

static SPEC: DSpec = DSpec {
    title: "Mimica Mută",
    vals: &[2, 3, 3],
    caps: &[2],
    mask: 6,
    rounds: 6,
    attr_names: &["viteză", "formă", "poziție"],
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
