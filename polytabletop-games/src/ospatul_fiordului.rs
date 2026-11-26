//! Ospățul Fiordului — variantă originală (lista B4), motor `wp`; clean-room: doar mecanica generală a genului.
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
        code: "vanatoare",
        cost: "",
        gain: "1b1c",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "tamplarie",
        cost: "1b",
        gain: "1c1d",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "banchet",
        cost: "2a1c",
        gain: "",
        vp: 3,
        fl: "",
    },
    Sp {
        code: "comert",
        cost: "1d",
        gain: "2m",
        vp: 0,
        fl: "",
    },
    Sp {
        code: "expeditie",
        cost: "2b1m",
        gain: "",
        vp: 3,
        fl: "x",
    },
    Sp {
        code: "bijuterie",
        cost: "1c1d",
        gain: "",
        vp: 3,
        fl: "",
    },
    Sp {
        code: "ospat",
        cost: "3a3b",
        gain: "",
        vp: 8,
        fl: "x",
    },
];

static SPEC: WpSpec = WpSpec {
    title: "Ospățul Fiordului",
    res: "abcdm",
    spaces: SPACES,
    workers: 4,
    rounds: 6,
    income: "",
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
