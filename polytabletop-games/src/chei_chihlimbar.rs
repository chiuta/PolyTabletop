//! Trei Chei de Chihlimbar — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Casa Clopotului contra Casa Fierarilor), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 7. Câștigi la 8 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "chei_chihlimbar",
    title: "Trei Chei de Chihlimbar",
    fac: ["Casa Clopotului", "Casa Fierarilor"],
    src: "chihlimbar",
    units: &[
        Unit {
            name: "Pițigoi Ager",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pitic Hamal",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Bufon Șmecher",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Căprior Mândru",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Ursul din Hambar",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Lacătul Viu",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Gnom Contabil",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler de Tinichea",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Zână din Butoi",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Uriașul din Pivniță",
            cost: 7,
            atk: 7,
            hp: 7,
            kw: GUARD,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Furtișag",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Vânt prin Hambar",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Cheia Potrivită",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Leac de Pădure",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Gong de Fier",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 4),
            (2, 4),
            (3, 1),
            (4, 2),
            (5, 4),
            (6, 1),
            (7, 2),
            (8, 4),
            (9, 2),
            (11, 3),
            (13, 1),
            (14, 1),
        ],
        &[
            (0, 3),
            (1, 1),
            (2, 4),
            (3, 2),
            (4, 1),
            (5, 4),
            (6, 1),
            (7, 2),
            (8, 1),
            (9, 4),
            (12, 2),
            (13, 2),
            (14, 3),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(8),
    face: Face::Free,
    hand0: 6,
    cap: 7,
    board_max: 6,
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
