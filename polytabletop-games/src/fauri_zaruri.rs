//! Făurarii de Zaruri — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: fiecare are două zaruri cu fețe de recompensă (aur, soare, lună, glorie). Fiecare rundă începe cu binecuvantare (se aruncă toate zarurile și se primesc recompensele); apoi ai două acțiuni: forja i d f (cumperi fața i din sanctuar cu aur și o pui peste fața f a zarului d), cartea c (plătești soare/lună pentru glorie) sau pas. 9 runde; câștigă cine are mai multă glorie.
use crate::c5_forge::*;

pub static SPEC: ForgeSpec = ForgeSpec {
    title: "Făurarii de Zaruri",
    rounds: 9,
    acts: 2,
    gold0: 3,
    items: &[
        (2, 0, 3, 4),
        (3, 1, 1, 4),
        (3, 2, 1, 4),
        (3, 3, 2, 4),
        (5, 3, 3, 3),
        (6, 0, 6, 3),
    ],
    cards: &[(2, 0, 4), (0, 2, 4), (3, 2, 9), (4, 4, 14)],
    dice0: [
        [(0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (1, 1)],
        [(0, 1), (0, 1), (0, 1), (0, 1), (2, 1), (3, 1)],
    ],
};

pub fn new() -> Forge {
    Forge::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Forge::with_seed(&SPEC, s), 400);
    }
}
