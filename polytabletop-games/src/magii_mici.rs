//! Magii Mici — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Elevii Pădurii contra Elevii Stâncilor), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 7), dar pierzi cartea. Câștigi când viața adversarului ajunge la 0 (start 18). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "magii_mici",
    title: "Magii Mici",
    fac: ["Elevii Pădurii", "Elevii Stâncilor"],
    src: "esență",
    units: &[
        Unit {
            name: "Gâză Amicală",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Peștișor Vrăjit",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pui de Dragon",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Broscuță Cântătoare",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Mag Pitic",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Pisică Fermecată",
            cost: 4,
            atk: 5,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Urs Blând",
            cost: 4,
            atk: 5,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Ghioc Mare",
            cost: 5,
            atk: 6,
            hp: 7,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Uriaș de Gheață",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Magul Cel Mare",
            cost: 7,
            atk: 7,
            hp: 8,
            kw: FLY | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Strop de Magie",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Cântec Adormitor",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Poțiune",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Bulgăre Zburător",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Bătaie de Aripi",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 3),
            (5, 3),
            (7, 1),
            (8, 1),
            (9, 1),
            (11, 1),
            (12, 3),
            (13, 2),
            (14, 2),
        ],
        &[
            (0, 4),
            (1, 3),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 3),
            (6, 1),
            (7, 2),
            (8, 4),
            (9, 3),
            (12, 2),
            (13, 1),
            (14, 2),
        ],
    ],
    res: Res::Ink,
    win: Win::Life(18),
    face: Face::Free,
    hand0: 5,
    cap: 7,
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
