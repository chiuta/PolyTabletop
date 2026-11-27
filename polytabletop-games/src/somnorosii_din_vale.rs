//! Somnoroșii din vale — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, COUNTER, LUCK};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Păstor",
        hp: 11,
        dice: 2,
        tn: 7,
        dmg: 1,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "apă sfințită",
    },
    Cls {
        name: "Luptător cu laț",
        hp: 11,
        dice: 2,
        tn: 7,
        dmg: 2,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "laț",
    },
    Cls {
        name: "Vrăjitoare de râu",
        hp: 9,
        dice: 2,
        tn: 7,
        dmg: 2,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "praf de ochi",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Somnoroșii din vale",
    res: "vis",
    sides: 12,
    flags: COUNTER | LUCK,
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
    fn somnorosii_din_vale_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
