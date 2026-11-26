//! Compunerea mutărilor text din click-uri pe pătrate/puncte.
//!
//! Trait-ul `GameObj` expune doar mutări text (`e2e4`, `c3xe5xg7`, `a1-d1xg7`,
//! `pass`, `4`). Aici descompunem fiecare mutare legală în „pătrate" și
//! lăsăm utilizatorul să le indice prin click, în ordine.

use crate::board::Kind;
use std::collections::BTreeSet;

/// Descompune o mutare în (pătrate, sufix). Un pătrat este o literă urmată de
/// cifre (`e2`, `o15`) sau un număr gol (`4`, coloana de la Patru în linie).
/// `-` și `x` sunt separatori; orice altă literă fără cifre este sufix
/// (`q` din `e7e8q`).
pub fn tokenize(m: &str) -> (Vec<String>, String) {
    if let Some(r) = shogi_tokens(m) {
        return r;
    }
    if let Some(r) = slash_tokens(m) {
        return r;
    }
    let c: Vec<char> = m.trim().chars().collect();
    if !c.is_empty() && c.iter().all(|ch| ch.is_ascii_digit()) {
        return (vec![m.trim().to_string()], String::new());
    }
    let mut sq = Vec::new();
    let mut suf = String::new();
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch.is_ascii_alphabetic() && i + 1 < c.len() && c[i + 1].is_ascii_digit() {
            let mut s = String::new();
            s.push(ch.to_ascii_lowercase());
            i += 1;
            while i < c.len() && c[i].is_ascii_digit() {
                s.push(c[i]);
                i += 1;
            }
            sq.push(s);
            continue;
        }
        if ch == '-' || ch == 'x' || ch == 'X' || ch.is_whitespace() {
            i += 1;
            continue;
        }
        if ch.is_alphabetic() {
            suf.push(ch);
        }
        i += 1;
    }
    (sq, suf)
}

/// Descompunerea unei mutări după jocul căruia îi aparține (`None` = regula generală).
/// * Puncte: toată mutarea (`oa1`, `va1`) este numele laturii;
/// * Rotire: `c3/2d` → pătratul `c3` + butonul `q2d`;
/// * Coridor: `d4h` / `d4v` → slotul `w-d4`, sufixul `h` / `v` (se alege după click);
/// * Drum: plasare `c3` / `sc3` / `cc3` → `c3` (+ prefix); mutare de stivă `3c3>21` →
///   `c3` și pătratul în care ajunge ultima piatră (+ cum se împart pietrele).
pub fn tokenize_k(kind: Option<Kind>, m: &str) -> (Vec<String>, String) {
    let t = m.trim();
    match kind {
        Some(Kind::Puncte) if !t.is_empty() => (vec![t.to_string()], String::new()),
        Some(Kind::Rotire) => {
            if let Some((sq, rest)) = t.split_once('/') {
                if !sq.is_empty() && !rest.is_empty() {
                    return (vec![sq.to_string(), format!("q{rest}")], String::new());
                }
            }
            tokenize(m)
        }
        Some(Kind::Coridor) => {
            let c: Vec<char> = t.chars().collect();
            if c.len() == 3 && matches!(c[2], 'h' | 'v') && c[1].is_ascii_digit() {
                return (vec![format!("w-{}{}", c[0], c[1])], c[2].to_string());
            }
            tokenize(m)
        }
        Some(Kind::Drum) => drum_tokens(t).unwrap_or_else(|| tokenize(m)),
        _ => tokenize(m),
    }
}

fn drum_tokens(t: &str) -> Option<(Vec<String>, String)> {
    let c: Vec<char> = t.chars().collect();
    let mut i = 0;
    let mut suffix = String::new();
    let mut take = 1;
    if c.first()?.is_ascii_digit() {
        take = c[0].to_digit(10)? as i32;
        suffix.push(c[0]);
        i = 1;
    }
    if matches!(c.get(i), Some('s' | 'c')) && c.get(i + 1)?.is_ascii_alphabetic() {
        suffix.push(c[i]);
        i += 1;
    }
    let file = *c.get(i)?;
    let rank = *c.get(i + 1)?;
    if !file.is_ascii_lowercase() || !rank.is_ascii_digit() {
        return None;
    }
    let from = format!("{file}{rank}");
    i += 2;
    let Some(&d) = c.get(i) else {
        return Some((vec![from], suffix));
    };
    let (dx, dy) = match d {
        '+' => (0, 1),
        '-' => (0, -1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => return None,
    };
    let drops: String = c[i + 1..].iter().collect();
    let n = if take > 1 {
        drops.chars().count() as i32
    } else {
        1
    };
    suffix.push_str(&drops);
    let fx = file as i32 - 'a' as i32 + dx * n;
    let fy = rank.to_digit(10)? as i32 + dy * n;
    if !(0..5).contains(&fx) || !(1..=5).contains(&fy) {
        return None;
    }
    let to = format!("{}{}", (b'a' + fx as u8) as char, fy);
    Some((vec![from, to], suffix))
}

/// Shogi: `7g7f`, `7g7f+` (promovare) și `P*5e` (plasare din mână). Pătratele sunt
/// cifră (coloana) + literă (rândul); piesa din mână este numele `P*`.
fn shogi_tokens(m: &str) -> Option<(Vec<String>, String)> {
    let c: Vec<char> = m.trim().chars().collect();
    let sq =
        |i: usize| matches!(c.get(i), Some('1'..='9')) && matches!(c.get(i + 1), Some('a'..='i'));
    let name = |i: usize| format!("{}{}", c[i], c[i + 1]);
    if (c.len() == 4 || (c.len() == 5 && c[4] == '+')) && sq(0) && sq(2) {
        let suf = if c.len() == 5 { "+" } else { "" };
        return Some((vec![name(0), name(2)], suf.to_string()));
    }
    if c.len() == 4 && c[0].is_ascii_uppercase() && c[1] == '*' && sq(2) {
        return Some((vec![format!("{}*", c[0]), name(2)], String::new()));
    }
    None
}

/// Table: `13/7`, `bar/22`, `3/off` → puncte (numere), `bar`, `off`.
fn slash_tokens(m: &str) -> Option<(Vec<String>, String)> {
    let parts: Vec<String> = m
        .trim()
        .split('/')
        .map(|p| p.trim().to_lowercase())
        .collect();
    if parts.len() < 2 {
        return None;
    }
    let ok = |p: &String| {
        p == "bar" || p == "off" || (!p.is_empty() && p.chars().all(|ch| ch.is_ascii_digit()))
    };
    if parts.iter().all(ok) {
        Some((parts, String::new()))
    } else {
        None
    }
}

/// O mutare care nu conține pătrate (ex. `pass`, `imparte`) → buton.
pub fn is_special(m: &str) -> bool {
    is_special_k(None, m)
}

pub fn is_special_k(kind: Option<Kind>, m: &str) -> bool {
    tokenize_k(kind, m).0.is_empty()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Click {
    /// Mutarea este completă și neambiguă: joacă-o.
    Play(String),
    /// Aceleași pătrate, sufixe diferite (promovare): alege.
    Choose(Vec<String>),
    /// Selecție parțială nouă; `exact` = mutare validă care se poate încheia aici.
    Pending {
        sel: Vec<String>,
        exact: Option<String>,
    },
    /// Nimic valid.
    Reset,
}

fn starts_with(sq: &[String], pre: &[String]) -> bool {
    sq.len() >= pre.len() && sq[..pre.len()] == *pre
}

fn eval(kind: Option<Kind>, legal: &[String], new: &[String]) -> Option<Click> {
    let m: Vec<(&String, Vec<String>, String)> = legal
        .iter()
        .map(|t| {
            let (s, f) = tokenize_k(kind, t);
            (t, s, f)
        })
        .filter(|(_, s, _)| !s.is_empty() && starts_with(s, new))
        .collect();
    if m.is_empty() {
        return None;
    }
    let exact: Vec<&String> = m
        .iter()
        .filter(|x| x.1.len() == new.len())
        .map(|x| x.0)
        .collect();
    if m.len() == exact.len() {
        // toate se termină aici
        return Some(if exact.len() == 1 {
            Click::Play(exact[0].clone())
        } else {
            Click::Choose(exact.into_iter().cloned().collect())
        });
    }
    Some(Click::Pending {
        sel: new.to_vec(),
        exact: if exact.len() == 1 {
            Some(exact[0].clone())
        } else {
            None
        },
    })
}

/// Procesează click-ul pe pătratul `sq`, având selecția curentă `sel`.
#[cfg(test)]
pub fn click(legal: &[String], sel: &[String], sq: &str) -> Click {
    click_k(None, legal, sel, sq)
}

pub fn click_k(kind: Option<Kind>, legal: &[String], sel: &[String], sq: &str) -> Click {
    if sel.last().map(|s| s == sq).unwrap_or(false) {
        let mut v = sel.to_vec();
        v.pop();
        return if v.is_empty() {
            Click::Reset
        } else {
            eval(kind, legal, &v).unwrap_or(Click::Reset)
        };
    }
    let mut new = sel.to_vec();
    new.push(sq.to_string());
    if let Some(c) = eval(kind, legal, &new) {
        return c;
    }
    if !sel.is_empty() {
        if let Some(c) = eval(kind, legal, &[sq.to_string()]) {
            return c;
        }
    }
    Click::Reset
}

/// Pătratele care pot fi indicate următoarele (pentru evidențiere).
pub fn next_squares(legal: &[String], sel: &[String]) -> BTreeSet<String> {
    next_squares_k(None, legal, sel)
}

pub fn next_squares_k(kind: Option<Kind>, legal: &[String], sel: &[String]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for t in legal {
        let (s, _) = tokenize_k(kind, t);
        if s.len() > sel.len() && starts_with(&s, sel) {
            out.insert(s[sel.len()].clone());
        }
    }
    out
}

/// Mutările care se încheie exact la selecția `sel`, deși există continuări mai lungi
/// (lanț de capturi la Fanorona, salt simplu înaintea unui salt multiplu): butoanele
/// „Termină aici”. Mai multe când aceleași pătrate au sufixe diferite (abordare/retragere).
#[cfg(test)]
pub fn ends_here(legal: &[String], sel: &[String]) -> Vec<String> {
    ends_here_k(None, legal, sel)
}

pub fn ends_here_k(kind: Option<Kind>, legal: &[String], sel: &[String]) -> Vec<String> {
    if sel.is_empty() {
        return vec![];
    }
    let longer = legal.iter().any(|t| {
        let (s, _) = tokenize_k(kind, t);
        s.len() > sel.len() && starts_with(&s, sel)
    });
    if !longer {
        return vec![];
    }
    legal
        .iter()
        .filter(|t| tokenize_k(kind, t).0 == sel)
        .cloned()
        .collect()
}

/// Împarte mutările în (cele indicabile prin click, cele „speciale" → butoane).
#[cfg(test)]
pub fn split_special(legal: &[String]) -> (Vec<String>, Vec<String>) {
    split_special_k(None, legal)
}

pub fn split_special_k(kind: Option<Kind>, legal: &[String]) -> (Vec<String>, Vec<String>) {
    legal.iter().cloned().partition(|m| !is_special_k(kind, m))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn tokens() {
        assert_eq!(tokenize("e2e4"), (v(&["e2", "e4"]), String::new()));
        assert_eq!(tokenize("e7e8q"), (v(&["e7", "e8"]), "q".into()));
        assert_eq!(
            tokenize("c3xe5xg7"),
            (v(&["c3", "e5", "g7"]), String::new())
        );
        assert_eq!(
            tokenize("a1-d1xg7"),
            (v(&["a1", "d1", "g7"]), String::new())
        );
        assert_eq!(tokenize("o15"), (v(&["o15"]), String::new()));
        assert_eq!(tokenize("4"), (v(&["4"]), String::new()));
        assert_eq!(tokenize("b2b9"), (v(&["b2", "b9"]), String::new()));
        assert_eq!(tokenize("7g7f"), (v(&["7g", "7f"]), String::new()));
        assert_eq!(tokenize("8h2b+"), (v(&["8h", "2b"]), "+".into()));
        assert_eq!(tokenize("P*5e"), (v(&["P*", "5e"]), String::new()));
        assert_eq!(tokenize("13/7"), (v(&["13", "7"]), String::new()));
        assert_eq!(tokenize("bar/22"), (v(&["bar", "22"]), String::new()));
        assert_eq!(tokenize("3/off"), (v(&["3", "off"]), String::new()));
        assert!(is_special("intră"));
        assert!(!is_special("12"));
        assert!(is_special("pass"));
        assert!(is_special("imparte"));
        assert!(!is_special("d3"));
    }

    #[test]
    fn shogi_promotion_and_drop() {
        let l = v(&["2b3a+", "2b3a", "P*5e", "L*5e"]);
        match click(&l, &v(&["2b"]), "3a") {
            Click::Choose(c) => assert_eq!(c.len(), 2),
            x => panic!("{x:?}"),
        }
        assert_eq!(click(&l, &v(&["P*"]), "5e"), Click::Play("P*5e".into()));
        assert_eq!(next_squares(&l, &[]).len(), 3);
    }

    #[test]
    fn backgammon_clicks() {
        let l = v(&["13/7", "13/9", "bar/22", "3/off"]);
        assert_eq!(click(&l, &v(&["13"]), "7"), Click::Play("13/7".into()));
        assert_eq!(click(&l, &v(&["bar"]), "22"), Click::Play("bar/22".into()));
        assert_eq!(click(&l, &v(&["3"]), "off"), Click::Play("3/off".into()));
    }

    #[test]
    fn new_game_tokens() {
        let k = |kd, m| tokenize_k(Some(kd), m);
        assert_eq!(k(Kind::Puncte, "oa1"), (v(&["oa1"]), String::new()));
        assert_eq!(k(Kind::Rotire, "c3/2d"), (v(&["c3", "q2d"]), String::new()));
        assert_eq!(k(Kind::Coridor, "d4h"), (v(&["w-d4"]), "h".into()));
        assert_eq!(k(Kind::Coridor, "e2"), (v(&["e2"]), String::new()));
        assert_eq!(k(Kind::Drum, "c3"), (v(&["c3"]), String::new()));
        assert_eq!(k(Kind::Drum, "sc3"), (v(&["c3"]), "s".into()));
        assert_eq!(k(Kind::Drum, "cc3"), (v(&["c3"]), "c".into()));
        assert_eq!(k(Kind::Drum, "c3>"), (v(&["c3", "d3"]), String::new()));
        assert_eq!(k(Kind::Drum, "3c3>21"), (v(&["c3", "e3"]), "321".into()));
        assert_eq!(k(Kind::Drum, "2b2+11"), (v(&["b2", "b4"]), "211".into()));
    }

    #[test]
    fn simple_two_clicks() {
        let l = v(&["e2e3", "e2e4", "d2d4"]);
        let c = click(&l, &[], "e2");
        assert_eq!(
            c,
            Click::Pending {
                sel: v(&["e2"]),
                exact: None
            }
        );
        assert_eq!(click(&l, &v(&["e2"]), "e4"), Click::Play("e2e4".into()));
        assert_eq!(click(&l, &v(&["e2"]), "h5"), Click::Reset);
        // schimbarea piesei selectate
        assert_eq!(
            click(&l, &v(&["e2"]), "d2"),
            Click::Pending {
                sel: v(&["d2"]),
                exact: None
            }
        );
        // deselectare
        assert_eq!(click(&l, &v(&["e2"]), "e2"), Click::Reset);
    }

    #[test]
    fn promotion_choose() {
        let l = v(&["e7e8q", "e7e8r", "e7e8b", "e7e8n", "a2a3"]);
        match click(&l, &v(&["e7"]), "e8") {
            Click::Choose(c) => assert_eq!(c.len(), 4),
            x => panic!("{x:?}"),
        }
    }

    #[test]
    fn multi_jump_with_exact_end() {
        let l = v(&["c3xe5", "c3xe5xg7"]);
        let c = click(&l, &v(&["c3"]), "e5");
        assert_eq!(
            c,
            Click::Pending {
                sel: v(&["c3", "e5"]),
                exact: Some("c3xe5".into())
            }
        );
        assert_eq!(
            click(&l, &v(&["c3", "e5"]), "g7"),
            Click::Play("c3xe5xg7".into())
        );
    }

    #[test]
    fn morris_place_and_take() {
        let l = v(&["a1xd7", "a1xg7", "d1"]);
        assert_eq!(click(&l, &[], "d1"), Click::Play("d1".into()));
        let c = click(&l, &[], "a1");
        assert!(matches!(c, Click::Pending { .. }));
        assert_eq!(click(&l, &v(&["a1"]), "g7"), Click::Play("a1xg7".into()));
    }

    #[test]
    fn single_click_and_special() {
        let l = v(&["4", "3", "pass"]);
        assert_eq!(click(&l, &[], "3"), Click::Play("3".into()));
        let (g, s) = split_special(&l);
        assert_eq!(g.len(), 2);
        assert_eq!(s, v(&["pass"]));
    }

    #[test]
    fn ends_here_ambiguous_suffix() {
        let l = v(&["a1-b2a", "a1-b2w", "a1-b2a-c3", "d1-d2"]);
        assert_eq!(ends_here(&l, &v(&["a1", "b2"])), v(&["a1-b2a", "a1-b2w"]));
        assert!(ends_here(&l, &v(&["d1", "d2"])).is_empty());
        assert!(ends_here(&l, &[]).is_empty());
    }

    #[test]
    fn amazons_three_clicks() {
        let l = v(&["d1-d5/d8", "d1-d5/d1", "d1-d6/d8"]);
        assert!(matches!(
            click(&l, &v(&["d1"]), "d5"),
            Click::Pending { exact: None, .. }
        ));
        assert_eq!(
            click(&l, &v(&["d1", "d5"]), "d8"),
            Click::Play("d1-d5/d8".into())
        );
    }

    #[test]
    fn next_sq() {
        let l = v(&["e2e3", "e2e4", "d2d4"]);
        assert_eq!(next_squares(&l, &[]).len(), 2);
        assert_eq!(
            next_squares(&l, &v(&["e2"])),
            ["e3", "e4"].iter().map(|s| s.to_string()).collect()
        );
    }
}
