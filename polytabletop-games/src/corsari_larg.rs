//! Corsarii din Larg — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: două corăbii pe un traseu de 22 de pătrate; abordezi o corabie rivală aterizând pe ea (se întoarce în port). Pătrate cu vânt bun (+3), cu calm (pierzi tura) și cu comoară (tură în plus). Câștigă cine își duce ambele corăbii la insulă.
use crate::c5_race::*;

pub static SPEC: RSpec = RSpec {
    title: "Corsarii din Larg",
    len: 22,
    toks: 2,
    need: 2,
    sides: 6,
    capture: true,
    exact: false,
    enter: 0,
    cap: 600,
    sq: &[(6, Sq::Extra), (10, Sq::Skip), (15, Sq::Fwd(3))],
};

pub fn new() -> Race {
    Race::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Race::with_seed(&SPEC, s), 660);
    }
}
