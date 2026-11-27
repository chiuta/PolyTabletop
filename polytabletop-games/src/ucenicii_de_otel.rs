//! Ucenicii de oțel — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, CRIT};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Ghicitor",
        hp: 10,
        dice: 1,
        tn: 51,
        dmg: 4,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "blestem",
    },
    Cls {
        name: "Duh însetat",
        hp: 12,
        dice: 1,
        tn: 51,
        dmg: 4,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "sorb viața",
    },
    Cls {
        name: "Spadasin",
        hp: 14,
        dice: 1,
        tn: 51,
        dmg: 4,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "lovitură zdrobitoare",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Ucenicii de oțel",
    res: "ritm",
    sides: 100,
    flags: BLEED | CRIT,
    turns: 14,
    potions: 1,
    classes: &CLASSES,
};

pub fn new() -> Duel {
    Duel::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ucenicii_de_otel_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
