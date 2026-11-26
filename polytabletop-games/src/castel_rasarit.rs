//! Castelul Răsăritului — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Castelul Răsăritului",
    map: 0,
    n: 16,
    w: 4,
    rounds: 12,
    ap: 2,
    fort: 1,
    win: Win::Capital,
    income_base: 1,
    income_div: 3,
    lev_cost: 2,
    start_troops: 2,
    p2_start: (3, 2),
    loot: 1,
    bribe: false,
    score_each: 0,
    unit: "samurai",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn castel_rasarit_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
