//! Mancala family, Battleship, Mastermind, Memory, Bingo/Lotería, Pachisi, Carrom:
//! AI strength, AI speed and secrecy checks.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::ayo::Ayo;
use polytabletop_games::battleship::Battleship;
use polytabletop_games::bingo::CallGame;
use polytabletop_games::carrom::Carrom;
use polytabletop_games::congkak::Congkak;
use polytabletop_games::mastermind::Mastermind;
use polytabletop_games::memory::Memory;
use polytabletop_games::pachisi::Pachisi;
use polytabletop_games::pallanguzhi::Pallanguzhi;
use std::time::Instant;

type Maker = fn(u64) -> Box<dyn GameObj>;

fn makers() -> Vec<(&'static str, Maker)> {
    vec![
        ("congkak", |_| Box::new(Congkak::new())),
        ("pallanguzhi", |_| Box::new(Pallanguzhi::new())),
        ("ayo", |_| Box::new(Ayo::new())),
        ("carrom", |_| Box::new(Carrom::new())),
        ("pachisi", |s| Box::new(Pachisi::with_seed(s))),
        ("memory", |s| Box::new(Memory::with_seed(s))),
        ("battleship", |s| {
            let mut b: Box<dyn GameObj> = Box::new(Battleship::with_seed(s));
            b.play("aleator").unwrap();
            b.play("plasez").unwrap();
            b
        }),
        ("mastermind", |s| Box::new(Mastermind::with_seed(s))),
        ("bingo", |s| Box::new(CallGame::bingo_seed(s))),
        ("loteria", |s| Box::new(CallGame::loteria_seed(s))),
    ]
}

/// The AI plays `ai`; the other side moves at random. Returns (ai wins, random wins, slowest AI move in ms).
fn ai_vs_random(make: Maker, games: u64) -> (u32, u32, u128) {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    let (mut ai_w, mut rnd_w, mut slowest) = (0, 0, 0u128);
    for seed in 1..=games {
        let ai = if seed % 2 == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = make(seed);
        let mut rng = Rng::new(seed * 31);
        let mut plies = 0;
        while g.outcome().is_none() {
            let mv = if g.to_move() == ai {
                let t = Instant::now();
                let m = g.ai_move(rng.next_u64()).expect("AI has a move");
                slowest = slowest.max(t.elapsed().as_millis());
                m
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            plies += 1;
            assert!(plies < 6000);
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == ai => ai_w += 1,
            Some(Outcome::Win(_)) => rnd_w += 1,
            _ => {}
        }
    }
    (ai_w, rnd_w, slowest)
}

#[test]
fn ai_is_not_worse_than_random_and_is_fast() {
    for (id, make) in makers() {
        // Battleship's AI is always Player Two in these tests (Player One places): skip parity switch there.
        let games = match id {
            "carrom" => 10,
            "pachisi" => 150,
            "congkak" | "pallanguzhi" | "ayo" => 20,
            "bingo" | "loteria" => 40,
            _ => 40,
        };
        let (a, r, slow) = ai_vs_random(make, games);
        eprintln!("{id}: AI {a} vs random {r}, slowest AI move {slow} ms");
        assert!(a >= r, "{id}: AI {a} vs random {r}");
        assert!(slow < 1000, "{id}: AI move took {slow} ms");
    }
}

#[test]
fn perfect_information_games_ai_never_loses_to_random() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    for (id, make) in makers() {
        if !matches!(id, "congkak" | "pallanguzhi" | "ayo") {
            continue;
        }
        for seed in [3u64, 99] {
            let mut g = make(seed);
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
                assert!(plies < 3000);
            }
            assert_ne!(
                g.outcome(),
                Some(Outcome::Win(Player::Two)),
                "{id} seed {seed}"
            );
        }
    }
}

#[test]
fn hidden_info_renders_do_not_leak() {
    let mut b: Box<dyn GameObj> = Box::new(Battleship::with_seed(4));
    b.play("aleator").unwrap();
    b.play("plasez").unwrap();
    let r = b.render();
    assert!(
        !r.contains("nedescoperită"),
        "flota adversă nu apare înainte de final"
    );

    let m: Box<dyn GameObj> = Box::new(Mastermind::with_seed(4));
    assert!(!m.render().contains("era"));

    for g in [CallGame::bingo_seed(2), CallGame::loteria_seed(2)] {
        let g: Box<dyn GameObj> = Box::new(g);
        assert!(g.render().contains("Fișa Jucătorului 1"));
        assert!(!g.render().contains("Fișa Jucătorului 2"));
    }
}
