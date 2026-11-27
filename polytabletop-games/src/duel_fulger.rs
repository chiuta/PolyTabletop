//! Duel Fulger — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Duelistul Alb contra Duelistul Negru), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 12 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 6. Câștigi când viața adversarului ajunge la 0 (start 20). Până la 4 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "duel_fulger",
    title: "Duel Fulger",
    fac: ["Duelistul Alb", "Duelistul Negru"],
    src: "puls",
    units: &[
        Unit {
            name: "Monstru Scurt",
            cost: 1,
            atk: 4,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Pumnul Roșu",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Vânător Iute",
            cost: 2,
            atk: 5,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Trompetist",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: FLY,
            val: 1,
        },
        Unit {
            name: "Cavaler Iute",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Sabie Iute",
            cost: 4,
            atk: 7,
            hp: 2,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Pasăre-Fulger",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Fiara din Jos",
            cost: 5,
            atk: 8,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Dragonul Iute",
            cost: 5,
            atk: 9,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Marele Fulger",
            cost: 6,
            atk: 10,
            hp: 3,
            kw: FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Trăsnet",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Capcana",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Scut Rapid",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Pescuit",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Salt Scurt",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 1),
            (2, 4),
            (3, 2),
            (4, 4),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (11, 3),
            (13, 2),
        ],
        &[
            (0, 3),
            (1, 2),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 4),
            (7, 1),
            (8, 3),
            (9, 2),
            (10, 2),
            (13, 3),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Life(20),
    face: Face::Free,
    hand0: 5,
    cap: 6,
    board_max: 4,
    lands: 0,
    big: 4,
    max_turns: 12,
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
