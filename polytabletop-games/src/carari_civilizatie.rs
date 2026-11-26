//! Cărări de Civilizație — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Cărări de Civilizație",
    map: 2,
    n: 16,
    w: 0,
    rounds: 10,
    ap: 2,
    fort: 1,
    win: Win::Points,
    income_base: 2,
    income_div: 2,
    lev_cost: 3,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 0,
    bribe: false,
    score_each: 5,
    unit: "triburi",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn carari_civilizatie_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
