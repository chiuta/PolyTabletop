//! Mănăstirea Harnică — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Cuptor", [0, 0, 0, 0], [4, 0, 0, 0], 0),
    sl("Turbărie", [0, 0, 0, 0], [0, 3, 0, 0], 0),
    sl("Lutărie", [0, 0, 0, 0], [0, 0, 3, 0], 0),
    sl("Sticlărie", [0, 0, 0, 0], [2, 0, 0, 1], 0),
    sl("Scriptoriu", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Grădină", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Capelă", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Bibliotecă", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Moară", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Cămară", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Fierărie", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Pivniță", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Brutărie", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Vitraliu mare", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Turn", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Ferma", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Hambar", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Atelier", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 2] = [
    ([0, 2, 0, 0], [0, 0, 0, 1], 0),
    ([0, 0, 2, 0], [1, 0, 0, 0], 0),
];

pub static CFG: Cfg = Cfg {
    title: "Mănăstirea Harnică",
    res: ["pâine", "turbă", "lut", "vitralii"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 2,
    rounds: 8,
    workers: 2,
    rondel: false,
    market: 4,
    upkeep: 1,
    upkeep_vp: 1,
    maj: 255,
    set_bonus: false,
    vp_div: 3,
    start: [3, 3, 3, 0],
};

pub fn new() -> Econ {
    Econ::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manastirea_harnica_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
