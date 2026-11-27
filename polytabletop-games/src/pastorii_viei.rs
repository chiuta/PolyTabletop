//! Păstorii viei — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, COUNTER};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Cioară de seară",
        hp: 12,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "sete",
    },
    Cls {
        name: "Alchimist",
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
        name: "Pavăzar",
        hp: 14,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "atac de pavăză",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Păstorii viei",
    res: "dreptate",
    sides: 6,
    flags: BLEED | COUNTER,
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
    fn pastorii_viei_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
