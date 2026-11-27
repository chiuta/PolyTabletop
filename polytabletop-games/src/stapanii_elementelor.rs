//! Stăpânii elementelor (`stapanii_elementelor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "stapanii_elementelor",
    title: "Stăpânii elementelor",
    facs: ["Pelerinii Aerului", "Garda Focului"],
    res: 1,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 3,
    wn: 6,
    big: 0,
    hand0: 5,
    maxt: 15,
    bmax: 5,
    twist: b'H',
    units: [
        U {
            name: "Novice de Apă",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Dansator de Vânt",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Sculptor de Pietre",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Gardian de Flăcări",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Arcaș Din Gheață",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Valuri Încolăcite",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Călăreț de Furtună",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Maestru de Lavă",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: HASTE,
        },
        U {
            name: "Fiara Glacială",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Marele Stăpân",
            cost: 6,
            atk: 5,
            hp: 7,
            kw: ARMOR,
        },
    ],
    spells: [
        S {
            name: "Val Rece",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Vâltoare de Vânt",
            cost: 5,
            fx: 1,
            n: 3,
        },
        S {
            name: "Glaciație",
            cost: 2,
            fx: 9,
            n: 0,
        },
        S {
            name: "Sfera Pământului",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Flacără Vindecătoare",
            cost: 3,
            fx: 7,
            n: 2,
        },
    ],
    deck: [
        [2, 4, 2, 2, 3, 3, 1, 3, 1, 2, 2, 1, 1, 2, 1],
        [4, 4, 2, 2, 3, 3, 1, 1, 1, 2, 1, 1, 2, 2, 1],
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
