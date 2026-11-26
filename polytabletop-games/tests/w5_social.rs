//! Jocurile cu masă simulată (roluri ascunse / multi-jucător): Player::Two = masa.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{apus, consiliul, fulger, hruba, lupii, roade, santinela, valea};
use std::time::Instant;

type Maker = fn(u64) -> Box<dyn GameObj>;

fn makers() -> Vec<(&'static str, Maker)> {
    vec![
        ("lupii", |s| Box::new(lupii::Lupii::with_seed(s))),
        ("santinela", |s| Box::new(santinela::Farul::with_seed(s))),
        ("consiliul", |s| {
            Box::new(consiliul::Consiliul::with_seed(s))
        }),
        ("valea", |s| Box::new(valea::Valea::with_seed(s))),
        ("hruba", |s| Box::new(hruba::Hruba::with_seed(s))),
        ("apus", |s| Box::new(apus::Apus::with_seed(s))),
        ("fulger", |s| Box::new(fulger::Fulger::with_seed(s))),
        ("roade", |s| Box::new(roade::Roade::with_seed(s))),
    ]
}

#[test]
fn table_moves_come_from_the_ai_and_are_fast() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for (name, mk) in makers() {
        for seed in 1..=12u64 {
            let mut g = mk(seed);
            let mut rng = Rng::new(seed * 7);
            let mut plies = 0;
            let mut worst = 0u128;
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::Two {
                    let t = Instant::now();
                    let m = g.ai_move(rng.next_u64()).expect("table has a move");
                    worst = worst.max(t.elapsed().as_millis());
                    m
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())].clone()
                };
                g.play(&mv).unwrap_or_else(|e| panic!("{name}: {e} ({mv})"));
                plies += 1;
                assert!(plies < 6000, "{name} too long");
            }
            assert!(worst < 1000, "{name}: ai move took {worst} ms");
            assert!(matches!(
                g.outcome(),
                Some(Outcome::Win(_)) | Some(Outcome::Draw)
            ));
        }
    }
}

#[test]
fn render_is_stable_and_never_empty() {
    for (name, mk) in makers() {
        let g = mk(5);
        let r = g.render();
        assert!(!r.is_empty(), "{name}");
        assert!(!g.legal_moves().is_empty(), "{name}");
    }
}

#[test]
fn hidden_roles_do_not_leak_in_start_render() {
    // la început, randarea nu conține tabelul final de roluri
    for (name, mk) in makers() {
        for seed in 1..=20u64 {
            let r = mk(seed).render();
            assert!(!r.contains("Roluri:"), "{name} leaks the role table");
        }
    }
}
