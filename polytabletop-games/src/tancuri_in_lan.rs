//! Tancuri în lan — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, FLANK, MOVEFIRE};

static TYPES: [Ut; 5] = [
    Ut {
        name: "Pușcași",
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
        name: "Tun antitanc",
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
        name: "Tanc greu",
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
        name: "Autoblindat",
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
        name: "Căpitan",
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
    (2, 0, 3),
    (2, 1, 5),
    (3, 0, 6),
    (4, 1, 7),
];

static TERRAIN: [(i8, i8, u8); 15] = [
    (3, 4, 3),
    (4, 1, 1),
    (2, 7, 1),
    (4, 5, 1),
    (2, 3, 1),
    (2, 6, 2),
    (4, 2, 2),
    (3, 5, 2),
    (3, 3, 2),
    (2, 8, 2),
    (4, 0, 2),
    (4, 8, 4),
    (2, 0, 4),
    (3, 8, 4),
    (3, 0, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Tancuri în lan",
    hex: false,
    w: 9,
    h: 7,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: MOVEFIRE | FLANK,
    rounds: 6,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tancuri_in_lan_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
