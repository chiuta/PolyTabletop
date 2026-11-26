//! Pacea Caravanelor — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Pacea Caravanelor",
    map: 1,
    n: 12,
    w: 0,
    rounds: 9,
    ap: 2,
    fort: 0,
    win: Win::Points,
    income_base: 3,
    income_div: 2,
    lev_cost: 3,
    start_troops: 1,
    p2_start: (3, 1),
    loot: 0,
    bribe: true,
    score_each: 3,
    unit: "loialiști",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pacea_caravane_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
