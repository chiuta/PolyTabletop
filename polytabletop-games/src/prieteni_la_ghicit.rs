//! Prieteni la Ghicit — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: patru jucării; la fiecare rundă (rolurile alternează) unul alege în secret o jucărie (alege f), celălalt ghicește (ghic g, cu miză s de 1–3 jetoane); ghicit corect: ghicitorul +s, greșit: cel care alege +1. Istoricul alegerilor dezvăluite e public. 8 runde.
use crate::b6_pick::*;

pub static SPEC: PSpec = PSpec {
    title: "Prieteni la Ghicit",
    n: 4,
    stake: true,
    rounds: 8,
    label: "jucărie",
};

pub fn new() -> Pick {
    Pick::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Pick::with_seed(&SPEC, s), 140);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
