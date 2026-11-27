//! Cununa veacurilor — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, CRIT, LUCK};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Alchimist",
        hp: 9,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "otravă lentă",
    },
    Cls {
        name: "Ciomăgar",
        hp: 14,
        dice: 1,
        tn: 11,
        dmg: 4,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "lovitură zdrobitoare",
    },
    Cls {
        name: "Vraci",
        hp: 12,
        dice: 1,
        tn: 11,
        dmg: 2,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "leac",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Cununa veacurilor",
    res: "faimă",
    sides: 20,
    flags: CRIT | LUCK,
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
    fn cununa_veacurilor_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
