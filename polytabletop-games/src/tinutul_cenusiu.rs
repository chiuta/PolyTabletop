//! Ținutul Cenușiu — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Păstorii Colinei contra Oamenii Mlaștinii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 8). Fiecare are 3 zone cu 10 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Coloane: unitatea k atacă doar unitatea k adversă sau direct dacă aceasta nu există (zburătorii aleg liber). Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "tinutul_cenusiu",
    title: "Ținutul Cenușiu",
    fac: ["Păstorii Colinei", "Oamenii Mlaștinii"],
    src: "ogor",
    units: &[
        Unit {
            name: "Hoinar cu Toiag",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Mistreț de Pădure",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Fierar de Sat",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Spiriduș de Fântână",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Pădurar Cenușiu",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Căprioară Albă",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Vrăjitoare de Mlaștină",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Rege al Ogoarelor",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Uriaș de Dealuri",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Ursul Înnoptat",
            cost: 7,
            atk: 7,
            hp: 7,
            kw: ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Sămânță Arsă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Ploaie Neagră",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Mână de Pământ",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Cântec de Holdă",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Furtuna Roșie",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 2),
            (5, 3),
            (6, 4),
            (7, 1),
            (9, 3),
            (11, 2),
            (12, 3),
        ],
        &[
            (0, 1),
            (1, 1),
            (2, 3),
            (3, 1),
            (4, 1),
            (5, 3),
            (6, 2),
            (7, 1),
            (8, 3),
            (9, 3),
            (10, 1),
            (11, 1),
            (12, 3),
            (13, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Zones(10),
    face: Face::Cols,
    hand0: 6,
    cap: 8,
    board_max: 5,
    lands: 11,
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
