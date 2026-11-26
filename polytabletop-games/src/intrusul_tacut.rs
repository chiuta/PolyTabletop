//! Intrusul Tăcut — variantă originală B6 (motor comun `b6_ask`; clean-room, fără nume sau texte comerciale).
//!
//! Variantă originală: un obiect secret dintre 16 (rând A, coloană B, valori 1–4), necunoscut ambilor; pe rând întrebați da/nu (cel mult 6 întrebări în total, răspunsurile sunt publice), iar oricând cineva poate ghici: corect +2, greșit +1 pentru celălalt. 6 runde.
use crate::b6_ask::*;

static SPEC: ASpec = ASpec {
    title: "Intrusul Tăcut",
    vals: &[4, 4],
    mode: AMode::Shared,
    rounds: 6,
    qmax: 6,
    attr_names: &["rând", "coloană"],
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
