//! Character generator with pluggable generic "systems" described in the
//! TOML-like data format (see `data/system-*.toml`).
//!
//! A system lists attributes, how to produce their values (`roll` an
//! expression times a multiplier, or `spread` points between a min and a
//! max), a skill list with base values and a skill-point budget, and
//! resources whose maximum can depend on an attribute.

use crate::data::{Document, Section};
use crate::dice::Expr;
use crate::sheet::Sheet;
use polytabletop_core::rng::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum AttrMethod {
    Roll { expr: String, mult: i32 },
    Spread { points: i32, min: i32, max: i32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceDef {
    pub name: String,
    pub dice: String,
    pub attr: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameSystem {
    pub id: String,
    pub name: String,
    pub kind: String,
    /// Suggested check preset (`use ID` in the CLI).
    pub preset: String,
    pub attributes: Vec<String>,
    pub attr_method: AttrMethod,
    pub skills: Vec<(String, i32)>,
    pub skill_points: i32,
    pub skill_step: i32,
    pub skill_cap: i32,
    pub skill_picks: usize,
    pub resources: Vec<ResourceDef>,
    pub res_attr_offset: i32,
    pub res_attr_div: i32,
}

fn int(s: &Section, k: &str, d: i64) -> i64 {
    s.get(k).and_then(|v| v.as_int()).unwrap_or(d)
}
fn string(s: &Section, k: &str, d: &str) -> String {
    s.get(k).and_then(|v| v.as_str()).unwrap_or(d).to_string()
}
fn strings(s: &Section, k: &str) -> Vec<String> {
    s.get(k)
        .and_then(|v| v.as_list())
        .map(|l| {
            l.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

impl GameSystem {
    pub fn parse(src: &str) -> Result<GameSystem, String> {
        let doc = Document::parse(src)?;
        let s = doc.section("system").ok_or("lipsește secțiunea [system]")?;
        let attributes = strings(s, "attributes");
        if attributes.is_empty() {
            return Err("system: attributes gol".into());
        }
        let attr_method = match string(s, "attr_method", "roll").as_str() {
            "roll" => {
                let expr = string(s, "attr_roll", "3d6");
                Expr::parse(&expr).map_err(|e| format!("attr_roll: {e}"))?;
                AttrMethod::Roll {
                    expr,
                    mult: int(s, "attr_mult", 1) as i32,
                }
            }
            "spread" => {
                let (min, max) = (int(s, "attr_min", 1) as i32, int(s, "attr_max", 5) as i32);
                let points = int(s, "attr_points", 0) as i32;
                if min > max || points < 0 {
                    return Err("system: spread invalid".into());
                }
                if points > (max - min) * attributes.len() as i32 {
                    return Err("system: attr_points depășește capacitatea".into());
                }
                AttrMethod::Spread { points, min, max }
            }
            o => return Err(format!("system: attr_method necunoscut: {o}")),
        };
        let mut skills = vec![];
        for e in strings(s, "skills") {
            match e.split_once(':') {
                Some((n, b)) => skills.push((
                    n.trim().to_string(),
                    b.trim()
                        .parse()
                        .map_err(|_| format!("skill invalid: {e}"))?,
                )),
                None => skills.push((e.trim().to_string(), 0)),
            }
        }
        let mut resources = vec![];
        for e in strings(s, "resources") {
            let (n, rest) = e
                .split_once(':')
                .ok_or_else(|| format!("resursă invalidă: {e}"))?;
            let (dice, attr) = match rest.split_once('@') {
                Some((d, a)) => (d.trim().trim_end_matches('+').to_string(), Some(a.trim())),
                None => (rest.trim().to_string(), None),
            };
            if let Some(a) = attr {
                if !attributes.iter().any(|x| x == a) {
                    return Err(format!("resursă {n}: atribut necunoscut {a}"));
                }
            }
            if !dice.is_empty() {
                Expr::parse(&dice).map_err(|e| format!("resursă {n}: {e}"))?;
            }
            resources.push(ResourceDef {
                name: n.trim().to_string(),
                dice,
                attr: attr.map(str::to_string),
            });
        }
        let div = int(s, "res_attr_div", 1).max(1) as i32;
        Ok(GameSystem {
            id: string(s, "id", "custom"),
            name: string(s, "name", "Custom"),
            kind: string(s, "kind", "generic"),
            preset: string(s, "preset", "d20-dc"),
            attributes,
            attr_method,
            skills,
            skill_points: int(s, "skill_points", 0) as i32,
            skill_step: int(s, "skill_step", 1).max(1) as i32,
            skill_cap: int(s, "skill_cap", 100) as i32,
            skill_picks: int(s, "skill_picks", 0).max(0) as usize,
            resources,
            res_attr_offset: int(s, "res_attr_offset", 0) as i32,
            res_attr_div: div,
        })
    }

    /// Generates a complete [`Sheet`].
    pub fn generate(&self, name: &str, rng: &mut Rng) -> Result<Sheet, String> {
        let mut sh = Sheet::new(name);
        let mut vals: Vec<i32> = match &self.attr_method {
            AttrMethod::Roll { expr, mult } => {
                let e = Expr::parse(expr).map_err(|e| e.to_string())?;
                self.attributes
                    .iter()
                    .map(|_| e.roll(rng).total as i32 * mult)
                    .collect()
            }
            AttrMethod::Spread { points, min, max } => {
                let mut v = vec![*min; self.attributes.len()];
                let mut left = *points;
                while left > 0 {
                    let open: Vec<usize> = (0..v.len()).filter(|&i| v[i] < *max).collect();
                    if open.is_empty() {
                        break;
                    }
                    v[open[rng.below(open.len())]] += 1;
                    left -= 1;
                }
                v
            }
        };
        for (n, v) in self.attributes.iter().zip(vals.drain(..)) {
            sh.set_attr(n, v);
        }
        // Skills: choose the trained subset, then spend the budget in steps.
        let mut idx: Vec<usize> = (0..self.skills.len()).collect();
        if self.skill_picks > 0 && self.skill_picks < idx.len() {
            for i in (1..idx.len()).rev() {
                idx.swap(i, rng.below(i + 1));
            }
            idx.truncate(self.skill_picks);
            idx.sort_unstable();
        }
        let mut level: Vec<i32> = self.skills.iter().map(|(_, b)| *b).collect();
        let mut left = self.skill_points;
        while left > 0 {
            let open: Vec<usize> = idx
                .iter()
                .copied()
                .filter(|&i| level[i] + self.skill_step <= self.skill_cap)
                .collect();
            if open.is_empty() {
                break;
            }
            let i = open[rng.below(open.len())];
            let step = self.skill_step.min(left);
            level[i] += step;
            left -= step;
        }
        for &i in &idx {
            sh.set_skill(&self.skills[i].0, level[i]);
        }
        for r in &self.resources {
            let mut v = if r.dice.is_empty() {
                0
            } else {
                Expr::parse(&r.dice)
                    .map_err(|e| e.to_string())?
                    .roll(rng)
                    .total as i32
            };
            if let Some(a) = &r.attr {
                let av = sh.attr(a).unwrap_or(0);
                v += (av - self.res_attr_offset).div_euclid(self.res_attr_div);
            }
            let v = v.max(1);
            sh.set_resource(&r.name, v, v);
        }
        Ok(sh)
    }
}

const FIRST: [&str; 12] = [
    "Ar", "Bel", "Cor", "Dra", "El", "Fen", "Gal", "Hal", "Ira", "Kes", "Lun", "Mor",
];
const LAST: [&str; 12] = [
    "dan", "wyn", "tis", "vek", "ra", "len", "mor", "sha", "bor", "ith", "ur", "ael",
];

/// Short invented name, deterministic for a seeded generator.
pub fn random_name(rng: &mut Rng) -> String {
    format!(
        "{}{}{}",
        FIRST[rng.below(FIRST.len())],
        LAST[rng.below(LAST.len())],
        if rng.below(2) == 0 {
            LAST[rng.below(LAST.len())]
        } else {
            ""
        }
    )
}

pub const BUILTIN: [(&str, &str); 4] = [
    ("percentile", include_str!("../data/system-percentile.toml")),
    ("d20-heroic", include_str!("../data/system-d20.toml")),
    ("grit-pool", include_str!("../data/system-pool.toml")),
    (
        "star-narrative",
        include_str!("../data/system-narrative.toml"),
    ),
];

pub fn builtin(id: &str) -> Option<GameSystem> {
    BUILTIN
        .iter()
        .find(|(i, _)| *i == id)
        .and_then(|(_, s)| GameSystem::parse(s).ok())
}
