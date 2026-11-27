//! Dosare sub sigiliu (`dosare_sub_sigiliu`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "dosare_sub_sigiliu",
    title: "Dosare sub sigiliu",
    facs: ["Biroul Anchetelor", "Agenția Tăcută"],
    res: 0,
    cap: 9,
    lands: 11,
    bank: 0,
    win: 3,
    wn: 6,
    big: 0,
    hand0: 6,
    maxt: 16,
    bmax: 5,
    twist: b'K',
    units: [
        U {
            name: "Stagiar Curios",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Arhivist",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Fotograf de Teren",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FIRST,
        },
        U {
            name: "Interpret de Semnale",
            cost: 3,
            atk: 2,
            hp: 5,
            kw: 0,
        },
        U {
            name: "Agent Dublu",
            cost: 3,
            atk: 1,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Expert în Neexplicabil",
            cost: 4,
            atk: 2,
            hp: 7,
            kw: GUARD,
        },
        U {
            name: "Șef de Birou",
            cost: 4,
            atk: 2,
            hp: 6,
            kw: FLY,
        },
        U {
            name: "Cercetător Dispărut",
            cost: 5,
            atk: 3,
            hp: 6,
            kw: VENOM,
        },
        U {
            name: "Martor Ascuns",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: 0,
        },
        U {
            name: "Directorul Din Umbră",
            cost: 7,
            atk: 6,
            hp: 8,
            kw: ARMOR,
        },
    ],
    spells: [
        S {
            name: "Interogatoriu",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Dosar Deschis",
            cost: 2,
            fx: 2,
            n: 2,
        },
        S {
            name: "Dispariție",
            cost: 3,
            fx: 5,
            n: 5,
        },
        S {
            name: "Cură de Odihnă",
            cost: 3,
            fx: 7,
            n: 2,
        },
        S {
            name: "Mandat de Căutare",
            cost: 2,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [3, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
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
