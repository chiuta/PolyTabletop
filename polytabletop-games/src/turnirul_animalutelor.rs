//! Turnirul animăluțelor (`turnirul_animalutelor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "turnirul_animalutelor",
    title: "Turnirul animăluțelor",
    facs: ["Echipa Fulgerilor", "Echipa Frunzelor"],
    res: 0,
    cap: 8,
    lands: 10,
    bank: 0,
    win: 2,
    wn: 14,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'K',
    units: [
        U {
            name: "Pisoi Jucăuș",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Iepuraș Iute",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Căței Zglobiu",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Pasăre Cântătoare",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Vulpiță Isteață",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: FIRST,
        },
        U {
            name: "Dragonel de Buzunar",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Urs de Pluș",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Fluture Uriaș",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Grifon Tânăr",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: ARMOR,
        },
        U {
            name: "Marele Campion",
            cost: 6,
            atk: 7,
            hp: 6,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Rația Dublă",
            cost: 3,
            fx: 2,
            n: 3,
        },
        S {
            name: "Masaj Bun",
            cost: 1,
            fx: 7,
            n: 1,
        },
        S {
            name: "Săritură Ușoară",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Premiul Mare",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Dorm Puțin",
            cost: 3,
            fx: 8,
            n: 0,
        },
    ],
    deck: [
        [3, 3, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0, 1],
        [2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1],
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
