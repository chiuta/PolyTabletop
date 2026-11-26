//! Lupta Imperiilor — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Lupta Imperiilor",
    map: 0,
    n: 16,
    w: 4,
    rounds: 6,
    ap: 3,
    fort: 1,
    win: Win::Points,
    income_base: 2,
    income_div: 3,
    lev_cost: 2,
    start_troops: 3,
    p2_start: (3, 3),
    loot: 1,
    bribe: false,
    score_each: 0,
    unit: "regimente",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imperii_lupta_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
