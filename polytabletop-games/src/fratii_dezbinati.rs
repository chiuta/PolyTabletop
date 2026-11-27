//! Frații dezbinați — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, FLANK, MORALE, MOVEFIRE};

static TYPES: [Ut; 5] = [
    Ut {
        name: "Pușcași blindați",
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
        name: "Trăgători grei",
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
        name: "Cuirasat",
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
        name: "Tun de asediu",
        hp: 2,
        mv: 2,
        rng: 6,
        dice: 2,
        tn: 4,
        save: 7,
        dmg: 2,
        cost: 5,
    },
    Ut {
        name: "Căpitan",
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

static TERRAIN: [(i8, i8, u8); 16] = [
    (4, 0, 3),
    (2, 6, 3),
    (3, 2, 1),
    (3, 4, 1),
    (4, 2, 1),
    (2, 4, 1),
    (2, 3, 2),
    (4, 3, 2),
    (2, 2, 2),
    (4, 4, 2),
    (3, 1, 2),
    (3, 5, 2),
    (2, 1, 4),
    (4, 5, 4),
    (2, 0, 4),
    (4, 6, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Frații dezbinați",
    hex: true,
    w: 7,
    h: 7,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: MOVEFIRE | FLANK | MORALE,
    rounds: 6,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fratii_dezbinati_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
