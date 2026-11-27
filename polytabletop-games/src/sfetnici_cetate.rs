//! Sfetnicii Cetății — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: în fiecare rundă ambii aruncă două zaruri (arunc) și, alternativ, ocupă sfetnici cu numerele 1–12 (pun n d: zarul d valorează n, sau d = 3 pentru suma celor două); un sfetnic poate fi ocupat o singură dată pe rundă și dă lemn, piatră, aur, glorie sau soldați. Apoi se construiește (constr b). După rundele 2, 4 și 6 vine o invazie: cine are prea puțini soldați pierde 2 glorie. 6 runde.
use crate::c5_alloc::*;

pub static SPEC: AdvSpec = AdvSpec {
    title: "Sfetnicii Cetății",
    rounds: 6,
    advisors: [
        (1, 0, 0, 0, 0),
        (0, 1, 0, 0, 0),
        (0, 0, 1, 0, 0),
        (1, 1, 0, 0, 0),
        (0, 0, 0, 1, 0),
        (0, 0, 2, 0, 0),
        (0, 0, 0, 0, 1),
        (2, 0, 0, 0, 0),
        (0, 2, 0, 0, 0),
        (1, 1, 1, 0, 0),
        (0, 0, 0, 0, 2),
        (0, 0, 0, 2, 0),
    ],
    buildings: &[
        (2, 0, 0, 1),
        (1, 1, 0, 2),
        (0, 2, 1, 3),
        (2, 1, 1, 4),
        (3, 2, 1, 6),
        (2, 2, 2, 5),
    ],
    invasions: &[(2, 1), (4, 2), (6, 3)],
    loss: 2,
};

pub fn new() -> Adv {
    Adv::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Adv::with_seed(&SPEC, s), 300);
    }
}
