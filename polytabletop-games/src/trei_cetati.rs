//! Cele Trei Cetăți — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Oastea Nordului contra Hoardele Pădurii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 8). Fiecare are 3 zone cu 9 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "trei_cetati",
    title: "Cele Trei Cetăți",
    fac: ["Oastea Nordului", "Hoardele Pădurii"],
    src: "tribut",
    units: &[
        Unit {
            name: "Gornist",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Pedestraș",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Bărbat de Câmp",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Sapator de Șanț",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Arbaletier",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Ofițer de Pază",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Berbec de Asediu",
            cost: 4,
            atk: 5,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Cavalerie Grea",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Magul de Război",
            cost: 6,
            atk: 7,
            hp: 8,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Marele Căpitan",
            cost: 7,
            atk: 8,
            hp: 7,
            kw: GUARD | FLY,
            val: 4,
        },
    ],
    spells: &[
        Spell {
            name: "Foc de Catapulta",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Zid de Scuturi",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Reparații",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Înrolare",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Lovitură de Fier",
            cost: 2,
            fx: Fx::Mend(3),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 3),
            (2, 1),
            (3, 2),
            (4, 4),
            (5, 3),
            (7, 3),
            (9, 3),
            (10, 1),
            (12, 2),
            (14, 2),
        ],
        &[
            (1, 2),
            (2, 3),
            (3, 1),
            (4, 1),
            (5, 4),
            (6, 1),
            (7, 3),
            (8, 3),
            (9, 3),
            (10, 1),
            (13, 2),
            (14, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Zones(9),
    face: Face::Free,
    hand0: 6,
    cap: 8,
    board_max: 6,
    lands: 12,
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
