//! `--level easy|normal|hard|expert` parsing (env `POLYTABLETOP_LEVEL` is the fallback,
//! read by `polytabletop_core::ai::level`).
use polytabletop_core::ai::{set_level, Level};

/// Parses `--level X` from the args and applies it globally. Returns an error
/// message for an invalid value.
pub fn apply(args: &[String]) -> Result<(), String> {
    if let Some(w) = args.windows(2).find(|w| w[0] == "--level") {
        let l = Level::parse(&w[1])
            .ok_or_else(|| format!("--level așteaptă easy|normal|hard|expert, nu «{}».", w[1]))?;
        set_level(l);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses() {
        assert!(apply(&["--level".into(), "hard".into()]).is_ok());
        assert_eq!(polytabletop_core::ai::level(), Level::Hard);
        assert!(apply(&["--level".into(), "zzz".into()]).is_err());
    }
}
