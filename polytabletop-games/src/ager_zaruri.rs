//! Agerul cu Zaruri — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: șase zaruri (cinci colorate, unul alb). Cel la rând aruncă (arunc) și alege până la trei zaruri (alege d a): cel colorat în zona lui, cel alb oriunde; zarurile mai mici decât cel ales trec pe tava de argint, de unde celălalt alege unul. Cinci zone: încrucișare 1–6, sumă cu zarul alb, urcare cu praguri, multiplicator și șir crescător. 6 ture fiecare.
use crate::c5_foaie::*;

pub static SPEC: FoaieSpec = FoaieSpec {
    title: "Agerul cu Zaruri",
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
            par: &[1, 2, 3, 4, 5, 5, 6],
            len: 7,
        },
        Area {
            kind: K::Mult,
            name: "portocaliu",
            par: &[1, 1, 2, 1, 2, 3],
            len: 6,
        },
        Area {
            kind: K::Run,
            name: "mov",
            par: &[],
            len: 8,
        },
    ],
    turns: 12,
    full_bonus: 5,
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
