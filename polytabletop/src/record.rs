//! Game transcripts: `polytabletop play ID --record FILE`.
//!
//! File format (date-free, plain text, one move per line after `---`):
//!
//! ```text
//! PolyTabletop-RECORD 1
//! game: chess
//! seed: 42
//! level: 3          (optional)
//! you: one
//! result: win1      (win1 | win2 | draw | none)
//! moves: 2
//! ---
//! e2e4
//! e7e5
//! ```

use polytabletop_core::{GameObj, Outcome, Player};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub game: String,
    pub seed: u64,
    pub level: Option<String>,
    /// The `--you` setting of the session (affects the hot-seat preparation and viewer).
    pub you: String,
    pub moves: Vec<String>,
    /// `win1`, `win2`, `draw` or `none` (unfinished).
    pub result: String,
}

pub fn outcome_str(o: Option<Outcome>) -> &'static str {
    match o {
        Some(Outcome::Win(Player::One)) => "win1",
        Some(Outcome::Win(Player::Two)) => "win2",
        Some(Outcome::Draw) => "draw",
        None => "none",
    }
}

/// Seed from the clock, for recording a game that was started without `--seed`.
pub fn fresh_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64 ^ ((std::process::id() as u64) << 32))
        .unwrap_or(0xC0FFEE)
        % 1_000_000_000_000
}

impl Record {
    pub fn new(game: &str, seed: u64, you: &str) -> Record {
        Record {
            game: game.to_string(),
            seed,
            level: None,
            you: you.to_string(),
            moves: Vec::new(),
            result: "none".into(),
        }
    }

    pub fn to_text(&self) -> String {
        let mut s = format!(
            "PolyTabletop-RECORD 1\ngame: {}\nseed: {}\n",
            self.game, self.seed
        );
        if let Some(l) = &self.level {
            s.push_str(&format!("level: {l}\n"));
        }
        s.push_str(&format!(
            "you: {}\nresult: {}\nmoves: {}\n---\n",
            self.you,
            self.result,
            self.moves.len()
        ));
        for m in &self.moves {
            s.push_str(m);
            s.push('\n');
        }
        s
    }

    pub fn parse(text: &str) -> Result<Record, String> {
        let mut lines = text.lines();
        if lines.next().map(str::trim) != Some("PolyTabletop-RECORD 1") {
            return Err(
                "nu e o transcriere PolyTabletop (lipsește antetul «PolyTabletop-RECORD 1»)".into(),
            );
        }
        let mut rec = Record::new("", 0, "one");
        let (mut have_game, mut have_seed, mut declared) = (false, false, None);
        let mut in_body = false;
        for line in lines {
            if in_body {
                let t = line.trim();
                if !t.is_empty() {
                    rec.moves.push(t.to_string());
                }
                continue;
            }
            if line.trim() == "---" {
                in_body = true;
                continue;
            }
            let Some((k, v)) = line.split_once(':') else {
                continue;
            };
            let v = v.trim();
            match k.trim() {
                "game" => {
                    rec.game = v.to_string();
                    have_game = true;
                }
                "seed" => {
                    rec.seed = v.parse().map_err(|_| format!("seed invalid: «{v}»"))?;
                    have_seed = true;
                }
                "level" => rec.level = Some(v.to_string()),
                "you" => rec.you = v.to_string(),
                "result" => rec.result = v.to_string(),
                "moves" => declared = v.parse::<usize>().ok(),
                _ => {}
            }
        }
        if !have_game || !have_seed || !in_body {
            return Err("antet incomplet (game/seed/---)".into());
        }
        if let Some(n) = declared {
            if n != rec.moves.len() {
                return Err(format!(
                    "antetul declară {n} mutări, fișierul are {}",
                    rec.moves.len()
                ));
            }
        }
        Ok(rec)
    }
}

/// Writes the transcript after every move, so a quit or crash loses nothing.
pub struct Recorder {
    path: PathBuf,
    pub rec: Record,
}

impl Recorder {
    pub fn new(path: &str, rec: Record) -> Recorder {
        let r = Recorder {
            path: PathBuf::from(path),
            rec,
        };
        r.flush();
        r
    }

    /// Call after `game` has played `mv` (canonical text).
    pub fn push(&mut self, game: &dyn GameObj, mv: &str) {
        self.rec.moves.push(mv.to_string());
        self.rec.result = outcome_str(game.outcome()).to_string();
        self.flush();
    }

    fn flush(&self) {
        if let Err(e) = std::fs::write(&self.path, self.rec.to_text()) {
            eprintln!("Nu pot scrie transcrierea {}: {e}", self.path.display());
        }
    }
}

#[cfg(test)]
pub static SEED_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_round_trip() {
        let mut r = Record::new("chess", 7, "one");
        r.level = Some("2".into());
        r.moves = vec!["e2e4".into(), "e7e5".into()];
        r.result = "none".into();
        assert_eq!(Record::parse(&r.to_text()).unwrap(), r);
    }

    #[test]
    fn rejects_bad_files() {
        assert!(Record::parse("hello").is_err());
        let t = "PolyTabletop-RECORD 1\ngame: chess\nseed: 1\nmoves: 3\n---\ne2e4\n";
        assert!(Record::parse(t).is_err());
    }
}
