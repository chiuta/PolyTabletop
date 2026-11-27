//! Ucenicii Vântului — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Satul Frunzișului contra Satul Dunelor), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Câștigi când viața adversarului ajunge la 0 (start 16). Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "ucenicii_vantului",
    title: "Ucenicii Vântului",
    fac: ["Satul Frunzișului", "Satul Dunelor"],
    src: "chakră",
    units: &[
        Unit {
            name: "Ucenic Zburdalnic",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Mesager cu Pergament",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Scuturător de Vânt",
            cost: 2,
            atk: 5,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cercetaș Mut",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
            val: 1,
        },
        Unit {
            name: "Maestru de Aruncat",
            cost: 3,
            atk: 6,
            hp: 2,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Ninja de Câmpie",
            cost: 3,
            atk: 6,
            hp: 2,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Sabie Vântoasă",
            cost: 4,
            atk: 7,
            hp: 3,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Șef de Echipă",
            cost: 4,
            atk: 7,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Vulpea cu Nouă Cozi",
            cost: 5,
            atk: 8,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Marele Căpitan",
            cost: 6,
            atk: 8,
            hp: 4,
            kw: FLY | FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Pergament Secret",
            cost: 2,
            fx: Fx::Bolt(2),
        },
        Spell {
            name: "Aruncare Dublă",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Scut de Vânt",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Ilustrare în Fum",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Salt Peste Zid",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 1),
            (2, 4),
            (3, 1),
            (6, 2),
            (7, 1),
            (8, 2),
            (9, 3),
            (10, 1),
            (11, 4),
            (12, 2),
            (14, 4),
        ],
        &[
            (0, 4),
            (1, 3),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 2),
            (6, 1),
            (7, 1),
            (8, 3),
            (9, 2),
            (10, 2),
            (11, 1),
            (12, 2),
            (13, 2),
        ],
    ],
    res: Res::Ink,
    win: Win::Life(16),
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
