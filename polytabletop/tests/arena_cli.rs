use std::process::Command;

fn run(args: &[&str]) -> (bool, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_polytabletop"))
        .args(args)
        .output()
        .unwrap();
    (
        o.status.success(),
        String::from_utf8_lossy(&o.stdout).into_owned(),
    )
}

#[test]
fn arena_two_seats_reports_stats() {
    let (ok, out) = run(&[
        "arena",
        "connect4",
        "--games",
        "4",
        "--ms",
        "5:10",
        "--threads",
        "2",
        "--seed",
        "3",
    ]);
    assert!(ok, "{out}");
    assert!(out.contains("4 partide"), "{out}");
    assert!(out.contains("victorii"), "{out}");
    assert!(out.contains("timp mediu/mutare"), "{out}");
    assert!(out.contains("cea mai lentă mutare"), "{out}");
}

#[test]
fn arena_is_deterministic_for_one_thread() {
    let a = [
        "arena",
        "belote",
        "--games",
        "4",
        "--ms",
        "5:5",
        "--threads",
        "1",
        "--seed",
        "9",
    ];
    let line = |o: String| o.lines().nth(1).unwrap().to_string();
    assert_eq!(line(run(&a).1), line(run(&a).1));
}

#[test]
fn arena_multi_seat_budgets() {
    let (ok, out) = run(&[
        "arena",
        "hruba",
        "--games",
        "2",
        "--ms",
        "5:5:5",
        "--threads",
        "1",
        "--max-plies",
        "150",
    ]);
    assert!(ok, "{out}");
    assert!(out.contains("loc 3"), "{out}");
}

#[test]
fn arena_levels_and_errors() {
    let (ok, out) = run(&["arena", "connect4", "--games", "1", "--level", "easy:easy"]);
    assert!(ok, "{out}");
    assert!(!run(&["arena", "nu-exista"]).0);
    assert!(!run(&["arena", "connect4", "--level", "foarte"]).0);
}

#[test]
fn bench_subset_and_ladder() {
    let (ok, out) = run(&["bench", "--only", "connect4,hex"]);
    assert!(ok, "{out}");
    assert!(out.contains("connect4") && out.contains("hex"), "{out}");
    assert!(out.contains("buget AI 20 ms"), "{out}");
    let (ok, out) = run(&["ladder", "connect4", "--games", "2", "--threads", "2"]);
    assert!(ok, "{out}");
    assert!(out.contains("320 ms") && out.contains("Elo"), "{out}");
}
