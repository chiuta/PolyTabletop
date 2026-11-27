//! Cursa prin Rețea — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Hoții de Date contra Corporația Albă), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 7. Câștigi la 7 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cursa_prin_retea",
    title: "Cursa prin Rețea",
    fac: ["Hoții de Date", "Corporația Albă"],
    src: "lățime de bandă",
    units: &[
        Unit {
            name: "Sondă Mută",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Virus Mărunt",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Agent de Paza",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Ghemotoc de Cod",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: FLY,
            val: 1,
        },
        Unit {
            name: "Păianjen de Fibră",
            cost: 3,
            atk: 3,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Gardian de Firewall",
            cost: 3,
            atk: 3,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Spion Învelit",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Cap de Pod",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Releu Fantomă",
            cost: 5,
            atk: 5,
            hp: 7,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Marele Server",
            cost: 6,
            atk: 5,
            hp: 9,
            kw: ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Furt de Pachete",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Cheie Falsă",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Reboot",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Recuperare",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Raid Nocturn",
            cost: 2,
            fx: Fx::Mend(3),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 2),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 4),
            (10, 3),
            (12, 2),
            (13, 4),
            (14, 1),
        ],
        &[
            (0, 1),
            (1, 2),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 1),
            (8, 3),
            (9, 4),
            (10, 1),
            (11, 3),
            (12, 2),
            (14, 4),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(7),
    face: Face::Free,
    hand0: 6,
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
