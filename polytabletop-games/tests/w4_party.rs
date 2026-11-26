//! Jocuri de petrecere / cuvinte (wave 3, copia w4): AI vs AI, AI vs aleator, viteză.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{dictionar, find, oprite, schita, treicai};
use std::time::Instant;

const IDS: [&str; 9] = [
    "spionii",
    "povestitor",
    "gura",
    "treicai",
    "oprite",
    "schita",
    "dictionar",
    "impostor",
    "cuvantunic",
];

fn play(mut g: Box<dyn GameObj>, seed: u64, ai_one: bool, ai_two: bool) -> Option<Outcome> {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    let mut rng = Rng::new(seed);
    let mut n = 0;
    while g.outcome().is_none() && n < 6000 {
        let p = g.to_move();
        let ai = (p == Player::One && ai_one) || (p == Player::Two && ai_two);
        let m = if ai {
            let t = Instant::now();
            let m = g.ai_move(rng.next_u64()).expect("AI has a move");
            assert!(t.elapsed().as_millis() < 1000, "AI prea lent");
            m
        } else {
            let mv = g.legal_moves();
            mv[rng.below(mv.len())].clone()
        };
        g.play(&m).unwrap();
        n += 1;
    }
    g.outcome()
}

#[test]
fn ai_vs_ai_finishes_everywhere() {
    for id in IDS {
        for seed in 1..=6u64 {
            let g = (find(id).unwrap().make)();
            assert!(play(g, seed, true, true).is_some(), "{id} seed {seed}");
        }
    }
}

#[test]
fn ai_is_not_worse_than_random() {
    for id in ["spionii", "schita", "treicai", "oprite", "dictionar"] {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=30u64 {
            let g = (find(id).unwrap().make)();
            match play(g, seed, true, false) {
                Some(Outcome::Win(Player::One)) => ai += 1,
                Some(Outcome::Win(Player::Two)) => rnd += 1,
                _ => {}
            }
        }
        println!("{id}: AI {ai} – aleator {rnd}");
        assert!(ai >= rnd, "{id}: AI {ai} – aleator {rnd}");
    }
}

#[test]
fn describe_games_are_lively() {
    use polytabletop_core::{ai, Game};
    // statistici de echilibru: AI vs AI, punctele totale pe rundă/tură
    fn run<G: Game>(mut g: G, seed: u64, sc: impl Fn(&G) -> (i32, i32)) -> (i32, i32) {
        let mut n = 0;
        while g.outcome().is_none() && n < 5000 {
            let m = ai::best_move(&g, seed + n as u64).unwrap();
            g.apply(&m);
            n += 1;
        }
        sc(&g)
    }
    for name in ["schita", "treicai", "oprite", "dictionar"] {
        let mut tot = (0i64, 0i64);
        let seeds = 30u64;
        for seed in 1..=seeds {
            let g = match name {
                "schita" => schita::new(),
                "treicai" => treicai::new(),
                "oprite" => oprite::new(),
                _ => dictionar::new(),
            };
            let (a, b) = run(g, seed, |g| g.scores());
            tot.0 += a as i64;
            tot.1 += b as i64;
        }
        println!(
            "{name}: media punctelor J1 {:.1}, J2 {:.1}",
            tot.0 as f64 / seeds as f64,
            tot.1 as f64 / seeds as f64
        );
        assert!(
            tot.0 + tot.1 > 2 * seeds as i64,
            "{name}: joc prea sărac în puncte"
        );
    }
}

/// Afișează stări reprezentative (rulează cu `--ignored --nocapture`).
#[test]
#[ignore]
fn show_renders() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for id in IDS {
        let mut g = (find(id).unwrap().make)();
        let mut rng = Rng::new(3);
        println!("================ {id} ================");
        for step in 0..14 {
            if g.outcome().is_some() {
                break;
            }
            if step % 4 == 0 {
                println!("{}", g.render());
                println!(
                    "mutări: {:?}\n",
                    g.legal_moves().iter().take(8).collect::<Vec<_>>()
                );
            }
            let m = g.ai_move(rng.next_u64()).unwrap();
            g.play(&m).unwrap();
        }
    }
}
