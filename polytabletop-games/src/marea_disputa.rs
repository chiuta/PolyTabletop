//! Marea Dispută — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Marea Dispută",
    map: 1,
    n: 16,
    w: 0,
    rounds: 10,
    ap: 3,
    fort: 1,
    win: Win::Metro(3),
    income_base: 2,
    income_div: 3,
    lev_cost: 2,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 1,
    bribe: true,
    score_each: 0,
    unit: "adepți",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn marea_disputa_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
