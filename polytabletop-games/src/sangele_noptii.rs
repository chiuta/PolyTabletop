//! Sângele Nopții — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Neamul Corbului contra Neamul Lupului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 9 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "sangele_noptii",
    title: "Sângele Nopții",
    fac: ["Neamul Corbului", "Neamul Lupului"],
    src: "sânge",
    units: &[
        Unit {
            name: "Slujitor Palid",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Ghoul de Pivniță",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pază de Conac",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Dansatoare de Bal",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Șoapta din Culise",
            cost: 3,
            atk: 6,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Călău Elegant",
            cost: 3,
            atk: 5,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Contesa Tăcută",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Prinț al Străzii",
            cost: 5,
            atk: 7,
            hp: 5,
            kw: FIRST,
            val: 3,
        },
        Unit {
            name: "Străvechiul",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Patriarhul",
            cost: 6,
            atk: 9,
            hp: 5,
            kw: VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Mușcătură",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Mască de Bal",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Ceață Roșie",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Vot în Consiliu",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Sigiliu Negru",
            cost: 2,
            fx: Fx::Mend(3),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 2),
            (2, 3),
            (3, 3),
            (4, 4),
            (5, 2),
            (6, 3),
            (7, 2),
            (8, 1),
            (9, 1),
            (12, 2),
            (13, 3),
        ],
        &[
            (0, 3),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 4),
            (6, 1),
            (7, 2),
            (8, 2),
            (9, 3),
            (11, 4),
            (12, 3),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(9),
    face: Face::Free,
    hand0: 5,
    cap: 8,
    board_max: 5,
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
