//! Orășelul Prăfuit — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Banda Pustiei contra Legea Orașului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 9 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "orasul_prafuit",
    title: "Orășelul Prăfuit",
    fac: ["Banda Pustiei", "Legea Orașului"],
    src: "dolar",
    units: &[
        Unit {
            name: "Cowboy Fugar",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Vânzător de Remedii",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Șerif Adjunct",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Pistolar de Ocazie",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: FLY,
            val: 1,
        },
        Unit {
            name: "Fierarul",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Sătean cu Furcă",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Nelegiuit",
            cost: 4,
            atk: 6,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Șerifului Șef",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: HASTE,
            val: 3,
        },
        Unit {
            name: "Banditul Roșu",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Marele Pistolar",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Duel la Răscruce",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Poker de Seară",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Vagon Blindat",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Doctorul",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Jaf la Bancă",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 3),
            (2, 4),
            (3, 3),
            (4, 2),
            (5, 3),
            (6, 2),
            (7, 3),
            (8, 2),
            (11, 2),
            (13, 1),
            (14, 2),
        ],
        &[
            (0, 2),
            (1, 3),
            (2, 3),
            (4, 2),
            (5, 1),
            (6, 3),
            (7, 4),
            (8, 4),
            (9, 3),
            (10, 2),
            (11, 1),
            (12, 1),
            (13, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(9),
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
