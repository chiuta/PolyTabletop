//! Lupta pe Etape — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Clanul Vulturului contra Clanul Taurului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 9). Câștigi când viața adversarului ajunge la 0 (start 28). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Coloane: unitatea k atacă doar unitatea k adversă sau direct dacă aceasta nu există (zburătorii aleg liber). Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "lupta_pe_etape",
    title: "Lupta pe Etape",
    fac: ["Clanul Vulturului", "Clanul Taurului"],
    src: "impuls",
    units: &[
        Unit {
            name: "Boxer Ușor",
            cost: 1,
            atk: 3,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Sprinter",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Luptător cu Frânghie",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Judoka",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Maestru de Lanț",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Sumo Greu",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Campion Local",
            cost: 4,
            atk: 6,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Arbitru Orb",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Fiara Ringului",
            cost: 6,
            atk: 8,
            hp: 6,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Marele Campion",
            cost: 7,
            atk: 9,
            hp: 6,
            kw: GUARD | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Prindere",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Salt la Margine",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Proiecție",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Tămăduire",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Aruncare",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 3),
            (2, 3),
            (3, 2),
            (4, 3),
            (5, 2),
            (6, 1),
            (7, 1),
            (8, 2),
            (9, 3),
            (10, 2),
            (11, 1),
            (12, 2),
        ],
        &[
            (0, 2),
            (2, 3),
            (3, 3),
            (4, 1),
            (5, 3),
            (6, 2),
            (7, 2),
            (9, 4),
            (10, 2),
            (11, 2),
            (12, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Life(28),
    face: Face::Cols,
    hand0: 5,
    cap: 9,
    board_max: 5,
    lands: 11,
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
