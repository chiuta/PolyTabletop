//! Roata Zilelor — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "padure",
        cost: "",
        gain: "1a",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "cariera",
        cost: "",
        gain: "1b",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "aur",
        cost: "",
        gain: "1m",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "templu",
        cost: "2a",
        gain: "",
        vp: 2,
        fl: "r",
    },
    Sp {
        code: "monument",
        cost: "2b1c",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "schimb",
        cost: "1a1b",
        gain: "2c",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "ofranda",
        cost: "1m",
        gain: "",
        vp: 1,
        fl: "r",
    },
    Sp {
        code: "cult",
        cost: "2c",
        gain: "1d",
        vp: 1,
        fl: "",
    },
    Sp {
        code: "constructie",
        cost: "1a1b1c1d",
        gain: "",
        vp: 8,
        fl: "x",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Roata Zilelor",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 8,
    income: "",
    start: "2a1b1m",
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
