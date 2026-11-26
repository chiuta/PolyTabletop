//! Elefanți peste Alpi — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Elefanți peste Alpi",
    map: 0,
    n: 15,
    w: 5,
    rounds: 12,
    ap: 2,
    fort: 1,
    win: Win::Capital,
    income_base: 1,
    income_div: 4,
    lev_cost: 3,
    start_troops: 2,
    p2_start: (1, 9),
    loot: 2,
    bribe: false,
    score_each: 0,
    unit: "legiuni",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elefanti_alpi_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
