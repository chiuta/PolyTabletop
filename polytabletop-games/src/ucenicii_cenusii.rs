//! Ucenicii Cenușii — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Ucenicii Focului contra Ucenicii Apei), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Câștigi când viața adversarului ajunge la 0 (start 15). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "ucenicii_cenusii",
    title: "Ucenicii Cenușii",
    fac: ["Ucenicii Focului", "Ucenicii Apei"],
    src: "zar",
    units: &[
        Unit {
            name: "Ucenic Zgâlțâit",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Fantomă de Cenușă",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Câine de Foc",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Mag de Cuptor",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Cerb de Flăcări",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Veveriță de Jar",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Fântânar",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Doamna Cenușii",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Titan de Zgură",
            cost: 5,
            atk: 7,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Păsărea Cenușii",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Zar Aruncat",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Vânt de Seară",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Scufundare",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Scrum",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Jar Reaprins",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 1),
            (2, 2),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 3),
            (7, 2),
            (8, 2),
            (9, 2),
            (10, 3),
            (11, 2),
            (13, 3),
            (14, 2),
        ],
        &[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 2),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 1),
            (8, 1),
            (9, 3),
            (10, 3),
            (12, 2),
            (14, 2),
        ],
    ],
    res: Res::Ink,
    win: Win::Life(15),
    face: Face::Free,
    hand0: 5,
    cap: 8,
    board_max: 5,
    lands: 0,
    big: 4,
    max_turns: 16,
    twist: T_BLEED,
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
