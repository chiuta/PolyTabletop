//! DICȚIONAR VIU — variantă originală de „explică cuvinte cât mai multe într-o tură”.
//!
//! Motorul [`crate::w4_ghici`] în mod **în rafală**: fiecare tură are un buget de acțiuni
//! (un indiciu arătat sau `treci` costă câte o acțiune); cuvintele curg unul după altul,
//! iar fiecare cuvânt ghicit valorează 1 punct. Nu ai voie să folosești rădăcina cuvântului
//! (faul −1). Cuvintele și explicațiile sunt originale (`dictionar_date`).

use crate::dictionar_date::WORDS;
use crate::w4_comun::*;
use crate::w4_ghici::{text_hint, Cfg, Engine, Item};
use std::sync::OnceLock;

fn build() -> Vec<Item> {
    WORDS
        .iter()
        .map(|&(word, cat, expl)| {
            let forbidden = vec![word.to_string()];
            let list: Vec<&str> = expl.split(';').collect();
            let hints: Vec<_> = list.iter().map(|c| text_hint(c, &forbidden)).collect();
            let mut profile = tokens(word);
            for e in list.iter().take(2) {
                for t in tokens(e) {
                    if !profile.contains(&t) {
                        profile.push(t);
                    }
                }
            }
            Item {
                word: word.to_string(),
                cat: cat.to_string(),
                note: "Nu spune cuvântul sau rădăcina lui.".to_string(),
                hints,
                profile,
            }
        })
        .collect()
}

fn items() -> &'static [Item] {
    static CELL: OnceLock<Vec<Item>> = OnceLock::new();
    CELL.get_or_init(build)
}

pub static CFG: Cfg = Cfg {
    title: "Dicționar Viu",
    items,
    modes: &["explicații"],
    max_hints: 3,
    rounds: 6,
    burst: Some(9),
    bot_margin: 1400,
    opp_margin: 1300,
    faul_pen: true,
    show_util: true,
    rules: "",
};

pub fn new() -> Engine {
    Engine::new(&CFG)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::w4_ghici::{check_cfg, check_no_peek};

    #[test]
    fn engine_invariants() {
        check_cfg(&CFG);
        check_no_peek(&CFG);
    }

    #[test]
    fn three_explanations_each() {
        for it in items() {
            assert!(it.hints.len() == 3, "{}", it.word);
        }
    }
}
