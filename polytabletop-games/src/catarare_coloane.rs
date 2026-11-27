//! Cățărarea pe Coloane — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: coloanele 2–12 au lungimi proprii (2,3,5,6,7,8,7,6,5,3,2). Arunci patru zaruri (arunc), le împerechezi (cuplu k) și avansezi cel mult trei alpiniști temporari; continu aruncă iar, opresc salvează, iar fără împerechere posibilă pierzi tot (bust). Câștigă cine închide primul două coloane.
use crate::c5_cs::*;

pub static SPEC: CsSpec = CsSpec {
    title: "Cățărarea pe Coloane",
    lens: [2, 3, 5, 6, 7, 8, 7, 6, 5, 3, 2],
    runners: 3,
    win: 2,
    cap: 700,
};

pub fn new() -> Cs {
    Cs::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Cs::with_seed(&SPEC, s), 760);
    }
}
