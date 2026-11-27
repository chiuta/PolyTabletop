//! Cercurile nopții (`cercurile_noptii`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cercurile_noptii",
    title: "Cercurile nopții",
    facs: ["Cercul Trandafirului", "Cercul Lanțului"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 1,
    wn: 4,
    big: 4,
    hand0: 6,
    maxt: 14,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Servitor de Noapte",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Muzician Palid",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Păstrător de Taine",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Curtezan Pătat",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Duelist Elegant",
            cost: 3,
            atk: 3,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Lup-Chip",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Erudit Sângeriu",
            cost: 4,
            atk: 7,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Prinț al Mahalalei",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Bătrân de Cavou",
            cost: 5,
            atk: 5,
            hp: 4,
            kw: VENOM,
        },
        U {
            name: "Arhonte Ascuns",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: FLY,
        },
    ],
    spells: [
        S {
            name: "Pecete Roșie",
            cost: 2,
            fx: 5,
            n: 3,
        },
        S {
            name: "Mască Falsă",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Dar al Sângelui",
            cost: 1,
            fx: 2,
            n: 1,
        },
        S {
            name: "Cădere Sub Voal",
            cost: 1,
            fx: 6,
            n: 0,
        },
        S {
            name: "Plecăciune Mortală",
            cost: 2,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [2, 4, 3, 2, 2, 2, 3, 1, 2, 2, 2, 2, 1, 1, 1],
        [2, 4, 4, 3, 3, 2, 1, 1, 2, 1, 2, 1, 1, 2, 1],
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
