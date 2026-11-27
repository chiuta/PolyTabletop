//! Zarurile Așezărilor — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: șase zaruri cu cinci resurse (lemn, argilă, oaie, grâu, minereu) și aur (joker). Trei aruncări (arunc, reia m, gata), apoi cheltui zarurile pe construcții din lanțul tău (constr x): patru drumuri, patru așezări (cer drumul cu același număr) și patru orașe (cer așezarea). Tura fără construcție costă 1 glorie. Primul la 12 glorie câștigă.
use crate::c5_roll3::*;

pub static SPEC: RollSpec = RollSpec {
    title: "Zarurile Așezărilor",
    goal: 12,
    road: ([1, 1, 0, 0, 0], 1),
    town: ([1, 1, 1, 1, 0], 2),
    city: ([0, 0, 0, 2, 3], 3),
    n: 4,
    cap: 700,
};

pub fn new() -> Roll3 {
    Roll3::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Roll3::with_seed(&SPEC, s), 760);
    }
}
