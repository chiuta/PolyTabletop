//! Provinciile de Jad — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Dragonii de Jad contra Corbii de Ceață), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Fiecare are 3 zone cu 9 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "provincii_de_jad",
    title: "Provinciile de Jad",
    fac: ["Dragonii de Jad", "Corbii de Ceață"],
    src: "orez",
    units: &[
        Unit {
            name: "Samurai de Pază",
            cost: 1,
            atk: 3,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Călugăr de Munte",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Arcaș de Crâng",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Duelist de Stuf",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Ronin de Drum",
            cost: 3,
            atk: 5,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Geisha cu Evantai",
            cost: 4,
            atk: 7,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Maestru de Ceai",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Căpitan de Cohortă",
            cost: 5,
            atk: 8,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Demon din Mlaștină",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Regent al Provinciei",
            cost: 6,
            atk: 8,
            hp: 4,
            kw: GUARD | FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Poartă Deschisă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Cântec de Ceai",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Sabie Scăpărată",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Pecete Răsucită",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Ploaie de Săgeți",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 1),
            (2, 4),
            (3, 3),
            (4, 2),
            (5, 3),
            (7, 1),
            (8, 2),
            (10, 3),
            (11, 1),
            (13, 3),
            (14, 4),
        ],
        &[
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 4),
            (6, 3),
            (7, 3),
            (8, 4),
            (9, 2),
            (10, 2),
            (11, 1),
            (13, 1),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Zones(9),
    face: Face::Free,
    hand0: 5,
    cap: 8,
    board_max: 5,
    lands: 0,
    big: 4,
    max_turns: 16,
    twist: 0,
};

pub fn new() -> Game5 {
    Game5::new(&SPEC, crate::brand_util::seed_now())
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays_and_ai_is_legal() {
        crate::brand_util::check(|s| Game5::new(&SPEC, s), 4000);
    }
}
