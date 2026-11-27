//! Prinși între ceruri — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, ITEM, TRAP};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Pavăzarul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Scut de lemn",
    },
    Hero {
        name: "Tâlharul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Frânghie cu cange",
    },
    Hero {
        name: "Judecătorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Cărți de vorbă",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Șarpe de piatră din templu",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Strigoi din templu",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Urs din templu",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Foc de tufiș prin templu",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Potecă alunecoasă prin templu",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Prăpastie prin templu",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Copil rătăcit din templu",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Negustor din templu",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Hangiu din templu",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Prinși între ceruri",
    res: "har",
    sides: 6,
    tn: 3,
    flags: TRAP | ITEM,
    stations: 6,
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
    fn prinsi_intre_ceruri_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
