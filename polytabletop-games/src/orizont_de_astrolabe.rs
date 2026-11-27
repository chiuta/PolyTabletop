//! Orizont de astrolabe — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Străjerul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Scut de lemn",
    },
    Hero {
        name: "Pândarul",
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
        name: "Tâlhar din stele",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Șarpe de piatră din stele",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Urs din stele",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Furtună prin stele",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Noroi lipicios prin stele",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Drum surpat prin stele",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Cerșetor din stele",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Pelerin din stele",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Călăuză din stele",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Orizont de astrolabe",
    res: "lumină",
    sides: 6,
    tn: 3,
    flags: TRAP | CALM,
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
    fn orizont_de_astrolabe_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
