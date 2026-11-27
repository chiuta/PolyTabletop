//! Războinica de bronz (`razboinica_de_bronz`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "razboinica_de_bronz",
    title: "Războinica de bronz",
    facs: ["Taberele Nordului", "Garda Cetății"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 20,
    big: 0,
    hand0: 6,
    maxt: 14,
    bmax: 6,
    twist: b'K',
    units: [
        U {
            name: "Arcaș de Munte",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Păstor Curajos",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Călăreț de Stepă",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Lăncier Bronzat",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Amazoană de Elită",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Căpetenie Tatuată",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Gladiator Vestit",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Prorociță de Piatră",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: ARMOR,
        },
        U {
            name: "Generalul de Oțel",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Marele Cuceritor",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Aruncare de Disc",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Strigăt de Luptă",
            cost: 3,
            fx: 4,
            n: 3,
        },
        S {
            name: "Pavăză Întărită",
            cost: 4,
            fx: 3,
            n: 2,
        },
        S {
            name: "Armură Nouă",
            cost: 5,
            fx: 1,
            n: 3,
        },
        S {
            name: "Sabia Zeilor",
            cost: 4,
            fx: 5,
            n: 9,
        },
    ],
    deck: [
        [4, 4, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1],
        [2, 2, 4, 2, 3, 2, 3, 3, 1, 1, 2, 1, 1, 2, 1],
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
