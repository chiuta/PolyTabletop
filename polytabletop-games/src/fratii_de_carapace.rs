//! Frații de carapace (`fratii_de_carapace`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "fratii_de_carapace",
    title: "Frații de carapace",
    facs: ["Frații din Canal", "Clanul Labei"],
    res: 1,
    cap: 9,
    lands: 0,
    bank: 0,
    win: 2,
    wn: 15,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'S',
    units: [
        U {
            name: "Ucenic de Canal",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Ninja Mărunt",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Pizzar Grăbit",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Savant Ciudat",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Țestoasă Cu Băț",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Țestoasă Cu Sabie",
            cost: 3,
            atk: 2,
            hp: 4,
            kw: FLY,
        },
        U {
            name: "Mutant Cu Gheare",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Maestru Șobolan",
            cost: 4,
            atk: 3,
            hp: 5,
            kw: FLY,
        },
        U {
            name: "Robot Cu Creier",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Stăpânul Din Turn",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: ARMOR,
        },
    ],
    spells: [
        S {
            name: "Lovitură Cu Bățul",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Pizza Caldă",
            cost: 5,
            fx: 3,
            n: 3,
        },
        S {
            name: "Salt de Pe Acoperiș",
            cost: 3,
            fx: 8,
            n: 0,
        },
        S {
            name: "Cutie Misterioasă",
            cost: 2,
            fx: 4,
            n: 2,
        },
        S {
            name: "Ceață Fumigenă",
            cost: 2,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [2, 4, 3, 3, 3, 2, 3, 1, 1, 1, 1, 2, 1, 1, 2],
        [4, 4, 3, 2, 3, 2, 1, 2, 1, 1, 1, 1, 1, 2, 2],
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
