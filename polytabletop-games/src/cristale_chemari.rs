//! Cristale și Chemări — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Gărzile Soarelui contra Magii Cenușii), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Câștigi când viața adversarului ajunge la 0 (start 18). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cristale_chemari",
    title: "Cristale și Chemări",
    fac: ["Gărzile Soarelui", "Magii Cenușii"],
    src: "cristal",
    units: &[
        Unit {
            name: "Paznic de Lumină",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Ucenic Alchimist",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Lancier de Cristal",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Arcașă din Fulger",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Chimera Albastră",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler de Argint",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Vrăjitoare de Gheață",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Dragon de Jad",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Spirit al Furtunii",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Titanul de Cristal",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: GUARD | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Val de Cristal",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Chemarea Veche",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Fulger Alb",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Cântec de Vindecare",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Mantia Luminii",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 4),
            (2, 2),
            (3, 3),
            (4, 3),
            (5, 3),
            (7, 1),
            (8, 3),
            (9, 2),
            (11, 2),
            (12, 3),
            (13, 3),
        ],
        &[
            (0, 2),
            (2, 1),
            (3, 3),
            (4, 3),
            (5, 4),
            (6, 2),
            (7, 2),
            (8, 3),
            (9, 3),
            (10, 3),
            (11, 1),
            (12, 3),
        ],
    ],
    res: Res::Ink,
    win: Win::Life(18),
    face: Face::Free,
    hand0: 6,
    cap: 8,
    board_max: 5,
    lands: 0,
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
