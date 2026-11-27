//! Cetățile din Furtună — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Casa Orașului contra Clanurile Stâncii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Resurse: 11 cărți-sursă în fiecare pachet; o dată pe tură pui o sursă (sursa i): energia +1 (maximum 8). Fiecare are 3 zone cu 10 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Coloane: unitatea k atacă doar unitatea k adversă sau direct dacă aceasta nu există (zburătorii aleg liber). Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cetatile_din_furtuna",
    title: "Cetățile din Furtună",
    fac: ["Casa Orașului", "Clanurile Stâncii"],
    src: "sudoare",
    units: &[
        Unit {
            name: "Soldat de Pază",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Arcaș de Turn",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Cercetaș cu Corn",
            cost: 2,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Sapator",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
            val: 1,
        },
        Unit {
            name: "Lăncier din Turn",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler cu Furtună",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Mag de Cetate",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Războinic de Stâncă",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Căpetenie de Clan",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Zeul Vântului",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Fulger",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Poartă Închisă",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Întăritură",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Bătaie de Tun",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Răbufnire",
            cost: 4,
            fx: Fx::Slay,
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 3),
            (2, 4),
            (3, 1),
            (4, 3),
            (5, 2),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
            (12, 2),
            (13, 1),
            (14, 1),
        ],
        &[
            (0, 4),
            (1, 4),
            (2, 1),
            (3, 2),
            (4, 2),
            (6, 1),
            (8, 4),
            (9, 2),
            (11, 3),
            (12, 1),
            (14, 2),
        ],
    ],
    res: Res::Land,
    win: Win::Zones(10),
    face: Face::Cols,
    hand0: 5,
    cap: 8,
    board_max: 6,
    lands: 11,
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
