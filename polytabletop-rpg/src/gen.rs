//! Spell / ability / item generators driven by tables with rarity weights.

use crate::data::{Document, Section, Value};
use polytabletop_core::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

pub const RARITIES: [Rarity; 5] = [
    Rarity::Common,
    Rarity::Uncommon,
    Rarity::Rare,
    Rarity::Epic,
    Rarity::Legendary,
];

impl Rarity {
    pub fn parse(s: &str) -> Option<Rarity> {
        Some(match s.trim().to_lowercase().as_str() {
            "common" => Rarity::Common,
            "uncommon" => Rarity::Uncommon,
            "rare" => Rarity::Rare,
            "epic" => Rarity::Epic,
            "legendary" => Rarity::Legendary,
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Rarity::Common => "common",
            Rarity::Uncommon => "uncommon",
            Rarity::Rare => "rare",
            Rarity::Epic => "epic",
            Rarity::Legendary => "legendary",
        }
    }
    /// Value multiplier applied to generated items.
    pub fn value_mult(self) -> i64 {
        match self {
            Rarity::Common => 1,
            Rarity::Uncommon => 2,
            Rarity::Rare => 5,
            Rarity::Epic => 15,
            Rarity::Legendary => 50,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub rarity: Rarity,
    pub fields: Vec<(String, String)>,
    pub value: i64,
}

impl Entry {
    pub fn field(&self, k: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(n, _)| n == k)
            .map(|(_, v)| v.as_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Generated {
    pub name: String,
    pub rarity: Rarity,
    pub value: i64,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GenLib {
    pub weights: [u32; 5],
    pub spells: Vec<Entry>,
    pub abilities: Vec<Entry>,
    pub gear: Vec<Entry>,
    pub affixes: Vec<Entry>,
}

fn entry(id: &str, s: &Section) -> Result<Entry, String> {
    let mut fields = vec![];
    for (k, v) in &s.entries {
        let t = match v {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::List(_) => continue,
        };
        fields.push((k.clone(), t));
    }
    let rname = s.get("rarity").and_then(|v| v.as_str()).unwrap_or("common");
    Ok(Entry {
        id: id.to_string(),
        name: s
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(id)
            .to_string(),
        rarity: Rarity::parse(rname)
            .ok_or_else(|| format!("{id}: raritate necunoscută {rname}"))?,
        value: s.get("value").and_then(|v| v.as_int()).unwrap_or(0),
        fields,
    })
}

/// Weighted pick; `w` must have a positive sum. Returns the index.
pub fn pick_weighted(w: &[u32], rng: &mut Rng) -> usize {
    let total: u64 = w.iter().map(|&x| u64::from(x)).sum();
    let mut r = rng.next_u64() % total.max(1);
    for (i, &x) in w.iter().enumerate() {
        if r < u64::from(x) {
            return i;
        }
        r -= u64::from(x);
    }
    w.len() - 1
}

impl GenLib {
    pub fn parse(src: &str) -> Result<GenLib, String> {
        let doc = Document::parse(src)?;
        let mut lib = GenLib {
            weights: [60, 25, 10, 4, 1],
            ..GenLib::default()
        };
        if let Some(r) = doc.section("rarity") {
            for (i, rar) in RARITIES.iter().enumerate() {
                if let Some(v) = r.get(rar.name()).and_then(|v| v.as_int()) {
                    lib.weights[i] = v.max(0) as u32;
                }
            }
        }
        for (id, s) in doc.with_prefix("spell") {
            lib.spells.push(entry(id, s)?);
        }
        for (id, s) in doc.with_prefix("ability") {
            lib.abilities.push(entry(id, s)?);
        }
        for (id, s) in doc.with_prefix("gear") {
            lib.gear.push(entry(id, s)?);
        }
        for (id, s) in doc.with_prefix("affix") {
            lib.affixes.push(entry(id, s)?);
        }
        Ok(lib)
    }

    pub fn builtin() -> GenLib {
        GenLib::parse(include_str!("../data/generators.toml")).expect("builtin generators")
    }

    /// Rarity-weighted pick among entries with `rarity >= min`.
    fn pick<'a>(&self, from: &'a [Entry], min: Rarity, rng: &mut Rng) -> Option<&'a Entry> {
        let c: Vec<&Entry> = from.iter().filter(|e| e.rarity >= min).collect();
        if c.is_empty() {
            return None;
        }
        let w: Vec<u32> = c
            .iter()
            .map(|e| self.weights[RARITIES.iter().position(|r| *r == e.rarity).unwrap()])
            .collect();
        if w.iter().all(|&x| x == 0) {
            return Some(c[rng.below(c.len())]);
        }
        Some(c[pick_weighted(&w, rng)])
    }

    pub fn spell(&self, min: Rarity, rng: &mut Rng) -> Option<Generated> {
        let e = self.pick(&self.spells, min, rng)?;
        Some(Generated {
            name: e.name.clone(),
            rarity: e.rarity,
            value: 0,
            text: format!(
                "{} [{}, nivel {}, {}]: {}",
                e.name,
                e.field("school").unwrap_or("?"),
                e.field("level").unwrap_or("?"),
                e.rarity.name(),
                e.field("effect").unwrap_or("")
            ),
        })
    }

    pub fn ability(&self, min: Rarity, rng: &mut Rng) -> Option<Generated> {
        let e = self.pick(&self.abilities, min, rng)?;
        Some(Generated {
            name: e.name.clone(),
            rarity: e.rarity,
            value: 0,
            text: format!(
                "{} [{}, {}]: {}",
                e.name,
                e.field("kind").unwrap_or("?"),
                e.rarity.name(),
                e.field("effect").unwrap_or("")
            ),
        })
    }

    /// A base item; the higher the rarity of the first roll, the more affixes.
    /// With `min` above common, affixes are guaranteed.
    pub fn item(&self, min: Rarity, rng: &mut Rng) -> Option<Generated> {
        let base = self.pick(&self.gear, Rarity::Common, rng)?;
        let mut rarity = base.rarity;
        let mut value = base.value;
        let mut name = base.name.clone();
        let mut bonuses: Vec<String> = vec![];
        // Try one prefix and one suffix; take them with decreasing odds,
        // forced while the requested minimum rarity is not met yet.
        for round in 0..2 {
            let want = rarity < min || rng.below(100) < if round == 0 { 45 } else { 20 };
            if !want {
                continue;
            }
            let pos = if round == 0 { "prefix" } else { "suffix" };
            let pool: Vec<Entry> = self
                .affixes
                .iter()
                .filter(|a| a.field("position") == Some(pos))
                .cloned()
                .collect();
            let Some(a) = self.pick(&pool, Rarity::Common, rng) else {
                continue;
            };
            if round == 0 {
                name = format!("{} {}", a.name, name);
            } else {
                name = format!("{} {}", name, a.name);
            }
            rarity = rarity.max(a.rarity);
            value *= a
                .field("mult")
                .and_then(|m| m.parse::<i64>().ok())
                .unwrap_or(2);
            bonuses.push(a.field("bonus").unwrap_or("").to_string());
        }
        // If the minimum is still unmet, raise the rarity label (and value).
        if rarity < min {
            rarity = min;
            value *= rarity.value_mult();
        }
        let mut text = format!("{name} ({}, valoare {value})", rarity.name());
        if !bonuses.is_empty() {
            text += &format!(": {}", bonuses.join("; "));
        }
        Some(Generated {
            name,
            rarity,
            value,
            text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_bias_toward_common() {
        let lib = GenLib::builtin();
        let mut rng = Rng::new(3);
        let mut count = [0u32; 5];
        for _ in 0..2000 {
            let s = lib.spell(Rarity::Common, &mut rng).unwrap();
            count[RARITIES.iter().position(|r| *r == s.rarity).unwrap()] += 1;
        }
        assert!(count[0] > count[3]);
    }
}
