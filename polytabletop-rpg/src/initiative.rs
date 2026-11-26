//! Initiative / round tracker.

use crate::dice::Expr;
use polytabletop_core::rng::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub init: i32,
    /// Higher wins ties (e.g. a modifier, or 1 for player characters).
    pub tiebreak: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Tracker {
    entries: Vec<Entry>,
    current: usize,
    round: u32,
    started: bool,
}

impl Tracker {
    pub fn new() -> Tracker {
        Tracker::default()
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    pub fn round(&self) -> u32 {
        self.round
    }
    pub fn started(&self) -> bool {
        self.started
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn current(&self) -> Option<&Entry> {
        if self.started {
            self.entries.get(self.current)
        } else {
            None
        }
    }

    fn pos_for(&self, init: i32, tb: i32) -> usize {
        // after every entry that sorts before-or-equal (stable by insertion)
        self.entries
            .iter()
            .position(|e| (e.init, e.tiebreak) < (init, tb))
            .unwrap_or(self.entries.len())
    }

    pub fn add(&mut self, name: &str, init: i32, tiebreak: i32) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("nume gol".into());
        }
        if self.index_of(name).is_some() {
            return Err(format!("{name} există deja"));
        }
        let pos = self.pos_for(init, tiebreak);
        self.entries.insert(
            pos,
            Entry {
                name: name.to_string(),
                init,
                tiebreak,
            },
        );
        if self.started && pos <= self.current {
            self.current += 1;
        }
        Ok(())
    }

    /// Roll `expr + modifier` for the entry and add it; returns the result.
    pub fn roll_add(
        &mut self,
        name: &str,
        expr: &str,
        modifier: i32,
        rng: &mut Rng,
    ) -> Result<i32, String> {
        let e = Expr::parse(expr).map_err(|e| e.to_string())?;
        let v = e.roll(rng).total as i32 + modifier;
        self.add(name, v, modifier)?;
        Ok(v)
    }

    fn index_of(&self, name: &str) -> Option<usize> {
        self.entries
            .iter()
            .position(|e| e.name.to_lowercase() == name.trim().to_lowercase())
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.entries.is_empty() {
            return Err("nimeni în inițiativă".into());
        }
        self.started = true;
        self.current = 0;
        self.round = 1;
        Ok(())
    }

    /// Advance to the next actor. Returns `(actor, new_round)`.
    pub fn advance(&mut self) -> Result<(String, bool), String> {
        if !self.started {
            return Err("lupta nu a început (start)".into());
        }
        self.current += 1;
        let mut new_round = false;
        if self.current >= self.entries.len() {
            self.current = 0;
            self.round += 1;
            new_round = true;
        }
        Ok((self.entries[self.current].name.clone(), new_round))
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        let i = self
            .index_of(name)
            .ok_or_else(|| format!("necunoscut: {name}"))?;
        self.entries.remove(i);
        if self.entries.is_empty() {
            self.started = false;
            self.current = 0;
            self.round = 0;
            return Ok(());
        }
        if i < self.current {
            self.current -= 1;
        } else if self.started && self.current >= self.entries.len() {
            self.current = 0;
            self.round += 1;
        }
        Ok(())
    }

    /// Change an entry's initiative (delay / ready); the active actor stays active.
    pub fn set_init(&mut self, name: &str, init: i32) -> Result<(), String> {
        let i = self
            .index_of(name)
            .ok_or_else(|| format!("necunoscut: {name}"))?;
        let active = self.current().map(|e| e.name.clone());
        let mut e = self.entries.remove(i);
        e.init = init;
        let pos = self.pos_for(e.init, e.tiebreak);
        self.entries.insert(pos, e);
        if let Some(a) = active {
            self.current = self.index_of(&a).unwrap_or(0);
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Tracker::default();
    }

    pub fn render(&self) -> String {
        if self.entries.is_empty() {
            return "(inițiativă goală)\n".into();
        }
        let mut o = if self.started {
            format!("Runda {}\n", self.round)
        } else {
            "Înainte de luptă\n".into()
        };
        for (i, e) in self.entries.iter().enumerate() {
            let mark = if self.started && i == self.current {
                ">"
            } else {
                " "
            };
            o += &format!("{mark} {:>3}  {}\n", e.init, e.name);
        }
        o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t3() -> Tracker {
        let mut t = Tracker::new();
        t.add("A", 15, 0).unwrap();
        t.add("B", 20, 0).unwrap();
        t.add("C", 10, 0).unwrap();
        t
    }

    #[test]
    fn ordering_and_ties() {
        let mut t = t3();
        let names: Vec<_> = t.entries().iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["B", "A", "C"]);
        t.add("D", 15, 2).unwrap(); // wins tie by tiebreak
        t.add("E", 15, 0).unwrap(); // later insertion loses to earlier on full tie
        let names: Vec<_> = t.entries().iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["B", "D", "A", "E", "C"]);
        assert!(t.add("a", 1, 0).is_err());
        assert!(t.add(" ", 1, 0).is_err());
    }

    #[test]
    fn rounds_cycle() {
        let mut t = t3();
        assert!(t.advance().is_err());
        assert!(t.current().is_none());
        t.start().unwrap();
        assert_eq!(t.current().unwrap().name, "B");
        assert_eq!(t.advance().unwrap(), ("A".into(), false));
        assert_eq!(t.advance().unwrap(), ("C".into(), false));
        assert_eq!(t.advance().unwrap(), ("B".into(), true));
        assert_eq!(t.round(), 2);
    }

    #[test]
    fn removal_keeps_turn() {
        let mut t = t3();
        t.start().unwrap();
        t.advance().unwrap(); // A active
        t.remove("B").unwrap(); // before current
        assert_eq!(t.current().unwrap().name, "A");
        t.remove("A").unwrap(); // active removed -> next actor (C)
        assert_eq!(t.current().unwrap().name, "C");
        t.remove("C").unwrap();
        assert!(t.is_empty() && !t.started());
        assert!(t.remove("C").is_err());
    }

    #[test]
    fn removing_last_active_wraps() {
        let mut t = t3();
        t.start().unwrap();
        t.advance().unwrap();
        t.advance().unwrap(); // C (last) active
        t.remove("C").unwrap();
        assert_eq!(t.current().unwrap().name, "B");
        assert_eq!(t.round(), 2);
    }

    #[test]
    fn insert_during_combat() {
        let mut t = t3();
        t.start().unwrap();
        t.advance().unwrap(); // A active
        t.add("Z", 30, 0).unwrap(); // goes first, A stays active
        assert_eq!(t.current().unwrap().name, "A");
        t.add("Y", 1, 0).unwrap(); // after, no change
        assert_eq!(t.current().unwrap().name, "A");
    }

    #[test]
    fn delay_changes_order_not_actor() {
        let mut t = t3();
        t.start().unwrap();
        t.set_init("B", 5).unwrap();
        assert_eq!(t.current().unwrap().name, "B");
        let names: Vec<_> = t.entries().iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["A", "C", "B"]);
        assert!(t.set_init("Q", 1).is_err());
    }

    #[test]
    fn roll_add_in_range() {
        let mut t = Tracker::new();
        let mut rng = Rng::new(3);
        for i in 0..50 {
            let v = t.roll_add(&format!("n{i}"), "1d20", 2, &mut rng).unwrap();
            assert!((3..=22).contains(&v));
        }
        assert!(t.entries().windows(2).all(|w| w[0].init >= w[1].init));
        assert!(t.roll_add("x", "bad", 0, &mut rng).is_err());
        assert!(t.render().contains("n0"));
    }
}
