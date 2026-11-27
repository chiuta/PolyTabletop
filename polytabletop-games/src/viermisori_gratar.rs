//! Viermișorii de pe Grătar — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: opt zaruri cu fețele 1–5 și un vierme (valorează 5). Arunci (arunc), alegi o față nefolosită (alege f) și pui deoparte toate zarurile cu ea, apoi continu sau opresc. Ai nevoie de 16 puncte și un vierme. Iei de pe grătar piesa exactă sau cea mai mare mai mică (poți fura piesa de sus a adversarului); altfel pierzi piesa de sus și cea mai mare piesă dispare. Câștigă cine are mai mulți viermi.
use crate::c5_worm::*;

pub static SPEC: WormSpec = WormSpec {
    title: "Viermișorii de pe Grătar",
    lo: 16,
    hi: 31,
    dice: 8,
    cap: 500,
};

pub fn new() -> Worm {
    Worm::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Worm::with_seed(&SPEC, s), 560);
    }
}
