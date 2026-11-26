//! Session journal: an append-only log with a line-based text format.

use crate::text::{esc, fields};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntry {
    pub seq: u32,
    pub kind: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Journal {
    entries: Vec<LogEntry>,
}

impl Journal {
    pub fn new() -> Journal {
        Journal::default()
    }
    pub fn add(&mut self, kind: &str, text: &str) -> u32 {
        let seq = self.entries.last().map_or(1, |e| e.seq + 1);
        let kind = kind.trim();
        self.entries.push(LogEntry {
            seq,
            kind: if kind.is_empty() {
                "note".into()
            } else {
                kind.to_string()
            },
            text: text.to_string(),
        });
        seq
    }
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn of_kind(&self, kind: &str) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.kind.to_lowercase() == kind.to_lowercase())
            .collect()
    }
    pub fn search(&self, needle: &str) -> Vec<&LogEntry> {
        let n = needle.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.text.to_lowercase().contains(&n))
            .collect()
    }
    pub fn last(&self, n: usize) -> &[LogEntry] {
        &self.entries[self.entries.len().saturating_sub(n)..]
    }
    pub fn to_text(&self) -> String {
        self.entries
            .iter()
            .map(|e| format!("{} | {} | {}\n", e.seq, esc(&e.kind), esc(&e.text)))
            .collect()
    }
    /// Markdown export: summary table, chronological timeline and (optionally)
    /// the character sheet. Multi-line entries are indented as continuation text.
    pub fn to_markdown(&self, title: &str, sheet: Option<&crate::sheet::Sheet>) -> String {
        let mut o = format!("# {}\n\n", md_inline(title));
        o += &format!("_{} intrări în jurnal_\n\n", self.entries.len());
        let mut kinds: Vec<(String, usize)> = vec![];
        for e in &self.entries {
            let k = e.kind.to_lowercase();
            match kinds.iter_mut().find(|(n, _)| *n == k) {
                Some((_, c)) => *c += 1,
                None => kinds.push((k, 1)),
            }
        }
        if !kinds.is_empty() {
            o += "| tip | intrări |\n|---|---|\n";
            for (k, c) in &kinds {
                o += &format!("| {} | {c} |\n", md_inline(k));
            }
            o.push('\n');
        }
        o += "## Cronologie\n\n";
        if self.entries.is_empty() {
            o += "_(gol)_\n";
        }
        for e in &self.entries {
            let mut lines = e.text.lines();
            o += &format!(
                "{}. **{}** — {}\n",
                e.seq,
                md_inline(&e.kind),
                md_inline(lines.next().unwrap_or(""))
            );
            for l in lines {
                o += &format!("   {}\n", md_inline(l));
            }
        }
        if let Some(sh) = sheet {
            o += "\n## Fișa personajului\n\n```\n";
            o += &sh.render().replace("```", "'''");
            o += "\n```\n";
        }
        o
    }
    pub fn from_text(s: &str) -> Result<Journal, String> {
        let mut j = Journal::new();
        for (i, line) in s.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let f = fields(line);
            if f.len() != 3 {
                return Err(format!("linia {}: se așteaptă nr | tip | text", i + 1));
            }
            let seq = f[0]
                .parse::<u32>()
                .map_err(|_| format!("linia {}: număr invalid", i + 1))?;
            if j.entries.last().is_some_and(|e| e.seq >= seq) {
                return Err(format!("linia {}: numere neordonate", i + 1));
            }
            j.entries.push(LogEntry {
                seq,
                kind: f[1].clone(),
                text: f[2].clone(),
            });
        }
        Ok(j)
    }
}

/// Escapes characters that would change Markdown structure inline.
fn md_inline(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(
            c,
            '\\' | '*' | '_' | '`' | '|' | '<' | '>' | '[' | ']' | '#'
        ) {
            o.push('\\');
        }
        o.push(c);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_query_roundtrip() {
        let mut j = Journal::new();
        assert_eq!(j.add("roll", "2d6 = 7"), 1);
        assert_eq!(j.add("", "Party | enters\ncave"), 2);
        j.add("combat", "Goblin falls");
        assert_eq!(j.len(), 3);
        assert_eq!(j.entries()[1].kind, "note");
        assert_eq!(j.of_kind("ROLL").len(), 1);
        assert_eq!(j.search("goblin").len(), 1);
        assert_eq!(j.last(2).len(), 2);
        assert_eq!(j.last(10).len(), 3);
        let j2 = Journal::from_text(&j.to_text()).unwrap();
        assert_eq!(j, j2);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(Journal::from_text("x | a | b").is_err());
        assert!(Journal::from_text("1 | a").is_err());
        assert!(Journal::from_text("2 | a | b\n1 | a | b").is_err());
        assert!(Journal::from_text("").unwrap().is_empty());
    }
}
