//! Drumul Mătăsii — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "bazar",
        cost: "",
        gain: "2a1b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "caravana",
        cost: "2m",
        gain: "1c1d",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "han",
        cost: "1a",
        gain: "2m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "port",
        cost: "1b1c",
        gain: "",
        vp: 3,
        fl: "",
    },
    Sp {
        code: "ambasada",
        cost: "2d",
        gain: "",
        vp: 3,
        fl: "x",
    },
    Sp {
        code: "ruta1",
        cost: "1a1b",
        gain: "",
        vp: 2,
        fl: "r",
    },
    Sp {
        code: "ruta2",
        cost: "2c2d",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "firman",
        cost: "1a1b1c1d",
        gain: "",
        vp: 8,
        fl: "x",
    },
    Sp {
        code: "piata",
        cost: "",
        gain: "1m",
        vp: 0,
        fl: "a",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Drumul Mătăsii",
    res: "abcdm",
    spaces: SPACES,
    workers: 3,
    rounds: 6,
    income: "2m",
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
