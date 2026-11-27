//! Fluturi de Prins — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: fluturi zburați de un elefant jucăuș. Ai patru încercări pe tură cu plasă mică (65%, 1 fluture), mare (45%, 2) sau dublă (30%, 4); orice ratare încheie tura, iar seria de reușite crește precizia cu 8%. Primul la 18 fluturi câștigă.
use crate::c5_shot::*;

pub static SPEC: ShotSpec = ShotSpec {
    title: "Fluturi de Prins",
    acts: &[
        Act {
            label: "plasă mică",
            p: 65,
            gain: 1,
            fail: 0,
            item: -1,
        },
        Act {
            label: "plasă mare",
            p: 45,
            gain: 2,
            fail: 0,
            item: -1,
        },
        Act {
            label: "plasă dublă",
            p: 30,
            gain: 4,
            fail: 0,
            item: -1,
        },
    ],
    shots: 4,
    goal: 18,
    collect: 0,
    risk: false,
    streak: 8,
    fail_ends: true,
    cap: 500,
    item_names: &[],
};

pub fn new() -> Shot {
    Shot::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Shot::with_seed(&SPEC, s), 560);
    }
}
