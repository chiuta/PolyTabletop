//! Forumul Imperial — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 12] = [
    sl("Cereale", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Tăiere", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Mină", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Taxe", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Forum", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Armată", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Drum", [0, 1, 1, 0], [1, 0, 0, 0], 3),
    sl("Port", [2, 0, 0, 0], [0, 0, 0, 0], 4),
    sl("Teatru", [0, 2, 2, 1], [0, 0, 0, 0], 7),
    sl("Termen", [0, 0, 0, 1], [2, 1, 1, 0], 0),
    sl("Colonie", [0, 0, 0, 0], [1, 1, 1, 0], 0),
    sl("Arc", [1, 0, 0, 1], [0, 0, 0, 0], 4),
];
static CARDS: [Card; 8] = [
    cd("Moară", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Bază", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Carieră imperială", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Arc de triumf", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Cazarmă", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Aqueduct", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Șantier", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Templu mare", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 0] = [];

pub static CFG: Cfg = Cfg {
    title: "Forumul Imperial",
    res: ["grâne", "lemn", "piatră", "monede"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 0,
    rounds: 6,
    workers: 3,
    rondel: true,
    market: 3,
    upkeep: 1,
    upkeep_vp: 1,
    maj: 255,
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
    fn forum_imperial_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
