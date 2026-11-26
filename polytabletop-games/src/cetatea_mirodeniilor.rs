//! Cetatea Mirodeniilor — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "plantatie",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "piper",
        cost: "",
        gain: "1b",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "scortisoara",
        cost: "",
        gain: "1c",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "nucsoara",
        cost: "1m",
        gain: "1d",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "negot",
        cost: "2a",
        gain: "2m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "fort",
        cost: "2b1c",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "comanda",
        cost: "1a1b1c1d",
        gain: "",
        vp: 7,
        fl: "x",
    },
    Sp {
        code: "corabie",
        cost: "1c1d",
        gain: "",
        vp: 3,
        fl: "r",
    },
    Sp {
        code: "licitatie",
        cost: "3m",
        gain: "",
        vp: 3,
        fl: "x",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Cetatea Mirodeniilor",
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
