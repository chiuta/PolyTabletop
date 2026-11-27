//! Ținutul zimțat — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, COUNTER, RAGE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Căpcăun flămând",
        hp: 12,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "sete",
    },
    Cls {
        name: "Ghicitor",
        hp: 10,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "blestem",
    },
    Cls {
        name: "Mercenar",
        hp: 13,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "lovitură zdrobitoare",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Ținutul zimțat",
    res: "mânie",
    sides: 20,
    flags: RAGE | COUNTER,
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
    fn tinutul_zimtat_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
