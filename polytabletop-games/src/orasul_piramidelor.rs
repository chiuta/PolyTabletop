//! Orașul Piramidelor — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "piatra",
        cost: "",
        gain: "2b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "trestii",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "soare",
        cost: "",
        gain: "1c",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "piramida",
        cost: "3b",
        gain: "",
        vp: 3,
        fl: "r",
    },
    Sp {
        code: "templu",
        cost: "2b2a",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "masca",
        cost: "2c",
        gain: "",
        vp: 2,
        fl: "r",
    },
    Sp {
        code: "observator",
        cost: "1d2m",
        gain: "",
        vp: 3,
        fl: "x",
    },
    Sp {
        code: "ritual",
        cost: "1a1b1c",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "schimb",
        cost: "2a",
        gain: "1d1m",
        vp: 0,
        fl: "",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Orașul Piramidelor",
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
