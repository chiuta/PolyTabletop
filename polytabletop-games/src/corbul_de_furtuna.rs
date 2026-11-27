//! Corbul de furtună — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, CRIT, LUCK};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Maestru de lanțuri",
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
    Cls {
        name: "Șoptitor",
        hp: 10,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "spori",
    },
    Cls {
        name: "Tâlhar cu pumnal",
        hp: 9,
        dice: 3,
        tn: 4,
        dmg: 1,
        armor: 0,
        sp: Sp::Pierce,
        cost: 2,
        val: 0,
        spname: "înjunghiere",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Corbul de furtună",
    res: "libertate",
    sides: 6,
    flags: CRIT | LUCK,
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
    fn corbul_de_furtuna_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
