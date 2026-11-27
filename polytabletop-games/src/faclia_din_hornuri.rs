//! Făclia din hornuri — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, BLOOD, CALM};

static HEROES: [Hero; 3] = [
    Hero {
        name: "Pușcașul",
        hp: 13,
        fight: 3,
        sneak: 1,
        talk: 1,
        item: "Armură cusută",
    },
    Hero {
        name: "Cercetașul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Praf de ochi",
    },
    Hero {
        name: "Predicatorul",
        hp: 9,
        fight: 1,
        sneak: 2,
        talk: 3,
        item: "Pungă de arginți",
    },
];

static ENC: [Enc; 9] = [
    Enc {
        name: "Mistreț din peșteră",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Ciobanul turbat din peșteră",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Urs din peșteră",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Foc de tufiș prin peșteră",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Furtună prin peșteră",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Prăpastie prin peșteră",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Vameș din peșteră",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Călăuză din peșteră",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Cerșetor din peșteră",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Făclia din hornuri",
    res: "torță",
    sides: 6,
    tn: 3,
    flags: BLOOD | CALM,
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
    fn faclia_din_hornuri_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
