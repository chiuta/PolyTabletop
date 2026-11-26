//! Cross-cutting checks run against every playable game.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{catalog, playable, Kind};

fn play_random(mut g: Box<dyn GameObj>, seed: u64, cap: usize) -> (Box<dyn GameObj>, usize) {
    let mut rng = Rng::new(seed);
    let mut plies = 0;
    while g.outcome().is_none() && plies < cap {
        let moves = g.legal_moves();
        assert!(
            !moves.is_empty(),
            "no legal moves but game not over\n{}",
            g.render()
        );
        // every move text is unique
        let mut sorted = moves.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            moves.len(),
            "duplicate move text\n{}",
            g.render()
        );
        let pick = &moves[rng.below(moves.len())];
        g.play(pick).expect("a listed legal move must be playable");
        plies += 1;
    }
    (g, plies)
}

#[test]
fn random_games_finish_cleanly() {
    for p in playable() {
        for seed in 1..=6u64 {
            let (g, plies) = play_random((p.make)(), seed, 4000);
            if p.id != "chess" && p.id != "draughts" {
                assert!(
                    g.outcome().is_some(),
                    "{} did not end in {plies} plies",
                    p.id
                );
            }
            assert!(!g.render().is_empty());
        }
    }
}

#[test]
fn play_rejects_garbage() {
    for p in playable() {
        let mut g = (p.make)();
        assert!(g.play("zzzz").is_err(), "{}", p.id);
        assert!(g.play("").is_err(), "{}", p.id);
    }
}

#[test]
fn play_is_case_and_space_insensitive() {
    for p in playable() {
        let mut g = (p.make)();
        let first = g.legal_moves()[0].clone();
        let shouty = format!("  {}  ", first.to_uppercase());
        assert_eq!(g.play(&shouty).unwrap(), first, "{}", p.id);
    }
}

/// The AI (playing One) must never lose to a random mover.
#[test]
fn ai_beats_or_ties_random() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in [
        "connect4",
        "reversi",
        "mancala",
        "morris",
        "gomoku",
        "draughts",
        "xiangqi",
        "shogi",
        "dara",
        "morabaraba",
        "sungka",
        "makruk",
        "janggi",
    ] {
        let p = playable().into_iter().find(|p| p.id == id).unwrap();
        let mut g = (p.make)();
        let mut rng = Rng::new(99);
        let mut plies = 0;
        while g.outcome().is_none() && plies < 1500 {
            let mv = if g.to_move() == Player::One {
                g.ai_move(rng.next_u64()).expect("AI has a move")
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            plies += 1;
        }
        assert_ne!(
            g.outcome(),
            Some(Outcome::Win(Player::Two)),
            "AI lost to a random mover in {id}\n{}",
            g.render()
        );
    }
}

#[test]
fn chess_ai_plays_legal_games() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    let mut g: Box<dyn GameObj> = Box::new(polytabletop_games::chess::Chess::new().with_depth(2));
    let mut rng = Rng::new(5);
    for _ in 0..80 {
        if g.outcome().is_some() {
            break;
        }
        let m = g.ai_move(rng.next_u64()).unwrap();
        g.play(&m).unwrap();
    }
}

#[test]
fn go_mcts_finishes_a_game() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    let mut g: Box<dyn GameObj> = Box::new(polytabletop_games::go::Go::new().with_iterations(60));
    let mut rng = Rng::new(11);
    let mut n = 0;
    while g.outcome().is_none() && n < 400 {
        let m = g.ai_move(rng.next_u64()).unwrap();
        g.play(&m).unwrap();
        n += 1;
    }
    assert!(g.outcome().is_some());
}

#[test]
fn catalog_is_consistent() {
    let c = catalog();
    assert!(c.len() > 150);
    let mut names: Vec<&str> = c.iter().map(|e| e.name).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), c.len(), "duplicate catalog names");
    let play_ids: Vec<&str> = playable().iter().map(|p| p.id).collect();
    for e in c {
        if e.kind == Kind::Implemented {
            assert!(play_ids.contains(&e.impl_id), "{} -> {}", e.name, e.impl_id);
        } else {
            assert!(e.impl_id.is_empty());
        }
    }
    for id in &play_ids {
        assert!(
            c.iter().any(|e| e.impl_id == *id),
            "{id} missing from catalog"
        );
    }
}
