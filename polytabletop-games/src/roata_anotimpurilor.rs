//! Roata Anotimpurilor — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Surorile Primăverii contra Frații Iernii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 10 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "roata_anotimpurilor",
    title: "Roata Anotimpurilor",
    fac: ["Surorile Primăverii", "Frații Iernii"],
    src: "fir",
    units: &[
        Unit {
            name: "Ucenică de Țesătoare",
            cost: 1,
            atk: 1,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Păstorul",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Hoinară de Vânt",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Căutătorul",
            cost: 2,
            atk: 2,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Sora Rătăcitoare",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Țesătoarea",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Străjer de Poartă",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Mag al Toamnei",
            cost: 4,
            atk: 4,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Stăpânul Gerului",
            cost: 5,
            atk: 5,
            hp: 7,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Cronicarul Vremii",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: GUARD | FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Fir Rupt",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Țesătura",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Furtuna de Iarnă",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Reînnoire",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Răsărit",
            cost: 4,
            fx: Fx::Slay,
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 4),
            (5, 1),
            (7, 2),
            (8, 1),
            (9, 2),
            (10, 3),
            (11, 1),
            (12, 1),
            (13, 4),
            (14, 2),
        ],
        &[
            (0, 1),
            (1, 2),
            (2, 4),
            (3, 2),
            (4, 1),
            (5, 4),
            (6, 2),
            (7, 2),
            (8, 3),
            (9, 2),
            (10, 1),
            (11, 1),
            (12, 2),
            (13, 2),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(10),
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
