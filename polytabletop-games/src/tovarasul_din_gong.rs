//! Tovarășul din Gong — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Frații Gongului contra Frații Tobei), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Fiecare are 5 scuturi: lovitura directă reușită rupe 1 scut (2 dacă atacul e cel puțin 3) și îl dă în mână adversarului; lovitura fără scuturi câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "tovarasul_din_gong",
    title: "Tovarășul din Gong",
    fac: ["Frații Gongului", "Frații Tobei"],
    src: "jeton",
    units: &[
        Unit {
            name: "Pui de Dragon Mic",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mascotă Zburdalnică",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Prieten Cu Armură",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler Curajos",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Zeul Mărunt",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Păzitor de Prag",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Dragon Prietenos",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Vulpe Roșie",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Maestrul Gongului",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Dragonul Celest",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Chemare Prietenoasă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Gong de Foc",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Scut de Prieten",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Joc de Zaruri",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Oglindă",
            cost: 2,
            fx: Fx::Mend(3),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 2),
            (8, 4),
            (9, 1),
            (10, 4),
            (11, 2),
            (14, 1),
        ],
        &[
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 2),
            (5, 4),
            (6, 4),
            (7, 3),
            (8, 2),
            (9, 3),
            (10, 2),
            (11, 4),
            (13, 1),
        ],
    ],
    res: Res::Ink,
    win: Win::Shield(5),
    face: Face::Free,
    hand0: 6,
    cap: 8,
    board_max: 5,
    lands: 0,
    big: 3,
    max_turns: 16,
    twist: T_TROPHY,
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
