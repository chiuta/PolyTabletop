//! Eroi contra Răufăcători — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Liga Eroilor contra Frăția Răufăcătorilor), mână ascunsă. Start: 6 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. O dată pe tură poți pune orice carte din mână ca sursă (sursa i): energia +1 (maximum 8), dar pierzi cartea. Câștigi când viața adversarului ajunge la 0 (start 28). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "eroi_contra_raufacatori",
    title: "Eroi contra Răufăcători",
    fac: ["Liga Eroilor", "Frăția Răufăcătorilor"],
    src: "rând",
    units: &[
        Unit {
            name: "Salvator de Cartier",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Băiat cu Pelerină",
            cost: 2,
            atk: 4,
            hp: 3,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Vulpea Urbană",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: GUARD,
            val: 1,
        },
        Unit {
            name: "Pumn de Oțel",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Scutul Orașului",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: FIRST,
            val: 2,
        },
        Unit {
            name: "Furtuna Albă",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Titanul Verde",
            cost: 4,
            atk: 6,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Doamna Fulger",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Arhimagul Orașului",
            cost: 5,
            atk: 6,
            hp: 6,
            kw: FLY,
            val: 3,
        },
        Unit {
            name: "Eroul Suprem",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: GUARD | FIRST,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Salvare",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Ripostă",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Plan Diabolic",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Reorganizare",
            cost: 2,
            fx: Fx::Buff(1),
        },
        Spell {
            name: "Distragere",
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
            (4, 4),
            (5, 2),
            (6, 2),
            (8, 4),
            (9, 1),
            (10, 2),
            (11, 1),
            (13, 3),
        ],
        &[
            (0, 3),
            (1, 1),
            (2, 4),
            (3, 2),
            (5, 4),
            (6, 3),
            (7, 4),
            (8, 1),
            (9, 1),
            (10, 3),
            (12, 4),
        ],
    ],
    res: Res::Ink,
    win: Win::Life(28),
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
