//! Companiile de pe Coastă — variantă originală (econ5); vezi regulile din catalog.
use crate::econ5::{cd, sl, Card, Cfg, Econ, Slot};

static SLOTS: [Slot; 12] = [
    sl("Plantație", [0, 0, 0, 0], [4, 0, 0, 0], 0),
    sl("Livadă", [0, 0, 0, 0], [0, 3, 0, 0], 0),
    sl("Bancă", [0, 0, 0, 0], [0, 0, 3, 0], 0),
    sl("Depozit", [0, 0, 0, 0], [2, 0, 0, 1], 0),
    sl("Port", [0, 2, 0, 0], [0, 0, 0, 2], 0),
    sl("Fabrică", [0, 0, 2, 0], [0, 0, 0, 2], 0),
    sl("Birou", [0, 1, 1, 0], [1, 0, 0, 0], 2),
    sl("Prăvălie", [2, 0, 0, 0], [0, 0, 0, 0], 3),
    sl("Piață", [0, 2, 2, 1], [0, 0, 0, 0], 6),
    sl("Companie", [0, 0, 0, 1], [2, 1, 1, 0], 0),
    sl("Han", [0, 0, 0, 0], [2, 1, 1, 0], 0),
    sl("Far", [1, 0, 0, 1], [0, 0, 0, 0], 3),
];
static CARDS: [Card; 8] = [
    cd("Depozit", [0, 2, 1, 0], [1, 0, 0, 0], 1),
    cd("Navă", [2, 0, 0, 1], [0, 1, 1, 0], 0),
    cd("Magazie", [0, 1, 2, 0], [0, 0, 0, 1], 2),
    cd("Birou nou", [0, 0, 0, 3], [0, 0, 0, 0], 6),
    cd("Atelier", [3, 0, 0, 0], [2, 0, 0, 0], 0),
    cd("Vamă", [0, 3, 0, 0], [0, 2, 0, 0], 1),
    cd("Seif", [0, 0, 3, 0], [0, 0, 2, 0], 1),
    cd("Acțiuni mari", [1, 1, 1, 1], [1, 0, 0, 1], 3),
];
static CONV: [([i8; 4], [i8; 4], i8); 0] = [];

pub static CFG: Cfg = Cfg {
    title: "Companiile de pe Coastă",
    res: ["bumbac", "cafea", "aur", "acțiuni"],
    slots: &SLOTS,
    cards: &CARDS,
    conv: &CONV,
    conv_lim: 0,
    rounds: 6,
    workers: 3,
    rondel: true,
    market: 3,
    upkeep: 0,
    upkeep_vp: 0,
    maj: 1,
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
    fn companii_coasta_ok() {
        crate::brand_util::check(|s| Econ::with_seed(&CFG, s), 700);
    }
}
