//! Bande în hrube — variantă originală (familia T2, bătălie de miniaturi).
use crate::t2_mini::{Cfg, Mn, Ut, DIAG, MORALE, RETAL};

static TYPES: [Ut; 4] = [
    Ut {
        name: "Pistolar",
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
        name: "Aruncător de grenade",
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
        name: "Șef de bandă",
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
        name: "Mutant uriaș",
        hp: 6,
        mv: 3,
        rng: 1,
        dice: 4,
        tn: 3,
        save: 4,
        dmg: 1,
        cost: 8,
    },
];

static ARMY: [(u8, i8, i8); 5] = [(0, 1, 0), (0, 0, 1), (1, 1, 2), (2, 0, 4), (3, 1, 5)];

static TERRAIN: [(i8, i8, u8); 16] = [
    (2, 6, 3),
    (4, 0, 3),
    (4, 4, 1),
    (2, 2, 1),
    (4, 1, 1),
    (2, 5, 1),
    (4, 5, 2),
    (2, 1, 2),
    (3, 4, 2),
    (3, 2, 2),
    (3, 0, 2),
    (3, 6, 2),
    (4, 3, 4),
    (2, 3, 4),
    (4, 2, 4),
    (2, 4, 4),
];

pub static CFG: Cfg = Cfg {
    title: "Bande în hrube",
    hex: false,
    w: 7,
    h: 7,
    types: &TYPES,
    army: &ARMY,
    terrain: &TERRAIN,
    flags: DIAG | MORALE | RETAL,
    rounds: 7,
};

pub fn new() -> Mn {
    Mn::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bande_in_hrube_ok() {
        crate::t2_mini::check_cfg(&CFG);
    }
}
