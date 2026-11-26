//! Meșterii Catedralei — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "cariera",
        cost: "",
        gain: "2b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "padure",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "atelier",
        cost: "1a1b",
        gain: "1c",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "turn",
        cost: "2b1c",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "vitraliu",
        cost: "1c1d",
        gain: "",
        vp: 3,
        fl: "r",
    },
    Sp {
        code: "donatie",
        cost: "2m",
        gain: "",
        vp: 1,
        fl: "r",
    },
    Sp {
        code: "harti",
        cost: "",
        gain: "1d",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "hram",
        cost: "1a1b1c1d",
        gain: "",
        vp: 8,
        fl: "x",
    },
    Sp {
        code: "taverna",
        cost: "",
        gain: "2m",
        vp: 0,
        fl: "x",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Meșterii Catedralei",
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
