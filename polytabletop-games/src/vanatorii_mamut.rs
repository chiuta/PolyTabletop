//! Vânătorii de Mamuți — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "vanatoare",
        cost: "",
        gain: "3a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "pietrar",
        cost: "",
        gain: "1b",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "lemn",
        cost: "",
        gain: "1c",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "foc",
        cost: "1b",
        gain: "1d",
        vp: 0,
        fl: "x",
    },
    Sp {
        code: "coliba",
        cost: "2c1b",
        gain: "",
        vp: 3,
        fl: "x",
    },
    Sp {
        code: "unealta",
        cost: "2b",
        gain: "",
        vp: 2,
        fl: "r",
    },
    Sp {
        code: "pictura",
        cost: "1d1a",
        gain: "",
        vp: 2,
        fl: "x",
    },
    Sp {
        code: "ritual",
        cost: "3a",
        gain: "",
        vp: 2,
        fl: "",
    },
    Sp {
        code: "cort",
        cost: "1a1b1c1d",
        gain: "",
        vp: 8,
        fl: "x",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Vânătorii de Mamuți",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 6,
    income: "1a",
    start: "",
    setb: 2,
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
