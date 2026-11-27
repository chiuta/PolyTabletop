//! Corsari și amirali (`corsari_si_amirali`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "corsari_si_amirali",
    title: "Corsari și amirali",
    facs: ["Frăția Pânzelor Negre", "Flota Coroanei"],
    res: 0,
    cap: 8,
    lands: 11,
    bank: 0,
    win: 2,
    wn: 15,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Mus Șmecher",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Marinar Cu Cuțit",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Tunar Surd",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Cercetător din Gabie",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: LEECH,
        },
        U {
            name: "Spadasin de Punte",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Căpitan de Briz",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FIRST,
        },
        U {
            name: "Navigator Vrăjit",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: HASTE,
        },
        U {
            name: "Fregată Rapidă",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: ARMOR,
        },
        U {
            name: "Amiral Încrâncenat",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Vasul-Fantomă",
            cost: 6,
            atk: 5,
            hp: 7,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Salvă de Tun",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Hartă Furată",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Vânt din Pupa",
            cost: 3,
            fx: 4,
            n: 3,
        },
        S {
            name: "Cală Spartă",
            cost: 1,
            fx: 2,
            n: 1,
        },
        S {
            name: "Rom și Prăjituri",
            cost: 1,
            fx: 7,
            n: 1,
        },
    ],
    deck: [
        [1, 1, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1],
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
