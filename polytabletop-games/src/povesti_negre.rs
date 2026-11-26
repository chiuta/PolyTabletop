//! Cazuri Întunecate — variantă originală B6 (motor comun `b6_ask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: un caz ascuns dintre 32, definit de cinci fapte da/nu (noaptea, afară, singur, armă, martor). Pe rând întrebați despre un fapt (cel mult 6 întrebări în total, răspunsuri publice) sau ghiciți cazul: corect +2, greșit +1 pentru celălalt. 6 runde.
use crate::b6_ask::*;

static SPEC: ASpec = ASpec {
    title: "Cazuri Întunecate",
    vals: &[2, 2, 2, 2, 2],
    mode: AMode::Shared,
    rounds: 6,
    qmax: 6,
    attr_names: &["noaptea", "afară", "singur", "armă", "martor"],
};

pub fn new() -> Ask {
    Ask::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Ask::with_seed(&SPEC, s), 400);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
