//! Trupe sub trei sori — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, DIAG, MORALE, MOVEFIRE};

static TYPES: [Ut; 5] = [
    Ut {
        name: "Pușcași stelari",
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
        name: "Trăgători de elită",
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
        name: "Mers blindat",
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
        name: "Tun de câmp",
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
        name: "Comandant",
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

static ARMY: [(u8, i8, i8); 7] = [
    (0, 1, 0),
    (0, 0, 1),
    (1, 1, 2),
    (1, 0, 3),
    (2, 1, 4),
    (3, 0, 5),
    (4, 1, 6),
];

static TERRAIN: [(i8, i8, u8); 16] = [
    (4, 3, 3),
    (3, 4, 3),
    (4, 7, 1),
    (3, 0, 1),
    (3, 6, 1),
    (4, 1, 1),
    (4, 2, 2),
    (3, 5, 2),
    (2, 6, 2),
    (5, 1, 2),
    (2, 3, 2),
    (5, 4, 2),
    (2, 0, 4),
    (5, 7, 4),
    (5, 0, 4),
    (2, 7, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Trupe sub trei sori",
    hex: false,
    w: 8,
    h: 8,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: DIAG | MOVEFIRE | MORALE,
    rounds: 7,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trupe_sub_trei_sori_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
