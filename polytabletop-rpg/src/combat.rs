//! Mass-combat / skirmish simulator: units, morale and terrain modifiers.
//!
//! Two sides fight for a limited number of rounds. Every round each active
//! unit picks a random active enemy unit and inflicts casualties; all attacks
//! of a round are resolved simultaneously. Heavy losses erode morale, and a
//! unit whose morale reaches zero routs. Everything is driven by [`Rng`], so a
//! seed reproduces a battle exactly.

use crate::data::Document;
use polytabletop_core::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terrain {
    Open,
    Forest,
    Hills,
    Marsh,
    Fort,
}

impl Terrain {
    pub fn parse(s: &str) -> Option<Terrain> {
        Some(match s.trim().to_lowercase().as_str() {
            "open" | "plain" | "plains" | "câmp" | "camp" => Terrain::Open,
            "forest" | "wood" | "pădure" | "padure" => Terrain::Forest,
            "hills" | "hill" | "deal" | "dealuri" => Terrain::Hills,
            "marsh" | "swamp" | "mlaștină" | "mlastina" => Terrain::Marsh,
            "fort" | "fortification" | "fortificație" | "fortificatie" => Terrain::Fort,
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Terrain::Open => "open",
            Terrain::Forest => "forest",
            Terrain::Hills => "hills",
            Terrain::Marsh => "marsh",
            Terrain::Fort => "fort",
        }
    }
    /// Defense bonus for a unit standing here against melee / ranged attacks.
    pub fn defense_bonus(self, ranged_attacker: bool) -> i32 {
        match (self, ranged_attacker) {
            (Terrain::Open, _) => 0,
            (Terrain::Forest, false) => 2,
            (Terrain::Forest, true) => 3,
            (Terrain::Hills, _) => 1,
            (Terrain::Marsh, _) => 0,
            (Terrain::Fort, false) => 4,
            (Terrain::Fort, true) => 3,
        }
    }
    /// Penalty to the attack of a *melee* unit standing here (mobility).
    pub fn melee_penalty(self) -> i32 {
        match self {
            Terrain::Marsh => 2,
            Terrain::Forest => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub id: String,
    pub name: String,
    /// 0 or 1.
    pub side: usize,
    pub count: u32,
    pub start: u32,
    pub attack: i32,
    pub defense: i32,
    pub morale: i32,
    pub max_morale: i32,
    pub ranged: bool,
    pub terrain: Terrain,
    pub routed: bool,
}

impl Unit {
    pub fn new(id: &str, side: usize, count: u32, attack: i32, defense: i32, morale: i32) -> Unit {
        Unit {
            id: id.to_string(),
            name: id.to_string(),
            side,
            count,
            start: count,
            attack,
            defense,
            morale,
            max_morale: morale,
            ranged: false,
            terrain: Terrain::Open,
            routed: false,
        }
    }
    pub fn active(&self) -> bool {
        self.count > 0 && !self.routed && self.morale > 0
    }
}

#[derive(Clone, Debug)]
pub struct Battle {
    pub side_names: [String; 2],
    pub units: Vec<Unit>,
    pub max_rounds: u32,
    pub round: u32,
    pub log: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub rounds: u32,
    /// `None` = draw (both sides still standing or both wiped out).
    pub winner: Option<usize>,
}

fn key(sec: &crate::data::Section, k: &str) -> Option<i64> {
    sec.get(k).and_then(|v| v.as_int())
}

impl Battle {
    pub fn new(a: &str, b: &str, units: Vec<Unit>, max_rounds: u32) -> Battle {
        Battle {
            side_names: [a.to_string(), b.to_string()],
            units,
            max_rounds,
            round: 0,
            log: vec![],
        }
    }

    /// `[battle] a="…" b="…" rounds=N` plus `[unit.ID] side="a"|"b" count attack defense morale ranged terrain name`.
    pub fn from_document(doc: &Document) -> Result<Battle, String> {
        let mut names = ["Side A".to_string(), "Side B".to_string()];
        let mut rounds = 20u32;
        if let Some(b) = doc.section("battle") {
            if let Some(s) = b.get("a").and_then(|v| v.as_str()) {
                names[0] = s.to_string();
            }
            if let Some(s) = b.get("b").and_then(|v| v.as_str()) {
                names[1] = s.to_string();
            }
            if let Some(n) = key(b, "rounds") {
                rounds = n.clamp(1, 1000) as u32;
            }
        }
        let mut units = vec![];
        for (id, sec) in doc.with_prefix("unit") {
            let side = match sec.get("side").and_then(|v| v.as_str()).unwrap_or("") {
                "a" | "A" => 0,
                "b" | "B" => 1,
                other => {
                    return Err(format!(
                        "unit.{id}: side trebuie să fie a sau b ({other:?})"
                    ))
                }
            };
            let count = key(sec, "count").unwrap_or(10);
            if !(1..=100_000).contains(&count) {
                return Err(format!("unit.{id}: count invalid"));
            }
            let mut u = Unit::new(
                id,
                side,
                count as u32,
                key(sec, "attack").unwrap_or(3) as i32,
                key(sec, "defense").unwrap_or(3) as i32,
                key(sec, "morale").unwrap_or(6).clamp(1, 100) as i32,
            );
            if let Some(n) = sec.get("name").and_then(|v| v.as_str()) {
                u.name = n.to_string();
            }
            u.ranged = sec.get("ranged").and_then(|v| v.as_bool()).unwrap_or(false);
            if let Some(t) = sec.get("terrain").and_then(|v| v.as_str()) {
                u.terrain =
                    Terrain::parse(t).ok_or_else(|| format!("unit.{id}: teren necunoscut {t}"))?;
            }
            units.push(u);
        }
        for s in 0..2 {
            if !units.iter().any(|u| u.side == s) {
                return Err(format!(
                    "partea {} nu are unități",
                    if s == 0 { "a" } else { "b" }
                ));
            }
        }
        Ok(Battle::new(&names[0], &names[1], units, rounds))
    }

    pub fn parse(src: &str) -> Result<Battle, String> {
        Battle::from_document(&Document::parse(src)?)
    }

    pub fn strength(&self, side: usize) -> u32 {
        self.units
            .iter()
            .filter(|u| u.side == side && u.active())
            .map(|u| u.count)
            .sum()
    }

    pub fn finished(&self) -> bool {
        self.round >= self.max_rounds || self.strength(0) == 0 || self.strength(1) == 0
    }

    /// Probability that a single attacking soldier removes one defender.
    pub fn kill_chance(att: &Unit, def: &Unit, outnumber: bool) -> f64 {
        let mut a = att.attack;
        if !att.ranged {
            a -= att.terrain.melee_penalty();
        }
        let d = def.defense + def.terrain.defense_bonus(att.ranged);
        let mut p = 0.25 + 0.05 * f64::from(a - d);
        if outnumber {
            p += 0.1;
        }
        p.clamp(0.03, 0.8)
    }

    /// Plays one round; returns false when the battle was already over.
    pub fn step(&mut self, rng: &mut Rng) -> bool {
        if self.finished() {
            return false;
        }
        self.round += 1;
        let n = self.units.len();
        let mut losses = vec![0u32; n];
        for i in 0..n {
            if !self.units[i].active() {
                continue;
            }
            let enemies: Vec<usize> = (0..n)
                .filter(|&j| self.units[j].side != self.units[i].side && self.units[j].active())
                .collect();
            if enemies.is_empty() {
                continue;
            }
            let t = enemies[rng.below(enemies.len())];
            let (a, d) = (&self.units[i], &self.units[t]);
            let p = Battle::kill_chance(a, d, a.count >= d.count.saturating_mul(2));
            // Sample at most 400 soldiers and scale up for huge units.
            let sample = a.count.min(400);
            let mut hits = 0u32;
            for _ in 0..sample {
                if (rng.below(10_000) as f64) < p * 10_000.0 {
                    hits += 1;
                }
            }
            let hits = (u64::from(hits) * u64::from(a.count) / u64::from(sample)) as u32;
            losses[t] += hits;
        }
        let before: Vec<u32> = self.units.iter().map(|u| u.count).collect();
        let str_a = self.strength(0);
        let str_b = self.strength(1);
        for i in 0..n {
            let u = &mut self.units[i];
            if before[i] == 0 {
                continue;
            }
            let lost = losses[i].min(u.count);
            u.count -= lost;
            if lost > 0 {
                self.log.push(format!(
                    "r{}: {} pierde {} ({} rămași)",
                    self.round, u.name, lost, u.count
                ));
            }
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            if before[i] == 0 || self.units[i].routed {
                continue;
            }
            let lost = before[i] - self.units[i].count;
            let pct = (u64::from(lost) * 100 / u64::from(before[i])) as i32;
            let (mine, theirs) = if self.units[i].side == 0 {
                (str_a, str_b)
            } else {
                (str_b, str_a)
            };
            let u = &mut self.units[i];
            if u.count == 0 {
                continue;
            }
            if pct >= 5 {
                u.morale -= 1 + pct / 10;
            } else if lost == 0 && u.morale < u.max_morale {
                u.morale += 1; // rallies when untouched
            }
            if mine * 2 < theirs {
                u.morale -= 1; // outnumbered 2:1
            }
            if u.morale <= 0 {
                u.routed = true;
                self.log.push(format!(
                    "r{}: {} se retrage în dezordine",
                    self.round, u.name
                ));
            }
        }
        true
    }

    pub fn run(&mut self, rng: &mut Rng) -> Outcome {
        while self.step(rng) {}
        let (a, b) = (self.strength(0), self.strength(1));
        let winner = match (a, b) {
            (0, 0) => None,
            (_, 0) => Some(0),
            (0, _) => Some(1),
            _ => None,
        };
        Outcome {
            rounds: self.round,
            winner,
        }
    }

    pub fn report(&self, o: &Outcome) -> String {
        let mut s = String::new();
        for l in &self.log {
            s += l;
            s.push('\n');
        }
        s += &format!("--- după {} runde ---\n", o.rounds);
        for u in &self.units {
            let state = if u.count == 0 {
                "distrusă"
            } else if u.routed {
                "în retragere"
            } else {
                "în luptă"
            };
            s += &format!(
                "{} [{}] {}: {}/{} soldați, moral {}/{}, {} ({})\n",
                self.side_names[u.side],
                u.id,
                u.name,
                u.count,
                u.start,
                u.morale.max(0),
                u.max_morale,
                state,
                u.terrain.name()
            );
        }
        s += &match o.winner {
            Some(w) => format!("Câștigă: {}\n", self.side_names[w]),
            None => "Egalitate / câmp nedecis\n".to_string(),
        };
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_helps_defender() {
        let a = Unit::new("a", 0, 10, 4, 3, 6);
        let mut d = Unit::new("d", 1, 10, 4, 3, 6);
        let open = Battle::kill_chance(&a, &d, false);
        d.terrain = Terrain::Fort;
        assert!(Battle::kill_chance(&a, &d, false) < open);
        assert!(Terrain::parse("Forest").is_some() && Terrain::parse("x").is_none());
    }
}
