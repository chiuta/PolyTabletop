//! Cristale de Aură — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Cavalerii Albaștri contra Cavalerii Roșii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 7), dar pierzi cartea. Fiecare are 5 scuturi: lovitura directă reușită rupe 1 scut (2 dacă atacul e cel puțin 4) și îl dă în mână adversarului; lovitura fără scuturi câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cristale_de_aura",
    title: "Cristale de Aură",
    fac: ["Cavalerii Albaștri", "Cavalerii Roșii"],
    src: "cristal",
    units: &[
        Unit {
            name: "Mic Cavaler",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Tânăr Zburător",
            cost: 1,
            atk: 3,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Statuie Vie",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: HASTE,
            val: 1,
        },
        Unit {
            name: "Ghepard",
            cost: 2,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Luptător în Armură",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Hoinar de Cristal",
            cost: 3,
            atk: 5,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Șoim Strălucitor",
            cost: 4,
            atk: 6,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Păzitor de Stâncă",
            cost: 5,
            atk: 7,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Titan Lucid",
            cost: 5,
            atk: 8,
            hp: 4,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Marele Cristal",
            cost: 6,
            atk: 8,
            hp: 4,
            kw: FLY | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Aură Albastră",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Aură Roșie",
            cost: 4,
            fx: Fx::Buff(2),
        },
        Spell {
            name: "Fisură",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Coroana",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Spărtura",
            cost: 4,
            fx: Fx::Sweep(1),
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 2),
            (2, 3),
            (3, 1),
            (4, 4),
            (5, 2),
            (6, 3),
            (8, 2),
            (9, 4),
            (11, 4),
            (12, 4),
            (14, 1),
        ],
        &[
            (0, 3),
            (1, 4),
            (2, 2),
            (3, 1),
            (4, 2),
            (5, 1),
            (6, 4),
            (7, 3),
            (8, 4),
            (9, 4),
            (11, 2),
            (12, 2),
            (13, 2),
        ],
    ],
    res: Res::Ink,
    win: Win::Shield(5),
    face: Face::Free,
    hand0: 5,
    cap: 7,
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
