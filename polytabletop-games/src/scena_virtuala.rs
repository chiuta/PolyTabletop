//! Scena virtuală (`scena_virtuala`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "scena_virtuala",
    title: "Scena virtuală",
    facs: ["Trupa Aurorei", "Trupa Mării"],
    res: 3,
    cap: 10,
    lands: 0,
    bank: 3,
    win: 2,
    wn: 13,
    big: 0,
    hand0: 6,
    maxt: 14,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Fan Entuziast",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Cântăreață de Început",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Dansator de Fundal",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: LEECH,
        },
        U {
            name: "Chitaristă Zâmbitoare",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Prezentatoare Glumeață",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Regizoare Fantezistă",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Vedetă în Ascensiune",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: LEECH,
        },
        U {
            name: "Idol Strălucitor",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Dirijor de Efecte",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Marea Divă",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: FIRST,
        },
    ],
    spells: [
        S {
            name: "Duet Surpriză",
            cost: 2,
            fx: 7,
            n: 1,
        },
        S {
            name: "Transmisie Live",
            cost: 3,
            fx: 2,
            n: 3,
        },
        S {
            name: "Aplauze",
            cost: 3,
            fx: 4,
            n: 3,
        },
        S {
            name: "Bis",
            cost: 2,
            fx: 8,
            n: 0,
        },
        S {
            name: "Reflector Orb",
            cost: 2,
            fx: 0,
            n: 3,
        },
    ],
    deck: [
        [2, 2, 2, 4, 3, 3, 2, 1, 2, 2, 2, 1, 1, 1, 2],
        [2, 4, 2, 2, 3, 3, 3, 2, 1, 1, 2, 2, 1, 1, 1],
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
