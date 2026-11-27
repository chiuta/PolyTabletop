//! Dragoni la Hotar — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Păstorii Dragonilor contra Rătăcitorii Munților), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 7. Fiecare are 3 zone cu 9 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "dragoni_la_hotar",
    title: "Dragoni la Hotar",
    fac: ["Păstorii Dragonilor", "Rătăcitorii Munților"],
    src: "solz",
    units: &[
        Unit {
            name: "Pui de Zmeu",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Cetățean Curios",
            cost: 2,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Prinț cu Ghioagă",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Paznic de Pas",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Spadasin de Poartă",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Dragon de Luncă",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Zmeoaică",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Fiara de Pas",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Căpetenie de Munte",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Dragonul Hotarelor",
            cost: 7,
            atk: 7,
            hp: 7,
            kw: FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Zbor Peste Munte",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Ofrandă",
            cost: 4,
            fx: Fx::Sweep(1),
        },
        Spell {
            name: "Cuib",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Tămăduire",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Foc din Peșteră",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 2),
            (2, 2),
            (3, 4),
            (4, 4),
            (5, 3),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 2),
            (10, 1),
            (13, 4),
        ],
        &[
            (0, 2),
            (2, 3),
            (3, 3),
            (4, 2),
            (5, 3),
            (6, 1),
            (7, 3),
            (8, 1),
            (9, 4),
            (11, 1),
            (12, 3),
            (13, 2),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Zones(9),
    face: Face::Free,
    hand0: 5,
    cap: 7,
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
