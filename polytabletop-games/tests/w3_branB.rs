#![allow(unused_imports)]
//! Cross-cutting checks run against every playable game.
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{catalog, playable, Kind};

/// Wave-3 (branch B) originals: the AI always returns a legal move, quickly, and games end.
#[test]
fn branb_ai_moves_are_legal_and_fast() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in [
        "arhipelag",
        "roata",
        "zaruri5",
        "placute",
        "turnul",
        "cunoastere",
        "ceata",
        "umbra",
        "zidurile",
        "prajituri",
        "portret",
    ] {
        let p = playable().into_iter().find(|p| p.id == id).unwrap();
        for seed in 1..=3u64 {
            let mut g = (p.make)();
            let mut rng = Rng::new(seed * 77);
            let mut plies = 0;
            let mut worst = std::time::Duration::ZERO;
            while g.outcome().is_none() && plies < 3000 {
                let legal = g.legal_moves();
                assert!(!legal.is_empty(), "{id}");
                let t = std::time::Instant::now();
                let mv = if plies % 2 == 0 || g.to_move() == Player::Two {
                    g.ai_move(rng.next_u64()).expect("AI has a move")
                } else {
                    legal[rng.below(legal.len())].clone()
                };
                worst = worst.max(t.elapsed());
                assert!(legal.contains(&mv), "{id}: AI chose an unlisted move {mv}");
                g.play(&mv).unwrap();
                plies += 1;
            }
            assert!(g.outcome().is_some(), "{id} did not end in {plies} plies");
            assert!(worst.as_secs_f32() < 1.0, "{id}: AI took {worst:?}");
        }
    }
}
