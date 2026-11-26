#![allow(unused_imports)]
//! Cross-cutting checks run against every playable game.
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{catalog, playable, Kind};

// ---------------------------------------------------------------- Asia 2 wave

type SeedMaker = fn(u64) -> Box<dyn GameObj>;

fn asia2_makers() -> Vec<(&'static str, SeedMaker)> {
    use polytabletop_games::*;
    vec![
        ("mahjong", |s| Box::new(mahjong::Mahjong::with_seed(s))),
        ("hanafuda", |s| Box::new(hanafuda::Hanafuda::with_seed(s))),
        ("hwatu", |s| Box::new(hwatu::with_seed(s))),
        ("karuta", |s| Box::new(karuta::Karuta::with_seed(s))),
        ("doudizhu", |s| Box::new(dou_dizhu::DouDizhu::with_seed(s))),
        ("paigow", |s| Box::new(pai_gow::PaiGow::with_seed(s))),
        ("teenpatti", |s| {
            Box::new(teen_patti::TeenPatti::with_seed(s))
        }),
        ("ganjifa", |s| Box::new(ganjifa::Ganjifa::with_seed(s))),
        ("yut", |s| Box::new(yut::Yut::with_seed(s))),
        ("sugoroku", |s| Box::new(sugoroku::Sugoroku::with_seed(s))),
        ("chaupar", |s| Box::new(chaupar::Chaupar::with_seed(s))),
    ]
}

/// Returns (ai wins, random wins, draws) over `games` games; the AI alternates sides.
fn asia2_ai_vs_random(make: SeedMaker, games: u64) -> (u32, u32, u32) {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    let (mut a, mut r, mut d) = (0, 0, 0);
    for seed in 1..=games {
        let ai = if seed % 2 == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = make(seed);
        let mut rng = Rng::new(seed * 17 + 3);
        let mut plies = 0;
        while g.outcome().is_none() {
            let mv = if g.to_move() == ai {
                g.ai_move(rng.next_u64()).expect("AI has a move")
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            plies += 1;
            assert!(plies < 6000);
        }
        match g.outcome().unwrap() {
            Outcome::Win(p) if p == ai => a += 1,
            Outcome::Win(_) => r += 1,
            Outcome::Draw => d += 1,
        }
    }
    (a, r, d)
}

#[test]
fn asia2_ai_is_not_worse_than_random() {
    for (id, make) in asia2_makers() {
        let (a, r, d) = asia2_ai_vs_random(make, 30);
        eprintln!("{id}: AI {a} - random {r} (draws {d})");
        assert!(a + 3 >= r, "{id}: AI {a} vs random {r}");
    }
}

#[test]
fn asia2_ai_is_fast() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for (id, make) in asia2_makers() {
        let mut g = make(5);
        let mut rng = Rng::new(5);
        let mut worst = std::time::Duration::ZERO;
        let mut plies = 0;
        while g.outcome().is_none() && plies < 400 {
            let t = std::time::Instant::now();
            let m = g.ai_move(rng.next_u64()).expect("AI has a move");
            worst = worst.max(t.elapsed());
            g.play(&m).unwrap();
            plies += 1;
        }
        assert!(worst.as_millis() < 1000, "{id}: {worst:?}");
    }
}
