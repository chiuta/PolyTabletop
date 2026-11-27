//! Gardienele de cristal (`gardiene_de_cristal`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "gardiene_de_cristal",
    title: "Gardienele de cristal",
    facs: ["Cercul Lunii", "Cercul Umbrei"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 0,
    wn: 22,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Fetiță Cu Stea",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Prietenă Credincioasă",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: LEECH,
        },
        U {
            name: "Dansatoare de Rouă",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Gardiană a Florii",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cântăreață de Argint",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Războinică Cu Panglică",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Prințesă Din Cer",
            cost: 4,
            atk: 4,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Regina Cristalului",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: ARMOR,
        },
        U {
            name: "Vrăjitoare Neagră",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Sfera Lunii Pline",
            cost: 6,
            atk: 5,
            hp: 7,
            kw: FLY,
        },
    ],
    spells: [
        S {
            name: "Dans de Primăvară",
            cost: 3,
            fx: 7,
            n: 2,
        },
        S {
            name: "Rază de Lună",
            cost: 2,
            fx: 0,
            n: 3,
        },
        S {
            name: "Petale Tămăduitoare",
            cost: 5,
            fx: 3,
            n: 3,
        },
        S {
            name: "Chemarea Prietenelor",
            cost: 2,
            fx: 8,
            n: 0,
        },
        S {
            name: "Strălucire",
            cost: 3,
            fx: 4,
            n: 3,
        },
    ],
    deck: [
        [4, 2, 2, 2, 3, 3, 2, 1, 2, 2, 1, 1, 1, 2, 2],
        [4, 4, 2, 3, 2, 3, 2, 1, 1, 1, 2, 2, 1, 1, 1],
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
