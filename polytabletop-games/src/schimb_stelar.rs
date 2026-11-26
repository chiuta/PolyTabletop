//! Schimb Stelar — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Reactor", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Extractor", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Laborator", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Ambasadă", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Piață stelară", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Hangar", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Observator", [0, 1, 1, 0], [1, 0, 0, 0], 3),
    sl("Coridor", [2, 0, 0, 0], [0, 0, 0, 0], 4),
    sl("Arhivă", [0, 2, 2, 1], [0, 0, 0, 0], 7),
    sl("Sanctuar", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 0] = [];
static CONV: [([i8; 4], [i8; 4], i8); 4] = [
    ([0, 2, 0, 0], [0, 0, 1, 0], 0),
    ([0, 0, 2, 0], [0, 0, 0, 1], 0),
    ([2, 0, 0, 0], [0, 1, 0, 0], 0),
    ([0, 0, 0, 2], [1, 0, 0, 0], 1),
];

pub static CFG: Cfg = Cfg {
    title: "Schimb Stelar",
    res: ["energie", "materie", "date", "cultură"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 3,
    rounds: 7,
    workers: 2,
    rondel: false,
    market: 0,
    upkeep: 0,
    upkeep_vp: 0,
    maj: 2,
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
    fn schimb_stelar_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
