#![allow(unused_imports)]
//! Cross-cutting checks run against every playable game.
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{catalog, playable, Kind};

/// The four collectible-card engines: the AI answers in well under a second per move.
#[test]
fn tcg_engines_ai_is_fast_and_legal() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in ["tcg_evo", "tcg_zone", "tcg_lant", "tcg_cerneala"] {
        let p = playable().into_iter().find(|p| p.id == id).unwrap();
        let mut g = (p.make)();
        let mut rng = Rng::new(7);
        let mut worst = std::time::Duration::ZERO;
        let mut n = 0;
        while g.outcome().is_none() && n < 3000 {
            let t = std::time::Instant::now();
            let m = g.ai_move(rng.next_u64()).expect("AI has a move");
            worst = worst.max(t.elapsed());
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(g.outcome().is_some(), "{id} did not finish");
        assert!(worst.as_secs_f64() < 1.0, "{id}: AI took {worst:?}");
    }
}
