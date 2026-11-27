//! Străjerii cerurilor (`straji_ceruri`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "straji_ceruri",
    title: "Străjerii cerurilor",
    facs: ["Oștirea Aurorei", "Legiunea Amurgului"],
    res: 0,
    cap: 8,
    lands: 12,
    bank: 0,
    win: 0,
    wn: 20,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 6,
    twist: b'H',
    units: [
        U {
            name: "Herald Mic",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Paznic de Nor",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Lăncier Luminos",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cântăreț de Coastă",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Arhanghel Tânăr",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: LEECH,
        },
        U {
            name: "Străjer de Poartă",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Războinic Înaripat",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: ARMOR,
        },
        U {
            name: "Judecător de Foc",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Serafim Albastru",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: FLY,
        },
        U {
            name: "Marele Străjer",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: GUARD,
        },
    ],
    spells: [
        S {
            name: "Rază Sfântă",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Aripi de Argint",
            cost: 5,
            fx: 3,
            n: 3,
        },
        S {
            name: "Binecuvântare",
            cost: 3,
            fx: 7,
            n: 2,
        },
        S {
            name: "Pământ Curățat",
            cost: 4,
            fx: 1,
            n: 2,
        },
        S {
            name: "Trezire",
            cost: 2,
            fx: 8,
            n: 0,
        },
    ],
    deck: [
        [2, 2, 2, 1, 1, 2, 1, 1, 1, 0, 1, 1, 1, 1, 1],
        [2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
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
