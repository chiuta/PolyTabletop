//! Vânătoarea de noapte (`vanatoarea_de_noapte`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "vanatoarea_de_noapte",
    title: "Vânătoarea de noapte",
    facs: ["Vânătorii din Liceu", "Frăția Colților"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 1,
    wn: 4,
    big: 5,
    hand0: 6,
    maxt: 14,
    bmax: 5,
    twist: b'K',
    units: [
        U {
            name: "Elevă Curajoasă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Bibliotecar Șmecher",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Vrăjitor Mărunt",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Atlet de Seară",
            cost: 2,
            atk: 4,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Vânătoare de Stafii",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Vampir Tânăr",
            cost: 3,
            atk: 3,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Căpitan de Echipă",
            cost: 4,
            atk: 5,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Demon Grațios",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Lider al Cavoului",
            cost: 5,
            atk: 7,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Stăpânul Nopții",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: FIRST,
        },
    ],
    spells: [
        S {
            name: "Țăruș de Lemn",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Argint în Buzunar",
            cost: 2,
            fx: 5,
            n: 3,
        },
        S {
            name: "Beție Neagră",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Ritual de Vindecare",
            cost: 2,
            fx: 2,
            n: 2,
        },
        S {
            name: "Slujbă Rapidă",
            cost: 3,
            fx: 7,
            n: 2,
        },
    ],
    deck: [
        [2, 2, 2, 3, 3, 3, 3, 3, 1, 1, 2, 1, 1, 2, 1],
        [2, 2, 2, 4, 2, 2, 3, 3, 2, 1, 1, 1, 2, 2, 1],
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
