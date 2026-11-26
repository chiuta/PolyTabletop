//! CUVINTE OPRITE — variantă originală de „descrie fără să spui cuvintele interzise”.
//!
//! Motorul [`crate::w4_ghici`]: descriptorul alege, pe rând, câte un indiciu-propoziție
//! dintre cele predefinite ale cuvântului; unele alunecă pe un cuvânt interzis (sau pe
//! cuvântul-țintă) și sunt **fauluri** (−1, detectate automat). Colegul-robot și
//! adversarul ghicesc dintre 6 candidați. Cuvinte, interdicții și indicii sunt originale
//! (`oprite_date`).

use crate::oprite_date::CARDS;
use crate::w4_comun::*;
use crate::w4_ghici::{text_hint, Cfg, Engine, Item};
use std::sync::OnceLock;

fn build() -> Vec<Item> {
    CARDS
        .iter()
        .map(|&(word, cat, forb, clues)| {
            let mut forbidden: Vec<String> = forb.split_whitespace().map(String::from).collect();
            forbidden.push(word.to_string());
            let list: Vec<&str> = clues.split(';').collect();
            let hints: Vec<_> = list.iter().map(|c| text_hint(c, &forbidden)).collect();
            let mut profile = tokens(forb);
            for t in tokens(word).into_iter().chain(tokens(list[0])) {
                if !profile.contains(&t) {
                    profile.push(t);
                }
            }
            Item {
                word: word.to_string(),
                cat: cat.to_string(),
                note: format!(
                    "Cuvinte interzise: {} (și cuvântul însuși).",
                    forb.split_whitespace().collect::<Vec<_>>().join(", ")
                ),
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
    title: "Cuvinte Oprite",
    items,
    modes: &["indicii vorbite"],
    max_hints: 4,
    rounds: 10,
    burst: None,
    bot_margin: 1000,
    opp_margin: 900,
    faul_pen: true,
    show_util: false,
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
    fn each_card_has_safe_and_faul_hints() {
        for it in items() {
            let f = it.hints.iter().filter(|h| h.faul).count();
            assert!(f >= 1, "{}: niciun faul", it.word);
            assert!(
                it.hints.len() - f >= 2,
                "{}: prea puține indicii sigure",
                it.word
            );
            assert!(it.hints.len() >= 4, "{}", it.word);
        }
    }

    #[test]
    fn faul_detection() {
        let f = vec![
            "miau".to_string(),
            "mustăți".to_string(),
            "pisică".to_string(),
        ];
        assert!(crate::w4_ghici::text_is_faul("Face miau seara", &f));
        assert!(crate::w4_ghici::text_is_faul("O PISICĂ neagră", &f));
        assert!(crate::w4_ghici::text_is_faul("cu mustățile lungi", &f));
        assert!(!crate::w4_ghici::text_is_faul(
            "Animal de casă care toarce",
            &f
        ));
    }
}
