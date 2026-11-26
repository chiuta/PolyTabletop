//! Esențe Arcane — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Izvor", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Grotă", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Vulcan", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Mină", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Altar", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Sală de ritualuri", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Turn", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Sanctuar", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Bibliotecă", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Cerc de piatră", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Amuletă", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Cristal", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Sceptru", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Monument vechi", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Cazan", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Lentilă", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Oglindă", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Grimoar", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 3] = [
    ([0, 2, 0, 0], [0, 0, 1, 0], 0),
    ([0, 0, 2, 0], [1, 0, 0, 0], 0),
    ([2, 0, 0, 0], [0, 0, 0, 1], 0),
];

pub static CFG: Cfg = Cfg {
    title: "Esențe Arcane",
    res: ["viață", "calm", "foc", "aur"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 3,
    rounds: 6,
    workers: 2,
    rondel: false,
    market: 3,
    upkeep: 0,
    upkeep_vp: 0,
    maj: 3,
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
    fn esente_arcane_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
