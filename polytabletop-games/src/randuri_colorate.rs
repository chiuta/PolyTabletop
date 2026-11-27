//! Rânduri Colorate — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: patru rânduri (roșu și galben urcă 2→12, verde și albastru coboară 12→2), marcaje doar spre dreapta. Suma zarurilor albe o poate folosi oricine (m r); apoi cel la rând poate folosi un zar alb plus zarul colorat al rândului (c w r). Un rând cu 4 marcaje se poate încheia. Penalizare dacă nu marchezi nimic; scorul crește triunghiular. Jocul se termină la două rânduri blocate sau 4 penalizări.
use crate::c5_rows::*;

pub static SPEC: RqSpec = RqSpec {
    title: "Rânduri Colorate",
    lockmin: 4,
    pen_val: 4,
    pen_max: 4,
    locks_end: 2,
    names: ["roșu", "galben", "verde", "albastru"],
    cap: 800,
};

pub fn new() -> Rq {
    Rq::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Rq::with_seed(&SPEC, s), 860);
    }
}
