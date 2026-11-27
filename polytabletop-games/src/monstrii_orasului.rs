//! Monștrii Orașului — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: șase zaruri (1, 2, 3, inimă, gheară, energie), până la trei aruncări (arunc, apoi reia m sau gata). Trei de același număr dau puncte; inimile vindecă (nu în centru); ghearele lovesc centrul sau, dacă ești tu în centru, pe celălalt, care poate ceda locul (cedez | raman). Energia se schimbă în glorie (e vp: 3 → 2 puncte) sau gheare mai tari (e gheare: 6). 15 puncte sau adversarul fără viață.
use crate::c5_duel::*;

pub static SPEC: DuelSpec = DuelSpec {
    title: "Monștrii Orașului",
    mode: DMode::City,
    dice: 6,
    hp: 10,
    goal: 15,
    heroes: [&NONE, &NONE],
    cap: 600,
};

pub fn new() -> Duel {
    Duel::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Duel::with_seed(&SPEC, s), 660);
    }
}
