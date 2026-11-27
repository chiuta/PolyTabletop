//! Furnici Zburdalnice — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: bobârnaci de furnici spre un buzunar. Alegi distanța (tir a: scurtă 80% 1 punct, medie 55% 3, lungă 30% 6); punctele turei nu sunt salvate până nu spui stau, iar o ratare le pierde și încheie tura. Primul la 25 de puncte câștigă.
use crate::c5_shot::*;

pub static SPEC: ShotSpec = ShotSpec {
    title: "Furnici Zburdalnice",
    acts: &[
        Act {
            label: "bobârnac scurt",
            p: 80,
            gain: 1,
            fail: 0,
            item: -1,
        },
        Act {
            label: "bobârnac mediu",
            p: 55,
            gain: 3,
            fail: 0,
            item: -1,
        },
        Act {
            label: "bobârnac lung",
            p: 30,
            gain: 6,
            fail: 0,
            item: -1,
        },
    ],
    shots: 4,
    goal: 25,
    collect: 0,
    risk: true,
    streak: 0,
    fail_ends: false,
    cap: 600,
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
        crate::brand_util::check(|s| Shot::with_seed(&SPEC, s), 660);
    }
}
