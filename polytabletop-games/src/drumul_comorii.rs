//! Drumul Comorii — variantă originală (lista B4), motor `db`; clean-room: doar mecanica generală a genului.
use crate::b4_db::*;

static SPEC: DbSpec = DbSpec {
    title: "Drumul Comorii",
    mode: 'r',
    market: 5,
    goal: 30,
    turns: 25,
    depth: 0,
};

pub fn new() -> Db {
    Db::new(&SPEC)
}

pub fn with_seed(seed: u64) -> Db {
    Db::with_seed(&SPEC, seed)
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 1500);
    }
}
