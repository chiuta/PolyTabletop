//! Orășelul caraghios (`orasel_caraghios`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "orasel_caraghios",
    title: "Orășelul caraghios",
    facs: ["Familia Din Colț", "Vecinii Zgomotoși"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 2,
    wn: 12,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 6,
    twist: b'H',
    units: [
        U {
            name: "Pisică de Canapea",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Bunică Cu Mătură",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Copil Cu Praștie",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Vânzător de Gogoși",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Primar Pomădat",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: ARMOR,
        },
        U {
            name: "Paznic Somnoros",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Profesor Cu Cretă",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Hoț de Vinerea",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: FLY,
        },
        U {
            name: "Șeful Poliției",
            cost: 5,
            atk: 7,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Starul Televiziunii",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Tort în Față",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Zvon Răspândit",
            cost: 3,
            fx: 2,
            n: 3,
        },
        S {
            name: "Glumă Veche",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Petrecere Mare",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Pană de Curent",
            cost: 2,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [2, 4, 4, 2, 2, 3, 2, 2, 1, 1, 2, 1, 1, 2, 1],
        [4, 4, 2, 3, 2, 2, 2, 1, 2, 1, 1, 1, 2, 2, 1],
    ],
};

pub fn new() -> Tcg {
    Tcg::new(&SPEC)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_sizes_and_random_games() {
        for p in 0..2 {
            let n: u32 = SPEC.deck[p].iter().map(|&x| x as u32).sum::<u32>() + SPEC.lands as u32;
            assert_eq!(n, 30);
        }
        crate::brand_util::check(|s| Tcg::with_seed(&SPEC, s), 6000);
    }

    #[test]
    fn ai_and_render_ignore_hidden_information() {
        blind_check(&SPEC);
    }
}
