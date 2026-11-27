//! Sania cu clopote — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Ciomăgarul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Scut de lemn",
    },
    Hero {
        name: "Hoțul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Praf de ochi",
    },
    Hero {
        name: "Predicatorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Sigiliu de pecete",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Hoitar din gheață",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Tâlhar din gheață",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Căpcăun din gheață",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Furtună prin gheață",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Pod putred prin gheață",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Drum surpat prin gheață",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Copil rătăcit din gheață",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Bătrân din gheață",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Cerșetor din gheață",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Sania cu clopote",
    res: "frig",
    sides: 6,
    tn: 3,
    flags: TRAP | CALM,
    stations: 7,
    turns: 18,
    bonus: 4,
    heroes: &HEROES,
    enc: &ENC,
};

pub fn new() -> Drum {
    Drum::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sania_cu_clopote_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
