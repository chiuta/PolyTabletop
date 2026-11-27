//! Sabatul de pe deal — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, FUMBLE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Pândar",
        hp: 11,
        dice: 2,
        tn: 46,
        dmg: 2,
        armor: 0,
        sp: Sp::Pierce,
        cost: 2,
        val: 0,
        spname: "săgeată ascuțită",
    },
    Cls {
        name: "Cioară de seară",
        hp: 11,
        dice: 1,
        tn: 51,
        dmg: 4,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "furt de suflu",
    },
    Cls {
        name: "Tâlhar de drumuri",
        hp: 10,
        dice: 1,
        tn: 51,
        dmg: 4,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "șoc",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Sabatul de pe deal",
    res: "farmec",
    sides: 100,
    flags: BLEED | FUMBLE,
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
    fn sabatul_de_pe_deal_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
