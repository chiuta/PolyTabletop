//! Mecenatul Meșterilor — variantă originală (auc5); vezi regulile din catalog.
use crate::auc5::{Auc, Cfg};

pub static CFG: Cfg = Cfg {
    title: "Mecenatul Meșterilor",
    kinds: ["pictor", "sculptor", "muzicant", "poet"],
    lots: 3,
    bundle: false,
    master: false,
    rounds: 6,
    money: 12,
    income: 4,
    scoring: 1,
};

pub fn new() -> Auc {
    Auc::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mecenatul_mesterilor_ok() {
        crate::brand_util::check(|s| Auc::with_seed(&CFG, s), 700);
    }
}
