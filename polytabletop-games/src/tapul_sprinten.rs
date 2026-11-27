//! Țapul Sprinten — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: trei țapi pe un drum de munte de 18 pătrate. Ieși din staul doar cu un 6 (și mai arunci o dată). Cine aterizează pe un țap străin îl trimite în staul. Câștigă cine aduce doi țapi pe pășune.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Țapul Sprinten",
    len: 18,
    toks: 3,
    need: 2,
    sides: 6,
    capture: true,
    exact: false,
    enter: 6,
    cap: 900,
    sq: &[(7, Sq::Extra), (12, Sq::Fwd(2))],
};

pub fn new() -> Race {
    Race::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Race::with_seed(&SPEC, s), 960);
    }
}
