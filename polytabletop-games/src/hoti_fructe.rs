//! Hoții de Fructe — variantă originală C5 (clean-room: reguli proprii inspirate doar de mecanica generală a genului; fără nume sau texte comerciale).
//!
//! Variantă originală: un păhar cu 13 zaruri de trei feluri (6 blânde, 4 mijlocii, 3 aprige). Tragi trei, le arunci (continu, apoi arunc): fruct (+1), mușcătură (câinele; trei te trimit acasă fără nimic) sau fugă (zarul se aruncă iar). Cu opresc salvezi fructele. Primul la 10 câștigă.
use crate::c5_push::*;

pub static SPEC: PushSpec = PushSpec {
    title: "Hoții de Fructe",
    mode: PMode::Cup,
    goal: 10,
    a: 0,
    b: 0,
    cap: 900,
};

pub fn new() -> Push {
    Push::new(&SPEC)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn plays() {
        crate::brand_util::check(|s| Push::with_seed(&SPEC, s), 960);
    }
}
