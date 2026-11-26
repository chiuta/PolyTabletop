//! Barajele — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "rau",
        cost: "",
        gain: "3a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "excavare",
        cost: "1a",
        gain: "2b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "beton",
        cost: "2b",
        gain: "1c",
        vp: 0,
        fl: "x",
    },
    Sp {
        code: "baraj",
        cost: "2c1a",
        gain: "",
        vp: 3,
        fl: "r",
    },
    Sp {
        code: "turbina",
        cost: "1c1d",
        gain: "",
        vp: 3,
        fl: "r",
    },
    Sp {
        code: "centrala",
        cost: "2b2c",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "energie",
        cost: "1d",
        gain: "3m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "contract",
        cost: "2a1d1m",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "minerit",
        cost: "",
        gain: "1d",
        vp: 0,
        fl: "a",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Barajele",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 6,
    income: "1a",
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
