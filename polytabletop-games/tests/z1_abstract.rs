#![allow(unused_imports)]
//! Cross-checks for the four z1 abstract games: hex, amazoane, linii, nim.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::playable;

const IDS: [&str; 4] = ["hex", "amazoane", "linii", "nim"];

fn make(id: &str) -> Box<dyn GameObj> {
    (playable().into_iter().find(|p| p.id == id).expect(id).make)()
}

#[test]
fn ids_are_registered_once() {
    for id in IDS {
        assert_eq!(playable().iter().filter(|p| p.id == id).count(), 1, "{id}");
    }
}

#[test]
fn random_games_end_with_unique_move_text() {
    for id in IDS {
        for seed in 1..=5u64 {
            let mut g = make(id);
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{id}: no moves yet not over");
                let mut s = mv.clone();
                s.sort();
                s.dedup();
                assert_eq!(s.len(), mv.len(), "{id}: duplicate move text");
                g.play(&mv[rng.below(mv.len())]).unwrap();
                plies += 1;
                assert!(plies < 1000, "{id} too long");
            }
            assert!(g.legal_moves().is_empty(), "{id}");
        }
    }
}

#[test]
fn ai_does_not_lose_to_random() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(15));
    for id in IDS {
        let mut g = make(id);
        let mut rng = Rng::new(42);
        while g.outcome().is_none() {
            let mv = if g.to_move() == Player::One {
                g.ai_move(rng.next_u64()).expect("AI move")
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
        }
        assert_ne!(
            g.outcome(),
            Some(Outcome::Win(Player::Two)),
            "{id}\n{}",
            g.render()
        );
    }
}
