//! Zero contra Infinit — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Legiunea Zero contra Nucleul Infinit), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 10 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 8). Câștigi când viața adversarului ajunge la 0 (start 18). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "zero_contra_infinit",
    title: "Zero contra Infinit",
    fac: ["Legiunea Zero", "Nucleul Infinit"],
    src: "nucleu",
    units: &[
        Unit {
            name: "Dronă Mică",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Fată cu Bază",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Ghidul Roșu",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Lunetist Zero",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Lupul Binar",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Tanc Infinit",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Mecanică Veche",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Cavaler Cuantic",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Arhanghel Roșu",
            cost: 6,
            atk: 8,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Zeul din Mașină",
            cost: 7,
            atk: 8,
            hp: 6,
            kw: ARMOR | VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Reset",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Salt de Nucleu",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Colaps",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Reîncărcare",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Blocare de Sistem",
            cost: 4,
            fx: Fx::Sweep(1),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 1),
            (2, 4),
            (3, 1),
            (4, 2),
            (5, 3),
            (6, 1),
            (7, 3),
            (8, 1),
            (9, 2),
            (10, 3),
            (12, 1),
            (14, 2),
        ],
        &[
            (0, 1),
            (1, 2),
            (2, 2),
            (3, 1),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 2),
            (9, 2),
            (10, 2),
            (11, 1),
            (13, 1),
            (14, 1),
        ],
    ],
    res: Res::Land,
    win: Win::Life(18),
    face: Face::Free,
    hand0: 5,
    cap: 8,
    board_max: 5,
    lands: 10,
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
