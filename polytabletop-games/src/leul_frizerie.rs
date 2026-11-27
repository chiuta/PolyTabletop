//! Leul la Frizerie — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: tunzi coama leului. Două încercări pe tură (tir a): tuns fin (85%, 1 punct), tunsoare medie (60%, 2, ratare −1) sau freză îndrăzneață (35%, 4, ratare −2). Primul la 15 puncte câștigă.
use crate::c5_shot::*;

pub static SPEC: ShotSpec = ShotSpec {
    title: "Leul la Frizerie",
    acts: &[
        Act {
            label: "tuns fin",
            p: 85,
            gain: 1,
            fail: 0,
            item: -1,
        },
        Act {
            label: "tunsoare medie",
            p: 60,
            gain: 2,
            fail: -1,
            item: -1,
        },
        Act {
            label: "freză îndrăzneață",
            p: 35,
            gain: 4,
            fail: -2,
            item: -1,
        },
    ],
    shots: 2,
    goal: 15,
    collect: 0,
    risk: false,
    streak: 0,
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
