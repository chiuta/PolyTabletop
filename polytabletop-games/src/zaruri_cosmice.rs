//! Zaruri Cosmice — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: fiecare are zaruri-muncitori (3 la început) cu fețele explorare, așezare, producție, livrare, dezvoltare și joker. După arunc, jokerii se fixează (fixez d f), iar un zar poate fi schimbat contra unui credit (ajust d f); gata rezolvă fazele pentru ambii: explorare +1 carte, așezare (2 cărți → lume, +1 glorie), producție (marfă pe lume), livrare (marfă → credit și glorie), dezvoltare (3 cărți → 3 glorie). Apoi recrut (zar nou contra 2 credite) sau fin. 8 runde.
use crate::c5_alloc::*;

pub static SPEC: GalSpec = GalSpec {
    title: "Zaruri Cosmice",
    rounds: 8,
    dice0: 3,
    dice_max: 6,
    recruit: 2,
    settle_cost: 2,
    develop_cost: 3,
};

pub fn new() -> Gal {
    Gal::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Gal::with_seed(&SPEC, s), 600);
    }
}
