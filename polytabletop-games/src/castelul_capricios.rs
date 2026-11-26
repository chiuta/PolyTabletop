//! Castelul Capricios — variantă originală (auc5); vezi regulile din catalog.
use crate::auc5::{Auc, Cfg};

pub static CFG: Cfg = Cfg {
    title: "Castelul Capricios",
    kinds: ["turn", "sală", "grădină", "pivniță"],
    lots: 3,
    bundle: false,
    master: true,
    rounds: 8,
    money: 8,
    income: 3,
    scoring: 3,
};

pub fn new() -> Auc {
    Auc::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn castelul_capricios_ok() {
        crate::brand_util::check(|s| Auc::with_seed(&CFG, s), 700);
    }
}
