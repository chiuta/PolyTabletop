//! Școala de Vrăji — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Casa Șoimului contra Casa Bursucului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 7). Câștigi când viața adversarului ajunge la 0 (start 20). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "scoala_de_vraji",
    title: "Școala de Vrăji",
    fac: ["Casa Șoimului", "Casa Bursucului"],
    src: "lecție",
    units: &[
        Unit {
            name: "Elev Zăpăcit",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Bufnița Școlii",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pisica Pădurarului",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Profesorul de Poțiuni",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Căpitanul Casei",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Spiriduș din Cămară",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Dirijor Fantomă",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Mag-Inspector",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Dragon din Grădină",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Directorul",
            cost: 7,
            atk: 7,
            hp: 7,
            kw: ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Descântec Simplu",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Test Neașteptat",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Poțiune de Vindecare",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Scară Fugară",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Fulger de Baghetă",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 2),
            (2, 4),
            (3, 2),
            (4, 2),
            (6, 2),
            (8, 2),
            (9, 2),
            (12, 3),
            (13, 3),
            (14, 3),
        ],
        &[
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 3),
            (4, 1),
            (5, 1),
            (6, 4),
            (7, 2),
            (8, 3),
            (9, 4),
            (12, 3),
            (14, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Life(20),
    face: Face::Free,
    hand0: 5,
    cap: 7,
    board_max: 5,
    lands: 11,
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
