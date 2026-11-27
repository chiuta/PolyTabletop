//! Caravanele de nisip (`caravane_nisip`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "caravane_nisip",
    title: "Caravanele de nisip",
    facs: ["Casa Fântânii", "Casa Șacalului"],
    res: 1,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 2,
    wn: 15,
    big: 0,
    hand0: 5,
    maxt: 16,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Cămilar Grăbit",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Negustor de Apă",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Păzitor de Oază",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Călăreț Mirajelor",
            cost: 3,
            atk: 2,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Scorpion Uriaș",
            cost: 3,
            atk: 1,
            hp: 5,
            kw: FLY,
        },
        U {
            name: "Gardă Cu Voal",
            cost: 4,
            atk: 2,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Maestru de Caravană",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Vierme de Dună",
            cost: 5,
            atk: 3,
            hp: 6,
            kw: VENOM,
        },
        U {
            name: "Vizir Lacom",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Marele Emir",
            cost: 7,
            atk: 6,
            hp: 8,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Furtună de Praf",
            cost: 2,
            fx: 9,
            n: 0,
        },
        S {
            name: "Fântână Ascunsă",
            cost: 1,
            fx: 2,
            n: 1,
        },
        S {
            name: "Drum Întors",
            cost: 1,
            fx: 6,
            n: 0,
        },
        S {
            name: "Lovitură de Iatagan",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Odihnă la Umbră",
            cost: 1,
            fx: 7,
            n: 1,
        },
    ],
    deck: [
        [4, 4, 2, 3, 2, 1, 3, 1, 2, 1, 1, 2, 2, 1, 1],
        [4, 4, 2, 3, 3, 3, 1, 1, 1, 1, 1, 2, 1, 1, 2],
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
