//! Favoritul Judecătorului — variantă originală B6 (motor comun `b6_pick`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: judecătorul alege în secret un favorit dintre 5 răspunsuri (alege f); celălalt ghicește cu miza 1–3 (ghic g s). Corect: ghicitorul +miză; greșit: judecătorul +1. Istoricul alegerilor e public. 8 runde, rolurile alternează.
use crate::b6_pick::*;

static SPEC: PSpec = PSpec {
    title: "Favoritul Judecătorului",
    n: 5,
    stake: true,
    rounds: 8,
    label: "Răspunsuri",
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
