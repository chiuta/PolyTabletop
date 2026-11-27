//! Urme în ceața adâncă — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, ITEM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Ciomăgarul",
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
        item: "Pelerină de ceață",
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
        name: "Zgripțuroaică din ceață",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Strigoi din ceață",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Urs din ceață",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Pod putred prin ceață",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Ceață deasă prin ceață",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Furtună prin ceață",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Cerșetor din ceață",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Pelerin din ceață",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Călăuză din ceață",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Urme în ceața adâncă",
    res: "luciditate",
    sides: 6,
    tn: 3,
    flags: ITEM | BLOOD,
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
    fn urme_in_ceata_adanca_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
