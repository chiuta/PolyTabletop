#![allow(unused_imports)]
//! Global-seed reproducibility: same seed -> same deal and same random-legal-move transcript.
use polytabletop_core::rng::Rng;
use polytabletop_games::playable;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

fn transcript(p: &polytabletop_games::Playable, seed: u64) -> String {
    Rng::set_global_seed(Some(seed));
    let mut g = (p.make)();
    let mut out = g.render();
    let mut pick = Rng::new(seed ^ 0xA5A5);
    for _ in 0..30 {
        if g.outcome().is_some() {
            break;
        }
        let moves = g.legal_moves();
        if moves.is_empty() {
            break;
        }
        let m = moves[pick.below(moves.len())].clone();
        let _ = g.play(&m);
        out.push_str(&format!("\n>> {m}\n{}", g.render()));
    }
    out
}

#[test]
fn same_global_seed_same_game_for_all_playables() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut varied = 0;
    let mut bad = vec![];
    for p in playable() {
        let a = transcript(&p, 12345);
        let b = transcript(&p, 12345);
        if a != b {
            bad.push(p.id);
        }
        if a != transcript(&p, 999) {
            varied += 1;
        }
    }
    Rng::set_global_seed(None);
    assert!(bad.is_empty(), "non-reproducible under seed: {bad:?}");
    assert!(varied > 10, "different seeds should change some games");
}

#[test]
fn global_seed_reset_restores_clock() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    Rng::set_global_seed(Some(1));
    let mut a = Rng::from_clock();
    Rng::set_global_seed(Some(1));
    let mut b = Rng::from_clock();
    assert_eq!(a.next_u64(), b.next_u64());
    let mut c = Rng::from_clock();
    assert_ne!(a.next_u64(), c.next_u64());
    Rng::set_global_seed(None);
}
