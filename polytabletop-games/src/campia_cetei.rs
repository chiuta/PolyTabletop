//! Câmpia Ceții — variantă originală (graf5); vezi regulile din catalog.
use crate::graf5::{Cfg, Graf, Win};

pub static CFG: Cfg = Cfg {
    title: "Câmpia Ceții",
    map: 1,
    n: 10,
    w: 0,
    rounds: 10,
    ap: 2,
    fort: 2,
    win: Win::Capital,
    income_base: 2,
    income_div: 4,
    lev_cost: 2,
    start_troops: 2,
    p2_start: (3, 3),
    loot: 0,
    bribe: false,
    score_each: 0,
    unit: "stindarde",
};

pub fn new() -> Graf {
    Graf::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn campia_cetei_ok() {
        crate::brand_util::check(|s| Graf::with_seed(&CFG, s), 900);
    }
}
