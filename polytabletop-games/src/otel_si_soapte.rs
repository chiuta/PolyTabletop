//! Oțel și șoapte — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, LUCK, RAGE};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Doctor de câmp",
        hp: 11,
        dice: 1,
        tn: 11,
        dmg: 2,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "bandaj",
    },
    Cls {
        name: "Vânător",
        hp: 11,
        dice: 2,
        tn: 10,
        dmg: 2,
        armor: 0,
        sp: Sp::Pierce,
        cost: 2,
        val: 0,
        spname: "săgeată ascuțită",
    },
    Cls {
        name: "Paznic de nopți",
        hp: 12,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Stun,
        cost: 2,
        val: 0,
        spname: "privire fixă",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Oțel și șoapte",
    res: "vrajă",
    sides: 20,
    flags: RAGE | LUCK,
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
    fn otel_si_soapte_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
