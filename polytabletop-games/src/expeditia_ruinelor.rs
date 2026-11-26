//! Expediția Ruinelor — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "tabara",
        cost: "",
        gain: "2a1m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "jungla",
        cost: "",
        gain: "1b1c",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "ruine1",
        cost: "2a",
        gain: "",
        vp: 2,
        fl: "r",
    },
    Sp {
        code: "ruine2",
        cost: "2b1c",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "templu",
        cost: "3c",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "asistent",
        cost: "3m",
        gain: "",
        vp: 0,
        fl: "gx",
    },
    Sp {
        code: "artefact",
        cost: "1b1c1d",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "cercetare",
        cost: "1a",
        gain: "1d",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "busola",
        cost: "",
        gain: "1d",
        vp: 0,
        fl: "a",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Expediția Ruinelor",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 6,
    income: "1m1a",
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
