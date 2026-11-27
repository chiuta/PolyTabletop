//! Șoapte din abis (`soapte_din_abis`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "soapte_din_abis",
    title: "Șoapte din abis",
    facs: ["Veghea Clopotului", "Ucenicii Mării"],
    res: 1,
    cap: 9,
    lands: 0,
    bank: 0,
    win: 0,
    wn: 24,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'S',
    units: [
        U {
            name: "Pescar Rătăcit",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Cărturar Neliniștit",
            cost: 2,
            atk: 3,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Locuitor de Far",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: LEECH,
        },
        U {
            name: "Prunc cu Ochi Mari",
            cost: 3,
            atk: 3,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Anchetator Obosit",
            cost: 3,
            atk: 2,
            hp: 5,
            kw: GUARD,
        },
        U {
            name: "Tentacul Plutitor",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Preot al Valurilor",
            cost: 4,
            atk: 2,
            hp: 5,
            kw: VENOM,
        },
        U {
            name: "Dormitor din Adânc",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: ARMOR,
        },
        U {
            name: "Chip Fără Nume",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: 0,
        },
        U {
            name: "Marele Dinafară",
            cost: 7,
            atk: 4,
            hp: 9,
            kw: VENOM,
        },
    ],
    spells: [
        S {
            name: "Ruinarea Minții",
            cost: 3,
            fx: 5,
            n: 5,
        },
        S {
            name: "Maree Neagră",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Șoaptă Tulburătoare",
            cost: 1,
            fx: 0,
            n: 2,
        },
        S {
            name: "Cântec de Adormire",
            cost: 2,
            fx: 9,
            n: 0,
        },
        S {
            name: "Rupere de Ceas",
            cost: 2,
            fx: 2,
            n: 2,
        },
    ],
    deck: [
        [4, 4, 2, 3, 2, 1, 3, 1, 2, 1, 1, 1, 2, 2, 1],
        [4, 4, 2, 2, 2, 3, 2, 2, 1, 1, 2, 2, 1, 1, 1],
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
