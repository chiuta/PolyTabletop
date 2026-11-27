//! Regimente pe câmpie — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, CHARGE, FLANK, MORALE};

static TYPES: [Ut; 5] = [
    Ut {
        name: "Pedestrași",
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
        rng: 5,
        dice: 3,
        tn: 4,
        save: 7,
        dmg: 1,
        cost: 4,
    },
    Ut {
        name: "Călăreți",
        hp: 4,
        mv: 5,
        rng: 1,
        dice: 3,
        tn: 4,
        save: 5,
        dmg: 1,
        cost: 5,
    },
    Ut {
        name: "Uriaș de război",
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
    (2, 0, 4),
    (3, 1, 5),
    (4, 0, 6),
];

static TERRAIN: [(i8, i8, u8); 16] = [
    (3, 1, 3),
    (4, 6, 3),
    (3, 6, 1),
    (4, 1, 1),
    (5, 1, 1),
    (2, 6, 1),
    (5, 4, 2),
    (2, 3, 2),
    (5, 7, 2),
    (2, 0, 2),
    (4, 7, 2),
    (3, 0, 2),
    (2, 2, 4),
    (5, 5, 4),
    (5, 0, 4),
    (2, 7, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Regimente pe câmpie",
    hex: false,
    w: 8,
    h: 8,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: FLANK | CHARGE | MORALE,
    rounds: 6,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn regimente_pe_cimpie_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
