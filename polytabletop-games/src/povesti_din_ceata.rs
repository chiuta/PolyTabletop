//! Povești din Ceață — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Cercul Cărturarilor contra Cultul Adâncurilor), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 7. Câștigi la 9 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "povesti_din_ceata",
    title: "Povești din Ceață",
    fac: ["Cercul Cărturarilor", "Cultul Adâncurilor"],
    src: "rațiune",
    units: &[
        Unit {
            name: "Student Palid",
            cost: 1,
            atk: 1,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pisoi Fantomă",
            cost: 2,
            atk: 2,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Detectiv Pe Curând",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Arhivar Tremurând",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Corb de Biblioteca",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Pictor Smintit",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Ucenic al Adâncului",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Lucrător de Far",
            cost: 5,
            atk: 4,
            hp: 7,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Vântul Bătrân",
            cost: 5,
            atk: 4,
            hp: 7,
            kw: VENOM,
            val: 3,
        },
        Unit {
            name: "Cel din Adânc",
            cost: 6,
            atk: 5,
            hp: 7,
            kw: FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Vis Urât",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Raport Secret",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Lumina Farului",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Șoaptă din Ceață",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Zid de Sare",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 2),
            (2, 3),
            (3, 2),
            (4, 3),
            (6, 2),
            (7, 2),
            (8, 2),
            (9, 1),
            (10, 4),
            (11, 2),
            (13, 2),
            (14, 2),
        ],
        &[
            (0, 3),
            (1, 2),
            (2, 2),
            (3, 1),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 1),
            (9, 3),
            (10, 1),
            (11, 2),
            (13, 2),
            (14, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(9),
    face: Face::Free,
    hand0: 6,
    cap: 7,
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
