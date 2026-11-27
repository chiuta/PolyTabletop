//! Spărtura dintre Lumi — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Cuțitele Verii contra Cuțitele Iernii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Fiecare are 3 zone cu 9 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "spartura_dintre_lumi",
    title: "Spărtura dintre Lumi",
    fac: ["Cuțitele Verii", "Cuțitele Iernii"],
    src: "ecou",
    units: &[
        Unit {
            name: "Rătăcitor de Praguri",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Gheară de Vid",
            cost: 2,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Șoimul Fisurii",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Copil al Nopții",
            cost: 2,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler al Marginii",
            cost: 3,
            atk: 5,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Ghidul Fisurii",
            cost: 4,
            atk: 5,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Strigoi de Coloană",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Monolit",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Ochiul Fisurii",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Stăpânul Spărturii",
            cost: 6,
            atk: 7,
            hp: 7,
            kw: FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Spărtura se Lărgește",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Poartă Întoarsă",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Ecou",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Cădere",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Cheag de Lumină",
            cost: 4,
            fx: Fx::Slay,
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 3),
            (2, 3),
            (3, 1),
            (4, 3),
            (5, 3),
            (7, 1),
            (8, 3),
            (9, 3),
            (10, 4),
            (13, 1),
        ],
        &[
            (0, 1),
            (2, 4),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 4),
            (8, 3),
            (9, 3),
            (10, 3),
            (12, 1),
            (13, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Zones(9),
    face: Face::Free,
    hand0: 5,
    cap: 9,
    board_max: 5,
    lands: 0,
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
