//! Ținuturi și Vrăji — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Regatul Gloriei contra Regatul Cenușii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 12 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 9). Fiecare are 3 zone cu 9 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "tinuturi_si_vraji",
    title: "Ținuturi și Vrăji",
    fac: ["Regatul Gloriei", "Regatul Cenușii"],
    src: "inel",
    units: &[
        Unit {
            name: "Pajul",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Țăranul Viteaz",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Lăncier de Câmp",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Călăreț Ușor",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Preot de Ținut",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Mag de Curte",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Cavaler de Fier",
            cost: 4,
            atk: 5,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Hidra Mică",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Marele Gryf",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Zmeul Regal",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Foc Alb",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Adunarea Oștii",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Blestem",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Binecuvântare",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Furtună de Mare",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 3),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 3),
            (7, 1),
            (8, 2),
            (9, 3),
            (10, 2),
            (11, 1),
            (13, 1),
            (14, 1),
        ],
        &[
            (0, 1),
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 2),
            (5, 4),
            (6, 1),
            (7, 3),
            (8, 2),
            (9, 2),
            (11, 2),
            (12, 2),
            (13, 1),
        ],
    ],
    res: Res::Land,
    win: Win::Zones(9),
    face: Face::Free,
    hand0: 6,
    cap: 9,
    board_max: 6,
    lands: 12,
    big: 4,
    max_turns: 16,
    twist: T_TROPHY,
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
