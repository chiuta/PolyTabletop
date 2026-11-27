//! Pumnul din Umbră — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Lotusul Negru contra Călugării Lunii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Fiecare are 3 zone cu 9 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Coloane: unitatea k atacă doar unitatea k adversă sau direct dacă aceasta nu există (zburătorii aleg liber). Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "pumnul_din_umbra",
    title: "Pumnul din Umbră",
    fac: ["Lotusul Negru", "Călugării Lunii"],
    src: "ki",
    units: &[
        Unit {
            name: "Ucenic de Dojo",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Ninja de Acoperiș",
            cost: 2,
            atk: 5,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pugilist de Port",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Spadasin Tăcut",
            cost: 3,
            atk: 6,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cobra Roșie",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Maestru Orb",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Tigrul de Hârtie",
            cost: 4,
            atk: 7,
            hp: 2,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Șef de Gang",
            cost: 5,
            atk: 8,
            hp: 4,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Dragon Orb",
            cost: 6,
            atk: 8,
            hp: 4,
            kw: HASTE,
            val: 3,
        },
        Unit {
            name: "Marele Maestru",
            cost: 7,
            atk: 10,
            hp: 4,
            kw: FIRST | VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Lovitură Șoptită",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Salt peste Acoperiș",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Respirația Lungă",
            cost: 4,
            fx: Fx::Buff(2),
        },
        Spell {
            name: "Dezarmare",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Trântă Dublă",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 4),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 2),
            (6, 1),
            (7, 4),
            (9, 3),
            (10, 3),
            (11, 2),
            (13, 1),
        ],
        &[
            (0, 1),
            (2, 4),
            (3, 1),
            (4, 3),
            (5, 3),
            (6, 1),
            (7, 2),
            (8, 1),
            (9, 4),
            (10, 2),
            (11, 1),
            (12, 3),
            (14, 3),
        ],
    ],
    res: Res::Auto,
    win: Win::Zones(9),
    face: Face::Cols,
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
