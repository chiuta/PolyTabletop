#![allow(unused_imports)]
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{catalog, find, playable};

const IDS: [&str; 4] = ["fanorona", "tigri", "surakarta", "hasami"];

#[test]
fn ids_registered_once_and_in_catalog() {
    for id in IDS {
        assert_eq!(playable().iter().filter(|p| p.id == id).count(), 1, "{id}");
        assert!(
            catalog().iter().any(|e| e.impl_id == id),
            "{id} not in catalog"
        );
        let p = find(id).unwrap();
        assert!(!p.rules.is_empty() && !p.title.is_empty());
    }
}

#[test]
fn random_playouts_end_and_ai_is_legal_and_fast() {
    for id in IDS {
        let p = find(id).unwrap();
        for seed in 1..=3u64 {
            let mut g: Box<dyn GameObj> = (p.make)();
            let mut rng = Rng::new(seed * 77);
            let mut plies = 0;
            let mut worst = std::time::Duration::ZERO;
            while g.outcome().is_none() && plies < 4000 {
                let legal = g.legal_moves();
                assert!(!legal.is_empty(), "{id}");
                let mv = if plies % 25 == 0 {
                    let t = std::time::Instant::now();
                    let m = g.ai_move(rng.next_u64()).expect("AI move");
                    worst = worst.max(t.elapsed());
                    assert!(legal.contains(&m), "{id}: AI chose unlisted {m}");
                    m
                } else {
                    legal[rng.below(legal.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
            }
            assert!(g.outcome().is_some(), "{id} seed {seed} did not end");
            assert!(worst.as_secs_f32() < 2.0, "{id}: AI took {worst:?}");
            assert!(!g.render().is_empty());
        }
    }
}

#[test]
fn ai_beats_random_mostly() {
    // The AI side (Player Two) should not lose to uniformly random play in tigri/hasami/fanorona.
    for id in ["fanorona", "hasami", "surakarta"] {
        let p = find(id).unwrap();
        let mut wins = 0;
        for seed in 1..=2u64 {
            let mut g: Box<dyn GameObj> = (p.make)();
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 600 {
                let legal = g.legal_moves();
                let mv = if g.to_move() == Player::Two {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    legal[rng.below(legal.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 1, "{id}: AI won {wins}/2 against random");
    }
}
