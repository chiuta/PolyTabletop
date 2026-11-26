//! Minimal TOML-like data files (no external crate) and typed bestiary/items.
//!
//! Supported: `# comments`, `[section.name]` headers, `key = value` with
//! integers, floats, booleans, `"strings"` (escapes `\" \\ \n \t`) and
//! single-line arrays of those. Not supported: nested tables in values,
//! multi-line strings/arrays, dates.

use crate::dice::Expr;
use crate::sheet::Sheet;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    pub fn as_list(&self) -> Option<&[Value]> {
        if let Value::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    pub name: String,
    pub entries: Vec<(String, Value)>,
}

impl Section {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    pub sections: Vec<Section>,
}

fn strip_comment(line: &str) -> &str {
    let mut in_str = false;
    let mut esc = false;
    for (i, c) in line.char_indices() {
        if in_str {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
        } else if c == '#' {
            return &line[..i];
        }
    }
    line
}

fn parse_string(s: &str) -> Result<(String, &str), String> {
    // s starts after the opening quote
    let mut out = String::new();
    let mut it = s.char_indices();
    while let Some((i, c)) = it.next() {
        match c {
            '"' => return Ok((out, &s[i + 1..])),
            '\\' => match it.next() {
                Some((_, 'n')) => out.push('\n'),
                Some((_, 't')) => out.push('\t'),
                Some((_, '"')) => out.push('"'),
                Some((_, '\\')) => out.push('\\'),
                _ => return Err("escape necunoscut în șir".into()),
            },
            _ => out.push(c),
        }
    }
    Err("șir neînchis".into())
}

fn parse_scalar(s: &str) -> Result<Value, String> {
    let s = s.trim();
    if let Some(r) = s.strip_prefix('"') {
        let (v, rest) = parse_string(r)?;
        if !rest.trim().is_empty() {
            return Err("text după șir".into());
        }
        return Ok(Value::Str(v));
    }
    match s {
        "true" => return Ok(Value::Bool(true)),
        "false" => return Ok(Value::Bool(false)),
        _ => {}
    }
    let clean: String = s.chars().filter(|&c| c != '_').collect();
    if let Ok(i) = clean.parse::<i64>() {
        return Ok(Value::Int(i));
    }
    if clean.contains(|c: char| c.is_ascii_digit()) {
        if let Ok(f) = clean.parse::<f64>() {
            if f.is_finite() {
                return Ok(Value::Float(f));
            }
        }
    }
    Err(format!("valoare invalidă: {s}"))
}

fn parse_value(s: &str) -> Result<Value, String> {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix('[') {
        let inner = inner
            .trim_end()
            .strip_suffix(']')
            .ok_or("listă neînchisă (pe o singură linie)")?;
        let mut items = Vec::new();
        let mut rest = inner.trim();
        while !rest.is_empty() {
            let (v, r) = if let Some(q) = rest.strip_prefix('"') {
                let (v, r) = parse_string(q)?;
                (Value::Str(v), r)
            } else {
                let end = rest.find(',').unwrap_or(rest.len());
                (parse_scalar(&rest[..end])?, &rest[end..])
            };
            items.push(v);
            let r = r.trim_start();
            rest = match r.strip_prefix(',') {
                Some(x) => x.trim_start(),
                None if r.is_empty() => r,
                None => return Err("se aștepta ',' în listă".into()),
            };
        }
        return Ok(Value::List(items));
    }
    parse_scalar(s)
}

impl Document {
    pub fn parse(src: &str) -> Result<Document, String> {
        let mut doc = Document::default();
        for (ln, raw) in src.lines().enumerate() {
            let line = strip_comment(raw).trim();
            if line.is_empty() {
                continue;
            }
            let at = |m: String| format!("linia {}: {m}", ln + 1);
            if let Some(h) = line.strip_prefix('[') {
                let name = h
                    .strip_suffix(']')
                    .ok_or_else(|| at("antet neînchis".into()))?
                    .trim();
                if name.is_empty() || name.contains(char::is_whitespace) {
                    return Err(at("nume de secțiune invalid".into()));
                }
                if doc.sections.iter().any(|s| s.name == name) {
                    return Err(at(format!("secțiune duplicată [{name}]")));
                }
                doc.sections.push(Section {
                    name: name.to_string(),
                    entries: vec![],
                });
                continue;
            }
            let (k, v) = line
                .split_once('=')
                .ok_or_else(|| at("lipsește '='".into()))?;
            let k = k.trim();
            if k.is_empty() || k.contains(char::is_whitespace) {
                return Err(at("cheie invalidă".into()));
            }
            let val = parse_value(v).map_err(&at)?;
            if doc.sections.is_empty() {
                doc.sections.push(Section::default());
            }
            let sec = doc.sections.last_mut().unwrap();
            if sec.get(k).is_some() {
                return Err(at(format!("cheie duplicată '{k}'")));
            }
            sec.entries.push((k.to_string(), val));
        }
        Ok(doc)
    }

    pub fn section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == name)
    }

    /// Sections named `prefix.<id>`, with the id.
    pub fn with_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = (&'a str, &'a Section)> {
        self.sections.iter().filter_map(move |s| {
            s.name
                .strip_prefix(prefix)
                .and_then(|r| r.strip_prefix('.'))
                .map(|id| (id, s))
        })
    }

    pub fn to_text(&self) -> String {
        fn val(v: &Value) -> String {
            match v {
                Value::Int(i) => i.to_string(),
                Value::Float(f) => {
                    let s = f.to_string();
                    if s.contains('.') || s.contains('e') {
                        s
                    } else {
                        format!("{s}.0")
                    }
                }
                Value::Bool(b) => b.to_string(),
                Value::Str(s) => {
                    let mut o = String::from("\"");
                    for c in s.chars() {
                        match c {
                            '"' => o.push_str("\\\""),
                            '\\' => o.push_str("\\\\"),
                            '\n' => o.push_str("\\n"),
                            '\t' => o.push_str("\\t"),
                            c => o.push(c),
                        }
                    }
                    o.push('"');
                    o
                }
                Value::List(l) => format!("[{}]", l.iter().map(val).collect::<Vec<_>>().join(", ")),
            }
        }
        let mut o = String::new();
        for s in &self.sections {
            if !s.name.is_empty() {
                o += &format!("[{}]\n", s.name);
            }
            for (k, v) in &s.entries {
                o += &format!("{k} = {}\n", val(v));
            }
            o.push('\n');
        }
        o
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Creature {
    pub id: String,
    pub name: String,
    pub hp: i32,
    pub defense: i32,
    pub attack: i32,
    pub damage: String,
    pub tags: Vec<String>,
    pub desc: String,
}

impl Creature {
    pub fn to_sheet(&self) -> Sheet {
        let mut s = Sheet::new(&self.name);
        s.set_attr("Defense", self.defense);
        s.set_attr("Attack", self.attack);
        s.set_resource("HP", self.hp, self.hp);
        if !self.desc.is_empty() {
            s.notes.push(self.desc.clone());
        }
        s
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub weight: f64,
    pub value: i64,
    pub tags: Vec<String>,
    pub desc: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Compendium {
    pub creatures: Vec<Creature>,
    pub items: Vec<ItemDef>,
}

fn tags(s: &Section) -> Result<Vec<String>, String> {
    match s.get("tags") {
        None => Ok(vec![]),
        Some(Value::List(l)) => l
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("[{}] tags: doar șiruri", s.name))
            })
            .collect(),
        Some(_) => Err(format!("[{}] tags: se așteaptă listă", s.name)),
    }
}

fn req_int(s: &Section, k: &str) -> Result<i64, String> {
    s.get(k)
        .and_then(Value::as_int)
        .ok_or_else(|| format!("[{}] lipsește întregul '{k}'", s.name))
}

fn opt_str(s: &Section, k: &str, default: &str) -> Result<String, String> {
    match s.get(k) {
        None => Ok(default.to_string()),
        Some(v) => v
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| format!("[{}] '{k}' trebuie să fie șir", s.name)),
    }
}

impl Compendium {
    pub fn from_document(doc: &Document) -> Result<Compendium, String> {
        let mut c = Compendium::default();
        for (id, s) in doc.with_prefix("creature") {
            let hp = req_int(s, "hp")?;
            if hp < 1 {
                return Err(format!("[{}] hp trebuie să fie >= 1", s.name));
            }
            let damage = opt_str(s, "damage", "1d4")?;
            Expr::parse(&damage).map_err(|e| format!("[{}] damage: {e}", s.name))?;
            c.creatures.push(Creature {
                id: id.to_string(),
                name: opt_str(s, "name", id)?,
                hp: hp as i32,
                defense: s.get("defense").and_then(Value::as_int).unwrap_or(10) as i32,
                attack: s.get("attack").and_then(Value::as_int).unwrap_or(0) as i32,
                damage,
                tags: tags(s)?,
                desc: opt_str(s, "desc", "")?,
            });
        }
        for (id, s) in doc.with_prefix("item") {
            let weight = match s.get("weight") {
                None => 0.0,
                Some(v) => v
                    .as_f64()
                    .filter(|w| *w >= 0.0)
                    .ok_or_else(|| format!("[{}] weight invalid", s.name))?,
            };
            c.items.push(ItemDef {
                id: id.to_string(),
                name: opt_str(s, "name", id)?,
                weight,
                value: s.get("value").and_then(Value::as_int).unwrap_or(0),
                tags: tags(s)?,
                desc: opt_str(s, "desc", "")?,
            });
        }
        Ok(c)
    }

    pub fn parse(src: &str) -> Result<Compendium, String> {
        Compendium::from_document(&Document::parse(src)?)
    }

    pub fn creature(&self, id_or_name: &str) -> Option<&Creature> {
        let q = id_or_name.to_lowercase();
        self.creatures
            .iter()
            .find(|c| c.id.to_lowercase() == q || c.name.to_lowercase() == q)
    }
    pub fn item(&self, id_or_name: &str) -> Option<&ItemDef> {
        let q = id_or_name.to_lowercase();
        self.items
            .iter()
            .find(|c| c.id.to_lowercase() == q || c.name.to_lowercase() == q)
    }
    pub fn with_tag(&self, tag: &str) -> Vec<&Creature> {
        self.creatures
            .iter()
            .filter(|c| c.tags.iter().any(|t| t == tag))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = r#"
# bestiary
title = "Demo"   # trailing comment
[creature.goblin]
name = "Goblin # scout"
hp = 7
defense = 12
attack = 3
damage = "1d6+1"
tags = ["humanoid", "small"]
desc = "Sly \"green\" thief\nwith a knife"

[creature.rat]
hp = 2

[item.rope]
name = "Rope"
weight = 2.5
value = 1_0
tags = []
"#;

    #[test]
    fn parses_values() {
        let d = Document::parse(SRC).unwrap();
        assert_eq!(
            d.section("").unwrap().get("title"),
            Some(&Value::Str("Demo".into()))
        );
        let g = d.section("creature.goblin").unwrap();
        assert_eq!(g.get("name").unwrap().as_str(), Some("Goblin # scout"));
        assert_eq!(g.get("hp").unwrap().as_int(), Some(7));
        assert_eq!(
            g.get("desc").unwrap().as_str(),
            Some("Sly \"green\" thief\nwith a knife")
        );
        assert_eq!(g.get("tags").unwrap().as_list().unwrap().len(), 2);
        let r = d.section("item.rope").unwrap();
        assert_eq!(r.get("weight").unwrap().as_f64(), Some(2.5));
        assert_eq!(r.get("value").unwrap().as_int(), Some(10));
        assert_eq!(d.with_prefix("creature").count(), 2);
    }

    #[test]
    fn document_roundtrip() {
        let d = Document::parse(SRC).unwrap();
        let d2 = Document::parse(&d.to_text()).unwrap();
        assert_eq!(d, d2);
        let mut d3 = Document::default();
        d3.sections.push(Section {
            name: "x".into(),
            entries: vec![
                ("f".into(), Value::Float(3.0)),
                ("b".into(), Value::Bool(true)),
                (
                    "l".into(),
                    Value::List(vec![Value::Int(1), Value::Str("a, b".into())]),
                ),
            ],
        });
        assert_eq!(Document::parse(&d3.to_text()).unwrap(), d3);
    }

    #[test]
    fn syntax_errors() {
        for bad in [
            "[a",
            "[a b]",
            "[]",
            "k",
            "k = ",
            "k = \"open",
            "k = [1, 2",
            "k = [1 2]",
            "k = 1 2",
            "k = \"a\" x",
            "k = 1\nk = 2",
            "[a]\n[a]",
            "k = \"bad \\q\"",
            "a b = 1",
            "k = nan",
        ] {
            assert!(Document::parse(bad).is_err(), "should fail: {bad:?}");
        }
    }

    #[test]
    fn compendium_typed() {
        let c = Compendium::parse(SRC).unwrap();
        assert_eq!(c.creatures.len(), 2);
        let g = c.creature("GOBLIN").unwrap();
        assert_eq!((g.hp, g.defense, g.attack), (7, 12, 3));
        assert_eq!(g.name, "Goblin # scout");
        let r = c.creature("rat").unwrap();
        assert_eq!(
            (r.name.as_str(), r.defense, r.damage.as_str()),
            ("rat", 10, "1d4")
        );
        assert_eq!(c.with_tag("small").len(), 1);
        assert_eq!(c.item("rope").unwrap().value, 10);
        let sh = g.to_sheet();
        assert_eq!(sh.resource("HP").unwrap().max, 7);
        assert_eq!(sh.attr("defense"), Some(12));
    }

    #[test]
    fn compendium_validation() {
        assert!(Compendium::parse("[creature.a]\nname = \"A\"").is_err());
        assert!(Compendium::parse("[creature.a]\nhp = 0").is_err());
        assert!(Compendium::parse("[creature.a]\nhp = 3\ndamage = \"zz\"").is_err());
        assert!(Compendium::parse("[creature.a]\nhp = 3\ntags = [1]").is_err());
        assert!(Compendium::parse("[creature.a]\nhp = 3\ntags = 4").is_err());
        assert!(Compendium::parse("[item.a]\nweight = -1.0").is_err());
        assert!(Compendium::parse("[item.a]\nname = 3").is_err());
        assert!(Compendium::parse("").unwrap().creatures.is_empty());
    }
}
