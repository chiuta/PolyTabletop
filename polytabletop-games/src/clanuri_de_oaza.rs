//! Clanurile oazei (`clanuri_de_oaza`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "clanuri_de_oaza",
    title: "Clanurile oazei",
    facs: ["Clanul Palmierului", "Clanul Scorpionului"],
    res: 3,
    cap: 10,
    lands: 0,
    bank: 3,
    win: 1,
    wn: 5,
    big: 4,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Culegător de Curmale",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Fântânar",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Tânăr Săgetător",
            cost: 2,
            atk: 1,
            hp: 4,
            kw: GUARD,
        },
        U {
            name: "Cămilă Încărcată",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Mag de Nisip",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: ARMOR,
        },
        U {
            name: "Garda Oazei",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Șef de Trib",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Vânător de Vipere",
            cost: 4,
            atk: 4,
            hp: 3,
            kw: VENOM,
        },
        U {
            name: "Marele Înțelept",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: HASTE,
        },
        U {
            name: "Gigantul Dunelor",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Ploaie Rară",
            cost: 4,
            fx: 3,
            n: 2,
        },
        S {
            name: "Săgeată de Vânt",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Dans de Foc",
            cost: 1,
            fx: 4,
            n: 1,
        },
        S {
            name: "Pecete de Clan",
            cost: 2,
            fx: 5,
            n: 3,
        },
        S {
            name: "Boală de Praf",
            cost: 1,
            fx: 7,
            n: 1,
        },
    ],
    deck: [
        [4, 4, 2, 2, 3, 3, 1, 1, 2, 1, 1, 1, 1, 2, 2],
        [4, 4, 3, 3, 2, 3, 1, 1, 1, 1, 2, 1, 1, 2, 1],
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
