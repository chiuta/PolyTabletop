//! Catapulta cu Nuci — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: catapulta trage în coroana unui palmier și face să cadă nuci de patru culori. Alegi forța (tir a: ușoară 75%, medie 55%, puternică 35% care aduce două nuci); mutarea forțată lansez decide, iar nuca căzută are o culoare la întâmplare. Câștigă cine strânge primul două nuci din fiecare culoare.
use crate::c5_shot::*;

pub static SPEC: ShotSpec = ShotSpec {
    title: "Catapulta cu Nuci",
    acts: &[
        Act {
            label: "tragere ușoară",
            p: 75,
            gain: 1,
            fail: 0,
            item: -1,
        },
        Act {
            label: "tragere medie",
            p: 55,
            gain: 1,
            fail: 0,
            item: -1,
        },
        Act {
            label: "tragere puternică",
            p: 35,
            gain: 2,
            fail: 0,
            item: -1,
        },
    ],
    shots: 2,
    goal: 2,
    collect: 4,
    risk: false,
    streak: 0,
    fail_ends: false,
    cap: 500,
    item_names: &["roșii", "galbene", "verzi", "brune"],
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
