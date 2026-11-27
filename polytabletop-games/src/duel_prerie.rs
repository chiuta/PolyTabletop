//! Duel în Prerie — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: cinci zaruri (săgeată, dinamită, foc aproape, foc departe, bere, mitralieră), trei aruncări (arunc, reia m, gata). Săgețile se iau din teanc și se blochează, dinamitele se blochează (trei = pierzi o viață și tura); când teancul se golește fiecare pierde atâtea vieți câte săgeți are. Tragi doar cu fața potrivită distanței (alternă). Cine ajunge la zero vieți pierde.
use crate::c5_duel::*;

pub static SPEC: DuelSpec = DuelSpec {
    title: "Duel în Prerie",
    mode: DMode::Prairie,
    dice: 5,
    hp: 8,
    goal: 0,
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
