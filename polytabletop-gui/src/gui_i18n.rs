//! Localizarea interfeței grafice (8 limbi): `tr(cheie)` din `data/gui.tsv`.
//!
//! Format TSV: `cheie<TAB>limbă<TAB>text`; `{0}`, `{1}`… sunt argumente (`trf`).
//! Căutare cu rezervă: limba curentă → en → ro → cheia însăși.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

/// Numele aplicației (în curs de redenumire: un singur loc de schimbat).
pub const APP_NAME: &str = "PolyTabletop";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Numele limbii în limba ei.
    pub fn native(self) -> &'static str {
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

    /// Acceptă `ro`, `RO`, `ro_RO.UTF-8`, `pt-BR`…
    pub fn parse(s: &str) -> Option<Lang> {
        let p: String = s
            .trim()
            .chars()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_ascii_lowercase();
        Lang::ALL.into_iter().find(|l| l.code() == p)
    }

    fn idx(self) -> u8 {
        Lang::ALL.iter().position(|l| *l == self).unwrap_or(1) as u8
    }
}

static CUR: AtomicU8 = AtomicU8::new(1); // ro

pub fn set_lang(l: Lang) {
    CUR.store(l.idx(), Ordering::Relaxed);
}

pub fn lang() -> Lang {
    Lang::ALL[(CUR.load(Ordering::Relaxed) as usize).min(7)]
}

type Table = HashMap<&'static str, [&'static str; 8]>;

fn rows() -> impl Iterator<Item = (&'static str, usize, &'static str)> {
    include_str!("../data/gui.tsv").lines().filter_map(|line| {
        let mut it = line.splitn(3, '\t');
        let (k, l, t) = (it.next()?, it.next()?, it.next()?);
        let i = Lang::parse(l)?.idx() as usize;
        Some((k, i, t))
    })
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let mut m: Table = HashMap::new();
        for (k, i, t) in rows() {
            m.entry(k).or_insert([""; 8])[i] = t;
        }
        m
    })
}

/// Text exact pentru (cheie, limbă), fără rezervă.
pub fn lookup(key: &str, l: Lang) -> Option<&'static str> {
    table()
        .get(key)
        .map(|r| r[l.idx() as usize])
        .filter(|s| !s.trim().is_empty())
}

/// Cu rezervă: limba curentă → en → ro.
pub fn tr_opt(key: &str) -> Option<&'static str> {
    lookup(key, lang())
        .or_else(|| lookup(key, Lang::En))
        .or_else(|| lookup(key, Lang::Ro))
}

/// Textul cheii în limba curentă (rezervă: en → ro → cheia).
pub fn tr(key: &'static str) -> &'static str {
    tr_opt(key).unwrap_or(key)
}

/// Ca `tr`, înlocuind `{0}`, `{1}`… cu `args`.
pub fn trf(key: &'static str, args: &[&str]) -> String {
    let mut s = tr(key).to_string();
    for (i, a) in args.iter().enumerate() {
        s = s.replace(&format!("{{{i}}}"), a);
    }
    s
}

/// Categoria de joc, tradusă pentru afișare (necunoscută → ea însăși).
pub fn category_title(cat: &str) -> String {
    tr_opt(&format!("cat.{cat}")).unwrap_or(cat).to_string()
}

/// Cârlig: titlul jocului în limba curentă (deocamdată doar rezerva).
pub fn game_title<'a>(_id: &str, fallback: &'a str) -> &'a str {
    fallback
}

/// Limba inițială: `--lang CODE` / `--lang=CODE`, apoi `POLYTABLETOP_LANG`, apoi `LANG`/`LC_ALL`, apoi ro.
pub fn detect(args: &[String]) -> Lang {
    let from_args = args.iter().enumerate().find_map(|(i, a)| {
        if a == "--lang" {
            args.get(i + 1).and_then(|v| Lang::parse(v))
        } else {
            a.strip_prefix("--lang=").and_then(Lang::parse)
        }
    });
    let env = |k: &str| std::env::var(k).ok().and_then(|v| Lang::parse(&v));
    from_args
        .or_else(|| env("POLYTABLETOP_LANG"))
        .or_else(|| env("LC_ALL"))
        .or_else(|| env("LANG"))
        .unwrap_or(Lang::Ro)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn keys() -> BTreeSet<&'static str> {
        table().keys().copied().collect()
    }

    #[test]
    fn every_key_has_eight_nonempty_languages() {
        for k in keys() {
            for l in Lang::ALL {
                let t = lookup(k, l).unwrap_or_else(|| panic!("{k}: lipsește {}", l.code()));
                assert!(!t.trim().is_empty(), "{k}/{} gol", l.code());
            }
        }
        assert_eq!(
            rows().count(),
            keys().len() * 8,
            "rânduri duplicate/necunoscute"
        );
    }

    #[test]
    fn placeholders_match_across_languages() {
        for k in keys() {
            let n = |l| {
                let t = lookup(k, l).unwrap();
                (0..5).filter(|i| t.contains(&format!("{{{i}}}"))).count()
            };
            for l in Lang::ALL {
                assert_eq!(n(l), n(Lang::Ro), "{k}/{}", l.code());
            }
        }
    }

    /// Toate cheile folosite în cod (`tr("…")`, `trf("…"`) există în gui.tsv.
    #[test]
    fn code_keys_exist() {
        let files = [
            include_str!("main.rs"),
            include_str!("session.rs"),
            include_str!("journal.rs"),
            include_str!("draw.rs"),
            include_str!("board.rs"),
            include_str!("menu.rs"),
        ];
        let known = keys();
        let mut used = BTreeSet::new();
        for src in files {
            for pat in ["tr(", "trf("] {
                let mut rest = src;
                while let Some(p) = rest.find(pat) {
                    let after = rest[p + pat.len()..].trim_start();
                    // nu număra `str(`, `fn tr(` etc.
                    let ok = !rest[..p].ends_with(|c: char| c.is_alphanumeric() || c == '_');
                    if let (true, Some(q)) = (ok, after.strip_prefix('"')) {
                        if let Some(e) = q.find('"') {
                            used.insert(q[..e].to_string());
                        }
                    }
                    rest = &rest[p + pat.len()..];
                }
            }
        }
        assert!(used.len() > 30, "prea puține chei găsite: {}", used.len());
        for k in &used {
            assert!(known.contains(k.as_str()), "cheie lipsă din gui.tsv: {k}");
        }
        // opusul: nicio cheie moartă (în afara categoriilor, folosite dinamic)
        for k in &known {
            if !k.starts_with("cat.") {
                assert!(used.contains(*k), "cheie nefolosită: {k}");
            }
        }
    }

    #[test]
    fn categories_translated() {
        for c in polytabletop_games::playable().iter().map(|p| p.category) {
            assert!(lookup(&format!("cat.{c}"), Lang::En).is_some(), "{c}");
        }
    }

    #[test]
    fn default_ro_and_fallback() {
        set_lang(Lang::Ro);
        assert_eq!(tr("undo"), "Anulează mutarea");
        assert_eq!(tr("nu_exista"), "nu_exista");
        assert_eq!(trf("wins", &["X"]), "Câștigă X!");
        assert_eq!(Lang::parse("hu_HU.UTF-8"), Some(Lang::Hu));
        assert_eq!(Lang::parse("xx"), None);
        let a = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(detect(&a(&["m", "--lang", "de"])), Lang::De);
        assert_eq!(detect(&a(&["m", "--lang=fr"])), Lang::Fr);
    }
}
