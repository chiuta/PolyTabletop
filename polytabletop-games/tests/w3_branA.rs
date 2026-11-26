#![allow(unused_imports)]
//! Cross-cutting checks run against every playable game.
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{catalog, playable, Kind};

/// Jocurile originale cu zaruri/cărți/hidden info: AI-ul răspunde repede și
/// poate duce un joc întreg împotriva unui jucător aleatoriu.
#[test]
fn bra_ai_games_are_fast_and_legal() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in [
        "culori",
        "mosii",
        "magistrale",
        "conac",
        "insula",
        "mozaic",
        "litere",
    ] {
        let p = playable().into_iter().find(|p| p.id == id).unwrap();
        let mut g = (p.make)();
        let mut rng = Rng::new(7);
        let mut slowest = std::time::Duration::ZERO;
        let mut plies = 0;
        while g.outcome().is_none() && plies < 3000 {
            let mv = if g.to_move() == Player::One {
                let t = std::time::Instant::now();
                let m = g.ai_move(rng.next_u64()).expect("AI has a move");
                slowest = slowest.max(t.elapsed());
                m
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            plies += 1;
        }
        assert!(g.outcome().is_some(), "{id} did not end");
        if !cfg!(debug_assertions) {
            assert!(slowest.as_secs_f64() < 1.0, "{id}: AI too slow {slowest:?}");
        }
    }
}
