//! Salvarea/încărcarea jurnalului de mutări.
//!
//! Reluarea (replay) este posibilă doar pentru jocurile deterministe: aceeași
//! secvență de mutări text, jucată pe un joc nou, dă aceeași poziție. Pentru
//! celelalte (cărți amestecate, zaruri) fișierul este doar export text.

use crate::gui_i18n::tr;

pub const MAGIC: &str = "POLYTABLETOP-JURNAL v1";

/// FNV-1a pe 64 de biți (stabil între versiuni de Rust).
pub fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Journal {
    pub game_id: String,
    /// "1", "2" sau "ambele"
    pub human: String,
    pub replay: bool,
    pub check: u64,
    pub moves: Vec<String>,
    pub log: Vec<String>,
}

impl Journal {
    pub fn to_text(&self) -> String {
        let mut s = String::new();
        s.push_str(MAGIC);
        s.push('\n');
        s.push_str(&format!("joc: {}\n", self.game_id));
        s.push_str(&format!("om: {}\n", self.human));
        s.push_str(&format!(
            "replay: {}\n",
            if self.replay { "da" } else { "nu" }
        ));
        s.push_str(&format!("verificare: {:016x}\n", self.check));
        s.push_str("mutari:\n");
        for m in &self.moves {
            s.push_str(m);
            s.push('\n');
        }
        s.push_str("---\n");
        for l in &self.log {
            s.push_str("# ");
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    pub fn parse(text: &str) -> Result<Journal, String> {
        let mut lines = text.lines();
        if lines.next().map(str::trim) != Some(MAGIC) {
            return Err(tr("j_bad_magic").into());
        }
        let (mut game_id, mut human, mut replay, mut check) = (None, None, false, 0u64);
        let mut moves = Vec::new();
        let mut log = Vec::new();
        let mut section = 0; // 0 antet, 1 mutări, 2 jurnal
        for l in lines {
            match section {
                0 => {
                    if l.trim() == "mutari:" {
                        section = 1;
                    } else if let Some((k, v)) = l.split_once(':') {
                        let v = v.trim();
                        match k.trim() {
                            "joc" => game_id = Some(v.to_string()),
                            "om" => human = Some(v.to_string()),
                            "replay" => replay = v == "da",
                            "verificare" => {
                                check = u64::from_str_radix(v, 16).map_err(|_| tr("j_bad_check"))?
                            }
                            _ => {}
                        }
                    }
                }
                1 => {
                    if l.trim() == "---" {
                        section = 2;
                    } else if !l.trim().is_empty() {
                        moves.push(l.trim().to_string());
                    }
                }
                _ => {
                    log.push(l.strip_prefix("# ").unwrap_or(l).to_string());
                }
            }
        }
        if section == 0 {
            return Err(tr("j_incomplete").into());
        }
        Ok(Journal {
            game_id: game_id.ok_or(tr("j_missing_game"))?,
            human: human.unwrap_or_else(|| "1".into()),
            replay,
            check,
            moves,
            log,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let j = Journal {
            game_id: "chess".into(),
            human: "1".into(),
            replay: true,
            check: fnv("abc"),
            moves: vec!["e2e4".into(), "e7e5".into()],
            log: vec![
                "Jucător 1: e2e4".into(),
                "Jucător 2 (calculator): e7e5".into(),
            ],
        };
        let t = j.to_text();
        assert_eq!(Journal::parse(&t).unwrap(), j);
    }

    #[test]
    fn rejects_garbage() {
        assert!(Journal::parse("hello").is_err());
        assert!(Journal::parse(&format!("{MAGIC}\njoc: x\n")).is_err());
    }

    #[test]
    fn fnv_known() {
        assert_eq!(fnv(""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv("a"), 0xaf63_dc4c_8601_ec8c);
    }
}
