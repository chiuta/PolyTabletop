//! Cartierele Catedralei — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 10] = [
    sl("Livadă", [0, 0, 0, 0], [3, 0, 0, 0], 0),
    sl("Podgorie", [0, 0, 0, 0], [0, 2, 0, 0], 0),
    sl("Carieră", [0, 0, 0, 0], [0, 0, 2, 0], 0),
    sl("Piață", [0, 0, 0, 0], [1, 0, 0, 1], 0),
    sl("Han", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Spital", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Mănăstire", [0, 1, 1, 0], [1, 0, 0, 0], 3),
    sl("Pod", [2, 0, 0, 0], [0, 0, 0, 0], 4),
    sl("Cămară", [0, 2, 2, 1], [0, 0, 0, 0], 7),
    sl("Primărie", [0, 0, 0, 1], [2, 1, 1, 0], 0),
];
static CARDS: [Card; 8] = [
    cd("Casă de breaslă", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Clopotniță", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Piață nouă", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Poartă", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Capelă", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Prăvălie", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Atelier", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Palat", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 0] = [];

pub static CFG: Cfg = Cfg {
    title: "Cartierele Catedralei",
    res: ["hrană", "vin", "pietre", "influență"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 0,
    rounds: 9,
    workers: 3,
    rondel: false,
    market: 3,
    upkeep: 2,
    upkeep_vp: 2,
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
    fn cartierele_catedralei_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
