//! Mașinile orei zero (`masini_ceas_zero`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "masini_ceas_zero",
    title: "Mașinile orei zero",
    facs: ["Rezistența Subterană", "Rețeaua de Oțel"],
    res: 1,
    cap: 10,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 24,
    big: 0,
    hand0: 5,
    maxt: 14,
    bmax: 6,
    twist: b'S',
    units: [
        U {
            name: "Scrutător Rapid",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Dronă de Pază",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: GUARD,
        },
        U {
            name: "Infiltrator Mecanic",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Vânător de Cabluri",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Tanc Păianjen",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Soldat Cromat",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: ARMOR,
        },
        U {
            name: "Distrugător de Bastion",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Comandant Metalic",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Turn de Foc",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Nucleul Neînvins",
            cost: 6,
            atk: 7,
            hp: 4,
            kw: VENOM,
        },
    ],
    spells: [
        S {
            name: "Fascicul de Plasmă",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Ucidere Țintită",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Salt în Timp",
            cost: 4,
            fx: 1,
            n: 2,
        },
        S {
            name: "Supraîncărcare",
            cost: 2,
            fx: 3,
            n: 1,
        },
        S {
            name: "Gaură Neagră",
            cost: 1,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [4, 4, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 1, 1, 2],
        [2, 2, 2, 2, 2, 3, 3, 3, 2, 2, 1, 1, 1, 2, 2],
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
