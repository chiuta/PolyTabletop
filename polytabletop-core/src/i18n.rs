//! Internationalisation: the eight UI languages, the process-global current
//! language and the UI string table (`data/ui.tsv`).
//!
//! Fallback chain for [`tr`]: current language -> English -> Romanian -> the key.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

/// Supported interface languages. Romanian is the source language.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Lang {
    En,
    Ro,
    Fr,
    It,
    Es,
    Pt,
    De,
    Hu,
}

impl Lang {
    pub const ALL: [Lang; 8] = [
        Lang::En,
        Lang::Ro,
        Lang::Fr,
        Lang::It,
        Lang::Es,
        Lang::Pt,
        Lang::De,
        Lang::Hu,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ro => "ro",
            Lang::Fr => "fr",
            Lang::It => "it",
            Lang::Es => "es",
            Lang::Pt => "pt",
            Lang::De => "de",
            Lang::Hu => "hu",
        }
    }

    pub fn name_native(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Ro => "Română",
            Lang::Fr => "Français",
            Lang::It => "Italiano",
            Lang::Es => "Español",
            Lang::Pt => "Português",
            Lang::De => "Deutsch",
            Lang::Hu => "Magyar",
        }
    }

    /// Parses a bare code (`de`) or a locale (`de_DE.UTF-8`, `pt-BR`), case-insensitive.
    pub fn from_code(s: &str) -> Option<Lang> {
        let s = s.trim().to_ascii_lowercase();
        let prefix: String = s.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        if prefix.len() != 2 {
            return None;
        }
        Lang::ALL.into_iter().find(|l| l.code() == prefix)
    }

    fn idx(self) -> u8 {
        Lang::ALL.iter().position(|l| *l == self).unwrap() as u8
    }
}

/// Comma-separated list of all codes (`en, ro, fr, ...`).
pub fn codes_list() -> String {
    Lang::ALL
        .iter()
        .map(|l| l.code())
        .collect::<Vec<_>>()
        .join(", ")
}

static CURRENT: AtomicU8 = AtomicU8::new(1); // Ro

pub fn set_lang(l: Lang) {
    CURRENT.store(l.idx(), Ordering::Relaxed);
}

pub fn lang() -> Lang {
    Lang::ALL[CURRENT.load(Ordering::Relaxed) as usize % 8]
}

/// Pure detection (testable): explicit flag, then `POLYTABLETOP_LANG`, then the
/// locale variables in order `LC_ALL`, `LC_MESSAGES`, `LANG`; default Romanian.
/// Unsupported locales (`C`, `POSIX`, `fi_FI`) are skipped.
pub fn detect_from(
    flag: Option<&str>,
    masa_lang: Option<&str>,
    lc_all: Option<&str>,
    lc_messages: Option<&str>,
    lang_var: Option<&str>,
) -> Lang {
    [flag, masa_lang, lc_all, lc_messages, lang_var]
        .into_iter()
        .flatten()
        .find_map(Lang::from_code)
        .unwrap_or(Lang::Ro)
}

/// Detection from the real process environment.
pub fn detect(flag: Option<&str>) -> Lang {
    let g = |n: &str| std::env::var(n).ok();
    detect_from(
        flag,
        g("POLYTABLETOP_LANG").as_deref(),
        g("LC_ALL").as_deref(),
        g("LC_MESSAGES").as_deref(),
        g("LANG").as_deref(),
    )
}

/// Application name (the project is being renamed: never hard-code it).
pub const APP_NAME: &str = "PolyTabletop";

const UI_TSV: &str = include_str!("../data/ui.tsv");

pub fn unescape(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() {
                Some('n') => o.push('\n'),
                Some('t') => o.push('\t'),
                Some(x) => o.push(x),
                None => o.push('\\'),
            }
        } else {
            o.push(c);
        }
    }
    o
}

type Table = HashMap<(&'static str, Lang), &'static str>;

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| parse_ui(UI_TSV))
}

fn parse_ui(src: &'static str) -> Table {
    let mut m = HashMap::new();
    for line in src.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut p = line.splitn(3, '\t');
        let (Some(k), Some(l), Some(t)) = (p.next(), p.next(), p.next()) else {
            continue;
        };
        if let Some(l) = Lang::from_code(l) {
            m.insert(
                (k, l),
                Box::leak(unescape(t).into_boxed_str()) as &'static str,
            );
        }
    }
    m
}

/// All UI keys with their languages (for tests / tooling).
pub fn ui_entries() -> Vec<(&'static str, Lang, &'static str)> {
    let mut v: Vec<_> = table().iter().map(|((k, l), t)| (*k, *l, *t)).collect();
    v.sort_by_key(|(k, l, _)| (*k, l.idx()));
    v
}

/// Translation of `key` in `l`, falling back to English, then Romanian, then the key.
pub fn tr_in(l: Lang, key: &'static str) -> &'static str {
    let t = table();
    [l, Lang::En, Lang::Ro]
        .iter()
        .find_map(|x| t.get(&(key, *x)).copied())
        .unwrap_or(key)
}

/// Translation in the current language.
pub fn tr(key: &'static str) -> &'static str {
    tr_in(lang(), key)
}

/// Replaces each `{name}` in `text` with its value.
pub fn fmt(text: &str, vars: &[(&str, &str)]) -> String {
    let mut s = text.to_string();
    for (k, v) in vars {
        s = s.replace(&format!("{{{k}}}"), v);
    }
    s
}

/// `tr(key)` followed by placeholder substitution.
pub fn tr_fmt(key: &'static str, vars: &[(&str, &str)]) -> String {
    fmt(tr(key), vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_has_all_languages() {
        let e = ui_entries();
        assert!(!e.is_empty());
        let mut keys: Vec<&str> = e.iter().map(|x| x.0).collect();
        keys.dedup();
        for k in keys {
            for l in Lang::ALL {
                let t = e.iter().find(|x| x.0 == k && x.1 == l);
                assert!(
                    t.is_some_and(|x| !x.2.trim().is_empty()),
                    "{k} / {}",
                    l.code()
                );
            }
        }
    }

    #[test]
    fn placeholders_match_across_languages() {
        let e = ui_entries();
        let ph = |s: &str| {
            let mut v: Vec<String> = s
                .split('{')
                .skip(1)
                .filter_map(|x| x.split('}').next().map(str::to_string))
                .collect();
            v.sort();
            v
        };
        for (k, l, t) in &e {
            let ro = e.iter().find(|x| x.0 == *k && x.1 == Lang::Ro).unwrap().2;
            assert_eq!(ph(t), ph(ro), "{k} / {}", l.code());
        }
    }

    #[test]
    fn codes_roundtrip() {
        for l in Lang::ALL {
            assert_eq!(Lang::from_code(l.code()), Some(l));
        }
        assert_eq!(Lang::from_code("de_DE.UTF-8"), Some(Lang::De));
        assert_eq!(Lang::from_code("PT-br"), Some(Lang::Pt));
        assert_eq!(Lang::from_code("xx"), None);
        assert_eq!(Lang::from_code("C"), None);
        assert_eq!(Lang::from_code("deu"), None);
    }

    #[test]
    fn detection_order() {
        let d = detect_from;
        assert_eq!(d(None, None, None, None, None), Lang::Ro);
        assert_eq!(d(Some("fr"), Some("de"), None, None, None), Lang::Fr);
        assert_eq!(
            d(None, Some("de"), Some("it_IT.UTF-8"), None, None),
            Lang::De
        );
        assert_eq!(
            d(None, None, Some("it_IT.UTF-8"), Some("es"), Some("pt")),
            Lang::It
        );
        assert_eq!(
            d(None, None, Some("C"), None, Some("hu_HU.UTF-8")),
            Lang::Hu
        );
        assert_eq!(d(None, None, None, None, Some("fi_FI")), Lang::Ro);
    }

    #[test]
    fn fallback_chain() {
        assert_eq!(tr_in(Lang::De, "no.such.key"), "no.such.key");
        assert_eq!(tr_in(Lang::De, "play.draw"), "Remis.");
        // Romanian source text is unchanged.
        assert_eq!(tr_in(Lang::Ro, "play.draw"), "Remiză.");
        // Missing in a language -> English, then Romanian.
        let t = parse_ui("a.b\ten\tEN\na.b\tro\tRO\nc.d\tro\tRO\n");
        let get = |k, l| {
            [l, Lang::En, Lang::Ro]
                .iter()
                .find_map(|x| t.get(&(k, *x)).copied())
        };
        assert_eq!(get("a.b", Lang::De), Some("EN"));
        assert_eq!(get("c.d", Lang::De), Some("RO"));
    }

    #[test]
    fn current_language_switch() {
        set_lang(Lang::Hu);
        assert_eq!(lang(), Lang::Hu);
        set_lang(Lang::Ro);
        assert_eq!(lang(), Lang::Ro);
    }
}
