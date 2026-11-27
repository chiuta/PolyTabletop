//! Eroi de Târg — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Gilda Negustorilor contra Gilda Mercenarilor), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Câștigi când viața adversarului ajunge la 0 (start 30). Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: cine ucide o unitate în luptă trage o carte. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "eroi_de_targ",
    title: "Eroi de Târg",
    fac: ["Gilda Negustorilor", "Gilda Mercenarilor"],
    src: "monedă",
    units: &[
        Unit {
            name: "Zilier de Târg",
            cost: 1,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Arcaș de Ocazie",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Paznic de Taraba",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Ghicitoare",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Bătăuș de Cârciumă",
            cost: 3,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Cavaler cu Datorii",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Preot Ambulant",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Vrăjitor de Bâlci",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: HASTE,
            val: 3,
        },
        Unit {
            name: "Gigant de Bâlci",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Marele Căpitan al Târgului",
            cost: 7,
            atk: 6,
            hp: 7,
            kw: HASTE | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Negustor Șmecher",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Căciulă Plină",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Bătaie la Han",
            cost: 3,
            fx: Fx::Boon(4),
        },
        Spell {
            name: "Împrumut",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Chemare la Arme",
            cost: 2,
            fx: Fx::Buff(1),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 1),
            (2, 3),
            (3, 4),
            (4, 1),
            (6, 4),
            (7, 3),
            (9, 2),
            (10, 2),
            (11, 2),
            (12, 3),
            (13, 1),
            (14, 3),
        ],
        &[
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 3),
            (5, 2),
            (6, 1),
            (7, 4),
            (8, 2),
            (9, 3),
            (10, 2),
            (11, 3),
            (12, 1),
            (13, 3),
            (14, 3),
        ],
    ],
    res: Res::Auto,
    win: Win::Life(30),
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
