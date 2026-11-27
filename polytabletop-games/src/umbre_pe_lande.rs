//! Umbre pe lande — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Lăncierul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Scut de lemn",
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
        name: "Lup din ceață",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Șarpe de piatră din ceață",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Hoitar din ceață",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Furtună prin ceață",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Pod putred prin ceață",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Drum surpat prin ceață",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Bătrân din ceață",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Hangiu din ceață",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Copil rătăcit din ceață",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Umbre pe lande",
    res: "liniște",
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
    fn umbre_pe_lande_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
