//! Seniorul hățișurilor — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, ITEM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Pavăzarul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Topor greu",
    },
    Hero {
        name: "Tâlharul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Pelerină de ceață",
    },
    Hero {
        name: "Vraciul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Flaut fermecat",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Mistreț din pădure",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Lup din pădure",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Strigoi din pădure",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Noroi lipicios prin pădure",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Drum surpat prin pădure",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Potecă alunecoasă prin pădure",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Pelerin din pădure",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Călăuză din pădure",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Bătrân din pădure",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Seniorul hățișurilor",
    res: "curaj",
    sides: 6,
    tn: 3,
    flags: ITEM | CALM,
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
    fn seniorul_hatisurilor_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
