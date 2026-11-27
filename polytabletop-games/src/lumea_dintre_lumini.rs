//! Lumea dintre Lumini — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Ordinul Zorilor contra Ordinul Amurgului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Fiecare are 3 zone cu 8 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "lumea_dintre_lumini",
    title: "Lumea dintre Lumini",
    fac: ["Ordinul Zorilor", "Ordinul Amurgului"],
    src: "har",
    units: &[
        Unit {
            name: "Pelerin cu Lanternă",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Gardian de Prag",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Cavaler de Lumină",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler de Umbră",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: FIRST,
            val: 1,
        },
        Unit {
            name: "Oracol Orb",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Căutătoarea",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Arhonte Palid",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Păsăre de Foc",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Colos Strălucitor",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Marele Echilibru",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: FLY | FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Rază Divină",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Val de Umbră",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Echilibru",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Pelerinaj",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Cădere de Stea",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 2),
            (2, 4),
            (3, 4),
            (4, 2),
            (5, 3),
            (6, 2),
            (7, 1),
            (8, 2),
            (9, 1),
            (10, 2),
            (11, 1),
            (12, 2),
        ],
        &[
            (0, 1),
            (1, 1),
            (2, 3),
            (3, 1),
            (4, 2),
            (5, 4),
            (6, 1),
            (7, 2),
            (8, 2),
            (9, 1),
            (10, 2),
            (11, 2),
            (13, 4),
            (14, 3),
        ],
    ],
    res: Res::Auto,
    win: Win::Zones(8),
    face: Face::Free,
    hand0: 5,
    cap: 9,
    board_max: 6,
    lands: 0,
    big: 4,
    max_turns: 16,
    twist: T_RUSH,
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
