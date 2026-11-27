//! Făptura din fag — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, ITEM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Lăncierul",
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
        item: "Pumnal otrăvit",
    },
    Hero {
        name: "Vraciul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Pungă de arginți",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Ciobanul turbat din pădure",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Strigoi din pădure",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Zgripțuroaică din pădure",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Ceață deasă prin pădure",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Prăpastie prin pădure",
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
        name: "Meșter din pădure",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Negustor din pădure",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Vameș din pădure",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Făptura din fag",
    res: "teamă",
    sides: 6,
    tn: 3,
    flags: ITEM | CALM,
    stations: 6,
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
    fn faptura_din_fag_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
