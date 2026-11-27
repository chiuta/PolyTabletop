//! Epoca cenușie — variantă originală (familia T2, duel de aventurieri).
use crate::t2_duel::{Cfg, Cls, Duel, Sp, BLEED, COUNTER};

static CLASSES: [Cls; 3] = [
    Cls {
        name: "Vrăjitoare de râu",
        hp: 8,
        dice: 2,
        tn: 6,
        dmg: 2,
        armor: 0,
        sp: Sp::Poison,
        cost: 2,
        val: 2,
        spname: "blestem",
    },
    Cls {
        name: "Frate pavăzar",
        hp: 12,
        dice: 2,
        tn: 6,
        dmg: 1,
        armor: 2,
        sp: Sp::Heal,
        cost: 2,
        val: 4,
        spname: "leac",
    },
    Cls {
        name: "Mercenar",
        hp: 13,
        dice: 2,
        tn: 6,
        dmg: 2,
        armor: 1,
        sp: Sp::Heavy,
        cost: 2,
        val: 2,
        spname: "izbitură de umăr",
    },
];

pub static CFG: Cfg = Cfg {
    title: "Epoca cenușie",
    res: "voință",
    sides: 10,
    flags: BLEED | COUNTER,
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
    fn epoca_cenusie_ok() {
        crate::t2_duel::check_cfg(&CFG);
    }
}
