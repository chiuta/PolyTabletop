//! Carierele cu Zaruri — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: fiecare are un sac de zaruri (6 novici și 2 străjeri). Tura începe cu trag (scoți cinci zaruri și le arunci); fețele de energie se adună, unitățile se așază pe teren (dep i, cel mult 3). Cu energia cumperi zaruri din rezerva publică (cumpar t); unitățile gata atacă (atac i j; j = 0 lovește direct dacă adversarul n-are unități) și câștigi glorie cât costă unitatea doborâtă (1 la lovitură directă). Primul la 12 glorie câștigă.
use crate::c5_bag::*;

pub static SPEC: BagSpec = BagSpec {
    title: "Carierele cu Zaruri",
    mode: BMode::Score,
    draw: 5,
    field: 3,
    goal: 12,
    types: &[
        DieT {
            name: "novice",
            cost: 0,
            stock: 0,
            faces: [
                Fc::Nrg(1),
                Fc::Nrg(1),
                Fc::Nrg(1),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "străjer",
            cost: 0,
            stock: 0,
            faces: [
                Fc::Unit(1, 1),
                Fc::Unit(1, 1),
                Fc::Unit(1, 1),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "cristal",
            cost: 2,
            stock: 5,
            faces: [
                Fc::Nrg(2),
                Fc::Nrg(2),
                Fc::Nrg(1),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "scutier",
            cost: 3,
            stock: 5,
            faces: [
                Fc::Unit(2, 2),
                Fc::Unit(2, 2),
                Fc::Unit(2, 1),
                Fc::Nrg(1),
                Fc::Blank,
                Fc::Blank,
            ],
        },
        DieT {
            name: "arcaș",
            cost: 4,
            stock: 5,
            faces: [
                Fc::Unit(3, 1),
                Fc::Unit(3, 1),
                Fc::Unit(2, 2),
                Fc::Unit(2, 2),
                Fc::Nrg(1),
                Fc::Blank,
            ],
        },
        DieT {
            name: "uriaș",
            cost: 6,
            stock: 4,
            faces: [
                Fc::Unit(4, 3),
                Fc::Unit(4, 3),
                Fc::Unit(3, 4),
                Fc::Unit(3, 4),
                Fc::Unit(5, 2),
                Fc::Blank,
            ],
        },
    ],
    start: [6, 2],
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
