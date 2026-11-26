use std::process::Command;

fn run(args: &[&str], env_lang: &str) -> (String, String, i32) {
    let o = Command::new(env!("CARGO_BIN_EXE_polytabletop"))
        .args(args)
        .env("POLYTABLETOP_LANG", env_lang)
        .env_remove("LC_ALL")
        .env_remove("LC_MESSAGES")
        .env_remove("LANG")
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&o.stdout).into_owned(),
        String::from_utf8_lossy(&o.stderr).into_owned(),
        o.status.code().unwrap_or(-1),
    )
}

#[test]
fn list_is_localized_with_flag() {
    let (de, _, _) = run(&["--lang", "de", "list"], "ro");
    assert!(de.contains("Schach"), "{}", &de[..200.min(de.len())]);
    let (ro, _, _) = run(&["list"], "ro");
    assert!(ro.contains("Șah"));
    // flag may come after the command, too
    let (hu, _, _) = run(&["list", "--lang", "hu"], "ro");
    assert!(hu.contains("Sakk"));
}

#[test]
fn catalog_total_is_localized() {
    let (de, _, _) = run(&["--lang", "de", "catalog"], "ro");
    assert!(de.contains("Gesamt"));
    assert!(de.contains("implementiert"));
    let (ro, _, _) = run(&["catalog"], "ro");
    assert!(ro.contains("Total ") && ro.contains("tradiționale"));
}

#[test]
fn env_language_and_flag_priority() {
    let (fr, _, _) = run(&["catalog"], "fr");
    assert!(fr.contains("traditionnels"));
    let (it, _, _) = run(&["--lang", "it", "catalog"], "fr");
    assert!(it.contains("tradizionali"));
}

#[test]
fn usage_and_langs() {
    let (u, _, _) = run(&["--lang", "de"], "ro");
    assert!(u.contains("Aufruf:") && u.contains("polytabletop langs"));
    let (l, _, _) = run(&["langs"], "pt");
    for n in [
        "English",
        "Română",
        "Français",
        "Italiano",
        "Español",
        "Português",
        "Deutsch",
        "Magyar",
    ] {
        assert!(l.contains(n), "{n}");
    }
}

#[test]
fn unknown_lang_errors() {
    let (_, e, c) = run(&["--lang", "xx", "list"], "ro");
    assert_eq!(c, 2);
    assert!(e.contains("xx") && e.contains("en, ro, fr"));
    let (_, e, c) = run(&["list", "--lang"], "ro");
    assert_eq!(c, 2);
    assert!(e.contains("--lang"));
}

#[test]
fn play_messages_localized() {
    let (_, e, c) = run(&["--lang", "es", "play", "nope"], "ro");
    assert_eq!(c, 2);
    assert!(e.contains("Juego desconocido"));
    let (_, e, _) = run(&["play", "nope"], "ro");
    assert!(e.contains("Joc necunoscut: nope. Încearcă `polytabletop list`."));
}
