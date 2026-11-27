//! Vinovatul Tăinuit — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: un vinovat ascuns dintre 18 suspecți (pălărie, haină, încălțăminte). Ținta nu o știe nimeni; pe rând puneți întrebări da/nu despre o trăsătură (intreb X=v, cel mult 4 pe jucător, răspunsurile sunt publice) sau ghiciți (ghic k): corect +2, greșit +1 pentru celălalt. 6 runde.
use crate::b6_ask::*;

pub static SPEC: ASpec = ASpec {
    title: "Vinovatul Tăinuit",
    vals: &[3, 3, 2],
    mode: AMode::Shared,
    rounds: 6,
    qmax: 4,
    attr_names: &["pălărie", "haină", "încălțăminte"],
};

pub fn new() -> Ask {
    Ask::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Ask::with_seed(&SPEC, s), 460);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        audit(&SPEC);
    }
}
