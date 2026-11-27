//! Lupta fuziunii (`lupta_fuziunii`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "lupta_fuziunii",
    title: "Lupta fuziunii",
    facs: ["Dojo Dinspre Soare", "Dojo Dinspre Lună"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 0,
    wn: 24,
    big: 0,
    hand0: 6,
    maxt: 14,
    bmax: 6,
    twist: b'P',
    units: [
        U {
            name: "Ucenic de Arte",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Gonitor de Aer",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Luptător Roșu",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: LEECH,
        },
        U {
            name: "Luptător Albastru",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Războinic Cu Aură",
            cost: 2,
            atk: 2,
            hp: 1,
            kw: VENOM,
        },
        U {
            name: "Maestru de Ki",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: ARMOR,
        },
        U {
            name: "Gemeni Fuzionați",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Căpitan al Stelei",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Zeu Cu Coadă",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Marele Războinic Fuzionat",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Pumn Cu Aură",
            cost: 1,
            fx: 4,
            n: 1,
        },
        S {
            name: "Fuziune",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Explozie de Putere",
            cost: 5,
            fx: 1,
            n: 3,
        },
        S {
            name: "Bile Lucitoare",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Strigăt de Ki",
            cost: 4,
            fx: 5,
            n: 9,
        },
    ],
    deck: [
        [4, 4, 4, 2, 2, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1],
        [2, 2, 4, 2, 3, 3, 3, 1, 2, 1, 2, 1, 1, 2, 1],
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
