//! Isteț la Cub — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: șapte ture fiecare și până la patru zaruri pe tură, deci tava de argint se umple mai repede. Zone mai generoase: urcare cu praguri mici, multiplicator până la ×5. Mutări: arunc, alege d a, gata, pas; zonă completă +4.
use crate::c5_foaie::*;

pub static SPEC: FoaieSpec = FoaieSpec {
    title: "Isteț la Cub",
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
            par: &[1, 1, 2, 3, 4, 5],
            len: 6,
        },
        Area {
            kind: K::Mult,
            name: "portocaliu",
            par: &[1, 2, 3, 3, 4, 5],
            len: 6,
        },
        Area {
            kind: K::Run,
            name: "mov",
            par: &[],
            len: 7,
        },
    ],
    turns: 14,
    full_bonus: 4,
    picks: 4,
};

pub fn new() -> Foaie {
    Foaie::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Foaie::with_seed(&SPEC, s), 150);
    }
}
