//! Maeștrii Zarului — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: duel de armate din zaruri, fiecare cu 12 viață. Tura: trag (patru zaruri aruncate), energie automată, dep i (cel mult 4 unități), cumpar t, atac i j (j = 0 lovitură directă doar dacă adversarul n-are unități: scade viața cu forța unității). Cine rămâne fără viață pierde.
use crate::c5_bag::*;

pub static SPEC: BagSpec = BagSpec {
    title: "Maeștrii Zarului",
    mode: BMode::Life,
    draw: 4,
    field: 4,
    goal: 12,
    types: &[
        DieT {
            name: "ucenic",
            cost: 0,
            stock: 0,
            faces: [
                Fc::Nrg(1),
                Fc::Nrg(1),
                Fc::Nrg(1),
                Fc::Nrg(2),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "ostaș",
            cost: 0,
            stock: 0,
            faces: [
                Fc::Unit(1, 1),
                Fc::Unit(1, 1),
                Fc::Unit(2, 1),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "talisman",
            cost: 2,
            stock: 6,
            faces: [
                Fc::Nrg(2),
                Fc::Nrg(2),
                Fc::Nrg(2),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "călăreț",
            cost: 3,
            stock: 6,
            faces: [
                Fc::Unit(3, 1),
                Fc::Unit(3, 1),
                Fc::Unit(2, 2),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "paznic",
            cost: 4,
            stock: 6,
            faces: [
                Fc::Unit(2, 4),
                Fc::Unit(2, 4),
                Fc::Unit(3, 3),
                Fc::Unit(3, 3),
                Fc::Nrg(1),
                Fc::Blank,
            ],
        },
        DieT {
            name: "dragon",
            cost: 7,
            stock: 4,
            faces: [
                Fc::Unit(5, 3),
                Fc::Unit(5, 3),
                Fc::Unit(4, 4),
                Fc::Unit(6, 2),
                Fc::Unit(4, 4),
                Fc::Blank,
            ],
        },
    ],
    start: [6, 3],
    cap: 800,
};

pub fn new() -> Bag {
    Bag::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Bag::with_seed(&SPEC, s), 860);
    }
}
