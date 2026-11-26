//! Cavalerii Hotarului — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "tabara",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "fierarie",
        cost: "1a",
        gain: "1b1c",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "antrenament",
        cost: "2m",
        gain: "",
        vp: 0,
        fl: "gx",
    },
    Sp {
        code: "patrula",
        cost: "1b",
        gain: "",
        vp: 1,
        fl: "r",
    },
    Sp {
        code: "asediu",
        cost: "2b2c",
        gain: "",
        vp: 6,
        fl: "x",
    },
    Sp {
        code: "scutier",
        cost: "1d",
        gain: "2m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "castel",
        cost: "3a1d",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "menestrel",
        cost: "",
        gain: "1d",
        vp: 0,
        fl: "a",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Cavalerii Hotarului",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 6,
    income: "1a1m",
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
