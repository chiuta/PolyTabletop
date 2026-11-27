//! Mâna stângă cu pumnal — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, CRIT, FUMBLE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Paznic de nopți",
        hp: 11,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "privire fixă",
    },
    Cls {
        name: "Pelerin",
        hp: 12,
        dice: 1,
        tn: 11,
        dmg: 2,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "încurajare",
    },
    Cls {
        name: "Cavaler vagabond",
        hp: 13,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "atac de pavăză",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Mâna stângă cu pumnal",
    res: "tertip",
    sides: 20,
    flags: CRIT | FUMBLE,
    turns: 14,
    potions: 2,
    classes: &CLASSES,
};

pub fn new() -> Duel {
    Duel::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mana_stanga_cu_pumnal_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
