//! TREI CĂI — variantă originală de joc „spune, mimează sau desenează” (în terminal).
//!
//! Motorul [`crate::w4_ghici`]: runda cere, pe rând, un mod diferit — **vorbit**
//! (descrii cu cuvinte), **mimă** (alegi gesturi) sau **schiță** (alegi tușe). Pentru fiecare
//! cuvânt-acțiune ai la dispoziție aspecte predefinite (`treicai_date`), redate în modul
//! rundei; colegul-robot și adversarul ghicesc dintre 6 candidați. Punctaj ca în motor: cu
//! cât mai puține indicii, cu atât mai bine.

use crate::treicai_date::{VOCAB, WORDS};
use crate::w4_ghici::{Cfg, Engine, Hint, Item};
use std::sync::OnceLock;

fn build() -> Vec<Item> {
    WORDS
        .iter()
        .map(|&(word, cat, feats)| {
            let mut keys: Vec<&str> = Vec::new();
            for k in feats.split_whitespace() {
                if !keys.contains(&k) {
                    keys.push(k);
                }
            }
            let hints = keys
                .iter()
                .map(|k| {
                    let v = VOCAB
                        .iter()
                        .find(|v| v.0 == *k)
                        .unwrap_or_else(|| panic!("aspect necunoscut: {k}"));
                    Hint {
                        texts: vec![capitalize(v.1), capitalize(v.2), capitalize(v.3)],
                        tokens: vec![k.to_string()],
                        faul: false,
                    }
                })
                .collect();
            Item {
                word: word.to_string(),
                cat: cat.to_string(),
                note: String::new(),
                hints,
                profile: keys.iter().map(|k| k.to_string()).collect(),
            }
        })
        .collect()
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn items() -> &'static [Item] {
    static CELL: OnceLock<Vec<Item>> = OnceLock::new();
    CELL.get_or_init(build)
}

pub static CFG: Cfg = Cfg {
    title: "Trei Căi",
    items,
    modes: &[
        "vorbit (descrie cu cuvinte)",
        "mimă (arată cu gesturi)",
        "schiță (desenează)",
    ],
    max_hints: 4,
    rounds: 12,
    burst: None,
    bot_margin: 650,
    opp_margin: 600,
    faul_pen: false,
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
    fn five_distinct_aspects_each() {
        for it in items() {
            assert!(it.hints.len() >= 5, "{}", it.word);
        }
        assert!(VOCAB.len() >= 70);
    }
}
