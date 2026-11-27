//! Genezele prafului — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Mercenarul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Buzdugan",
    },
    Hero {
        name: "Tâlharul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Praf de ochi",
    },
    Hero {
        name: "Sfătuitorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Cărți de vorbă",
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
        name: "Lup din deșert",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Șarpe de piatră din deșert",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Apă învolburată prin deșert",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Pod putred prin deșert",
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
        name: "Bătrân din deșert",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Hangiu din deșert",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Negustor din deșert",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Genezele prafului",
    res: "ecou",
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
    fn genezele_prafului_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
