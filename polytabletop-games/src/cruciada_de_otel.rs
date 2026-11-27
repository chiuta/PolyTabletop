//! Cruciada de oțel (`cruciada_de_otel`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "cruciada_de_otel",
    title: "Cruciada de oțel",
    facs: ["Oștirea Sfântă", "Hoarda Fărâmițată"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 24,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 6,
    twist: b'S',
    units: [
        U {
            name: "Pușcaș Înnegrit",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Penitent Cu Flacără",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Sergent de Oțel",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Tunar Mecanic",
            cost: 3,
            atk: 1,
            hp: 5,
            kw: LEECH,
        },
        U {
            name: "Cavaler în Armură",
            cost: 3,
            atk: 2,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Preot de Război",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Titan Mărunt",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Căpitan Cu Sabie",
            cost: 5,
            atk: 2,
            hp: 9,
            kw: GUARD,
        },
        U {
            name: "Mașina Sfântă",
            cost: 6,
            atk: 6,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Marele Cruciat",
            cost: 7,
            atk: 6,
            hp: 7,
            kw: VENOM,
        },
    ],
    spells: [
        S {
            name: "Bombă de Asediu",
            cost: 3,
            fx: 5,
            n: 5,
        },
        S {
            name: "Rugăciune de Luptă",
            cost: 5,
            fx: 1,
            n: 3,
        },
        S {
            name: "Atac Orbital",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Lovitură Dreaptă",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Binecuvântare Aprinsă",
            cost: 3,
            fx: 7,
            n: 2,
        },
    ],
    deck: [
        [4, 4, 4, 2, 2, 1, 3, 1, 2, 0, 2, 1, 2, 1, 1],
        [2, 4, 2, 2, 3, 3, 3, 1, 2, 1, 2, 1, 1, 1, 2],
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
