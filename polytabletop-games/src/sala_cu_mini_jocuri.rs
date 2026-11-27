//! Sala cu Mini-Jocuri — variantă originală (lista C1), motor `b6_crowd`; clean-room: doar mecanica generală a genului.
use crate::b6_crowd::*;

static SPEC: CSpec = CSpec {
    title: "Sala cu Mini-Jocuri",
    pick: 2,
    scene: Scene::Prev,
    rounds: 7,
    dims: ["glumă", "ciudățenie", "șarm"],
};

pub fn new() -> Crowd {
    Crowd::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Crowd {
    Crowd::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 200);
    }

    #[test]
    fn hidden_information_and_ai_honesty() {
        crate::b6_crowd::audit(&super::SPEC);
    }
}
