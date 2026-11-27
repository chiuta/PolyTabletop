//! Creaturile Necunoscute — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Vânătorii de Urme contra Păzitorii Pădurii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 7). Câștigi când viața adversarului ajunge la 0 (start 20). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Coloane: unitatea k atacă doar unitatea k adversă sau direct dacă aceasta nu există (zburătorii aleg liber). Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "creaturi_necunoscute",
    title: "Creaturile Necunoscute",
    fac: ["Vânătorii de Urme", "Păzitorii Pădurii"],
    src: "amprentă",
    units: &[
        Unit {
            name: "Iepure cu Coarne",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Bufnița Tăcută",
            cost: 2,
            atk: 2,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Omul-Molie",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Lup Pitic",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Vârcolac Tânăr",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mama Pădurii",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: VENOM,
            val: 3,
        },
        Unit {
            name: "Zmeu Mic",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Ursul Fantomă",
            cost: 5,
            atk: 5,
            hp: 7,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Hoinarul din Ceață",
            cost: 6,
            atk: 6,
            hp: 8,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Marele Necunoscut",
            cost: 7,
            atk: 5,
            hp: 8,
            kw: GUARD | VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Urmă Proaspătă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Tabără de Noapte",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Capcană",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Foc de Tabără",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Lumină în Noapte",
            cost: 2,
            fx: Fx::Mend(3),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 1),
            (7, 3),
            (8, 3),
            (9, 2),
            (12, 3),
            (13, 2),
            (14, 2),
        ],
        &[
            (0, 2),
            (1, 1),
            (3, 3),
            (4, 4),
            (5, 4),
            (6, 1),
            (7, 2),
            (8, 4),
            (10, 1),
            (11, 2),
            (13, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Life(20),
    face: Face::Cols,
    hand0: 6,
    cap: 7,
    board_max: 5,
    lands: 12,
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
