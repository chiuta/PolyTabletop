//! Encounter-balance calculator (challenge-rating style budget) and loot
//! distribution among party members.

use crate::data::Document;
use crate::gen::{GenLib, Generated, Rarity};
use polytabletop_core::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Deadly,
}

impl Difficulty {
    pub fn parse(s: &str) -> Option<Difficulty> {
        Some(match s.to_lowercase().as_str() {
            "easy" | "usor" | "ușor" => Difficulty::Easy,
            "medium" | "mediu" => Difficulty::Medium,
            "hard" | "greu" => Difficulty::Hard,
            "deadly" | "mortal" => Difficulty::Deadly,
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
            Difficulty::Deadly => "deadly",
        }
    }
    /// Budget factor in tenths of a point per PC level.
    fn tenths(self) -> u32 {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Medium => 10,
            Difficulty::Hard => 15,
            Difficulty::Deadly => 22,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Foe {
    pub id: String,
    pub name: String,
    pub cr: f64,
}

/// Threat points of a single foe: 10 per CR, squared growth above CR 4.
pub fn foe_cost(cr: f64) -> f64 {
    let c = cr.max(0.0);
    10.0 * c
        + if c > 4.0 {
            (c - 4.0) * (c - 4.0) * 2.0
        } else {
            0.0
        }
}

/// Crowd multiplier: more bodies are worth more than their sum.
pub fn group_mult(n: usize) -> f64 {
    match n {
        0 | 1 => 1.0,
        2 => 1.5,
        3..=6 => 2.0,
        7..=10 => 2.5,
        _ => 3.0,
    }
}

/// Budget for a party (10 points per level per PC, scaled by difficulty).
pub fn budget(party: u32, level: u32, d: Difficulty) -> f64 {
    f64::from(party * level * 10 * d.tenths()) / 10.0
}

/// Adjusted cost of a group of foes (by CR).
pub fn group_cost(crs: &[f64]) -> f64 {
    crs.iter().map(|&c| foe_cost(c)).sum::<f64>() * group_mult(crs.len())
}

/// Which difficulty band the group falls in (`None` = trivial).
pub fn rate(crs: &[f64], party: u32, level: u32) -> Option<Difficulty> {
    let c = group_cost(crs);
    [
        Difficulty::Deadly,
        Difficulty::Hard,
        Difficulty::Medium,
        Difficulty::Easy,
    ]
    .into_iter()
    .find(|&d| c >= budget(party, level, d))
}

pub fn parse_foes(src: &str) -> Result<Vec<Foe>, String> {
    let doc = Document::parse(src)?;
    let mut v = vec![];
    for (id, s) in doc.with_prefix("foe") {
        let cr = s
            .get("cr")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| format!("foe.{id}: lipsește cr"))?;
        v.push(Foe {
            id: id.to_string(),
            name: s
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(id)
                .to_string(),
            cr,
        });
    }
    if v.is_empty() {
        return Err("nicio creatură [foe.ID]".into());
    }
    Ok(v)
}

pub fn builtin_foes() -> Vec<Foe> {
    parse_foes(include_str!("../data/foes.toml")).expect("builtin foes")
}

/// Fills the budget with random foes (at most `max_foes`); the result never
/// exceeds the budget. Returns indices into `pool`.
pub fn build(pool: &[Foe], budget: f64, max_foes: usize, rng: &mut Rng) -> Vec<usize> {
    let mut chosen: Vec<usize> = vec![];
    for _ in 0..max_foes * 4 {
        if chosen.len() >= max_foes {
            break;
        }
        let i = rng.below(pool.len());
        let mut trial: Vec<f64> = chosen.iter().map(|&k| pool[k].cr).collect();
        trial.push(pool[i].cr);
        if group_cost(&trial) <= budget {
            chosen.push(i);
        }
    }
    chosen.sort_unstable();
    chosen
}

/// Treasure value (in coin) for an encounter of this adjusted cost.
pub fn treasure_value(cost: f64) -> i64 {
    (cost * 5.0).round() as i64
}

/// Splits `total` into `parts` random non-negative shares with exact sum.
pub fn split_random(total: i64, parts: usize, rng: &mut Rng) -> Vec<i64> {
    if parts == 0 {
        return vec![];
    }
    let weights: Vec<u64> = (0..parts).map(|_| 50 + rng.below(100) as u64).collect();
    let sum: u64 = weights.iter().sum();
    let mut shares: Vec<i64> = weights
        .iter()
        .map(|&w| (total as i128 * i128::from(w) / i128::from(sum)) as i64)
        .collect();
    let given: i64 = shares.iter().sum();
    shares[0] += total - given;
    shares
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hoard {
    pub coins: i64,
    pub items: Vec<Generated>,
}

/// Builds a hoard worth about `value`: a third in coins, the rest in items
/// (as many as fit, rarer when the budget is large).
pub fn make_hoard(value: i64, lib: &GenLib, rng: &mut Rng) -> Hoard {
    let coins = value / 3;
    let mut left = value - coins;
    let min = match value {
        v if v >= 5000 => Rarity::Rare,
        v if v >= 1000 => Rarity::Uncommon,
        _ => Rarity::Common,
    };
    let mut items = vec![];
    for _ in 0..24 {
        let Some(it) = lib.item(min, rng) else { break };
        if it.value > left {
            continue;
        }
        left -= it.value;
        items.push(it);
    }
    Hoard {
        coins: coins + left,
        items,
    }
}

/// Gives each member a share of coins and deals items out so that item
/// values stay as even as possible (best item to the poorest member).
pub fn distribute(h: &Hoard, members: &[String], rng: &mut Rng) -> Vec<(String, i64, Vec<String>)> {
    let coins = split_random(h.coins, members.len(), rng);
    let mut out: Vec<(String, i64, Vec<String>)> = members
        .iter()
        .zip(coins)
        .map(|(m, c)| (m.clone(), c, vec![]))
        .collect();
    let mut worth = vec![0i64; members.len()];
    let mut items: Vec<&Generated> = h.items.iter().collect();
    items.sort_by(|a, b| b.value.cmp(&a.value).then(a.name.cmp(&b.name)));
    for it in items {
        if out.is_empty() {
            break;
        }
        let j = (0..worth.len()).min_by_key(|&k| worth[k]).unwrap();
        worth[j] += it.value;
        out[j].2.push(it.name.clone());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_scales() {
        assert!(budget(4, 3, Difficulty::Hard) > budget(4, 3, Difficulty::Medium));
        assert_eq!(budget(4, 3, Difficulty::Medium), 120.0);
        assert_eq!(rate(&[0.25], 4, 5), None);
    }
}
