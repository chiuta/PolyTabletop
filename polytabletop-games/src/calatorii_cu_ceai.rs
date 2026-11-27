//! Călătorii cu ceai — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, ITEM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Lăncierul",
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
        item: "Pumnal otrăvit",
    },
    Hero {
        name: "Predicatorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Cărți de vorbă",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Șarpe de piatră din câmpie",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Zgripțuroaică din câmpie",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Tâlhar din câmpie",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Drum surpat prin câmpie",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Pod putred prin câmpie",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Ceață deasă prin câmpie",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Paznic din câmpie",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Vameș din câmpie",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Pelerin din câmpie",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Călătorii cu ceai",
    res: "popas",
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
    fn calatorii_cu_ceai_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
