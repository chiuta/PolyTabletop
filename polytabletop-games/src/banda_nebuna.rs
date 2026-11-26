//! Banda Desenată Nebună — variantă originală B6 (motor comun `b6_crowd`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: banda continuă: scena unei runde e cartea câștigătoare din runda precedentă. Alegeți în secret o carte (joc i); publicul are gusturi ascunse; scorul = valorile cărții × (gusturi + scenă). Cel mai mare ia runda. 6 runde.
use crate::b6_crowd::*;

static SPEC: CSpec = CSpec {
    title: "Banda Desenată Nebună",
    pick: 1,
    scene: Scene::Prev,
    rounds: 6,
    dims: ["absurd", "negru", "dulce"],
};

pub fn new() -> Crowd {
    Crowd::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Crowd::with_seed(&SPEC, s), 100);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
