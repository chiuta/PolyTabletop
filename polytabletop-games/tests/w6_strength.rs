#![allow(unused_imports)]
//! AI strength audit (ignored). Run:
//! W6_ONLY=hex,konane cargo test --release -p polytabletop-games --test w6_strength -- --ignored --nocapture
//! Phase 1: AI(100 ms) vs random, both colours (W6_RND games, default 40).
//! Phase 2: AI(500 ms) vs AI(50 ms), colours alternate (W6_VS games, default 20).
//! Also reports the worst single-move time against 2.5x the budget.
use polytabletop_core::ai::set_time_budget_override_ms;
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::amazoane::Amazoane;
use polytabletop_games::coridor::Coridor;
use polytabletop_games::drum::Drum;
use polytabletop_games::fanorona::Fanorona;
use polytabletop_games::hasami::Hasami;
use polytabletop_games::hex::Hex;
use polytabletop_games::izolare::Izolare;
use polytabletop_games::konane::Konane;
use polytabletop_games::linii::Linii;
use polytabletop_games::puncte::Puncte;
use polytabletop_games::rotire::Rotire;
use polytabletop_games::strapungere::Strapungere;
use polytabletop_games::surakarta::Surakarta;
use polytabletop_games::tigri::Tigri;
use polytabletop_games::tripierd::TriPierd;
use std::time::Instant;

type Maker = fn() -> Box<dyn GameObj>;

fn list() -> Vec<(&'static str, Maker)> {
    vec![
        ("coridor", || Box::new(Coridor::new())),
        ("drum", || Box::new(Drum::new())),
        ("tripierd", || Box::new(TriPierd::new())),
        ("konane", || Box::new(Konane::new())),
        ("hex", || Box::new(Hex::new())),
        ("amazoane", || Box::new(Amazoane::new())),
        ("linii", || Box::new(Linii::new())),
        ("fanorona", || Box::new(Fanorona::new())),
        ("tigri", || Box::new(Tigri::new())),
        ("surakarta", || Box::new(Surakarta::new())),
        ("hasami", || Box::new(Hasami::new())),
        ("puncte", || Box::new(Puncte::new())),
        ("strapungere", || Box::new(Strapungere::new())),
        ("izolare", || Box::new(Izolare::new())),
        ("rotire", || Box::new(Rotire::new())),
    ]
}

fn envn(k: &str, d: u64) -> u64 {
    std::env::var(k)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

/// Returns (ai wins, draws, losses, worst ms).
fn vs_random(make: Maker, n: u64, ms: u64) -> (u32, u32, u32, u128) {
    set_time_budget_override_ms(Some(ms));
    let (mut w, mut d, mut l, mut worst) = (0, 0, 0, 0u128);
    for i in 0..n {
        let ai = if i % 2 == 0 { Player::One } else { Player::Two };
        let mut g = make();
        let mut rng = Rng::new(i * 7919 + 13);
        let mut plies = 0;
        while g.outcome().is_none() {
            let mv = if g.to_move() == ai {
                let t = Instant::now();
                let m = g.ai_move(rng.next_u64()).expect("ai move");
                worst = worst.max(t.elapsed().as_millis());
                m
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            plies += 1;
            assert!(plies < 5000, "game too long");
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == ai => w += 1,
            Some(Outcome::Win(_)) => l += 1,
            _ => d += 1,
        }
    }
    (w, d, l, worst)
}

/// Strong (500 ms) vs weak (50 ms). Returns (strong wins, draws, losses, worst strong ms).
fn strong_weak(make: Maker, n: u64) -> (u32, u32, u32, u128) {
    let (mut w, mut d, mut l, mut worst) = (0, 0, 0, 0u128);
    for i in 0..n {
        let strong = if i % 2 == 0 { Player::One } else { Player::Two };
        let mut g = make();
        let mut rng = Rng::new(i * 104729 + 5);
        let mut plies = 0;
        while g.outcome().is_none() {
            let st = g.to_move() == strong;
            set_time_budget_override_ms(Some(if st { 500 } else { 50 }));
            let t = Instant::now();
            let mv = g.ai_move(rng.next_u64()).expect("ai move");
            if st {
                worst = worst.max(t.elapsed().as_millis());
            }
            g.play(&mv).unwrap();
            plies += 1;
            assert!(plies < 5000, "game too long");
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == strong => w += 1,
            Some(Outcome::Win(_)) => l += 1,
            _ => d += 1,
        }
    }
    (w, d, l, worst)
}

#[test]
#[ignore]
fn w6_strength_report() {
    let nr = envn("W6_RND", 40);
    let nv = envn("W6_VS", 20);
    let only = std::env::var("W6_ONLY").ok();
    println!("game        vsRnd W/D/L (100ms)  win%  worst_ms | strong>weak W/D/L (500v50)  score%  worst_ms");
    for (name, make) in list() {
        if let Some(o) = &only {
            if !o.split(',').any(|x| x == name) {
                continue;
            }
        }
        let (w, d, l, wm) = vs_random(make, nr, 100);
        let (sw, sd, sl, swm) = strong_weak(make, nv);
        let score = 100.0 * (sw as f64 + 0.5 * sd as f64) / nv as f64;
        let flag = |ok: bool| if ok { "" } else { "  <-- FAIL" };
        println!(
            "{:11} {:3}/{:2}/{:2}  {:6.1}%  {:6}{} | {:3}/{:2}/{:2}  {:6.1}%  {:6}{}",
            name,
            w,
            d,
            l,
            100.0 * w as f64 / nr as f64,
            wm,
            if wm > 250 { " SLOW" } else { "" },
            sw,
            sd,
            sl,
            score,
            swm,
            if swm > 1250 { " SLOW" } else { "" },
        );
        let _ = flag;
    }
    set_time_budget_override_ms(None);
}
