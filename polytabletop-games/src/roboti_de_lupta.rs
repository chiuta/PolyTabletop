//! Roboții de Luptă — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Lancea de Oțel contra Legiunea Gri), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 8). Câștigi când viața adversarului ajunge la 0 (start 24). Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "roboti_de_lupta",
    title: "Roboții de Luptă",
    fac: ["Lancea de Oțel", "Legiunea Gri"],
    src: "reactor",
    units: &[
        Unit {
            name: "Dronă Grea",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mech Mărunt",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mech de Recunoaștere",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Mech cu Scut",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Mech de Asalt",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mech de Sprijin",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Mech cu Laser",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Tanc-Mech",
            cost: 5,
            atk: 7,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Mech de Comandă",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: HASTE,
            val: 3,
        },
        Unit {
            name: "Mech Titan",
            cost: 7,
            atk: 9,
            hp: 6,
            kw: VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Salvă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Reparații",
            cost: 6,
            fx: Fx::Sweep(2),
        },
        Spell {
            name: "Camuflaj",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Jump Jet",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Sabotaj",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 4),
            (5, 1),
            (6, 4),
            (7, 3),
            (8, 1),
            (9, 1),
            (11, 1),
            (12, 1),
            (13, 3),
        ],
        &[
            (0, 1),
            (1, 2),
            (2, 2),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (8, 4),
            (9, 4),
            (10, 1),
            (12, 2),
            (13, 4),
            (14, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Life(24),
    face: Face::Free,
    hand0: 5,
    cap: 8,
    board_max: 6,
    lands: 11,
    big: 4,
    max_turns: 16,
    twist: T_RUSH,
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
