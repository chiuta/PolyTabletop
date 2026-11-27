//! Porcușorul Lacom — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: un singur zar. Spui continu și mutarea forțată arunc dă un număr: 1 îți pierde punctele turei, un 6 te obligă să mai arunci, altfel se adună. Cu opresc salvezi punctele turei. Primul la 50 câștigă.
use crate::c5_push::*;

pub static SPEC: PushSpec = PushSpec {
    title: "Porcușorul Lacom",
    mode: PMode::Pig,
    goal: 50,
    a: 1,
    b: 6,
    cap: 700,
};

pub fn new() -> Push {
    Push::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Push::with_seed(&SPEC, s), 760);
    }
}
