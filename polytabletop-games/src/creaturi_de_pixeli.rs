//! Creaturi de pixeli (`creaturi_de_pixeli`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "creaturi_de_pixeli",
    title: "Creaturi de pixeli",
    facs: ["Echipa Verde", "Echipa Violet"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 2,
    wn: 14,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 6,
    twist: b'H',
    units: [
        U {
            name: "Boț Pufos",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Pui de Pixel",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Sprinter Cubic",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Dragonel de Octeți",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Cățel de Cod",
            cost: 2,
            atk: 2,
            hp: 1,
            kw: VENOM,
        },
        U {
            name: "Șarpe Cu Ecran",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Gigant de Biți",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Zburător Holografic",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: HASTE,
        },
        U {
            name: "Gazda Cuantică",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Dragonul Suprem",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Evoluție Rapidă",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Upgrade Total",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Atac de Pixeli",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Cartela Norocoasă",
            cost: 1,
            fx: 2,
            n: 1,
        },
        S {
            name: "Reîncărcare",
            cost: 3,
            fx: 8,
            n: 0,
        },
    ],
    deck: [
        [4, 2, 4, 2, 3, 2, 2, 2, 1, 1, 1, 1, 2, 2, 1],
        [4, 4, 2, 2, 2, 3, 3, 1, 2, 1, 2, 1, 1, 1, 1],
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
