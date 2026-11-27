//! Muncile semizeului (`muncile_semizeului`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "muncile_semizeului",
    title: "Muncile semizeului",
    facs: ["Eroii Olimpului", "Monștrii Labirintului"],
    res: 1,
    cap: 8,
    lands: 0,
    bank: 0,
    win: 1,
    wn: 5,
    big: 4,
    hand0: 5,
    maxt: 15,
    bmax: 5,
    twist: b'P',
    units: [
        U {
            name: "Păstor de Capre",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Atlet Tânăr",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FIRST,
        },
        U {
            name: "Satir Glumeț",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Nimfă a Izvorului",
            cost: 3,
            atk: 4,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Centaur Arcaș",
            cost: 3,
            atk: 1,
            hp: 4,
            kw: VENOM,
        },
        U {
            name: "Hidră Mică",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Leul de Piatră",
            cost: 4,
            atk: 2,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Taur de Bronz",
            cost: 5,
            atk: 2,
            hp: 9,
            kw: GUARD,
        },
        U {
            name: "Gorgonă Mohorâtă",
            cost: 6,
            atk: 5,
            hp: 8,
            kw: 0,
        },
        U {
            name: "Titanul Încătușat",
            cost: 7,
            atk: 5,
            hp: 10,
            kw: GUARD,
        },
    ],
    spells: [
        S {
            name: "Binecuvântarea Muzelor",
            cost: 5,
            fx: 3,
            n: 3,
        },
        S {
            name: "Lovitură de Trăsnet",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Cântec de Liră",
            cost: 1,
            fx: 7,
            n: 1,
        },
        S {
            name: "Daruri de Nuntă",
            cost: 3,
            fx: 5,
            n: 5,
        },
        S {
            name: "Pietrificare",
            cost: 1,
            fx: 9,
            n: 0,
        },
    ],
    deck: [
        [4, 2, 4, 3, 3, 2, 1, 1, 2, 1, 2, 1, 2, 1, 1],
        [2, 4, 4, 3, 2, 1, 3, 1, 2, 1, 2, 1, 2, 1, 1],
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
