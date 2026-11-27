//! Vizuina cu comori — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, ITEM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Străjerul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Scut de lemn",
    },
    Hero {
        name: "Pândarul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Cheie-scheletică",
    },
    Hero {
        name: "Vraciul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Scrisoare de liberă trecere",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Șarpe de piatră din peșteră",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Urs din peșteră",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Mistreț din peșteră",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Drum surpat prin peșteră",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Apă învolburată prin peșteră",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Noroi lipicios prin peșteră",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Paznic din peșteră",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Vameș din peșteră",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Călăuză din peșteră",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Vizuina cu comori",
    res: "mană",
    sides: 6,
    tn: 3,
    flags: ITEM | CALM,
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
    fn vizuina_cu_comori_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
