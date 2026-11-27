//! Dublu Isteț — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală cu aceleași șase zaruri, dar zone rearanjate: șir lung (mov), multiplicator 1–4 (portocaliu), încrucișare, sumă și urcare cu praguri mai mari. Aceleași mutări: arunc, alege d a, gata, pas. 6 ture fiecare; zonă completă +6.
use crate::c5_foaie::*;

pub static SPEC: FoaieSpec = FoaieSpec {
    title: "Dublu Isteț",
    areas: [
        Area {
            kind: K::Cross,
            name: "galben",
            par: &[],
            len: 6,
        },
        Area {
            kind: K::Sum,
            name: "albastru",
            par: &[],
            len: 11,
        },
        Area {
            kind: K::Climb,
            name: "verde",
            par: &[2, 2, 3, 3, 4, 5, 6],
            len: 7,
        },
        Area {
            kind: K::Mult,
            name: "portocaliu",
            par: &[1, 2, 1, 2, 3, 3, 4],
            len: 7,
        },
        Area {
            kind: K::Run,
            name: "mov",
            par: &[],
            len: 9,
        },
    ],
    turns: 12,
    full_bonus: 6,
    picks: 3,
};

pub fn new() -> Foaie {
    Foaie::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Foaie::with_seed(&SPEC, s), 130);
    }
}
