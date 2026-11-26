use polytabletop_games::playable;
use std::io::Write;
use std::process::{Command, Stdio};

fn run(id: &str, seed: u64) -> String {
    let mut c = Command::new(env!("CARGO_BIN_EXE_polytabletop"))
        .args(["play", id, "--seed", &seed.to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    c.stdin
        .take()
        .unwrap()
        .write_all(b"?\nhint\n?\nq\n")
        .unwrap();
    String::from_utf8_lossy(&c.wait_with_output().unwrap().stdout).into_owned()
}

#[test]
fn seeded_cli_runs_are_identical() {
    let ids: Vec<&str> = playable()
        .iter()
        .filter(|p| !matches!(p.category, "Abstract" | "Clasic"))
        .map(|p| p.id)
        .take(10)
        .collect();
    assert_eq!(ids.len(), 10);
    for id in ids {
        assert_eq!(run(id, 77), run(id, 77), "{id}");
    }
}
