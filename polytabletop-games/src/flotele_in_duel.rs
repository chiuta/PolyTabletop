//! Flotele în duel (`flotele_in_duel`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "flotele_in_duel",
    title: "Flotele în duel",
    facs: ["Armada Albastră", "Flota Răzvrătită"],
    res: 0,
    cap: 8,
    lands: 12,
    bank: 0,
    win: 3,
    wn: 6,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Navă de Marfă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Vânător Ușor",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: GUARD,
        },
        U {
            name: "Corvetă Iute",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Navă de Asalt",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Bombardier Greoi",
            cost: 3,
            atk: 5,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Fregată Blindată",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Distrugător Greu",
            cost: 4,
            atk: 6,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Nava Comandă",
            cost: 4,
            atk: 7,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Gigant de Război",
            cost: 5,
            atk: 6,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Stația Mobilă",
            cost: 6,
            atk: 7,
            hp: 4,
            kw: VENOM,
        },
    ],
    spells: [
        S {
            name: "Bombardament Stelar",
            cost: 3,
            fx: 1,
            n: 1,
        },
        S {
            name: "Salvă Unită",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Zid de Scuturi",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Capcană Magnetică",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Alarmă Generală",
            cost: 2,
            fx: 2,
            n: 2,
        },
    ],
    deck: [
        [1, 1, 2, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0],
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
