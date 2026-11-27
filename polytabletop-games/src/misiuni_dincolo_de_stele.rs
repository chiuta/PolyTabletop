//! Misiuni dincolo de Stele — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Federația Exploratorilor contra Hegemonia Nordului), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 8 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "misiuni_dincolo_de_stele",
    title: "Misiuni dincolo de Stele",
    fac: ["Federația Exploratorilor", "Hegemonia Nordului"],
    src: "energie",
    units: &[
        Unit {
            name: "Tehnician de Punte",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Inginer Prim",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Medic de Bord",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Scut-Dronă",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Pilot de Cursă",
            cost: 3,
            atk: 2,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Comandant de Nava",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Diplomat Stelar",
            cost: 4,
            atk: 4,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Cuirasat Mic",
            cost: 5,
            atk: 6,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Nava-Tendă",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Marea Navă-Amiral",
            cost: 7,
            atk: 6,
            hp: 10,
            kw: ARMOR,
            val: 4,
        },
    ],
    spells: &[
        Spell {
            name: "Scanare Profundă",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Raport de Misiune",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Dilemă Morală",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Reparații",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Salt Cuantic",
            cost: 4,
            fx: Fx::Slay,
        },
    ],
    deck: [
        &[
            (0, 2),
            (1, 1),
            (2, 2),
            (3, 1),
            (4, 3),
            (5, 4),
            (6, 1),
            (7, 3),
            (8, 1),
            (9, 1),
            (10, 4),
            (12, 3),
            (13, 3),
            (14, 1),
        ],
        &[
            (0, 2),
            (1, 4),
            (2, 4),
            (3, 1),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 3),
            (9, 3),
            (11, 1),
            (12, 4),
            (13, 2),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(8),
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
