//! Vârful Fumegând — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Compania Drumului contra Slujitorii Umbrei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 9 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "varful_fumegand",
    title: "Vârful Fumegând",
    fac: ["Compania Drumului", "Slujitorii Umbrei"],
    src: "merinde",
    units: &[
        Unit {
            name: "Pitic Bătăios",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Arcaș Elf",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Pădurar",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Hoinarul",
            cost: 2,
            atk: 2,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Piticul cu Topor",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Pitic de Ceață",
            cost: 4,
            atk: 4,
            hp: 6,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Elf de Umbră",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Vrăjitor Gri",
            cost: 5,
            atk: 4,
            hp: 7,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Uriaș de Stâncă",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Căpitanul Companiei",
            cost: 6,
            atk: 5,
            hp: 9,
            kw: FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Popas",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Frunziș Magic",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Foc Nestins",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Strigăt în Noapte",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Ocolișul",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 2),
            (5, 1),
            (7, 1),
            (8, 3),
            (9, 1),
            (10, 2),
            (11, 1),
            (12, 3),
            (13, 3),
            (14, 2),
        ],
        &[
            (0, 1),
            (1, 2),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 4),
            (8, 3),
            (9, 1),
            (10, 2),
            (11, 1),
            (12, 2),
            (13, 4),
            (14, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(9),
    face: Face::Free,
    hand0: 6,
    cap: 8,
    board_max: 5,
    lands: 0,
    big: 4,
    max_turns: 16,
    twist: T_REGEN,
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
