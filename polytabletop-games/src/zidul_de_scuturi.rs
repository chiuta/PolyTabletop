//! Zidul de scuturi — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, CHARGE, FLANK, MORALE, RETAL};

static TYPES: [Ut; 5] = [
    Ut {
        name: "Scutieri",
        hp: 5,
        mv: 3,
        rng: 1,
        dice: 3,
        tn: 4,
        save: 5,
        dmg: 1,
        cost: 4,
    },
    Ut {
        name: "Arcași",
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
        name: "Lăncieri călare",
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
        name: "Monstru de luptă",
        hp: 7,
        mv: 3,
        rng: 1,
        dice: 4,
        tn: 3,
        save: 4,
        dmg: 1,
        cost: 8,
    },
    Ut {
        name: "Stegar",
        hp: 3,
        mv: 4,
        rng: 1,
        dice: 3,
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
    (2, 0, 4),
    (3, 1, 5),
    (4, 0, 6),
];

static TERRAIN: [(i8, i8, u8); 14] = [
    (2, 0, 1),
    (5, 7, 1),
    (3, 1, 1),
    (4, 6, 1),
    (2, 2, 2),
    (5, 5, 2),
    (3, 5, 2),
    (4, 2, 2),
    (4, 3, 2),
    (3, 4, 2),
    (4, 0, 4),
    (3, 7, 4),
    (4, 5, 4),
    (3, 2, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Zidul de scuturi",
    hex: true,
    w: 8,
    h: 8,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: FLANK | CHARGE | MORALE | RETAL,
    rounds: 6,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zidul_de_scuturi_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
