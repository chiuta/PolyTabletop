//! Veacul regimentelor — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, CHARGE, FLANK, RETAL};

static TYPES: [Ut; 5] = [
    Ut {
        name: "Halebardieri",
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
        name: "Arbaletieri",
        hp: 3,
        mv: 3,
        rng: 5,
        dice: 3,
        tn: 4,
        save: 7,
        dmg: 1,
        cost: 4,
    },
    Ut {
        name: "Cavaleri",
        hp: 4,
        mv: 6,
        rng: 1,
        dice: 3,
        tn: 4,
        save: 5,
        dmg: 1,
        cost: 5,
    },
    Ut {
        name: "Bestie de asediu",
        hp: 6,
        mv: 3,
        rng: 1,
        dice: 4,
        tn: 3,
        save: 4,
        dmg: 1,
        cost: 8,
    },
    Ut {
        name: "Comandant",
        hp: 3,
        mv: 4,
        rng: 1,
        dice: 4,
        tn: 3,
        save: 5,
        dmg: 2,
        cost: 5,
    },
];

static ARMY: [(u8, i8, i8); 6] = [
    (0, 1, 0),
    (0, 0, 1),
    (1, 1, 2),
    (2, 0, 3),
    (3, 1, 4),
    (4, 0, 5),
];

static TERRAIN: [(i8, i8, u8); 15] = [
    (3, 3, 3),
    (2, 5, 1),
    (4, 1, 1),
    (3, 4, 1),
    (3, 2, 1),
    (2, 1, 2),
    (4, 5, 2),
    (2, 6, 2),
    (4, 0, 2),
    (4, 4, 2),
    (2, 2, 2),
    (4, 6, 4),
    (2, 0, 4),
    (3, 0, 4),
    (3, 6, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Veacul regimentelor",
    hex: true,
    w: 7,
    h: 7,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: CHARGE | FLANK | RETAL,
    rounds: 6,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn veacul_regimentelor_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
