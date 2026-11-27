//! Turnul răzvrătiților — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Pușcașul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Pavăză veche",
    },
    Hero {
        name: "Vânătorul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Cheie-scheletică",
    },
    Hero {
        name: "Tălmaciul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Cărți de vorbă",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Hoitar din turn",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Zgripțuroaică din turn",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Strigoi din turn",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Noroi lipicios prin turn",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Prăpastie prin turn",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Foc de tufiș prin turn",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Cerșetor din turn",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Meșter din turn",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Hangiu din turn",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Turnul răzvrătiților",
    res: "ură",
    sides: 10,
    tn: 5,
    flags: TRAP,
    stations: 7,
    turns: 18,
    bonus: 5,
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
    fn turnul_razvratitilor_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
