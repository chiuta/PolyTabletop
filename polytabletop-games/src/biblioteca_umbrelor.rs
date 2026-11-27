//! Biblioteca Umbrelor — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Arhivarii contra Cititorii de Noapte), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Câștigi când viața adversarului ajunge la 0 (start 20). Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "biblioteca_umbrelor",
    title: "Biblioteca Umbrelor",
    fac: ["Arhivarii", "Cititorii de Noapte"],
    src: "amintire",
    units: &[
        Unit {
            name: "Copist Grăbit",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pisică de Raft",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Bibliotecar Orb",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: ARMOR,
            val: 1,
        },
        Unit {
            name: "Cavaler-Paznic",
            cost: 2,
            atk: 2,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Fantoma Cititorului",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Gargui de Piatră",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Cărturar Rătăcit",
            cost: 4,
            atk: 5,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mag al Indexului",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: ARMOR,
            val: 3,
        },
        Unit {
            name: "Mamutul Arhivelor",
            cost: 5,
            atk: 4,
            hp: 8,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Marele Cronicar",
            cost: 6,
            atk: 6,
            hp: 8,
            kw: FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Pagină Smulsă",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Index Secret",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Praf de Veacuri",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Ex Libris",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Foc în Raft",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 3),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 1),
            (8, 3),
            (9, 2),
            (10, 1),
            (11, 2),
            (12, 3),
            (13, 1),
            (14, 4),
        ],
        &[
            (1, 3),
            (2, 2),
            (3, 1),
            (4, 1),
            (5, 3),
            (6, 3),
            (7, 2),
            (8, 2),
            (9, 1),
            (10, 1),
            (11, 3),
            (12, 2),
            (13, 3),
            (14, 2),
        ],
    ],
    res: Res::Ink,
    win: Win::Life(20),
    face: Face::Free,
    hand0: 5,
    cap: 8,
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
