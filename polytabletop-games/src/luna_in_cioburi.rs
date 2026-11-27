//! Luna în cioburi — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, LUCK};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Frate pavăzar",
        hp: 13,
        dice: 2,
        tn: 6,
        dmg: 1,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "încurajare",
    },
    Cls {
        name: "Pavăzar",
        hp: 15,
        dice: 2,
        tn: 6,
        dmg: 2,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "lovitură zdrobitoare",
    },
    Cls {
        name: "Fermecător",
        hp: 9,
        dice: 2,
        tn: 6,
        dmg: 2,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "spori",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Luna în cioburi",
    res: "cioburi",
    sides: 10,
    flags: BLEED | LUCK,
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
    fn luna_in_cioburi_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
