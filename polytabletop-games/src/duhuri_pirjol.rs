//! Duhurile Pârjolului — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Clanul Cremenii contra Clanul Cenușii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Fiecare are 5 scuturi: lovitura directă reușită rupe 1 scut (2 dacă atacul e cel puțin 4) și îl dă în mână adversarului; lovitura fără scuturi câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "duhuri_pirjol",
    title: "Duhurile Pârjolului",
    fac: ["Clanul Cremenii", "Clanul Cenușii"],
    src: "jar",
    units: &[
        Unit {
            name: "Mic Duh de Scânteie",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Lup de Jar",
            cost: 2,
            atk: 5,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Vultur Roșcat",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: FLY,
            val: 1,
        },
        Unit {
            name: "Șarpe de Lavă",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Războinic de Cărbune",
            cost: 3,
            atk: 5,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Taur Arzător",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Fenix Tânăr",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Căpetenie de Pară",
            cost: 5,
            atk: 7,
            hp: 5,
            kw: HASTE,
            val: 3,
        },
        Unit {
            name: "Gigant de Scorie",
            cost: 6,
            atk: 9,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Dragonul Ultimului Foc",
            cost: 7,
            atk: 9,
            hp: 6,
            kw: VENOM,
            val: 4,
        },
    ],
    spells: &[
        Spell {
            name: "Scânteie",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Foc de Veghe",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Zid de Cenușă",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Chemare de Jar",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Cuib de Foc",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 3),
            (2, 4),
            (3, 3),
            (4, 4),
            (5, 3),
            (8, 2),
            (9, 3),
            (11, 2),
            (12, 3),
            (13, 2),
            (14, 2),
        ],
        &[
            (0, 4),
            (1, 1),
            (2, 4),
            (3, 3),
            (4, 2),
            (5, 4),
            (6, 4),
            (7, 3),
            (8, 1),
            (9, 3),
            (10, 2),
            (12, 2),
            (13, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Shield(5),
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
