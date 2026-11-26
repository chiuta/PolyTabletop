#![allow(unused_imports)]
//! Teste de forță / terminare pentru AI-urile îmbunătățite (macao, focare, hruba, gomoku, expediții).
//! Cele lungi sunt `#[ignore]`: `cargo test --release -p polytabletop-games --test y4_ai -- --ignored --nocapture`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{ai, Game, Outcome, Player};
use polytabletop_games::expeditii::Expeditii;
use polytabletop_games::focare::Focare;
use polytabletop_games::gomoku::Gomoku;
use polytabletop_games::hruba::Hruba;
use polytabletop_games::macao::Macao;

fn play(g: &mut Gomoku, s: &str) {
    let m = Game::legal_moves(g)
        .into_iter()
        .find(|m| g.format_move(m) == s)
        .unwrap_or_else(|| panic!("illegal {s}"));
    g.apply(&m);
}

#[test]
fn gomoku_completes_five_and_blocks_four() {
    // X are patru în rând deschis; X la mutare trebuie să facă cinci.
    let mut g = Gomoku::new();
    for (a, b) in [("h8", "h10"), ("i8", "j10"), ("j8", "l10"), ("k8", "g10")] {
        play(&mut g, a);
        play(&mut g, b);
    }
    let m = ai::alphabeta_time(&g, 300, 6, 1).unwrap();
    let t = g.format_move(&m);
    assert!(t == "g8" || t == "l8", "{t}");
    // patru închis la un capăt: O trebuie să blocheze la l8.
    let mut g = Gomoku::new();
    play(&mut g, "h8");
    play(&mut g, "g8");
    play(&mut g, "i8");
    play(&mut g, "h10");
    play(&mut g, "j8");
    play(&mut g, "j10");
    play(&mut g, "k8");
    let m = ai::alphabeta_time(&g, 300, 6, 1).unwrap();
    assert_eq!(g.format_move(&m), "l8");
}

#[test]
fn gomoku_blocks_open_three() {
    let mut g = Gomoku::new();
    for (a, b) in [("h8", "h10"), ("i8", "j10"), ("j8", "")] {
        play(&mut g, a);
        if !b.is_empty() {
            play(&mut g, b);
        }
    }
    // O la mutare, X are trei deschise (h8 i8 j8): trebuie să se apere la capăt.
    let m = ai::alphabeta_time(&g, 300, 6, 1).unwrap();
    let t = g.format_move(&m);
    assert!(["g8", "k8", "f8", "l8"].contains(&t.as_str()), "{t}");
}

#[test]
fn expeditii_ai_vs_ai_terminates() {
    for seed in 1..=15u64 {
        let mut g = Expeditii::with_seed(seed);
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while Game::outcome(&g).is_none() {
            let m = ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            n += 1;
            assert!(n < 1500, "seed {seed} nu se termină");
        }
    }
}

use polytabletop_core::GameObj;

fn versus_random(make: fn(u64) -> Box<dyn polytabletop_core::GameObj>, games: u64) -> f64 {
    let mut wins = 0;
    for seed in 1..=games {
        let ai_seat = if seed % 2 == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = make(seed);
        let mut rng = Rng::new(seed * 31 + 7);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = if g.to_move() == ai_seat {
                g.ai_move(rng.next_u64()).unwrap()
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            n += 1;
            assert!(n < 20000);
        }
        if g.outcome() == Some(Outcome::Win(ai_seat)) {
            wins += 1;
        }
    }
    100.0 * wins as f64 / games as f64
}

#[test]
fn macao_ai_clearly_beats_random() {
    let w = versus_random(|s| Box::new(Macao::with_seed(s)), 60);
    assert!(w >= 65.0, "{w}");
}

#[test]
fn focare_ai_wins_more_than_a_third_of_seeds() {
    let mut wins = 0;
    for seed in 1..=40u64 {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Focare::with_seed(seed));
        let mut rng = Rng::new(seed * 31 + 7);
        while g.outcome().is_none() {
            let m = g.ai_move(rng.next_u64()).unwrap();
            g.play(&m).unwrap();
        }
        if g.outcome() == Some(Outcome::Win(Player::One)) {
            wins += 1;
        }
    }
    assert!(wins >= 10, "{wins}/40");
}

#[test]
fn hruba_ai_game_ends() {
    for seed in 1..=6u64 {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Hruba::with_seed(seed));
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = if g.to_move() == Player::One {
                g.ai_move(rng.next_u64()).unwrap()
            } else {
                g.legal_moves()[0].clone()
            };
            g.play(&m).unwrap();
            n += 1;
            assert!(n < 6000);
        }
    }
}
