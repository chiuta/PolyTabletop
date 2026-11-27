//! Umbre de neon (`umbre_de_neon`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "umbre_de_neon",
    title: "Umbre de neon",
    facs: ["Gașca Străzii", "Corporația Aurie"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 20,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'P',
    units: [
        U {
            name: "Curier pe Bicicletă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Hoț de Buzunar",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Hacker de Cartier",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Samurai de Stradă",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Șaman Urban",
            cost: 3,
            atk: 5,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Mercenar Cromat",
            cost: 3,
            atk: 5,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Drone-Căpitan",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: LEECH,
        },
        U {
            name: "Fixer Șiret",
            cost: 4,
            atk: 4,
            hp: 3,
            kw: VENOM,
        },
        U {
            name: "Asasin Fantomă",
            cost: 5,
            atk: 7,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Dragonul Din Turn",
            cost: 6,
            atk: 9,
            hp: 4,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Glonț Fantomă",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Mască Falsă",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Ținta Pierdută",
            cost: 1,
            fx: 6,
            n: 0,
        },
        S {
            name: "Injecție",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Pană de Curent",
            cost: 5,
            fx: 1,
            n: 3,
        },
    ],
    deck: [
        [4, 2, 2, 3, 3, 2, 3, 1, 1, 2, 1, 2, 1, 2, 1],
        [2, 2, 4, 3, 2, 3, 1, 3, 1, 2, 1, 2, 1, 2, 1],
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
