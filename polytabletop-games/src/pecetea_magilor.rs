//! Pecetea Magilor — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Cercul Verde contra Cercul Roșu), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 9). Câștigi când viața adversarului ajunge la 0 (start 20). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "pecetea_magilor",
    title: "Pecetea Magilor",
    fac: ["Cercul Verde", "Cercul Roșu"],
    src: "rună",
    units: &[
        Unit {
            name: "Elev cu Rună",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Șoimul Vrăjit",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Golem de Lut",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Lunatecul",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Mag de Frontieră",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Bufniță Sfătoasă",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Cavalerul Pecetei",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: VENOM,
            val: 3,
        },
        Unit {
            name: "Vrăjitor de Lanț",
            cost: 5,
            atk: 6,
            hp: 7,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Titan de Piatră",
            cost: 6,
            atk: 7,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Arhimagul",
            cost: 7,
            atk: 6,
            hp: 7,
            kw: GUARD | VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Rună de Foc",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Rună de Apă",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Rună de Piatră",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Rună de Vânt",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Pecete Sfărâmată",
            cost: 4,
            fx: Fx::Sweep(1),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 2),
            (2, 3),
            (3, 3),
            (4, 4),
            (5, 2),
            (6, 1),
            (7, 2),
            (9, 1),
            (10, 1),
            (11, 3),
            (13, 2),
        ],
        &[
            (0, 4),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 3),
            (7, 3),
            (8, 1),
            (9, 3),
            (12, 3),
            (13, 2),
            (14, 3),
        ],
    ],
    res: Res::Land,
    win: Win::Life(20),
    face: Face::Free,
    hand0: 5,
    cap: 9,
    board_max: 5,
    lands: 11,
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
