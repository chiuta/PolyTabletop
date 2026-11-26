//! Gânduri Împărtășite — variantă originală B6 (motor comun `b6_pick`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: unul alege în secret un răspuns din 6 (alege f), celălalt ghicește (ghic g). Ghicitorul primește max(0, 3 − |f − g|), cel care a ales primește min(3, |f − g|). Istoricul e public. 8 runde, rolurile alternează.
use crate::b6_pick::*;

static SPEC: PSpec = PSpec {
    title: "Gânduri Împărtășite",
    n: 6,
    stake: false,
    rounds: 8,
    label: "Variante de răspuns",
};

pub fn new() -> Pick {
    Pick::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Pick::with_seed(&SPEC, s), 100);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
