//! Piratul Cernit — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: trei pirați pe un traseu de 20 de pătrate; cine aterizează pe un pirat al adversarului îl trimite înapoi la start. Pătrate de recif (înapoi), de vânt (tură în plus) și de furtună (pierzi tura). Câștigă cine își aduce toți cei trei pirați la țărm.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Piratul Cernit",
    len: 20,
    toks: 3,
    need: 3,
    sides: 6,
    capture: true,
    exact: false,
    enter: 0,
    cap: 800,
    sq: &[
        (4, Sq::Back(2)),
        (9, Sq::Extra),
        (13, Sq::Skip),
        (16, Sq::Back(3)),
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
        crate::brand_util::check(|s| Race::with_seed(&SPEC, s), 860);
    }
}
