//! Zarul tăios — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, FUMBLE, RAGE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Căpitan de strajă",
        hp: 15,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "atac de pavăză",
    },
    Cls {
        name: "Maestru de lanțuri",
        hp: 11,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "lovitură la tâmplă",
    },
    Cls {
        name: "Doctor de câmp",
        hp: 13,
        dice: 1,
        tn: 11,
        dmg: 2,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "bandaj",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Zarul tăios",
    res: "uzură",
    sides: 20,
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
    fn zarul_taios_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
