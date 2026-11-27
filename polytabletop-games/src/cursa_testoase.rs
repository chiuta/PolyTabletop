//! Cursa Țestoaselor — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: cinci țestoase colorate; mai întâi își alege fiecare câte două (alege t), apoi, la fiecare tură, încurajezi o culoare (incurajez t) și o mutare forțată (trag) trage o culoare la întâmplare: aceea avansează 1, iar dacă a fost cea încurajată avansează încă 1. Câștigă proprietarul primei țestoase care ajunge la 12.
use crate::c5_bet::*;

pub static SPEC: BSpec = BSpec {
    title: "Cursa Țestoaselor",
    len: 12,
    bonus: 1,
    cap: 400,
    names: &["verde", "galbenă", "roșie", "albastră", "mov"],
};

pub fn new() -> Bet {
    Bet::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Bet::with_seed(&SPEC, s), 460);
    }
}
