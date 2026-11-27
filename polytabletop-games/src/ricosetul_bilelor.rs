//! Ricoșeul Bilelor — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală de îndemânare transformată în șansă controlată: ai trei lovituri pe tură (tir a, apoi lansez). Lovitură scurtă (70%, 1 punct), ricoșeu pe o bandă (45%, 2) sau pe două benzi (25%, 4, ratarea costă 1); fiecare reușită consecutivă adaugă 5% precizie. Primul la 14 puncte câștigă.
use crate::c5_shot::*;

pub static SPEC: ShotSpec = ShotSpec {
    title: "Ricoșeul Bilelor",
    acts: &[
        Act {
            label: "lovitură scurtă",
            p: 70,
            gain: 1,
            fail: 0,
            item: -1,
        },
        Act {
            label: "ricoșeu pe o bandă",
            p: 45,
            gain: 2,
            fail: 0,
            item: -1,
        },
        Act {
            label: "ricoșeu pe două benzi",
            p: 25,
            gain: 4,
            fail: -1,
            item: -1,
        },
    ],
    shots: 3,
    goal: 14,
    collect: 0,
    risk: false,
    streak: 5,
    fail_ends: false,
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
