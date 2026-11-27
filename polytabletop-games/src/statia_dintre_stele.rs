//! Stația dintre Stele — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Consiliul Stației contra Flota Exilaților), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi la 10 puncte: unitățile pregătite fac misiune (miss k: obosesc și aduc valoarea lor), iar atacurile lovesc doar unități obosite; nu există lovitură directă. Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "statia_dintre_stele",
    title: "Stația dintre Stele",
    fac: ["Consiliul Stației", "Flota Exilaților"],
    src: "credit",
    units: &[
        Unit {
            name: "Funcționar de Dană",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Medic de Gardă",
            cost: 1,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Pilot de Navetă",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Ofițer de Securitate",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: ARMOR,
            val: 1,
        },
        Unit {
            name: "Diplomat",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Spion Telepat",
            cost: 3,
            atk: 3,
            hp: 6,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Căpitan de Convoi",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Cuirasat de Pază",
            cost: 5,
            atk: 4,
            hp: 8,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Nava-Oraș",
            cost: 5,
            atk: 4,
            hp: 7,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Marele Consul",
            cost: 6,
            atk: 5,
            hp: 6,
            kw: GUARD | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Raport Stelar",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Zvon în Port",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Intervenție",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Reparații Generale",
            cost: 2,
            fx: Fx::Mend(3),
        },
        Spell {
            name: "Vot Dublu",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 3),
            (6, 2),
            (8, 3),
            (9, 3),
            (10, 4),
            (11, 2),
            (12, 1),
            (14, 3),
        ],
        &[
            (0, 1),
            (2, 4),
            (3, 4),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 1),
            (8, 3),
            (9, 2),
            (12, 2),
            (13, 2),
            (14, 3),
        ],
    ],
    res: Res::Auto,
    win: Win::Points(10),
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
