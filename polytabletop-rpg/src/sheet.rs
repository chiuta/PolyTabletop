//! Generic, rules-agnostic character sheet with a line-based text format.
//!
//! ```text
//! name: Aria Voss
//! attr: Vigor | 14
//! skill: Stealth | 3
//! res: Health | 10 | 12
//! item: Rope | 2 | 15 m, hemp
//! cond: Poisoned | 3
//! note: Afraid of heights
//! ```

use crate::text::{esc, fields, same};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resource {
    pub name: String,
    pub cur: i32,
    pub max: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub qty: u32,
    pub note: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Condition {
    pub name: String,
    /// Remaining rounds; `None` = until removed.
    pub turns: Option<u32>,
}

/// How an attribute score turns into a modifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModStyle {
    /// The score itself.
    Raw,
    /// `floor((score - base) / step)`.
    Step { base: i32, step: i32 },
    /// `score / divisor` (floor).
    Div(i32),
}

impl ModStyle {
    pub fn apply(self, score: i32) -> i32 {
        match self {
            ModStyle::Raw => score,
            ModStyle::Step { base, step } => (score - base).div_euclid(step.max(1)),
            ModStyle::Div(d) => score.div_euclid(d.max(1)),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sheet {
    pub name: String,
    pub attrs: Vec<(String, i32)>,
    pub skills: Vec<(String, i32)>,
    pub resources: Vec<Resource>,
    pub items: Vec<Item>,
    pub conditions: Vec<Condition>,
    pub notes: Vec<String>,
}

fn set_pair(v: &mut Vec<(String, i32)>, name: &str, val: i32) {
    if let Some(p) = v.iter_mut().find(|(n, _)| same(n, name)) {
        p.1 = val;
    } else {
        v.push((name.trim().to_string(), val));
    }
}

fn get_pair(v: &[(String, i32)], name: &str) -> Option<i32> {
    v.iter().find(|(n, _)| same(n, name)).map(|p| p.1)
}

impl Sheet {
    pub fn new(name: &str) -> Sheet {
        Sheet {
            name: name.trim().to_string(),
            ..Default::default()
        }
    }

    pub fn set_attr(&mut self, name: &str, v: i32) {
        set_pair(&mut self.attrs, name, v)
    }
    pub fn attr(&self, name: &str) -> Option<i32> {
        get_pair(&self.attrs, name)
    }
    pub fn set_skill(&mut self, name: &str, v: i32) {
        set_pair(&mut self.skills, name, v)
    }
    pub fn skill(&self, name: &str) -> Option<i32> {
        get_pair(&self.skills, name)
    }
    pub fn modifier(&self, attr: &str, style: ModStyle) -> Option<i32> {
        self.attr(attr).map(|s| style.apply(s))
    }

    pub fn set_resource(&mut self, name: &str, cur: i32, max: i32) {
        let max = max.max(0);
        let cur = cur.clamp(0, max);
        if let Some(r) = self.resources.iter_mut().find(|r| same(&r.name, name)) {
            r.cur = cur;
            r.max = max;
        } else {
            self.resources.push(Resource {
                name: name.trim().to_string(),
                cur,
                max,
            });
        }
    }
    pub fn resource(&self, name: &str) -> Option<&Resource> {
        self.resources.iter().find(|r| same(&r.name, name))
    }
    /// Add `delta` (may be negative) clamped to `0..=max`; returns the new value.
    pub fn adjust(&mut self, name: &str, delta: i32) -> Result<i32, String> {
        match self.resources.iter_mut().find(|r| same(&r.name, name)) {
            Some(r) => {
                r.cur = (r.cur as i64 + delta as i64).clamp(0, r.max as i64) as i32;
                Ok(r.cur)
            }
            None => Err(format!("resursă necunoscută: {name}")),
        }
    }

    pub fn add_item(&mut self, name: &str, qty: u32, note: &str) {
        if qty == 0 {
            return;
        }
        if let Some(i) = self.items.iter_mut().find(|i| same(&i.name, name)) {
            i.qty = i.qty.saturating_add(qty);
            if !note.is_empty() {
                i.note = note.to_string();
            }
        } else {
            self.items.push(Item {
                name: name.trim().to_string(),
                qty,
                note: note.to_string(),
            });
        }
    }
    pub fn remove_item(&mut self, name: &str, qty: u32) -> Result<u32, String> {
        let pos = self
            .items
            .iter()
            .position(|i| same(&i.name, name))
            .ok_or_else(|| format!("nu ai: {name}"))?;
        if self.items[pos].qty < qty {
            return Err(format!(
                "ai doar {} × {}",
                self.items[pos].qty, self.items[pos].name
            ));
        }
        self.items[pos].qty -= qty;
        let left = self.items[pos].qty;
        if left == 0 {
            self.items.remove(pos);
        }
        Ok(left)
    }
    pub fn item_qty(&self, name: &str) -> u32 {
        self.items
            .iter()
            .find(|i| same(&i.name, name))
            .map_or(0, |i| i.qty)
    }

    pub fn add_condition(&mut self, name: &str, turns: Option<u32>) {
        if let Some(c) = self.conditions.iter_mut().find(|c| same(&c.name, name)) {
            c.turns = turns;
        } else {
            self.conditions.push(Condition {
                name: name.trim().to_string(),
                turns,
            });
        }
    }
    pub fn remove_condition(&mut self, name: &str) -> bool {
        let n = self.conditions.len();
        self.conditions.retain(|c| !same(&c.name, name));
        n != self.conditions.len()
    }
    pub fn has_condition(&self, name: &str) -> bool {
        self.conditions.iter().any(|c| same(&c.name, name))
    }
    /// Advance one round: timed conditions lose a turn; expired ones are
    /// removed and returned.
    pub fn tick_conditions(&mut self) -> Vec<String> {
        let mut gone = Vec::new();
        for c in &mut self.conditions {
            if let Some(t) = c.turns.as_mut() {
                *t = t.saturating_sub(1);
                if *t == 0 {
                    gone.push(c.name.clone());
                }
            }
        }
        self.conditions.retain(|c| c.turns != Some(0));
        gone
    }

    pub fn to_text(&self) -> String {
        let mut o = format!("name: {}\n", esc(&self.name));
        for (n, v) in &self.attrs {
            o += &format!("attr: {} | {}\n", esc(n), v);
        }
        for (n, v) in &self.skills {
            o += &format!("skill: {} | {}\n", esc(n), v);
        }
        for r in &self.resources {
            o += &format!("res: {} | {} | {}\n", esc(&r.name), r.cur, r.max);
        }
        for i in &self.items {
            o += &format!("item: {} | {} | {}\n", esc(&i.name), i.qty, esc(&i.note));
        }
        for c in &self.conditions {
            match c.turns {
                Some(t) => o += &format!("cond: {} | {}\n", esc(&c.name), t),
                None => o += &format!("cond: {}\n", esc(&c.name)),
            }
        }
        for n in &self.notes {
            o += &format!("note: {}\n", esc(n));
        }
        o
    }

    pub fn from_text(s: &str) -> Result<Sheet, String> {
        let mut sh = Sheet::default();
        for (ln, line) in s.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (k, rest) = line
                .split_once(':')
                .ok_or_else(|| format!("linia {}: lipsește ':'", ln + 1))?;
            let f = fields(rest);
            let bad = |m: &str| format!("linia {}: {}", ln + 1, m);
            let int = |x: &str| x.parse::<i32>().map_err(|_| bad("număr invalid"));
            match k.trim() {
                "name" => sh.name = f[0].clone(),
                "attr" | "skill" => {
                    if f.len() != 2 || f[0].is_empty() {
                        return Err(bad("se așteaptă nume | valoare"));
                    }
                    let v = int(&f[1])?;
                    if k.trim() == "attr" {
                        sh.set_attr(&f[0], v)
                    } else {
                        sh.set_skill(&f[0], v)
                    }
                }
                "res" => {
                    if f.len() != 3 || f[0].is_empty() {
                        return Err(bad("se așteaptă nume | cur | max"));
                    }
                    sh.set_resource(&f[0], int(&f[1])?, int(&f[2])?);
                }
                "item" => {
                    if f.len() < 2 || f[0].is_empty() {
                        return Err(bad("se așteaptă nume | cantitate | notă"));
                    }
                    let q = f[1].parse::<u32>().map_err(|_| bad("cantitate invalidă"))?;
                    sh.add_item(&f[0], q, f.get(2).map_or("", |x| x.as_str()));
                }
                "cond" => {
                    if f[0].is_empty() {
                        return Err(bad("condiție fără nume"));
                    }
                    let t = match f.get(1) {
                        Some(x) => Some(x.parse::<u32>().map_err(|_| bad("durată invalidă"))?),
                        None => None,
                    };
                    sh.add_condition(&f[0], t);
                }
                "note" => sh.notes.push(f[0].clone()),
                other => return Err(bad(&format!("cheie necunoscută '{other}'"))),
            }
        }
        Ok(sh)
    }

    pub fn render(&self) -> String {
        let mut o = format!("== {} ==\n", self.name);
        if !self.attrs.is_empty() {
            o += "Atribute: ";
            o += &self
                .attrs
                .iter()
                .map(|(n, v)| format!("{n} {v}"))
                .collect::<Vec<_>>()
                .join(", ");
            o += "\n";
        }
        if !self.skills.is_empty() {
            o += "Abilități: ";
            o += &self
                .skills
                .iter()
                .map(|(n, v)| format!("{n} {v}"))
                .collect::<Vec<_>>()
                .join(", ");
            o += "\n";
        }
        for r in &self.resources {
            o += &format!("{}: {}/{}\n", r.name, r.cur, r.max);
        }
        if !self.items.is_empty() {
            o += "Inventar: ";
            o += &self
                .items
                .iter()
                .map(|i| format!("{} ×{}", i.name, i.qty))
                .collect::<Vec<_>>()
                .join(", ");
            o += "\n";
        }
        if !self.conditions.is_empty() {
            o += "Condiții: ";
            o += &self
                .conditions
                .iter()
                .map(|c| match c.turns {
                    Some(t) => format!("{} ({t})", c.name),
                    None => c.name.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            o += "\n";
        }
        o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Sheet {
        let mut s = Sheet::new("Aria | Voss");
        s.set_attr("Vigor", 14);
        s.set_attr("Agilitate", 9);
        s.set_skill("Stealth", 3);
        s.set_resource("Health", 10, 12);
        s.add_item("Rope", 2, "15 m | hemp");
        s.add_item("Torch", 3, "");
        s.add_condition("Poisoned", Some(3));
        s.add_condition("Blessed", None);
        s.notes.push("line1\nline2".into());
        s
    }

    #[test]
    fn text_roundtrip() {
        let s = sample();
        let t = s.to_text();
        assert_eq!(Sheet::from_text(&t).unwrap(), s);
    }

    #[test]
    fn attrs_and_modifiers() {
        let mut s = sample();
        assert_eq!(s.attr("vigor"), Some(14));
        s.set_attr("VIGOR", 16);
        assert_eq!(s.attrs.len(), 2);
        assert_eq!(
            s.modifier("vigor", ModStyle::Step { base: 10, step: 2 }),
            Some(3)
        );
        assert_eq!(
            s.modifier("agilitate", ModStyle::Step { base: 10, step: 2 }),
            Some(-1)
        );
        assert_eq!(ModStyle::Div(5).apply(14), 2);
        assert_eq!(ModStyle::Raw.apply(7), 7);
        assert_eq!(s.modifier("nu", ModStyle::Raw), None);
    }

    #[test]
    fn resources_clamp() {
        let mut s = sample();
        assert_eq!(s.adjust("health", 5), Ok(12));
        assert_eq!(s.adjust("health", -100), Ok(0));
        assert!(s.adjust("mana", 1).is_err());
        s.set_resource("Health", 99, 12);
        assert_eq!(s.resource("Health").unwrap().cur, 12);
    }

    #[test]
    fn inventory() {
        let mut s = sample();
        s.add_item("rope", 1, "");
        assert_eq!(s.item_qty("Rope"), 3);
        assert_eq!(s.remove_item("Rope", 2), Ok(1));
        assert!(s.remove_item("Rope", 5).is_err());
        assert_eq!(s.remove_item("Rope", 1), Ok(0));
        assert_eq!(s.item_qty("Rope"), 0);
        assert!(s.remove_item("Rope", 1).is_err());
        s.add_item("x", 0, "");
        assert_eq!(s.item_qty("x"), 0);
    }

    #[test]
    fn conditions_tick() {
        let mut s = sample();
        assert!(s.has_condition("poisoned"));
        assert!(s.tick_conditions().is_empty());
        assert!(s.tick_conditions().is_empty());
        assert_eq!(s.tick_conditions(), vec!["Poisoned".to_string()]);
        assert!(!s.has_condition("Poisoned"));
        assert!(s.has_condition("Blessed"));
        assert!(s.remove_condition("blessed"));
        assert!(!s.remove_condition("blessed"));
    }

    #[test]
    fn parse_errors() {
        assert!(Sheet::from_text("attr: A | x").is_err());
        assert!(Sheet::from_text("foo: bar").is_err());
        assert!(Sheet::from_text("garbage").is_err());
        assert!(Sheet::from_text("res: A | 1").is_err());
        assert!(Sheet::from_text("# c\n\nname: X").is_ok());
    }

    #[test]
    fn render_mentions_everything() {
        let r = sample().render();
        for k in [
            "Vigor 14",
            "Stealth 3",
            "Health: 10/12",
            "Torch ×3",
            "Poisoned (3)",
        ] {
            assert!(r.contains(k), "{k} in {r}");
        }
    }
}
