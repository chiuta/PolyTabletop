//! Raid în ținuturile vechi (`raid_in_tinuturi_vechi`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "raid_in_tinuturi_vechi",
    title: "Raid în ținuturile vechi",
    facs: ["Alianța Cetăților", "Hoarda Pustie"],
    res: 0,
    cap: 8,
    lands: 12,
    bank: 0,
    win: 1,
    wn: 5,
    big: 5,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'-',
    units: [
        U {
            name: "Recrut de Poartă",
            cost: 1,
            atk: 2,
            hp: 1,
            kw: 0,
        },
        U {
            name: "Arcaș de Creastă",
            cost: 2,
            atk: 2,
            hp: 3,
            kw: 0,
        },
        U {
            name: "Preot de Sat",
            cost: 2,
            atk: 2,
            hp: 2,
            kw: FLY,
        },
        U {
            name: "Războinic Cu Scut",
            cost: 3,
            atk: 3,
            hp: 4,
            kw: 0,
        },
        U {
            name: "Vrăjitor de Foc",
            cost: 3,
            atk: 1,
            hp: 5,
            kw: ARMOR,
        },
        U {
            name: "Paladin Epuizat",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: GUARD,
        },
        U {
            name: "Căpitan de Raid",
            cost: 4,
            atk: 3,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Dragon Tânăr",
            cost: 5,
            atk: 4,
            hp: 6,
            kw: FIRST,
        },
        U {
            name: "Căpetenie de Hoardă",
            cost: 6,
            atk: 4,
            hp: 7,
            kw: VENOM,
        },
        U {
            name: "Zmeul Cel Bătrân",
            cost: 7,
            atk: 4,
            hp: 11,
            kw: GUARD,
        },
    ],
    spells: [
        S {
            name: "Dragostea de Sat",
            cost: 4,
            fx: 5,
            n: 9,
        },
        S {
            name: "Foc de Vrăjitor",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Bule de Blestem",
            cost: 5,
            fx: 3,
            n: 3,
        },
        S {
            name: "Strigăt de Raid",
            cost: 5,
            fx: 1,
            n: 3,
        },
        S {
            name: "Reînvierea Cetății",
            cost: 2,
            fx: 7,
            n: 1,
        },
    ],
    deck: [
        [1, 2, 2, 2, 1, 2, 1, 1, 1, 0, 1, 1, 1, 1, 1],
        [1, 1, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1],
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
