//! Spărgătorii de rețea (`spargatori_retea`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "spargatori_retea",
    title: "Spărgătorii de rețea",
    facs: ["Echipa Furișată", "Paznicii Nodului"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 2,
    wn: 14,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Curier Fantomă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Sondă Mută",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Spărgător de Parole",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Păianjen de Cod",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Sentinelă Digitală",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Vierme de Releu",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Gardian de Pod",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Arhitect de Breșe",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Ceață Neagră",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Nucleul Orb",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Impuls Invers",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Descărcare Rapidă",
            cost: 1,
            fx: 2,
            n: 1,
        },
        S {
            name: "Îngheț de Port",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Retragere Forțată",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Ghid de Spargere",
            cost: 3,
            fx: 4,
            n: 3,
        },
    ],
    deck: [
        [4, 4, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 2, 1, 2],
        [2, 2, 2, 2, 3, 3, 2, 3, 2, 2, 1, 1, 2, 1, 2],
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
