//! Coborârea fără lumină — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Spadasinul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Topor greu",
    },
    Hero {
        name: "Cercetașul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Pelerină de ceață",
    },
    Hero {
        name: "Tălmaciul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Flaut fermecat",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Hoitar din mină",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Mistreț din mină",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Lup din mină",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Apă învolburată prin mină",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Foc de tufiș prin mină",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Potecă alunecoasă prin mină",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Pelerin din mină",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Copil rătăcit din mină",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Meșter din mină",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Coborârea fără lumină",
    res: "dor",
    sides: 10,
    tn: 5,
    flags: TRAP | BLOOD,
    stations: 7,
    turns: 18,
    bonus: 3,
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
    fn coborarea_fara_lumina_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
