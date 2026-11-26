//! Duelul Cărților — variantă originală (deck5); vezi regulile din catalog.
use crate::deck5::{hero, mon, Cfg, Deck, C};

static POOL: [C; 16] = [
    hero("Novice", 0, 1, 0, 0, 0),
    hero("Strajă", 0, 0, 1, 0, 0),
    hero("Ucenic mistic", 3, 2, 0, 0, 0),
    hero("Arcaș", 2, 0, 2, 0, 1),
    hero("Vrăjitoare", 3, 2, 1, 0, 1),
    hero("Paznic", 4, 0, 3, 0, 1),
    hero("Maestru", 5, 3, 1, 0, 2),
    hero("Negustor", 3, 3, 0, 0, 1),
    hero("Căpitan", 6, 1, 4, 0, 2),
    hero("Oracol", 5, 4, 0, 0, 2),
    mon("Goblin", 3, 1, 1, 0),
    mon("Troll", 5, 2, 1, 0),
    mon("Strigoi", 6, 3, 2, 0),
    mon("Dragon mic", 8, 4, 2, 0),
    mon("Spectru", 4, 2, 0, 0),
    mon("Colos", 9, 5, 3, 0),
];

pub static CFG: Cfg = Cfg {
    title: "Duelul Cărților",
    pool: &POOL,
    row: 5,
    hrow: 0,
    turns: 15,
    target: 28,
};

pub fn new() -> Deck {
    Deck::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duelul_cartilor_ok() {
        crate::brand_util::check(|s| Deck::with_seed(&CFG, s), 800);
    }
}
