//! Ucenicii luminii (`ucenicii_luminii`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "ucenicii_luminii",
    title: "Ucenicii luminii",
    facs: ["Școala Zorilor", "Școala Cețurilor"],
    res: 0,
    cap: 7,
    lands: 10,
    bank: 0,
    win: 0,
    wn: 18,
    big: 0,
    hand0: 7,
    maxt: 16,
    bmax: 5,
    twist: b'H',
    units: [
        U {
            name: "Novice Zâmbitor",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Pază Mică",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Copil Cu Bețișor",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Elev Neastâmpărat",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Pilot de Antrenament",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Ucenic Strălucitor",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Maestru de Sabie",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Gardian de Templu",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Cavaler Întunecat",
            cost: 5,
            atk: 6,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Marele Îndrumător",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: LEECH,
        },
    ],
    spells: [
        S {
            name: "Antrenament Dublu",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Rază Slabă",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Meditație Adâncă",
            cost: 1,
            fx: 2,
            n: 1,
        },
        S {
            name: "Binecuvântarea Maestrului",
            cost: 2,
            fx: 7,
            n: 1,
        },
        S {
            name: "Unda Liniștirii",
            cost: 2,
            fx: 3,
            n: 1,
        },
    ],
    deck: [
        [1, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        [2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1],
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
