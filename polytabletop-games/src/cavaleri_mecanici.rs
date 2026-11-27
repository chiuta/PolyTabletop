//! Cavaleri Mecanici — variantă originală T5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală pentru doi: pachete fixe construite de joc (Echipa Albastră contra Echipa Roșie), mână ascunsă. Start: 5 cărți (J2 primește una în plus); cine nu mai poate trage la începutul turei pierde; după 16 ture pe jucător decide scorul. Energia crește singură cu 1 la fiecare tură proprie, până la 9. Fiecare are 6 scuturi: lovitura directă reușită rupe 1 scut (2 dacă atacul e cel puțin 3) și îl dă în mână adversarului; lovitura fără scuturi câștigă. Până la 6 unități cu cost, atac/rezistență și cuvinte-cheie (Elan, Gardian, Zburător, Lovește primul, Armură, Venin); în luptă ambele lovesc, daunele rămân; vrăji: pescuit, lovitură, măturare, binecuvântare, întărire, distrugere, întoarcere în mână, vindecare. Răsucire: nicio unitate nu are boală de invocare. Mutări: joc i [țintă], sursa i, atac k [eN|zN], miss k, fin. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

use crate::t5_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cavaleri_mecanici",
    title: "Cavaleri Mecanici",
    fac: ["Echipa Albastră", "Echipa Roșie"],
    src: "combustibil",
    units: &[
        Unit {
            name: "Dronă de Recunoaștere",
            cost: 1,
            atk: 3,
            hp: 1,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Pilot Stagiar",
            cost: 2,
            atk: 4,
            hp: 2,
            kw: 0,
            val: 1,
        },
        Unit {
            name: "Armură Ușoară",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
            val: 2,
        },
        Unit {
            name: "Tanc de Fier",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Cavaler cu Lance",
            cost: 3,
            atk: 4,
            hp: 4,
            kw: ARMOR,
            val: 2,
        },
        Unit {
            name: "Scutier Mecanic",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Mech Zburător",
            cost: 4,
            atk: 6,
            hp: 5,
            kw: 0,
            val: 3,
        },
        Unit {
            name: "Armură Grea",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: FLY,
            val: 2,
        },
        Unit {
            name: "Căpitan de Flotă",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: HASTE,
            val: 2,
        },
        Unit {
            name: "Gigantul de Oțel",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: FLY | ARMOR,
            val: 3,
        },
    ],
    spells: &[
        Spell {
            name: "Rachetă",
            cost: 3,
            fx: Fx::Bolt(3),
        },
        Spell {
            name: "Atelier de Reparații",
            cost: 4,
            fx: Fx::Buff(2),
        },
        Spell {
            name: "Câmp Magnetic",
            cost: 3,
            fx: Fx::Draw(2),
        },
        Spell {
            name: "Pilot de Rezervă",
            cost: 4,
            fx: Fx::Slay,
        },
        Spell {
            name: "Protocol de Atac",
            cost: 2,
            fx: Fx::Mend(3),
        },
    ],
    deck: [
        &[
            (0, 1),
            (1, 4),
            (2, 4),
            (3, 2),
            (4, 4),
            (5, 3),
            (6, 2),
            (7, 4),
            (8, 2),
            (12, 3),
            (13, 4),
            (14, 2),
        ],
        &[
            (1, 4),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 1),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 2),
            (10, 3),
            (11, 2),
            (14, 2),
        ],
    ],
    res: Res::Auto,
    win: Win::Shield(6),
    face: Face::Free,
    hand0: 5,
    cap: 9,
    board_max: 6,
    lands: 0,
    big: 3,
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
