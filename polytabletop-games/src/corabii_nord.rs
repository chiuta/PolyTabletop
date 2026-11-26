//! Corăbii spre Nord — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "port",
        cost: "",
        gain: "2a1b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "echipaj",
        cost: "2m",
        gain: "",
        vp: 0,
        fl: "gx",
    },
    Sp {
        code: "raid1",
        cost: "2a",
        gain: "",
        vp: 3,
        fl: "x",
    },
    Sp {
        code: "raid2",
        cost: "2a2b",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "raid3",
        cost: "3a2b1d",
        gain: "",
        vp: 9,
        fl: "x",
    },
    Sp {
        code: "comert",
        cost: "1b",
        gain: "2m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "prada",
        cost: "",
        gain: "1c1d",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "templu",
        cost: "2c",
        gain: "",
        vp: 2,
        fl: "r",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Corăbii spre Nord",
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
