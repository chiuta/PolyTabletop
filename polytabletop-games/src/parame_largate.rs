//! Parâme Largate — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: trei bărci pe un râu de 20 de pătrate, zar cu patru fețe. Curenți (+2), vârtejuri (−2) și un vad (tură în plus); la sosire e nevoie de zar exact. Câștigă cine ajunge cu două bărci la ponton.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Parâme Largate",
    len: 20,
    toks: 3,
    need: 2,
    sides: 4,
    capture: false,
    exact: true,
    enter: 0,
    cap: 800,
    sq: &[(5, Sq::Fwd(2)), (10, Sq::Back(2)), (14, Sq::Extra)],
};

pub fn new() -> Race {
    Race::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Race::with_seed(&SPEC, s), 860);
    }
}
