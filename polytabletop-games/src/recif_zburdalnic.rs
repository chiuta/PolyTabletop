//! Recif zburdalnic (`recif_zburdalnic`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "recif_zburdalnic",
    title: "Recif zburdalnic",
    facs: ["Clanul Coralilor", "Clanul Meduzei"],
    res: 1,
    cap: 9,
    lands: 0,
    bank: 0,
    win: 1,
    wn: 5,
    big: 4,
    hand0: 6,
    maxt: 14,
    bmax: 6,
    twist: b'-',
    units: [
        U {
            name: "Puiet de Pește",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Crab Zgâlțâit",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Melc Încet",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Calmar Artist",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Sepie Cu Aripi",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Rechin Pitic",
            cost: 3,
            atk: 2,
            hp: 3,
            kw: VENOM,
        },
        U {
            name: "Crevete Boxer",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: GUARD,
        },
        U {
            name: "Țestoasă Bătrână",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Caracatiță Mare",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Balena Cântătoare",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: FLY,
        },
    ],
    spells: [
        S {
            name: "Val Scurt",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Bule de Săpun",
            cost: 3,
            fx: 2,
            n: 3,
        },
        S {
            name: "Ghidușie",
            cost: 1,
            fx: 4,
            n: 1,
        },
        S {
            name: "Furtună de Nisip",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Vâltoare",
            cost: 1,
            fx: 6,
            n: 0,
        },
    ],
    deck: [
        [2, 2, 2, 3, 2, 3, 3, 3, 2, 1, 1, 2, 1, 2, 1],
        [4, 4, 2, 2, 3, 1, 3, 2, 1, 1, 1, 1, 1, 2, 2],
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
