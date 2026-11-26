//! Cupolele Abisului — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
use crate::b4_wp::*;

static SPACES: &[Sp] = &[
    Sp {
        code: "foraj",
        cost: "",
        gain: "2a",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "alge",
        cost: "",
        gain: "2b",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "cupola",
        cost: "2a2b",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "tub",
        cost: "1a1b",
        gain: "",
        vp: 1,
        fl: "r",
    },
    Sp {
        code: "reactor",
        cost: "2m2b",
        gain: "",
        vp: 4,
        fl: "x",
    },
    Sp {
        code: "nautilus",
        cost: "",
        gain: "1c1d",
        vp: 0,
        fl: "a",
    },
    Sp {
        code: "submarin",
        cost: "1c",
        gain: "",
        vp: 1,
        fl: "r",
    },
    Sp {
        code: "contract",
        cost: "2c1d",
        gain: "",
        vp: 5,
        fl: "x",
    },
    Sp {
        code: "scafandru",
        cost: "2m",
        gain: "",
        vp: 0,
        fl: "gx",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Cupolele Abisului",
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
