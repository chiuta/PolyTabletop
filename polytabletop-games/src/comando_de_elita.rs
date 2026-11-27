//! Comando de elită (`comando_de_elita`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "comando_de_elita",
    title: "Comando de elită",
    facs: ["Echipa Albastră", "Rețeaua Roșie"],
    res: 2,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 22,
    big: 0,
    hand0: 6,
    maxt: 14,
    bmax: 6,
    twist: b'P',
    units: [
        U {
            name: "Soldat Cercetaș",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Pușcaș de Elită",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Medic de Teren",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: HASTE,
        },
        U {
            name: "Parașutist",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Tunar Greu",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Specialist în Demolări",
            cost: 3,
            atk: 4,
            hp: 2,
            kw: ARMOR,
        },
        U {
            name: "Pilot de Elicopter",
            cost: 4,
            atk: 7,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Căpitan de Echipă",
            cost: 4,
            atk: 3,
            hp: 4,
            kw: VENOM,
        },
        U {
            name: "Tanc Ușor",
            cost: 5,
            atk: 8,
            hp: 3,
            kw: 0,
        },
        U {
            name: "General Cu Ochelari",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: FIRST,
        },
    ],
    spells: [
        S {
            name: "Foc de Acoperire",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Grenadă",
            cost: 3,
            fx: 1,
            n: 1,
        },
        S {
            name: "Parașutare",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Încărcător Nou",
            cost: 1,
            fx: 4,
            n: 1,
        },
        S {
            name: "Asalt General",
            cost: 2,
            fx: 6,
            n: 0,
        },
    ],
    deck: [
        [4, 2, 3, 3, 2, 2, 3, 1, 1, 2, 1, 2, 1, 1, 2],
        [2, 2, 4, 2, 3, 2, 3, 1, 2, 2, 1, 2, 2, 1, 1],
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
