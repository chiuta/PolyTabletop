//! Șase Lumi în Flăcări — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Legiunea Aurie contra Roiul Verde), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Fiecare are 3 zone cu 8 viață; unitatea lovește o zonă (atac k zN); două zone distruse câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "sase_lumi_in_flacari",
    title: "Șase Lumi în Flăcări",
    fac: ["Legiunea Aurie", "Roiul Verde"],
    src: "comoară",
    units: &[
        Unit {
            name: "Soldat de Linie",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Recrut Ghiolbănit",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Paznic de Ferestre",
            cost: 1,
            atk: 3,
            hp: 1,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Sniper Sinistru",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Tanc Ușor",
            cost: 3,
            atk: 5,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Biocopil",
            cost: 3,
            atk: 5,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Predator Roșu",
            cost: 4,
            atk: 6,
            hp: 4,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Căpitan Lancier",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: VENOM,
            val: 3,
        },
        Unit {
            name: "Titan Orbital",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Imperiul-Creștet",
            cost: 6,
            atk: 7,
            hp: 4,
            kw: FLY | VENOM,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Bombardament Orbital",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Reinforțare",
            cost: 6,
            fx: Fx::Sweep(2),
        },
        Spell {
            name: "Cuib Scos",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Sirenă de Alarmă",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Evacuare",
            cost: 3,
            fx: Fx::Draw(2),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 2),
            (2, 4),
            (3, 2),
            (4, 3),
            (5, 2),
            (7, 2),
            (8, 1),
            (9, 1),
            (10, 3),
            (11, 2),
            (12, 1),
            (13, 1),
            (14, 2),
        ],
        &[
            (0, 2),
            (1, 3),
            (2, 2),
            (3, 3),
            (4, 2),
            (5, 3),
            (6, 2),
            (7, 1),
            (8, 4),
            (9, 1),
            (10, 1),
            (11, 2),
            (12, 1),
            (13, 1),
            (14, 1),
        ],
    ],
    res: Res::Auto,
    win: Win::Zones(8),
    face: Face::Free,
    hand0: 5,
    cap: 9,
    board_max: 6,
    lands: 0,
    big: 4,
    max_turns: 16,
    twist: T_RUSH,
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
