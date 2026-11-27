//! Caiacele Nordului — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: două caiace pe un fiord de 28 de pătrate, zar cu trei fețe; lovești un caiac rival aterizând pe el (îl trimiți la mal). Copci de pește dau tură în plus, gheața pierde tura. Câștigă cine aduce ambele caiace la tabără.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Caiacele Nordului",
    len: 28,
    toks: 2,
    need: 2,
    sides: 3,
    capture: true,
    exact: false,
    enter: 0,
    cap: 700,
    sq: &[
        (6, Sq::Extra),
        (12, Sq::Extra),
        (19, Sq::Skip),
        (23, Sq::Fwd(3)),
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
