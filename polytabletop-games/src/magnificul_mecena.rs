//! Magnificul Mecena — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Turn", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Ferma", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Carieră", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Palat", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Mănăstire", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Piață", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Capelă", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Banchet", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Atelier", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Bibliotecă", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Fresca", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Statuie", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Turn", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Palat mic", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Ghilda", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Grădină", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Biserică", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Capelă ducală", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 0] = [];

pub static CFG: Cfg = Cfg {
    title: "Magnificul Mecena",
    res: ["hrană", "lemn", "piatră", "faimă"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 0,
    rounds: 6,
    workers: 3,
    rondel: false,
    market: 4,
    upkeep: 0,
    upkeep_vp: 0,
    maj: 3,
    set_bonus: false,
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
    fn magnificul_mecena_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
