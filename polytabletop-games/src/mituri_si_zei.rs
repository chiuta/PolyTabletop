//! Mituri și zei (`mituri_si_zei`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "mituri_si_zei",
    title: "Mituri și zei",
    facs: ["Cetele Soarelui", "Cetele Trăsnetului"],
    res: 0,
    cap: 8,
    lands: 12,
    bank: 0,
    win: 1,
    wn: 5,
    big: 4,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'H',
    units: [
        U {
            name: "Păstor de Munte",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cântăreț de Șoapte",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Războinic Tânăr",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Sătean Curajos",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Lăncier Zeificat",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Vânător de Dragoni",
            cost: 3,
            atk: 2,
            hp: 3,
            kw: VENOM,
        },
        U {
            name: "Semizeu Brav",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Demon al Prăpastiei",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Zeul Furtunii",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: FIRST,
        },
        U {
            name: "Titanul Veșnic",
            cost: 6,
            atk: 4,
            hp: 9,
            kw: GUARD,
        },
    ],
    spells: [
        S {
            name: "Rază de Soare",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Binecuvântare Zeiască",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Capcană de Ceață",
            cost: 2,
            fx: 9,
            n: 0,
        },
        S {
            name: "Pedeapsa Cerului",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Jertfă Curată",
            cost: 1,
            fx: 7,
            n: 1,
        },
    ],
    deck: [
        [1, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1],
        [2, 2, 2, 2, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
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
