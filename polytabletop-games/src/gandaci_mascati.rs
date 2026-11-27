//! Gândacii Mascați — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: 16 plăci cu fața în jos (7 perechi de gândaci și 2 jokeri). Întorci o placă (volta i, apoi arata), apoi încă una: perechea potrivită se ia (jokerul se potrivește cu orice gândac) și mai întorci; altfel tura trece, dar plăcile văzute rămân cunoscute. Primul la 5 perechi câștigă.
use crate::c5_memo::*;

pub static SPEC: MemSpec = MemSpec {
    title: "Gândacii Mascați",
    mode: MMode::Pairs,
    w: 4,
    h: 4,
    kinds: 7,
    cats: 0,
    wilds: 2,
    goal: 5,
    cap: 900,
    names: &[
        "roșu",
        "galben",
        "verde",
        "albastru",
        "mov",
        "portocaliu",
        "roz",
    ],
};

pub fn new() -> Memo {
    Memo::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Memo::with_seed(&SPEC, s), 960);
    }
}
