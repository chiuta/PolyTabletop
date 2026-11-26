//! Legenda Potrivită — variantă originală B6 (motor comun `b6_crowd`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: pentru fiecare imagine-scenă (public, trei valori 0–3) alegeți în secret o legendă din mână (joc i); publicul are gusturi ascunse, iar scorul = valorile legendei × (gusturi + scenă). Cel mai mare ia runda. 6 runde.
use crate::b6_crowd::*;

static SPEC: CSpec = CSpec {
    title: "Legenda Potrivită",
    pick: 1,
    scene: Scene::Random,
    rounds: 6,
    dims: ["umor", "ironie", "tandrețe"],
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
