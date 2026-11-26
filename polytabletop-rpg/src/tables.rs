//! Random tables with dice-range rows and nested references.
//!
//! ```text
//! table: Encounter
//! dice: 1d6
//! 1-2: Goblins {Loot}
//! 3: A strange fog
//! 4-6: Nothing
//! ```
//!
//! `{Name}` inside a row rolls on another table of the same set.

use crate::dice::Expr;
use polytabletop_core::rng::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub lo: i64,
    pub hi: i64,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct RandomTable {
    pub name: String,
    pub dice: Expr,
    pub rows: Vec<Row>,
}

impl RandomTable {
    /// Build a table from weights; rows cover `1..=sum` and the die is `1d{sum}`.
    pub fn weighted(name: &str, items: &[(u32, &str)]) -> Result<RandomTable, String> {
        let mut rows = Vec::new();
        let mut at = 1i64;
        for (w, t) in items {
            if *w == 0 {
                return Err("pondere 0".into());
            }
            rows.push(Row {
                lo: at,
                hi: at + *w as i64 - 1,
                text: t.to_string(),
            });
            at += *w as i64;
        }
        if rows.is_empty() {
            return Err("tabel gol".into());
        }
        let dice = Expr::parse(&format!("1d{}", at - 1)).map_err(|e| e.to_string())?;
        RandomTable::new(name, dice, rows)
    }

    pub fn new(name: &str, dice: Expr, rows: Vec<Row>) -> Result<RandomTable, String> {
        let t = RandomTable {
            name: name.to_string(),
            dice,
            rows,
        };
        t.validate()?;
        Ok(t)
    }

    fn validate(&self) -> Result<(), String> {
        if self.rows.is_empty() {
            return Err(format!("{}: tabel fără rânduri", self.name));
        }
        let mut at = self.dice.min();
        for r in &self.rows {
            if r.lo != at {
                return Err(format!(
                    "{}: rândul {}-{} nu continuă de la {at} (goluri sau suprapuneri)",
                    self.name, r.lo, r.hi
                ));
            }
            if r.hi < r.lo {
                return Err(format!(
                    "{}: interval inversat {}-{}",
                    self.name, r.lo, r.hi
                ));
            }
            at = r.hi + 1;
        }
        if at - 1 != self.dice.max() {
            return Err(format!(
                "{}: rândurile acoperă până la {} dar zarul dă până la {}",
                self.name,
                at - 1,
                self.dice.max()
            ));
        }
        Ok(())
    }

    pub fn lookup(&self, n: i64) -> Option<&Row> {
        self.rows.iter().find(|r| (r.lo..=r.hi).contains(&n))
    }

    /// Roll once; returns the die result and the row text.
    pub fn roll(&self, rng: &mut Rng) -> (i64, &str) {
        let n = self.dice.roll(rng).total;
        let row = self.lookup(n).expect("validated table covers every roll");
        (n, &row.text)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TableSet {
    pub tables: Vec<RandomTable>,
}

const MAX_DEPTH: u32 = 8;

impl TableSet {
    pub fn get(&self, name: &str) -> Option<&RandomTable> {
        self.tables
            .iter()
            .find(|t| t.name.to_lowercase() == name.trim().to_lowercase())
    }

    pub fn from_text(s: &str) -> Result<TableSet, String> {
        struct Cur {
            name: String,
            dice: Option<Expr>,
            rows: Vec<Row>,
        }
        fn finish(c: Cur, out: &mut Vec<RandomTable>) -> Result<(), String> {
            let dice = match c.dice {
                Some(d) => d,
                None => {
                    let max = c.rows.last().map_or(0, |r| r.hi);
                    Expr::parse(&format!("1d{max}")).map_err(|e| format!("{}: {e}", c.name))?
                }
            };
            if out
                .iter()
                .any(|t| t.name.to_lowercase() == c.name.to_lowercase())
            {
                return Err(format!("tabel duplicat: {}", c.name));
            }
            out.push(RandomTable::new(&c.name, dice, c.rows)?);
            Ok(())
        }
        let mut out = Vec::new();
        let mut cur: Option<Cur> = None;
        for (ln, line) in s.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (k, v) = line
                .split_once(':')
                .ok_or_else(|| format!("linia {}: lipsește ':'", ln + 1))?;
            let (k, v) = (k.trim(), v.trim());
            if k == "table" {
                if let Some(c) = cur.take() {
                    finish(c, &mut out)?;
                }
                if v.is_empty() {
                    return Err(format!("linia {}: tabel fără nume", ln + 1));
                }
                cur = Some(Cur {
                    name: v.to_string(),
                    dice: None,
                    rows: vec![],
                });
                continue;
            }
            let c = cur
                .as_mut()
                .ok_or_else(|| format!("linia {}: înainte de 'table:'", ln + 1))?;
            if k == "dice" {
                c.dice = Some(Expr::parse(v).map_err(|e| format!("linia {}: {e}", ln + 1))?);
                continue;
            }
            let (lo, hi) = match k.split_once('-') {
                Some((a, b)) if !a.trim().is_empty() => {
                    (a.trim().parse::<i64>(), b.trim().parse::<i64>())
                }
                _ => (k.parse::<i64>(), k.parse::<i64>()),
            };
            match (lo, hi) {
                (Ok(lo), Ok(hi)) => c.rows.push(Row {
                    lo,
                    hi,
                    text: v.to_string(),
                }),
                _ => return Err(format!("linia {}: interval invalid '{k}'", ln + 1)),
            }
        }
        if let Some(c) = cur.take() {
            finish(c, &mut out)?;
        }
        if out.is_empty() {
            return Err("niciun tabel".into());
        }
        let set = TableSet { tables: out };
        set.check_refs()?;
        Ok(set)
    }

    fn check_refs(&self) -> Result<(), String> {
        for t in &self.tables {
            for r in &t.rows {
                for name in refs(&r.text) {
                    if self.get(&name).is_none() {
                        return Err(format!(
                            "{}: referință la tabel inexistent {{{name}}}",
                            t.name
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Roll a table, expanding `{Other}` references (depth-limited).
    pub fn roll(&self, name: &str, rng: &mut Rng) -> Result<String, String> {
        self.roll_depth(name, rng, 0)
    }

    fn roll_depth(&self, name: &str, rng: &mut Rng, depth: u32) -> Result<String, String> {
        if depth > MAX_DEPTH {
            return Err("referințe circulare între tabele".into());
        }
        let t = self
            .get(name)
            .ok_or_else(|| format!("tabel necunoscut: {name}"))?;
        let (_, text) = t.roll(rng);
        let text = text.to_string();
        let mut out = String::new();
        let mut rest = text.as_str();
        while let Some(a) = rest.find('{') {
            let Some(b) = rest[a..].find('}') else { break };
            out += &rest[..a];
            let inner = &rest[a + 1..a + b];
            out += &self.roll_depth(inner, rng, depth + 1)?;
            rest = &rest[a + b + 1..];
        }
        out += rest;
        Ok(out)
    }
}

fn refs(text: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut rest = text;
    while let Some(a) = rest.find('{') {
        let Some(b) = rest[a..].find('}') else { break };
        v.push(rest[a + 1..a + b].to_string());
        rest = &rest[a + b + 1..];
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "\
# demo
table: Encounter
dice: 1d6
1-2: Goblins carrying {Loot}
3: Fog
4-6: Nothing

table: Loot
1: a coin
2: a gem
";

    #[test]
    fn parse_and_roll() {
        let set = TableSet::from_text(SRC).unwrap();
        assert_eq!(set.tables.len(), 2);
        assert_eq!(set.get("loot").unwrap().dice.max(), 2);
        let mut rng = Rng::new(1);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..300 {
            let s = set.roll("Encounter", &mut rng).unwrap();
            assert!(!s.contains('{'));
            seen.insert(s);
        }
        assert!(seen.contains("Fog") && seen.contains("Nothing"));
        assert!(
            seen.contains("Goblins carrying a coin") && seen.contains("Goblins carrying a gem")
        );
    }

    #[test]
    fn frequencies_follow_ranges() {
        let set = TableSet::from_text(SRC).unwrap();
        let mut rng = Rng::new(2);
        let mut nothing = 0;
        for _ in 0..6000 {
            if set.roll("Encounter", &mut rng).unwrap() == "Nothing" {
                nothing += 1;
            }
        }
        assert!((2700..3300).contains(&nothing), "{nothing}");
    }

    #[test]
    fn validation_errors() {
        // gap
        assert!(TableSet::from_text("table: T\ndice: 1d6\n1-2: a\n4-6: b").is_err());
        // overlap
        assert!(TableSet::from_text("table: T\ndice: 1d6\n1-3: a\n3-6: b").is_err());
        // not reaching max
        assert!(TableSet::from_text("table: T\ndice: 1d6\n1-5: a").is_err());
        // exceeding
        assert!(TableSet::from_text("table: T\ndice: 1d4\n1-6: a").is_err());
        // unknown ref, duplicate, junk
        assert!(TableSet::from_text("table: T\n1: {X}").is_err());
        assert!(TableSet::from_text("table: T\n1: a\ntable: t\n1: b").is_err());
        assert!(TableSet::from_text("1: a").is_err());
        assert!(TableSet::from_text("table: T\nx: a").is_err());
        assert!(TableSet::from_text("").is_err());
    }

    #[test]
    fn two_dice_table_offset() {
        let s = "table: T\ndice: 2d6\n2-6: low\n7: mid\n8-12: high";
        let set = TableSet::from_text(s).unwrap();
        assert!(set.roll("T", &mut Rng::new(1)).is_ok());
    }

    #[test]
    fn circular_refs_detected() {
        let s = "table: A\n1: {B}\ntable: B\n1: {A}";
        let set = TableSet::from_text(s).unwrap();
        assert!(set.roll("A", &mut Rng::new(1)).is_err());
        assert!(set.roll("Z", &mut Rng::new(1)).is_err());
    }

    #[test]
    fn weighted_builder() {
        let t = RandomTable::weighted("W", &[(1, "rare"), (9, "common")]).unwrap();
        assert_eq!(t.dice.max(), 10);
        assert_eq!(t.lookup(1).unwrap().text, "rare");
        assert_eq!(t.lookup(10).unwrap().text, "common");
        assert!(RandomTable::weighted("W", &[]).is_err());
        assert!(RandomTable::weighted("W", &[(0, "x")]).is_err());
    }
}
