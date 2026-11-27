//! Păpușile dorinței (`papusile_dorintei`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "papusile_dorintei",
    title: "Păpușile dorinței",
    facs: ["Atelierul Porțelanului", "Cutia Cu Chei"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 3,
    wn: 8,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'P',
    units: [
        U {
            name: "Păpușă de Cârpă",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Păpușă-Cheie",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Păpușă Cu Panglică",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Păpușă de Sticlă",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Păpușă Cu Voal",
            cost: 3,
            atk: 1,
            hp: 5,
            kw: ARMOR,
        },
        U {
            name: "Păpușă Veghetoare",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Păpușă Cu Ceas",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Păpușă Fără Chip",
            cost: 5,
            atk: 4,
            hp: 5,
            kw: VENOM,
        },
        U {
            name: "Păpușă Regină",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: FLY,
        },
        U {
            name: "Marea Păpușă Dorită",
            cost: 7,
            atk: 7,
            hp: 8,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Dorință Mică",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Fir de Păpușar",
            cost: 4,
            fx: 3,
            n: 2,
        },
        S {
            name: "Retragere în Cutie",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Dar Ascuns",
            cost: 1,
            fx: 7,
            n: 1,
        },
        S {
            name: "Dorință Rea",
            cost: 1,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [3, 2, 4, 2, 2, 3, 3, 1, 2, 1, 1, 1, 1, 2, 2],
        [4, 4, 2, 3, 2, 3, 2, 1, 1, 1, 2, 1, 1, 2, 1],
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
