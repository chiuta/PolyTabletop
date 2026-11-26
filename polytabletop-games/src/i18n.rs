//! Localised game titles and rules.
//!
//! Data: `data/i18n/titles_*.tsv`, lines `id<TAB>lang<TAB>title<TAB>rules`
//! (`\n`, `\t`, `\\` escapes; `#` comments). Later files override earlier ones.
//! Missing entries fall back to the Romanian `Playable.title` / `Playable.rules`.

use polytabletop_core::i18n::{unescape, Lang};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Embedded files, in override order (later files override earlier ones).
const FILES: [&str; 20] = [
    include_str!("../data/i18n/titles_00.tsv"),
    include_str!("../data/i18n/titles_01.tsv"),
    include_str!("../data/i18n/titles_02.tsv"),
    include_str!("../data/i18n/titles_03.tsv"),
    include_str!("../data/i18n/titles_P1.tsv"),
    include_str!("../data/i18n/titles_P2.tsv"),
    include_str!("../data/i18n/titles_P3.tsv"),
    include_str!("../data/i18n/titles_P4.tsv"),
    include_str!("../data/i18n/titles_Q1.tsv"),
    include_str!("../data/i18n/titles_Q2.tsv"),
    include_str!("../data/i18n/titles_Q3.tsv"),
    include_str!("../data/i18n/titles_Q4.tsv"),
    include_str!("../data/i18n/titles_Q5.tsv"),
    include_str!("../data/i18n/titles_Q6.tsv"),
    include_str!("../data/i18n/titles_B2.tsv"),
    include_str!("../data/i18n/titles_B1.tsv"),
    include_str!("../data/i18n/titles_B6.tsv"),
    include_str!("../data/i18n/titles_B3.tsv"),
    include_str!("../data/i18n/titles_B5.tsv"),
    include_str!("../data/i18n/titles_B4.tsv"),
];

type Entry = (&'static str, &'static str);
type Table = HashMap<Lang, HashMap<&'static str, Entry>>;

fn leak(s: &str) -> &'static str {
    Box::leak(unescape(s).into_boxed_str())
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let mut m: Table = HashMap::new();
        for f in FILES {
            for line in f.lines() {
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let mut p = line.splitn(4, '\t');
                let (Some(id), Some(l), Some(t)) = (p.next(), p.next(), p.next()) else {
                    continue;
                };
                let r = p.next().unwrap_or("");
                if let Some(l) = Lang::from_code(l) {
                    // ids are 'static only through the leak.
                    m.entry(l).or_default().insert(leak(id), (leak(t), leak(r)));
                }
            }
        }
        m
    })
}

/// Raw lookup of an id in a language (no fallback), as `(title, rules)`; either may be empty.
pub fn entry(id: &str, lang: Lang) -> Option<(&'static str, &'static str)> {
    table().get(&lang)?.get(id).copied()
}

/// Title of game `id` in `lang`; Romanian `Playable.title` if no translation. `None` for unknown ids.
pub fn title(id: &str, lang: Lang) -> Option<&'static str> {
    let ro = crate::find(id)?;
    Some(
        entry(id, lang)
            .map(|e| e.0)
            .filter(|t| !t.is_empty())
            .unwrap_or(ro.title),
    )
}

/// Rules of game `id` in `lang`; Romanian `Playable.rules` if no translation.
pub fn rules(id: &str, lang: Lang) -> Option<&'static str> {
    let ro = crate::find(id)?;
    Some(
        entry(id, lang)
            .map(|e| e.1)
            .filter(|t| !t.is_empty())
            .unwrap_or(ro.rules),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chess_sample_all_languages() {
        let want = [
            "Chess", "Șah", "Échecs", "Scacchi", "Ajedrez", "Xadrez", "Schach", "Sakk",
        ];
        for (l, w) in Lang::ALL.into_iter().zip(want) {
            assert_eq!(title("chess", l), Some(w));
            assert!(rules("chess", l).unwrap().contains("e2e4"));
        }
    }

    #[test]
    fn fallback_to_romanian() {
        let ro = crate::find("chess").unwrap();
        assert_eq!(rules("chess", Lang::Ro), Some(ro.rules));
        assert_eq!(title("no-such-game", Lang::De), None);
    }

    #[test]
    fn every_playable_game_is_translated_in_all_languages() {
        for p in crate::playable() {
            for l in Lang::ALL {
                let t = title(p.id, l).unwrap_or_else(|| panic!("no title {} {:?}", p.id, l));
                let r = rules(p.id, l).unwrap_or_else(|| panic!("no rules {} {:?}", p.id, l));
                assert!(
                    !t.trim().is_empty() && !r.trim().is_empty(),
                    "{} {:?}",
                    p.id,
                    l
                );
                if l == Lang::Ro {
                    assert_eq!(t, p.title, "{}", p.id);
                    assert_eq!(r, p.rules, "{}", p.id);
                }
            }
        }
    }
    #[test]
    fn move_tokens_are_kept_in_every_language() {
        let need: &[(&str, &[&str])] = &[
            ("pachisi", &["intră", "= 25", "= 10", "6, 10"]),
            (
                "roata",
                &[
                    "avansez facultate",
                    "avansez ucenicie",
                    "avansez afacere",
                    "avansez salariu",
                    "avansez cumpar",
                ],
            ),
            ("reverdire", &["generatie", "compra N", "pune c3"]),
            ("fanorona", &["c3-d3a-d4w-e5a", "e2-e3"]),
            (
                "tcg_cerneala",
                &[
                    "pesca",
                    "cerneala N",
                    "joaca N",
                    "misiune P",
                    "provocare P e Q",
                    "fin",
                ],
            ),
            (
                "tinuturi",
                &["24", "48", "pune e6-e7", "alege N", "descarta"],
            ),
            (
                "portret",
                &[
                    "intreb ochelari",
                    "raspund da/nu",
                    "ghicesc Matei",
                    "verdict corect / verdict gresit",
                ],
            ),
        ];
        for (id, toks) in need {
            for l in Lang::ALL {
                let r = rules(id, l).unwrap();
                for t in *toks {
                    // `raspund da/nu` apare în toate limbile ca atare; restul la fel.
                    assert!(r.contains(t), "{id} {:?}: lipsește «{t}» în: {r}", l);
                }
            }
        }
    }
}
