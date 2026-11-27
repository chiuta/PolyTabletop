//! Strajă pe culme — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, CHARGE, DIAG, RETAL};

static TYPES: [Ut; 4] = [
    Ut {
        name: "Străji",
        hp: 4,
        mv: 3,
        rng: 1,
        dice: 3,
        tn: 4,
        save: 5,
        dmg: 1,
        cost: 4,
    },
    Ut {
        name: "Arcași de potecă",
        hp: 3,
        mv: 3,
        rng: 4,
        dice: 3,
        tn: 4,
        save: 7,
        dmg: 1,
        cost: 4,
    },
    Ut {
        name: "Căpetenie",
        hp: 3,
        mv: 4,
        rng: 1,
        dice: 4,
        tn: 3,
        save: 5,
        dmg: 2,
        cost: 5,
    },
    Ut {
        name: "Iscoade călare",
        hp: 4,
        mv: 6,
        rng: 1,
        dice: 3,
        tn: 4,
        save: 5,
        dmg: 1,
        cost: 5,
    },
];

static ARMY: [(u8, i8, i8); 6] = [
    (0, 1, 0),
    (0, 0, 1),
    (1, 1, 2),
    (2, 0, 4),
    (2, 1, 5),
    (3, 0, 6),
];

static TERRAIN: [(i8, i8, u8); 16] = [
    (2, 5, 3),
    (4, 2, 3),
    (3, 3, 1),
    (3, 4, 1),
    (2, 1, 1),
    (4, 6, 1),
    (3, 1, 2),
    (3, 6, 2),
    (4, 1, 2),
    (2, 6, 2),
    (2, 4, 2),
    (4, 3, 2),
    (4, 7, 4),
    (2, 0, 4),
    (3, 0, 4),
    (3, 7, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Strajă pe culme",
    hex: false,
    w: 8,
    h: 7,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: DIAG | RETAL | CHARGE,
    rounds: 7,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn straja_pe_culme_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
