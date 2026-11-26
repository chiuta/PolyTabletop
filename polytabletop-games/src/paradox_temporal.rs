//! Paradoxul Temporal — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 12] = [
    sl("Fermă", [0, 0, 0, 0], [4, 0, 0, 0], 0),
    sl("Mină", [0, 0, 0, 0], [0, 3, 0, 0], 0),
    sl("Cristalărie", [0, 0, 0, 0], [0, 0, 3, 0], 0),
    sl("Cronograf", [0, 0, 0, 0], [2, 0, 0, 1], 0),
    sl("Atelier", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Poartă", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Observator", [0, 1, 1, 0], [1, 0, 0, 0], 3),
    sl("Arhivă", [2, 0, 0, 0], [0, 0, 0, 0], 4),
    sl("Bază", [0, 2, 2, 1], [0, 0, 0, 0], 7),
    sl("Turn", [0, 0, 0, 1], [2, 1, 1, 0], 0),
    sl("Laborator", [0, 0, 0, 0], [2, 1, 1, 0], 0),
    sl("Depou", [1, 0, 0, 1], [0, 0, 0, 0], 4),
];
static CARDS: [Card; 8] = [
    cd("Cronometru", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Scut", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Exo-schelet", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Generator", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Seră", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Pod temporal", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Arcă", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Monument", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 2] = [
    ([0, 2, 0, 0], [0, 0, 1, 0], 0),
    ([0, 0, 2, 0], [1, 0, 0, 0], 1),
];

pub static CFG: Cfg = Cfg {
    title: "Paradoxul Temporal",
    res: ["hrană", "metal", "cristal", "timp"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 1,
    rounds: 7,
    workers: 3,
    rondel: true,
    market: 3,
    upkeep: 2,
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
    fn paradox_temporal_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
