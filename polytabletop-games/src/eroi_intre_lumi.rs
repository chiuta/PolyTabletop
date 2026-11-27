//! Eroi între lumi (`eroi_intre_lumi`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "eroi_intre_lumi",
    title: "Eroi între lumi",
    facs: ["Tărâmul Cheii", "Tărâmul Furtunii"],
    res: 1,
    cap: 9,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 22,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 6,
    twist: b'-',
    units: [
        U {
            name: "Cercetaș de Graniță",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Călător Cu Hartă",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: GUARD,
        },
        U {
            name: "Pază de Prag",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Meșter de Aburi",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Mag al Cheii",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Cavaler al Portalului",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Exploratoare de Lumi",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Eroul Rătăcit",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Căpitan Dintre Lumi",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: FLY,
        },
        U {
            name: "Marele Cartograf",
            cost: 6,
            atk: 7,
            hp: 4,
            kw: VENOM,
        },
    ],
    spells: [
        S {
            name: "Val de Pietre",
            cost: 3,
            fx: 1,
            n: 1,
        },
        S {
            name: "Salt Între Lumi",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Arma Aleasă",
            cost: 3,
            fx: 4,
            n: 3,
        },
        S {
            name: "Ucidere Țintită",
            cost: 3,
            fx: 5,
            n: 5,
        },
        S {
            name: "Conexiune",
            cost: 1,
            fx: 2,
            n: 1,
        },
    ],
    deck: [
        [2, 4, 3, 2, 3, 2, 1, 3, 2, 1, 1, 2, 1, 1, 2],
        [2, 2, 2, 4, 2, 3, 1, 3, 2, 2, 1, 2, 2, 1, 1],
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
