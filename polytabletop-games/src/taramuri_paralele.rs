//! Tărâmuri Paralele — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Tribul Subteranilor contra Tribul Cerului), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 8. Câștigi când viața adversarului ajunge la 0 (start 20). Până la 5 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: unitățile se vindecă la începutul turei tale. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "taramuri_paralele",
    title: "Tărâmuri Paralele",
    fac: ["Tribul Subteranilor", "Tribul Cerului"],
    src: "amprenta",
    units: &[
        Unit {
            name: "Cercetaș Nenumit",
            cost: 1,
            atk: 2,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Zburător Mic",
            cost: 2,
            atk: 3,
            hp: 3,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Șopârlă de Umbră",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: HASTE,
            val: 1,
        },
        Unit {
            name: "Cântăreț de Piatră",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: GUARD,
            val: 2,
        },
        Unit {
            name: "Ghearele Negre",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Șoimul din Nor",
            cost: 4,
            atk: 6,
            hp: 5,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Vrăjitor Dublu",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: GUARD,
            val: 3,
        },
        Unit {
            name: "Colos Cenușiu",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Regina Tunelului",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Uriașul Ultimului Tărâm",
            cost: 7,
            atk: 7,
            hp: 8,
            kw: FLY,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Portal",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Cântec Mut",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Zgomot Surd",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Aripă Puternică",
            cost: 2,
            fx: Fx::Bounce,
        },
        Spell {
            name: "Poartă Închisă",
            cost: 3,
            fx: Fx::Boon(4),
        },
    ],
    deck: [
        &[
            (0, 3),
            (1, 3),
            (2, 4),
            (3, 1),
            (4, 3),
            (5, 2),
            (6, 3),
            (7, 3),
            (9, 2),
            (10, 4),
            (13, 1),
        ],
        &[
            (0, 4),
            (1, 2),
            (2, 1),
            (3, 2),
            (5, 2),
            (6, 4),
            (7, 1),
            (8, 4),
            (9, 3),
            (10, 1),
            (11, 1),
            (12, 2),
            (14, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Life(20),
    face: Face::Free,
    hand0: 5,
    cap: 8,
    board_max: 5,
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
