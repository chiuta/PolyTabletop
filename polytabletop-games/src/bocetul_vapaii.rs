//! Bocetul văpăii — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, FUMBLE, RAGE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Iscoadă",
        hp: 9,
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
        name: "Luptător cu laț",
        hp: 11,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "lovitură la tâmplă",
    },
    Cls {
        name: "Căpcăun flămând",
        hp: 10,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "sărutul umbrei",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Bocetul văpăii",
    res: "dispreț",
    sides: 6,
    flags: BLEED | RAGE | FUMBLE,
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
    fn bocetul_vapaii_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
