//! Cetăți în Ascensiune — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Cetăți în Ascensiune",
    map: 0,
    n: 16,
    w: 4,
    rounds: 8,
    ap: 3,
    fort: 0,
    win: Win::Points,
    income_base: 1,
    income_div: 1,
    lev_cost: 2,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 0,
    bribe: false,
    score_each: 0,
    unit: "colonii",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cetati_ascensiune_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
