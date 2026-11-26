//! Insulele Zeilor — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Insulele Zeilor",
    map: 2,
    n: 14,
    w: 0,
    rounds: 8,
    ap: 3,
    fort: 0,
    win: Win::Metro(3),
    income_base: 2,
    income_div: 3,
    lev_cost: 2,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 1,
    bribe: false,
    score_each: 0,
    unit: "armate",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insule_zeii_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
