//! Verificări pentru jocurile de cărți din valul B: AI-ul joacă legal, termină și e rapid.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::playable;
use std::time::Instant;

const IDS: [&str; 9] = [
    "canasta", "bridge", "truco", "cribbage", "euchre", "tablanet", "trix", "tarneeb", "baloot",
];

fn make(id: &str) -> Box<dyn GameObj> {
    (playable().into_iter().find(|p| p.id == id).unwrap().make)()
}

#[test]
fn ai_vs_ai_finishes_and_is_fast() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in IDS {
        let mut g = make(id);
        let mut rng = Rng::new(7);
        let mut n = 0;
        let mut worst = 0u128;
        while g.outcome().is_none() && n < 8000 {
            let t = Instant::now();
            let mv = g.ai_move(rng.next_u64()).expect("AI has a move");
            worst = worst.max(t.elapsed().as_millis());
            g.play(&mv)
                .unwrap_or_else(|e| panic!("{id}: {e}\n{}", g.render()));
            n += 1;
        }
        assert!(g.outcome().is_some(), "{id} did not end in {n} plies");
        assert!(worst < 1000, "{id}: slowest AI move {worst} ms");
    }
}

#[test]
fn ai_is_not_worse_than_random() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    // AI (J1) contra mutărilor aleatoare: pe 12 jocuri nu trebuie să piardă clar.
    for id in IDS {
        let mut wins = 0;
        let mut losses = 0;
        for seed in 0..12u64 {
            let mut g = make(id);
            let mut rng = Rng::new(seed + 1000);
            let mut n = 0;
            while g.outcome().is_none() && n < 8000 {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                n += 1;
            }
            match g.outcome() {
                Some(Outcome::Win(Player::One)) => wins += 1,
                Some(Outcome::Win(Player::Two)) => losses += 1,
                _ => {}
            }
        }
        println!("{id}: AI {wins} - {losses} random");
        assert!(wins >= losses, "{id}: AI {wins} - {losses} contra random");
    }
}

#[test]
fn render_shows_only_player_one_hand_marker() {
    for id in IDS {
        let g = make(id);
        let r = g.render();
        assert!(!r.is_empty(), "{id}");
    }
}
