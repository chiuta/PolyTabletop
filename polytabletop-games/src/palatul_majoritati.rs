//! Palatul cu Majorități — variantă originală (auc5); vezi regulile din catalog.
use crate::auc5::{Auc, Cfg};

pub static CFG: Cfg = Cfg {
    title: "Palatul cu Majorități",
    kinds: ["jad", "mătase", "mirodenii", "aur"],
    lots: 3,
    bundle: false,
    master: false,
    rounds: 7,
    money: 12,
    income: 3,
    scoring: 2,
};

pub fn new() -> Auc {
    Auc::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn palatul_majoritati_ok() {
        crate::brand_util::check(|s| Auc::with_seed(&CFG, s), 700);
    }
}
