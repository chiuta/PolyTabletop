//! Căutătorii de cripte (`cautatorii_de_cripte`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cautatorii_de_cripte",
    title: "Căutătorii de cripte",
    facs: ["Expediția Aurie", "Șacalii Cenușii"],
    res: 0,
    cap: 8,
    lands: 12,
    bank: 0,
    win: 3,
    wn: 8,
    big: 0,
    hand0: 5,
    maxt: 16,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Hamal de Torțe",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cercetător Zelos",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Cățărător Iute",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Pușcaș de Capcană",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Hoț de Morminte",
            cost: 3,
            atk: 5,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Cartograf Neînfricat",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Paznic Împietrit",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Mumie Încătușată",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Gardian de Sarcofag",
            cost: 5,
            atk: 6,
            hp: 3,
            kw: VENOM,
        },
        U {
            name: "Regele Îngropat",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: ARMOR,
        },
    ],
    spells: [
        S {
            name: "Aruncare de Cuțit",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Hartă Veche",
            cost: 2,
            fx: 2,
            n: 2,
        },
        S {
            name: "Prăbușirea Tavanului",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Cheie Fermecată",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Frânghie și Cârlig",
            cost: 3,
            fx: 4,
            n: 3,
        },
    ],
    deck: [
        [2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0, 1, 1],
        [2, 2, 1, 2, 2, 2, 1, 1, 1, 0, 1, 1, 1, 0, 1],
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
