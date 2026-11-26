//! Podgoria — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "vita",
        cost: "",
        gain: "1a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "plantez",
        cost: "1m",
        gain: "1a1b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "recolta",
        cost: "",
        gain: "2c",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "vin",
        cost: "2c",
        gain: "",
        vp: 3,
        fl: "r",
    },
    Sp {
        code: "degustare",
        cost: "1c1d",
        gain: "",
        vp: 2,
        fl: "r",
    },
    Sp {
        code: "piata",
        cost: "1c",
        gain: "2m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "angajez",
        cost: "2m",
        gain: "",
        vp: 0,
        fl: "gx",
    },
    Sp {
        code: "contract",
        cost: "2a2c",
        gain: "",
        vp: 6,
        fl: "x",
    },
    Sp {
        code: "butoi",
        cost: "",
        gain: "1d",
        vp: 0,
        fl: "",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Podgoria",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 7,
    income: "1m",
    start: "",
    setb: 1,
    conv: [0, 0, 0, 0, 1],
    depth: 2,
};

pub fn new() -> Wp {
    Wp::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Wp {
    Wp::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 800);
    }
}
