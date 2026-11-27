//! Cimitirul de păpuși — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, RAGE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Rob liber",
        hp: 11,
        dice: 2,
        tn: 6,
        dmg: 2,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "bubuitură de clopot",
    },
    Cls {
        name: "Vânător",
        hp: 10,
        dice: 3,
        tn: 6,
        dmg: 1,
        armor: 0,
        sp: Sp::Pierce,
        cost: 2,
        val: 0,
        spname: "țintă fixă",
    },
    Cls {
        name: "Duh însetat",
        hp: 11,
        dice: 2,
        tn: 6,
        dmg: 2,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "sărutul umbrei",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Cimitirul de păpuși",
    res: "șoaptă",
    sides: 10,
    flags: BLEED | RAGE,
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
    fn cimitirul_de_papusi_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
