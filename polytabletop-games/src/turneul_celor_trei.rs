//! Turneul celor trei — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, COUNTER, CRIT};

static CLASSES: [Cls; 3] = [
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
        name: "Vampir de hrubă",
        hp: 11,
        dice: 2,
        tn: 4,
        dmg: 2,
        armor: 1,
        sp: Sp::Drain,
        cost: 2,
        val: 0,
        spname: "sărutul umbrei",
    },
    Cls {
        name: "Hoț de acoperișuri",
        hp: 10,
        dice: 3,
        tn: 4,
        dmg: 1,
        armor: 0,
        sp: Sp::Pierce,
        cost: 2,
        val: 0,
        spname: "țintă fixă",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Turneul celor trei",
    res: "avânt",
    sides: 6,
    flags: COUNTER | CRIT,
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
    fn turneul_celor_trei_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
