//! Șoricei și Caș — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: 20 de plăci cu fața în jos: 8 feluri de caș (câte două) și 4 pisici. Întorci două plăci (volta i, arata): perechea de caș se ia, mai întorci; o pisică descoperită te costă un punct și încheie tura. Primul la 6 caș câștigă.
use crate::c5_memo::*;

pub static SPEC: MemSpec = MemSpec {
    title: "Șoricei și Caș",
    mode: MMode::Pairs,
    w: 5,
    h: 4,
    kinds: 8,
    cats: 4,
    wilds: 0,
    goal: 6,
    cap: 1000,
    names: &[
        "cedar",
        "emmental",
        "brie",
        "gouda",
        "edam",
        "feta",
        "mozzarella",
        "parmezan",
    ],
};

pub fn new() -> Memo {
    Memo::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Memo::with_seed(&SPEC, s), 1060);
    }
}
