//! Cronica Națiunilor — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Fermă", [0, 0, 0, 0], [4, 0, 0, 0], 0),
    sl("Pădure", [0, 0, 0, 0], [0, 3, 0, 0], 0),
    sl("Mină", [0, 0, 0, 0], [0, 0, 3, 0], 0),
    sl("Bibliotecă", [0, 0, 0, 0], [2, 0, 0, 1], 0),
    sl("Șantier", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Port", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Școală", [0, 1, 1, 0], [1, 0, 0, 0], 3),
    sl("Barăci", [2, 0, 0, 0], [0, 0, 0, 0], 4),
    sl("Templu", [0, 2, 2, 1], [0, 0, 0, 0], 7),
    sl("Piață", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Moară", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Gazdă", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Bancă", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Universitate", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Pod", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Arsenal", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Observator", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Teatru", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 0] = [];

pub static CFG: Cfg = Cfg {
    title: "Cronica Națiunilor",
    res: ["hrană", "lemn", "aur", "cărți"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 0,
    rounds: 8,
    workers: 2,
    rondel: false,
    market: 4,
    upkeep: 2,
    upkeep_vp: 1,
    maj: 255,
    set_bonus: false,
    vp_div: 2,
    start: [3, 3, 3, 0],
};

pub fn new() -> Econ {
    Econ::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cronica_natiunilor_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
