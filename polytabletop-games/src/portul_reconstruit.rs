//! Portul Reconstruit — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Moară", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Carieră", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Olărie", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Bancă", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Șantier", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Port", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Piață", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Bisericuță", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Palat", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Cheu", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Casă", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Magazie", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Biserică", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Navă", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Bancă", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Piață nouă", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Moară", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Palat", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 0] = [];

pub static CFG: Cfg = Cfg {
    title: "Portul Reconstruit",
    res: ["grâu", "piatră", "ceramică", "aur"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 0,
    rounds: 8,
    workers: 2,
    rondel: false,
    market: 4,
    upkeep: 1,
    upkeep_vp: 1,
    maj: 0,
    set_bonus: true,
    vp_div: 0,
    start: [3, 3, 3, 0],
};

pub fn new() -> Econ {
    Econ::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn portul_reconstruit_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
