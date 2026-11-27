//! Balaurul cu Dinți de Lapte — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: doi pui de balaur pe un drum de 30 de pătrate cu scări (înainte) și tobogane (înapoi). Sosirea se face cu zar exact. Câștigă primul pui de balaur care ajunge în peșteră.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Balaurul cu Dinți de Lapte",
    len: 30,
    toks: 2,
    need: 1,
    sides: 6,
    capture: false,
    exact: true,
    enter: 0,
    cap: 700,
    sq: &[
        (3, Sq::Fwd(6)),
        (9, Sq::Fwd(8)),
        (14, Sq::Back(6)),
        (18, Sq::Fwd(5)),
        (22, Sq::Back(9)),
        (26, Sq::Back(8)),
    ],
};

pub fn new() -> Race {
    Race::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Race::with_seed(&SPEC, s), 760);
    }
}
