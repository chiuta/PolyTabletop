//! Haita furioasă (`haita_furioasa`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "haita_furioasa",
    title: "Haita furioasă",
    facs: ["Haita Lunii Roșii", "Stejarii Bătrâni"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 0,
    wn: 24,
    big: 0,
    hand0: 6,
    maxt: 14,
    bmax: 6,
    twist: b'K',
    units: [
        U {
            name: "Pui de Lup",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Vânător de Hățiș",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Urs Cenușiu",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Pază de Pădure",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Mistreț Sălbatic",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: LEECH,
        },
        U {
            name: "Vulpe Fantomă",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Alfa Furios",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Căprioară de Foc",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: FIRST,
        },
        U {
            name: "Gigant al Stâncii",
            cost: 5,
            atk: 5,
            hp: 5,
            kw: HASTE,
        },
        U {
            name: "Fiara Primordială",
            cost: 6,
            atk: 5,
            hp: 6,
            kw: VENOM,
        },
    ],
    spells: [
        S {
            name: "Mușcătură",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Chemarea Haitei",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Urlet Cumplit",
            cost: 4,
            fx: 1,
            n: 2,
        },
        S {
            name: "Capcană de Ramuri",
            cost: 2,
            fx: 9,
            n: 0,
        },
        S {
            name: "Furie Oarbă",
            cost: 1,
            fx: 2,
            n: 1,
        },
    ],
    deck: [
        [4, 2, 4, 3, 2, 3, 1, 2, 1, 1, 1, 1, 2, 2, 1],
        [2, 2, 2, 3, 2, 3, 3, 3, 2, 1, 1, 2, 2, 1, 1],
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
