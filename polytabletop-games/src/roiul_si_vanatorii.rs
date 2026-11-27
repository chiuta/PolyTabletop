//! Roiul și vânătorii (`roiul_si_vanatorii`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "roiul_si_vanatorii",
    title: "Roiul și vânătorii",
    facs: ["Roiul din Peșteri", "Vânătorii de Trofee"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 3,
    wn: 7,
    big: 0,
    hand0: 5,
    maxt: 14,
    bmax: 6,
    twist: b'H',
    units: [
        U {
            name: "Larvă Flămândă",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Pândar Lacom",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Alergător Cu Gheare",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: HASTE,
        },
        U {
            name: "Gonitor Subteran",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Soldat Chitinos",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: ARMOR,
        },
        U {
            name: "Vânător Cu Mască",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Ucigaș Tăcut",
            cost: 3,
            atk: 3,
            hp: 2,
            kw: VENOM,
        },
        U {
            name: "Matcă Mică",
            cost: 4,
            atk: 6,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Căpetenie Cu Coarne",
            cost: 5,
            atk: 7,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Regina Roiului",
            cost: 6,
            atk: 7,
            hp: 5,
            kw: HASTE,
        },
    ],
    spells: [
        S {
            name: "Sfâșiere",
            cost: 5,
            fx: 1,
            n: 3,
        },
        S {
            name: "Tălpi Sub Pământ",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Eclozare",
            cost: 4,
            fx: 3,
            n: 2,
        },
        S {
            name: "Rețea de Plasă",
            cost: 1,
            fx: 4,
            n: 1,
        },
        S {
            name: "Capcană Termică",
            cost: 1,
            fx: 2,
            n: 1,
        },
    ],
    deck: [
        [4, 2, 2, 3, 3, 2, 2, 2, 2, 1, 1, 1, 2, 2, 1],
        [4, 2, 4, 3, 2, 3, 2, 1, 1, 1, 2, 1, 2, 1, 1],
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
