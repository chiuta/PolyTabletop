//! Țesătura Epocilor — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Câmp", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Mină", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Școală", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Turn", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Atelier", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Teatru", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Muzeu", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Observator", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Bibliotecă", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Hambar", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Roată", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Cetate", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Telescop", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Teatru", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Moară", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Pod", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Cărți", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Fabrică", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 3] = [
    ([0, 2, 0, 0], [0, 0, 1, 0], 0),
    ([0, 0, 2, 0], [0, 0, 0, 1], 0),
    ([2, 0, 0, 0], [0, 1, 0, 0], 0),
];

pub static CFG: Cfg = Cfg {
    title: "Țesătura Epocilor",
    res: ["hrană", "metal", "cultură", "știință"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 1,
    rounds: 5,
    workers: 3,
    rondel: false,
    market: 4,
    upkeep: 0,
    upkeep_vp: 0,
    maj: 255,
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
    fn tesatura_epocilor_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
