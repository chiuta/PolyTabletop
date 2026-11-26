//! SCHIȚA — variantă originală de „desenează și ghicește”, în terminal.
//!
//! Desenatorul alege, una câte una, tușe dintr-un set de schițe textuale ale
//! cuvântului secret (cerc, coadă, urechi ascuțite…). Colegul-robot și adversarul
//! se uită la tușe și ghicesc dintre 6 candidați; cu cât mai puține tușe, cu atât
//! mai multe puncte. Motorul: [`crate::w4_ghici`].

use crate::schita_date::{VOCAB, WORDS};
use crate::w4_ghici::{Cfg, Engine, Hint, Item};
use std::sync::OnceLock;

fn build() -> Vec<Item> {
    WORDS
        .iter()
        .map(|&(word, cat, feats)| {
            let keys: Vec<&str> = feats.split_whitespace().collect();
            let hints = keys
                .iter()
                .map(|k| {
                    let text = VOCAB
                        .iter()
                        .find(|(kk, _)| kk == k)
                        .unwrap_or_else(|| panic!("tușă necunoscută: {k}"))
                        .1;
                    Hint {
                        texts: vec![text.to_string()],
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

fn items() -> &'static [Item] {
    static CELL: OnceLock<Vec<Item>> = OnceLock::new();
    CELL.get_or_init(build)
}

pub static CFG: Cfg = Cfg {
    title: "Schița",
    items,
    modes: &["desen"],
    max_hints: 5,
    rounds: 10,
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
    fn six_features_each_and_known_keys() {
        for it in items() {
            assert!(it.hints.len() >= 5, "{}", it.word);
        }
        assert!(VOCAB.len() >= 80);
    }

    #[test]
    fn no_unsubstituted_placeholder_in_hints() {
        for (k, text) in VOCAB {
            assert!(
                !text.contains('_') && !text.contains('{') && !text.contains('}'),
                "indiciu cu placeholder: {k} -> {text}"
            );
        }
        for it in items() {
            for h in &it.hints {
                for t in &h.texts {
                    assert!(
                        !t.contains('_') && !t.contains('{') && !t.contains('}'),
                        "{}: indiciu cu placeholder: {t}",
                        it.word
                    );
                }
            }
        }
    }
}
