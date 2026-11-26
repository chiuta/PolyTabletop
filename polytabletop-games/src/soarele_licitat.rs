//! Soarele Licitat — variantă originală (auc5); vezi regulile din catalog.
use crate::auc5::{Auc, Cfg};

pub static CFG: Cfg = Cfg {
    title: "Soarele Licitat",
    kinds: ["aur", "granit", "papirus", "zei"],
    lots: 4,
    bundle: true,
    master: false,
    rounds: 6,
    money: 10,
    income: 3,
    scoring: 0,
};

pub fn new() -> Auc {
    Auc::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soarele_licitat_ok() {
        crate::brand_util::check(|s| Auc::with_seed(&CFG, s), 700);
    }
}
