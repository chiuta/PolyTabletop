//! Marginea nevăzută (`marginea_nevazuta`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "marginea_nevazuta",
    title: "Marginea nevăzută",
    facs: ["Cercul Veghetorilor", "Societatea Uitării"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 3,
    wn: 7,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'S',
    units: [
        U {
            name: "Martor Tulburat",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cititor de Semne",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Medium Mărunt",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Colecționar de Ecouri",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Pază Fără Chip",
            cost: 3,
            atk: 1,
            hp: 4,
            kw: VENOM,
        },
        U {
            name: "Străjer al Pragului",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Vânător de Fantome",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Oracol Orb",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: FIRST,
        },
        U {
            name: "Gardian al Golului",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Marele Neștiut",
            cost: 6,
            atk: 5,
            hp: 7,
            kw: FLY,
        },
    ],
    spells: [
        S {
            name: "Fulger de Eter",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Zid de Ceață",
            cost: 2,
            fx: 9,
            n: 0,
        },
        S {
            name: "Revelație",
            cost: 1,
            fx: 6,
            n: 0,
        },
        S {
            name: "Rătăcire",
            cost: 3,
            fx: 2,
            n: 3,
        },
        S {
            name: "Dizolvare",
            cost: 2,
            fx: 5,
            n: 3,
        },
    ],
    deck: [
        [4, 4, 2, 3, 3, 2, 1, 1, 1, 2, 2, 1, 1, 1, 2],
        [4, 2, 3, 2, 2, 2, 3, 1, 2, 2, 1, 1, 2, 2, 1],
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
