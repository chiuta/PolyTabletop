//! Pașii evoluției (`pasi_de_evolutie`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "pasi_de_evolutie",
    title: "Pașii evoluției",
    facs: ["Cultul Mugurilor", "Cultul Penelor"],
    res: 0,
    cap: 8,
    lands: 11,
    bank: 0,
    win: 0,
    wn: 20,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 6,
    twist: b'K',
    units: [
        U {
            name: "Pui de Floare",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Spiriduș de Ramuri",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Pază de Seară",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Vrăjitoare Cu Pălărie",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Călăreț de Cerb",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FIRST,
        },
        U {
            name: "Dragon de Pergament",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Cavaler Cu Aripi",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: ARMOR,
        },
        U {
            name: "Zeiță Înfloritoare",
            cost: 4,
            atk: 5,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Fiară Dezlănțuită",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Zmeul Metamorfic",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Mugur Tăcut",
            cost: 3,
            fx: 4,
            n: 3,
        },
        S {
            name: "Înflorire",
            cost: 4,
            fx: 3,
            n: 2,
        },
        S {
            name: "Lovitură Rapidă",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Cercul Schimbării",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Sfărâmare",
            cost: 3,
            fx: 8,
            n: 0,
        },
    ],
    deck: [
        [1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        [3, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
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
