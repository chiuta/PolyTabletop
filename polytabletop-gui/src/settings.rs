//! Setări persistente minime (doar limba): `gui.conf`, linii `cheie=valoare`.
//! Cale: `$XDG_CONFIG_HOME/polytabletop/gui.conf`, altfel `~/.config/polytabletop/gui.conf`.
//! Orice eroare de citire/scriere este ignorată (fallback tăcut). Fără dependențe noi.

use crate::gui_i18n::Lang;
use std::path::{Path, PathBuf};

pub fn conf_path() -> Option<PathBuf> {
    conf_path_from(
        std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
        std::env::var_os("HOME").map(PathBuf::from),
    )
}

pub fn conf_path_from(xdg: Option<PathBuf>, home: Option<PathBuf>) -> Option<PathBuf> {
    let base = xdg
        .filter(|p| p.is_absolute())
        .or_else(|| home.map(|h| h.join(".config")))?;
    Some(base.join("polytabletop").join("gui.conf"))
}

pub fn parse_lang(text: &str) -> Option<Lang> {
    text.lines().find_map(|l| {
        let (k, v) = l.split_once('=')?;
        if k.trim() == "lang" {
            Lang::parse(v)
        } else {
            None
        }
    })
}

pub fn load_lang_from(path: &Path) -> Option<Lang> {
    parse_lang(&std::fs::read_to_string(path).ok()?)
}

pub fn save_lang_to(path: &Path, l: Lang) -> std::io::Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    std::fs::write(path, format!("lang={}\n", l.code()))
}

/// Limba salvată (dacă există și e validă).
pub fn load_lang() -> Option<Lang> {
    load_lang_from(&conf_path()?)
}

/// Salvează limba; erorile se ignoră.
pub fn save_lang(l: Lang) {
    if let Some(p) = conf_path() {
        let _ = save_lang_to(&p, l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        let h = Some(PathBuf::from("/home/u"));
        assert_eq!(
            conf_path_from(Some("/x/cfg".into()), h.clone()),
            Some("/x/cfg/polytabletop/gui.conf".into())
        );
        // XDG relativ sau lipsă → ~/.config
        for x in [None, Some(PathBuf::from("rel")), Some(PathBuf::new())] {
            assert_eq!(
                conf_path_from(x, h.clone()),
                Some("/home/u/.config/polytabletop/gui.conf".into())
            );
        }
        assert_eq!(conf_path_from(None, None), None);
    }

    #[test]
    fn parse_and_roundtrip() {
        assert_eq!(parse_lang("x=1\n lang = hu \n"), Some(Lang::Hu));
        assert_eq!(parse_lang("lang=xx"), None);
        assert_eq!(parse_lang(""), None);
        let d = std::env::temp_dir().join(format!("pt-gui-conf-{}", std::process::id()));
        let p = d.join("sub").join("gui.conf");
        for l in Lang::ALL {
            save_lang_to(&p, l).unwrap();
            assert_eq!(load_lang_from(&p), Some(l));
        }
        let _ = std::fs::remove_dir_all(&d);
        // cale imposibilă: eroare, nu panică
        assert!(save_lang_to(Path::new("/proc/nu/exista/gui.conf"), Lang::Ro).is_err());
        assert_eq!(load_lang_from(Path::new("/nu/exista")), None);
    }
}
