//! Bolizi pe Oval — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: câte două mașini pe un oval de 24 de pătrate. Arunci zarul (arunc), muți una dintre mașini (muta k). Pe pătratele de turbo mergi încă 3, pe cele de pană pierzi tura. Câștigă cine aduce ambele mașini la linia de sosire.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Bolizi pe Oval",
    len: 24,
    toks: 2,
    need: 2,
    sides: 6,
    capture: false,
    exact: false,
    enter: 0,
    cap: 500,
    sq: &[
        (5, Sq::Fwd(3)),
        (8, Sq::Skip),
        (11, Sq::Fwd(3)),
        (17, Sq::Fwd(3)),
        (20, Sq::Skip),
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
        crate::brand_util::check(|s| Race::with_seed(&SPEC, s), 560);
    }
}
