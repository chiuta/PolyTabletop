//! Domeniul Monștrilor — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Hoardele de Foc contra Hoardele de Apă), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Fiecare are 5 scuturi: lovitura directă reușită rupe 1 scut (2 dacă atacul e cel puțin 4) și îl dă în mână adversarului; lovitura fără scuturi câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Coloane: unitatea k atacă doar unitatea k adversă sau direct dacă aceasta nu există (zburătorii aleg liber). Răsucire: loviturile directe sunt cu 1 mai tari. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "domeniul_monstrilor",
    title: "Domeniul Monștrilor",
    fac: ["Hoardele de Foc", "Hoardele de Apă"],
    src: "mană",
    units: &[
        Unit {
            name: "Fiară de Râpă",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Lupul Roșu",
            cost: 1,
            atk: 3,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Păianjen Albastru",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Gnom Pietros",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Hidra Verde",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Zmeul de Piatră",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Dragon-Trăznet",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: VENOM,
            val: 2,
        },
        Unit {
            name: "Colos Luminos",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Hoardă de Mlaștină",
            cost: 5,
            atk: 7,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Părintele Monștrilor",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: FLY | VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Tsunami",
            cost: 6,
            fx: Fx::Sweep(2),
        },
        Spell {
            name: "Piatră Rostogolită",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Fluier de Hoardă",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Rugăciune Fiarei",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Spectrul Furtunii",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 4),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 1),
            (6, 1),
            (7, 2),
            (8, 2),
            (9, 3),
            (10, 1),
            (11, 2),
            (12, 3),
            (13, 1),
            (14, 2),
        ],
        &[
            (0, 3),
            (1, 2),
            (2, 2),
            (3, 3),
            (5, 1),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 3),
            (10, 2),
            (11, 2),
            (13, 2),
            (14, 3),
        ],
    ],
    res: Res::Ink,
    win: Win::Shield(5),
    face: Face::Cols,
    hand0: 6,
    cap: 8,
    board_max: 6,
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
