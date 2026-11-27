//! Cosașii de Suflete — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Gărzile Sufletelor contra Umbrele Cenușii), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi când viața adversarului ajunge la 0 (start 22). Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cosasii_de_suflete",
    title: "Cosașii de Suflete",
    fac: ["Gărzile Sufletelor", "Umbrele Cenușii"],
    src: "suflu",
    units: &[
        Unit {
            name: "Suflet Rătăcit",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Pază Palidă",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Spirit-Pisică",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Cosaș Tânăr",
            cost: 3,
            atk: 5,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Căpitan cu Lamă",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Doamna Cețoasă",
            cost: 4,
            atk: 6,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Fantoma Lungă",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Monstru-Mască",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Comandant de Noapte",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Regele Cosașilor",
            cost: 7,
            atk: 9,
            hp: 6,
            kw: FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Tăiș de Lamă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Eliberarea",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Aripă de Ceață",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Dublul",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Strigăt din Lumea de Dincolo",
            cost: 4,
            fx: Fx::Sweep(1),
        },
    ],
    deck: [
        &[
            (0, 4),
            (1, 2),
            (2, 1),
            (3, 3),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (9, 2),
            (10, 1),
            (11, 4),
            (14, 2),
        ],
        &[
            (0, 3),
            (1, 3),
            (4, 4),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 4),
            (9, 4),
            (10, 1),
            (14, 4),
        ],
    ],
    res: Res::Auto,
    win: Win::Life(22),
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
