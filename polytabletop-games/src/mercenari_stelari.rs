//! Mercenarii Stelari — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Flota Liberă contra Imperiul Ceții), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Câștigi când viața adversarului ajunge la 0 (start 25). Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "mercenari_stelari",
    title: "Mercenarii Stelari",
    fac: ["Flota Liberă", "Imperiul Ceții"],
    src: "credit",
    units: &[
        Unit {
            name: "Pilot Novice",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Droid de Rampă",
            cost: 1,
            atk: 4,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Contrabandist",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: ARMOR,
            val: 1,
        },
        Unit {
            name: "Lunetist",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Cavaler cu Sabie de Plasmă",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Navă de Escortă",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Vânător de Recompense",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Căpitan de Crucișător",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Mech de Asalt",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Nava-Mamă",
            cost: 6,
            atk: 10,
            hp: 4,
            kw: ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Salvă de Tun",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Reparații în Zbor",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Câmp de Forță",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Contract Nou",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Salt în Hiperspațiu",
            cost: 2,
            fx: Fx::Bounce,
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 1),
            (2, 3),
            (3, 1),
            (4, 3),
            (5, 2),
            (6, 3),
            (8, 3),
            (9, 3),
            (10, 3),
            (11, 1),
            (13, 3),
            (14, 3),
        ],
        &[
            (0, 4),
            (2, 3),
            (3, 2),
            (4, 1),
            (5, 3),
            (6, 3),
            (7, 4),
            (8, 3),
            (9, 2),
            (10, 1),
            (12, 2),
            (14, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Life(25),
    face: Face::Free,
    hand0: 6,
    cap: 9,
    board_max: 6,
    lands: 0,
    big: 4,
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
