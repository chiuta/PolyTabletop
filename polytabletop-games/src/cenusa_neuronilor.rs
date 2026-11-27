//! Cenușa neuronilor — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, ITEM, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Pavăzarul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Pavăză veche",
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
        name: "Negociatorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Scrisoare de liberă trecere",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Tâlhar din cenușă",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Mistreț din cenușă",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Strigoi din cenușă",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Ceață deasă prin cenușă",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Avalanșă prin cenușă",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Foc de tufiș prin cenușă",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Copil rătăcit din cenușă",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Meșter din cenușă",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Vameș din cenușă",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Cenușa neuronilor",
    res: "nervi",
    sides: 6,
    tn: 3,
    flags: TRAP | ITEM | BLOOD,
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
    fn cenusa_neuronilor_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
