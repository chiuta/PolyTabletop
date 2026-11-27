//! Vânătorii de Imagini — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: 25 de imagini ascunse (5 feluri); la fiecare rundă ținta publică e o pereche „A + B”. Întorci două plăci (volta i, arata): dacă sunt exact A și B, le iei (+1) și apare o țintă nouă; altfel se întorc, dar sunt cunoscute de amândoi. Primul la 5 puncte câștigă.
use crate::c5_memo::*;

pub static SPEC: MemSpec = MemSpec {
    title: "Vânătorii de Imagini",
    mode: MMode::Target,
    w: 5,
    h: 5,
    kinds: 5,
    cats: 0,
    wilds: 0,
    goal: 5,
    cap: 1500,
    names: &["pisică", "casă", "copac", "barcă", "stea"],
};

pub fn new() -> Memo {
    Memo::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Memo::with_seed(&SPEC, s), 1560);
    }
}
