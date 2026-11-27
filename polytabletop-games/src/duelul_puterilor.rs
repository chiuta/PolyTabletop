//! Duelul puterilor (`duelul_puterilor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "duelul_puterilor",
    title: "Duelul puterilor",
    facs: ["Liga Strălucirii", "Frăția Prăpastiei"],
    res: 1,
    cap: 9,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 22,
    big: 0,
    hand0: 6,
    maxt: 14,
    bmax: 5,
    twist: b'P',
    units: [
        U {
            name: "Cursă Rapidă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Mânuitor de Scut",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Om-Piatră",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Zburător Solar",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Telepat Șoptitor",
            cost: 3,
            atk: 5,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Arcaș de Plasmă",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Colos de Fier",
            cost: 4,
            atk: 7,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Fulgerul Alb",
            cost: 4,
            atk: 5,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Titan Subteran",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: GUARD,
        },
        U {
            name: "Zeul Furtunii",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: ARMOR,
        },
    ],
    spells: [
        S {
            name: "Rafală de Energie",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Voință de Oțel",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Forță Înzecită",
            cost: 3,
            fx: 4,
            n: 3,
        },
        S {
            name: "Prăbușirea Puterii",
            cost: 3,
            fx: 5,
            n: 5,
        },
        S {
            name: "Tăișul Zilei",
            cost: 5,
            fx: 1,
            n: 3,
        },
    ],
    deck: [
        [4, 2, 2, 4, 2, 3, 1, 1, 2, 2, 1, 1, 1, 2, 2],
        [4, 2, 2, 2, 3, 2, 1, 3, 2, 2, 2, 2, 1, 1, 1],
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
