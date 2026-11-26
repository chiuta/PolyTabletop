//! Negustorii din Port — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "pescuit",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "padure",
        cost: "",
        gain: "2b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "mina",
        cost: "",
        gain: "1c1d",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "banca",
        cost: "",
        gain: "3m",
        vp: 0,
        fl: "x",
    },
    Sp {
        code: "contract1",
        cost: "2a1b",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "contract2",
        cost: "2c1d",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "contract3",
        cost: "1a1b1c1d",
        gain: "",
        vp: 7,
        fl: "x",
    },
    Sp {
        code: "schimb",
        cost: "2a",
        gain: "1c1m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "livrare",
        cost: "2m1d",
        gain: "",
        vp: 3,
        fl: "",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Negustorii din Port",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 6,
    income: "1m",
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
