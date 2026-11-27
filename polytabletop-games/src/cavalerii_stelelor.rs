//! Cavalerii stelelor (`cavalerii_stelelor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cavalerii_stelelor",
    title: "Cavalerii stelelor",
    facs: ["Ordinul Strălucitor", "Cetatea Neagră"],
    res: 1,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 20,
    big: 0,
    hand0: 5,
    maxt: 16,
    bmax: 5,
    twist: b'P',
    units: [
        U {
            name: "Scutier Silitor",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Lăncier Spațial",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Pilot de Recunoaștere",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Duelist de Gardă",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Cavaler Mărunt",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: LEECH,
        },
        U {
            name: "Paznic de Pod",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Mistic Tăcut",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Maestru-Cavaler",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Căpitan Înzăuat",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Marele Magistru",
            cost: 6,
            atk: 5,
            hp: 7,
            kw: FIRST,
        },
    ],
    spells: [
        S {
            name: "Lamă Spirituală",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Pavăza Credinței",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Salt de Forță",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Cântec Calm",
            cost: 1,
            fx: 7,
            n: 1,
        },
        S {
            name: "Furtună de Stele",
            cost: 4,
            fx: 1,
            n: 2,
        },
    ],
    deck: [
        [4, 4, 2, 3, 2, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1],
        [4, 4, 2, 3, 2, 3, 1, 1, 1, 2, 2, 2, 1, 1, 1],
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
