//! Drum peste Munți — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Tovarășii contra Umbrele Drumului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Câștigi la 9 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "drum_peste_munti",
    title: "Drum peste Munți",
    fac: ["Tovarășii", "Umbrele Drumului"],
    src: "merinde",
    units: &[
        Unit {
            name: "Călăuza",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Mic Ajutor",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Arcaș de Potecă",
            cost: 2,
            atk: 2,
            hp: 4,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Piticul Cărăuș",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: ARMOR,
            val: 1,
        },
        Unit {
            name: "Șoimul Cercetaș",
            cost: 3,
            atk: 3,
            hp: 5,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Vrăjitorul Bătrân",
            cost: 3,
            atk: 3,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Războinic din Nord",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Elf de Frunziș",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Arborele Mers",
            cost: 5,
            atk: 5,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Căpitanul Companiei",
            cost: 6,
            atk: 6,
            hp: 8,
            kw: FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Popas la Foc",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Cărare Ascunsă",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Vânt Potrivnic",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Hrană de Drum",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Piatra Căzută",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 4),
            (2, 3),
            (3, 3),
            (4, 2),
            (5, 2),
            (7, 1),
            (8, 2),
            (10, 2),
            (12, 1),
            (13, 3),
            (14, 3),
        ],
        &[
            (0, 4),
            (1, 1),
            (2, 3),
            (3, 1),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 2),
            (9, 2),
            (11, 1),
            (12, 2),
            (13, 2),
            (14, 3),
        ],
    ],
    res: Res::Ink,
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
