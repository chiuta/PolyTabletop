//! Păpușarii din Umbră — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Cercul Îngust contra Cercul Larg), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 8 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "papusarii_umbrei",
    title: "Păpușarii din Umbră",
    fac: ["Cercul Îngust", "Cercul Larg"],
    src: "influență",
    units: &[
        Unit {
            name: "Emisar Discret",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Secretar",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Ziarist Cumpărat",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: FIRST,
            val: 1,
        },
        Unit {
            name: "Agent Dublu",
            cost: 3,
            atk: 2,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Bancher în Penumbră",
            cost: 3,
            atk: 3,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Deputat de Fațadă",
            cost: 4,
            atk: 4,
            hp: 7,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Șef de Cabinet",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Prelat Ocult",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Magnat",
            cost: 6,
            atk: 6,
            hp: 8,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Eminența Cenușie",
            cost: 7,
            atk: 6,
            hp: 8,
            kw: FIRST | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Șantaj",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Scurgere de Informații",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Alegeri Trucate",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Mită Mare",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Dosar Închis",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 3),
            (7, 2),
            (8, 3),
            (9, 1),
            (11, 3),
            (12, 4),
            (13, 2),
        ],
        &[
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 2),
            (4, 3),
            (5, 1),
            (6, 3),
            (7, 1),
            (8, 3),
            (9, 2),
            (10, 1),
            (11, 2),
            (12, 2),
            (13, 1),
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
