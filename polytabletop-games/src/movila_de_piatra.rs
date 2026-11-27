//! Movila de piatră — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, CALM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Străjerul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Buzdugan",
    },
    Hero {
        name: "Vânzătorul de zvonuri",
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
        item: "Flaut fermecat",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Șarpe de piatră din munte",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Tâlhar din munte",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Strigoi din munte",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Apă învolburată prin munte",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Drum surpat prin munte",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Noroi lipicios prin munte",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Pelerin din munte",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Călăuză din munte",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Cerșetor din munte",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Movila de piatră",
    res: "temei",
    sides: 6,
    tn: 3,
    flags: BLOOD | CALM,
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
    fn movila_de_piatra_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
