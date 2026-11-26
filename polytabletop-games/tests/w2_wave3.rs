//! Controale pentru jocurile-variantă ale valului 3 (copia w2): AI-ul rapid și mutări curate.

use polytabletop_core::rng::Rng;
use polytabletop_games::find;
use std::time::Instant;

const IDS: [&str; 7] = [
    "epoci",
    "solzi",
    "catun",
    "tinuturi",
    "poieni",
    "salina",
    "reverdire",
];

#[test]
fn ai_moves_are_fast_and_games_finish() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in IDS {
        let p = find(id).unwrap_or_else(|| panic!("{id} lipsește"));
        for seed in 1..=2u64 {
            let mut g = (p.make)();
            let mut rng = Rng::new(seed * 31);
            let mut worst = 0.0f64;
            let mut plies = 0;
            while g.outcome().is_none() && plies < 4000 {
                let mv = if g.to_move() == polytabletop_core::Player::One {
                    let t = Instant::now();
                    let m = g.ai_move(rng.next_u64()).expect("mutare AI");
                    worst = worst.max(t.elapsed().as_secs_f64());
                    m
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())].clone()
                };
                g.play(&mv).expect("mutare legală");
                plies += 1;
            }
            assert!(g.outcome().is_some(), "{id} nu s-a terminat");
            assert!(worst < 1.0, "{id}: mutare AI de {worst:.2}s");
        }
    }
}
