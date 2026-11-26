//! Peștera cu Livezi — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "livada",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "mina",
        cost: "",
        gain: "2b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "sapa",
        cost: "1b",
        gain: "1c",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "ferma",
        cost: "1a1b",
        gain: "1d",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "camera",
        cost: "2b1c",
        gain: "",
        vp: 2,
        fl: "gx",
    },
    Sp {
        code: "expeditie",
        cost: "1d",
        gain: "3m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "armare",
        cost: "1c",
        gain: "",
        vp: 1,
        fl: "r",
    },
    Sp {
        code: "recolta",
        cost: "2a1d",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "targ",
        cost: "1m",
        gain: "1a1b",
        vp: 0,
        fl: "",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Peștera cu Livezi",
    res: "abcdm",
    spaces: SPACES,
    workers: 2,
    rounds: 7,
    income: "1a",
    start: "2a2b",
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
