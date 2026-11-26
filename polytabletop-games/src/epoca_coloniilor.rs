//! Epoca Coloniilor — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Epoca Coloniilor",
    map: 2,
    n: 12,
    w: 0,
    rounds: 8,
    ap: 3,
    fort: 0,
    win: Win::Points,
    income_base: 2,
    income_div: 2,
    lev_cost: 2,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 2,
    bribe: false,
    score_each: 0,
    unit: "coloniști",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn epoca_coloniilor_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
