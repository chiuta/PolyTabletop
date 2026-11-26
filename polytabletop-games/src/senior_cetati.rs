//! Seniorul Cetăților — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Seniorul Cetăților",
    map: 0,
    n: 15,
    w: 5,
    rounds: 6,
    ap: 3,
    fort: 2,
    win: Win::Points,
    income_base: 2,
    income_div: 2,
    lev_cost: 3,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 1,
    bribe: false,
    score_each: 3,
    unit: "cavaleri",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn senior_cetati_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
