//! Pustiul mutanților — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, TRAP};

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
        name: "Pândarul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Pelerină de ceață",
    },
    Hero {
        name: "Judecătorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Flaut fermecat",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Strigoi din deșert",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Mistreț din deșert",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Căpcăun din deșert",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Furtună prin deșert",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Apă învolburată prin deșert",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Avalanșă prin deșert",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Călăuză din deșert",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Cerșetor din deșert",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Meșter din deșert",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Pustiul mutanților",
    res: "mutație",
    sides: 6,
    tn: 3,
    flags: TRAP | BLOOD,
    stations: 6,
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
    fn pustiul_mutantilor_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
