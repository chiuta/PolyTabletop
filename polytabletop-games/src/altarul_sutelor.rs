//! Altarul sutelor (`altarul_sutelor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "altarul_sutelor",
    title: "Altarul sutelor",
    facs: ["Cultul Lumânării", "Ordinul Cenușii"],
    res: 0,
    cap: 8,
    lands: 12,
    bank: 0,
    win: 1,
    wn: 5,
    big: 4,
    hand0: 5,
    maxt: 16,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Acolit Tăcut",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Slujitor de Ceară",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Cântăreț de Veghe",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Strigoi Mărunt",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Preot Mohorât",
            cost: 3,
            atk: 3,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Cavaler Osuar",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Prorocul Orb",
            cost: 4,
            atk: 4,
            hp: 3,
            kw: VENOM,
        },
        U {
            name: "Stăpân de Cripte",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Umbra Altarului",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: 0,
        },
        U {
            name: "Hecatomba Vie",
            cost: 7,
            atk: 5,
            hp: 10,
            kw: GUARD,
        },
    ],
    spells: [
        S {
            name: "Ofrandă Neagră",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Rug Curățitor",
            cost: 2,
            fx: 7,
            n: 1,
        },
        S {
            name: "Vindecare Amară",
            cost: 2,
            fx: 5,
            n: 3,
        },
        S {
            name: "Sunetul Clopotului",
            cost: 4,
            fx: 1,
            n: 2,
        },
        S {
            name: "Jertfă Mare",
            cost: 2,
            fx: 2,
            n: 2,
        },
    ],
    deck: [
        [1, 2, 2, 2, 1, 1, 2, 1, 0, 1, 1, 1, 1, 1, 1],
        [1, 2, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
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
