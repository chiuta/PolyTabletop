//! Hornul cu mașinării — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, ITEM, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Pavăzarul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Armură cusută",
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
        name: "Vraciul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Cărți de vorbă",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Urs din oraș",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Căpcăun din oraș",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Ciobanul turbat din oraș",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Ceață deasă prin oraș",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Noroi lipicios prin oraș",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Prăpastie prin oraș",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Paznic din oraș",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Copil rătăcit din oraș",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Vameș din oraș",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Hornul cu mașinării",
    res: "scânteie",
    sides: 6,
    tn: 3,
    flags: TRAP | ITEM,
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
    fn hornul_cu_masinarii_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
