//! Povești din han — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, COUNTER, LUCK};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Pândar",
        hp: 10,
        dice: 3,
        tn: 4,
        dmg: 1,
        armor: 0,
        sp: Sp::Pierce,
        cost: 2,
        val: 0,
        spname: "săgeată ascuțită",
    },
    Cls {
        name: "Pavăzar",
        hp: 13,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "lovitură zdrobitoare",
    },
    Cls {
        name: "Hipnotizor",
        hp: 10,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "laț",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Povești din han",
    res: "poftă",
    sides: 6,
    flags: COUNTER | LUCK,
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
    fn povesti_din_han_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
