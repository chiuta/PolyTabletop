//! Peștera Monștrilor — variantă originală (deck5); vezi regulile din catalog.
use crate::deck5::{hero, mon, Cfg, Deck, C};

static POOL: [C; 16] = [
    hero("Novice", 0, 1, 0, 0, 0),
    hero("Strajă", 0, 0, 1, 0, 0),
    hero("Felinar", 2, 1, 0, 1, 0),
    hero("Arcaș", 3, 0, 2, 0, 1),
    hero("Preot", 3, 1, 1, 1, 1),
    hero("Războinic", 4, 0, 3, 0, 1),
    hero("Cercetaș", 3, 1, 1, 1, 0),
    hero("Mag", 5, 1, 2, 1, 2),
    hero("Luptător greu", 6, 0, 4, 0, 2),
    hero("Ghid", 4, 2, 0, 1, 1),
    mon("Șobolani", 3, 1, 1, 1),
    mon("Schelet", 5, 2, 1, 2),
    mon("Cobold", 4, 2, 2, 1),
    mon("Mumie", 7, 3, 2, 3),
    mon("Hidră", 8, 4, 3, 2),
    mon("Behemot", 10, 6, 3, 3),
];

pub static CFG: Cfg = Cfg {
    title: "Peștera Monștrilor",
    pool: &POOL,
    row: 6,
    hrow: 3,
    turns: 15,
    target: 28,
};

pub fn new() -> Deck {
    Deck::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pestera_monstrilor_ok() {
        crate::brand_util::check(|s| Deck::with_seed(&CFG, s), 800);
    }
}
