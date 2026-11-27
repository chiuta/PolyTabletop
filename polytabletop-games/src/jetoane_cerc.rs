//! Jetoane în Cerc — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală de noroc pur: fiecare are 3 jetoane și aruncă (arunc) până la 3 zaruri după câte jetoane are. Fețele: dau (un jeton merge la celălalt), pot (un jeton în potul comun), iau (iei un jeton de la celălalt), trei fețe goale. Rămâi fără jetoane și pierzi.
use crate::c5_push::*;

pub static SPEC: PushSpec = PushSpec {
    title: "Jetoane în Cerc",
    mode: PMode::Chips,
    goal: 0,
    a: 3,
    b: 0,
    cap: 500,
};

pub fn new() -> Push {
    Push::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Push::with_seed(&SPEC, s), 560);
    }
}
