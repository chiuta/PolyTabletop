use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Player};
use polytabletop_games::playable;

/// AI of the two x2 games always answers with a legal move, quickly, and games end.
#[test]
fn x2_ai_moves_are_legal_and_fast() {
    for id in ["codru", "sideral"] {
        let p = playable().into_iter().find(|p| p.id == id).unwrap();
        for seed in 1..=4u64 {
            let mut g: Box<dyn GameObj> = (p.make)();
            let mut rng = Rng::new(seed * 31);
            let mut plies = 0;
            let mut worst = std::time::Duration::ZERO;
            while g.outcome().is_none() && plies < 6000 {
                let legal = g.legal_moves();
                assert!(!legal.is_empty(), "{id}");
                let t = std::time::Instant::now();
                let mv = if g.to_move() == Player::Two || plies % 3 == 0 {
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
            assert!(!g.render().is_empty());
        }
    }
}
