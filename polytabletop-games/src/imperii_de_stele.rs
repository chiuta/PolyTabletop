//! Imperii de stele (`imperii_de_stele`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "imperii_de_stele",
    title: "Imperii de stele",
    facs: ["Imperiul Zorilor", "Liga Neagră"],
    res: 1,
    cap: 9,
    lands: 0,
    bank: 0,
    win: 1,
    wn: 4,
    big: 5,
    hand0: 5,
    maxt: 14,
    bmax: 6,
    twist: b'-',
    units: [
        U {
            name: "Pilot de Patrulă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Dronă de Recunoaștere",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Fregată Ușoară",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Nava-Spital",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Crucișător Mic",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Distrugător",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Cuirasat Pitic",
            cost: 4,
            atk: 5,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Nava Amiral",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Stație de Luptă",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Planeta-Fortăreață",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: GUARD,
        },
    ],
    spells: [
        S {
            name: "Salvă Orbitală",
            cost: 3,
            fx: 1,
            n: 1,
        },
        S {
            name: "Scut Reînnoit",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Armistițiu Rupt",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Reparații",
            cost: 2,
            fx: 7,
            n: 1,
        },
        S {
            name: "Mobilizare",
            cost: 2,
            fx: 8,
            n: 0,
        },
    ],
    deck: [
        [2, 4, 4, 2, 3, 2, 1, 1, 2, 2, 1, 2, 1, 2, 1],
        [4, 4, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1],
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
