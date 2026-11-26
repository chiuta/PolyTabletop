//! Duelul Argumentelor — variantă originală B6 (motor comun `b6_crowd`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: mână de 5 cărți cu trei valori 0–4 (ridicol, convingător, periculos). Alegeți în secret două cărți (joc i,j); publicul are gusturi ascunse 0–3 (unul arătat) și scorul fiecărei alegeri e suma valorilor × gusturi; cel mai mare ia runda. 6 runde.
use crate::b6_crowd::*;

static SPEC: CSpec = CSpec {
    title: "Duelul Argumentelor",
    pick: 2,
    scene: Scene::None,
    rounds: 6,
    dims: ["ridicol", "convingător", "periculos"],
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
