//! Călătoria Oracolului — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Câmp", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Pădure", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Carieră", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Altar", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Piață de lemn", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Piață de piatră", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Atelier", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Templu", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Monument", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Port", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 0] = [];
static CONV: [([i8; 4], [i8; 4], i8); 3] = [
    ([0, 2, 0, 0], [0, 0, 1, 0], 0),
    ([0, 0, 2, 0], [1, 1, 0, 0], 0),
    ([2, 0, 0, 0], [0, 0, 0, 0], 1),
];

pub static CFG: Cfg = Cfg {
    title: "Călătoria Oracolului",
    res: ["grâne", "lemn", "piatră", "daruri"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 2,
    rounds: 7,
    workers: 3,
    rondel: false,
    market: 0,
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
    fn calatoria_oracolului_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
