//! Detectivii spiritelor (`detectivii_spiritelor`): variantă originală, motor comun `t6_tcg` (tabele proprii).
use crate::t6_tcg::*;

pub static SPEC: Spec = Spec {
    id: "detectivii_spiritelor",
    title: "Detectivii spiritelor",
    facs: ["Biroul Dincolo", "Banda Pragului"],
    res: 3,
    cap: 9,
    lands: 0,
    bank: 2,
    win: 3,
    wn: 7,
    big: 0,
    hand0: 6,
    maxt: 15,
    bmax: 5,
    twist: b'K',
    units: [
        U {
            name: "Cercetător Tânăr",
            cost: 1,
            atk: 1,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Iscoadă Mică",
            cost: 1,
            atk: 1,
            hp: 1,
            kw: FIRST,
        },
        U {
            name: "Călăreț de Aer",
            cost: 2,
            atk: 3,
            hp: 1,
            kw: FLY,
        },
        U {
            name: "Fantomă Zâmbăreață",
            cost: 2,
            atk: 1,
            hp: 3,
            kw: FLY,
        },
        U {
            name: "Luptător de Stradă",
            cost: 3,
            atk: 5,
            hp: 2,
            kw: 0,
        },
        U {
            name: "Pistolar al Sufletelor",
            cost: 3,
            atk: 1,
            hp: 4,
            kw: VENOM,
        },
        U {
            name: "Demon Cu Bici",
            cost: 4,
            atk: 5,
            hp: 3,
            kw: HASTE,
        },
        U {
            name: "Maestru Îngheț",
            cost: 4,
            atk: 4,
            hp: 4,
            kw: FIRST,
        },
        U {
            name: "Stăpânul Turnirului",
            cost: 5,
            atk: 5,
            hp: 6,
            kw: 0,
        },
        U {
            name: "Spiritul Primordial",
            cost: 6,
            atk: 6,
            hp: 7,
            kw: 0,
        },
    ],
    spells: [
        S {
            name: "Pumn Spiritual",
            cost: 1,
            fx: 9,
            n: 0,
        },
        S {
            name: "Fulg Dublu",
            cost: 3,
            fx: 0,
            n: 4,
        },
        S {
            name: "Revelație",
            cost: 2,
            fx: 2,
            n: 2,
        },
        S {
            name: "Chemare Înapoi",
            cost: 2,
            fx: 6,
            n: 0,
        },
        S {
            name: "Transă",
            cost: 2,
            fx: 8,
            n: 0,
        },
    ],
    deck: [
        [4, 2, 3, 3, 2, 2, 3, 1, 1, 2, 1, 1, 2, 1, 2],
        [4, 4, 3, 2, 2, 3, 2, 1, 1, 1, 1, 1, 1, 2, 2],
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
