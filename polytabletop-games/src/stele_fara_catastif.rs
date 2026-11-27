//! Stele fără catastif — variantă originală (familia T2, drum cu întâlniri).
use crate::t2_drum::{Cfg, Drum, Enc, Hero, CALM, ITEM};

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
        name: "Pândarul",
        hp: 10,
        fight: 1,
        sneak: 3,
        talk: 1,
        item: "Frânghie cu cange",
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
        name: "Mistreț din stele",
        kind: 0,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Strigoi din stele",
        kind: 0,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Urs din stele",
        kind: 0,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Apă învolburată prin stele",
        kind: 1,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Noroi lipicios prin stele",
        kind: 1,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Pod putred prin stele",
        kind: 1,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
    Enc {
        name: "Paznic din stele",
        kind: 2,
        threat: 1,
        reward: 2,
        dmg: 1,
    },
    Enc {
        name: "Bătrân din stele",
        kind: 2,
        threat: 2,
        reward: 3,
        dmg: 2,
    },
    Enc {
        name: "Vameș din stele",
        kind: 2,
        threat: 3,
        reward: 5,
        dmg: 3,
    },
];

pub static CFG: Cfg = Cfg {
    title: "Stele fără catastif",
    res: "credite",
    sides: 20,
    tn: 9,
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
    fn stele_fara_catastif_ok() {
        crate::t2_drum::check_cfg(&CFG);
    }
}
