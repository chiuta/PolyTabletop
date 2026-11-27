//! Școala darurilor (`scoala_darurilor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "scoala_darurilor",
    title: "Școala darurilor",
    facs: ["Elevii Curții", "Clanul Trecutului"],
    res: 0,
    cap: 8,
    lands: 11,
    bank: 0,
    win: 3,
    wn: 7,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Copil Cu Ochi Aurii",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cititor de Gânduri",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Elev Cu Aripi",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Fetiță Cu Gheață",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Profesor de Fizică",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Păzitor de Coridoare",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Lider de Echipă",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Mutant Rătăcit",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Directoarea Cu Voce",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: FLY,
        },
        U {
            name: "Marele Dar Ascuns",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: FLY,
        },
    ],
    spells: [
        S {
            name: "Reînnoire",
            cost: 2,
            fx: 8,
            n: 0,
        },
        S {
            name: "Rază Optică",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Control Mental",
            cost: 1,
            fx: 4,
            n: 1,
        },
        S {
            name: "Mingea de Foc",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Salt Prin Perete",
            cost: 1,
            fx: 6,
            n: 0,
        },
    ],
    deck: [
        [1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 1, 1],
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
