//! Zmeul de cremene — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, FUMBLE, RAGE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Ghicitor",
        hp: 9,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "otravă lentă",
    },
    Cls {
        name: "Luptător cu laț",
        hp: 10,
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
        name: "Cleric bătrân",
        hp: 13,
        dice: 2,
        tn: 4,
        dmg: 1,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "rugă",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Zmeul de cremene",
    res: "foc",
    sides: 6,
    flags: RAGE | FUMBLE,
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
    fn zmeul_de_cremene_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
