//! Luptători cu Aură — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Frații Muntelui contra Oștenii Vântului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Câștigi când viața adversarului ajunge la 0 (start 25). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "luptatori_cu_aura",
    title: "Luptători cu Aură",
    fac: ["Frații Muntelui", "Oștenii Vântului"],
    src: "aură",
    units: &[
        Unit {
            name: "Elev Zburdalnic",
            cost: 1,
            atk: 4,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Boxer de Port",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Călugăr Sprinten",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: ARMOR,
            val: 1,
        },
        Unit {
            name: "Mercenar Umbros",
            cost: 2,
            atk: 5,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Luptătoare cu Panglică",
            cost: 3,
            atk: 6,
            hp: 2,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Maestru cu Toiag",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Titan de Fier",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Monstru de Piatră",
            cost: 5,
            atk: 9,
            hp: 3,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Căpetenie Verde",
            cost: 5,
            atk: 7,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Marele Războinic",
            cost: 6,
            atk: 8,
            hp: 4,
            kw: FIRST | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Val de Aură",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Explozie de Energie",
            cost: 4,
            fx: Fx::Buff(2),
        },
        Spell {
            name: "Antrenament Dur",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Stâncă în Cale",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Ultima Suflare",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 3),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 2),
            (8, 3),
            (9, 1),
            (10, 3),
            (11, 3),
            (13, 3),
            (14, 2),
        ],
        &[
            (1, 3),
            (2, 2),
            (3, 3),
            (4, 1),
            (5, 4),
            (6, 4),
            (7, 1),
            (8, 2),
            (9, 3),
            (11, 3),
            (13, 2),
            (14, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Life(25),
    face: Face::Free,
    hand0: 6,
    cap: 9,
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
