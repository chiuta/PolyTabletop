//! Tiny helpers shared by the line-based text formats.

/// Escape `\`, newline, `|` and `=`-free separators so a field survives a round trip.
pub fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => {}
            '|' => o.push_str("\\p"),
            _ => o.push(c),
        }
    }
    o
}

pub fn unesc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() {
                Some('n') => o.push('\n'),
                Some('p') => o.push('|'),
                Some('\\') => o.push('\\'),
                Some(x) => {
                    o.push('\\');
                    o.push(x)
                }
                None => o.push('\\'),
            }
        } else {
            o.push(c);
        }
    }
    o
}

/// Split on unescaped `|` (the escape already removed raw pipes), trimming fields.
pub fn fields(s: &str) -> Vec<String> {
    s.split('|').map(|f| unesc(f.trim())).collect()
}

pub fn same(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let s = "a|b\\c\nd";
        assert_eq!(unesc(&esc(s)), s);
        assert_eq!(
            fields(&format!("{} | x", esc(s))),
            vec![s.to_string(), "x".into()]
        );
    }
}
