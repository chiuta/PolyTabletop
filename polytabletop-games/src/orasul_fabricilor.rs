//! Orașul Fabricilor — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Moară", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Mină", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Oțelărie", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Fabrică", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Port", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Piață", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Școală", [0, 1, 1, 0], [1, 0, 0, 0], 3),
    sl("Gară", [2, 0, 0, 0], [0, 0, 0, 0], 4),
    sl("Teatru", [0, 2, 2, 1], [0, 0, 0, 0], 7),
    sl("Docuri", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Brutărie", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Furnal", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Pod", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Hală", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Turn de apă", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Atelier", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Bancă", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Expoziție", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 2] = [
    ([0, 2, 0, 0], [0, 0, 1, 0], 0),
    ([0, 0, 2, 0], [1, 0, 0, 1], 0),
];

pub static CFG: Cfg = Cfg {
    title: "Orașul Fabricilor",
    res: ["pâine", "cărbune", "oțel", "mărfuri"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 2,
    rounds: 8,
    workers: 3,
    rondel: false,
    market: 4,
    upkeep: 1,
    upkeep_vp: 2,
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
    fn orasul_fabricilor_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
