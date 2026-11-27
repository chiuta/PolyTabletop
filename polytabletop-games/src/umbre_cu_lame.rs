//! Umbre cu lame — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, COUNTER, CRIT};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Gladiator",
        hp: 14,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "ofensivă",
    },
    Cls {
        name: "Mag de furtună",
        hp: 9,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "otravă lentă",
    },
    Cls {
        name: "Bastonar",
        hp: 10,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "bubuitură de clopot",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Umbre cu lame",
    res: "tăcere",
    sides: 6,
    flags: COUNTER | CRIT,
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
    fn umbre_cu_lame_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
