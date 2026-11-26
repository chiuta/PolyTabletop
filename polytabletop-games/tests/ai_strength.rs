//! AI strength harness (ignored by default).
//! Run: AI_GAMES=100 [AI_ONLY=skat] cargo test --release -p polytabletop-games --test ai_strength -- --ignored --nocapture
//! Competitive games: AI vs uniformly random, seats alternate. Cooperative games
//! (focare, cripta, strajile, cenusii): AI team vs random team (Win(One) = team win).
//! Hruba: AI occupies seat One against the two built-in robots.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::belote::Belote;
use polytabletop_games::cenusii::Cenusii;
use polytabletop_games::cripta::Cripta;
use polytabletop_games::focare::Focare;
use polytabletop_games::hruba::Hruba;
use polytabletop_games::macao::Macao;
use polytabletop_games::poker::Poker;
use polytabletop_games::rummy::Rummy;
use polytabletop_games::septica::Septica;
use polytabletop_games::skat::Skat;
use polytabletop_games::strajile::Strajile;
use polytabletop_games::tarot::Tarot;
use std::time::Instant;

type Maker = fn(u64) -> Box<dyn GameObj>;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Versus,
    Coop,
    Solo,
}

fn list() -> Vec<(&'static str, Maker, Kind)> {
    vec![
        ("skat", |s| Box::new(Skat::with_seed(s)), Kind::Versus),
        ("tarot", |s| Box::new(Tarot::with_seed(s)), Kind::Versus),
        ("hruba", |s| Box::new(Hruba::with_seed(s)), Kind::Solo),
        ("focare", |s| Box::new(Focare::with_seed(s)), Kind::Coop),
        ("cripta", |s| Box::new(Cripta::with_seed(s)), Kind::Coop),
        ("strajile", |s| Box::new(Strajile::with_seed(s)), Kind::Coop),
        ("cenusii", |s| Box::new(Cenusii::with_seed(s)), Kind::Coop),
        ("poker", |s| Box::new(Poker::with_seed(s)), Kind::Versus),
        ("belote", |s| Box::new(Belote::with_seed(s)), Kind::Versus),
        ("macao", |s| Box::new(Macao::with_seed(s)), Kind::Versus),
        ("rummy", |s| Box::new(Rummy::with_seed(s)), Kind::Versus),
        ("septica", |s| Box::new(Septica::with_seed(s)), Kind::Versus),
    ]
}

struct Res {
    win: u32,
    loss: u32,
    draw: u32,
    max_ms: u128,
    tot_ms: u128,
    moves: u128,
}

fn play(make: Maker, kind: Kind, games: u64) -> Res {
    let mut r = Res {
        win: 0,
        loss: 0,
        draw: 0,
        max_ms: 0,
        tot_ms: 0,
        moves: 0,
    };
    for seed in 1..=games {
        let ai_seat = if seed % 2 == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = make(seed);
        let mut rng = Rng::new(seed * 31 + 7);
        let mut plies = 0;
        while g.outcome().is_none() {
            let ai_turn = match kind {
                Kind::Coop => true,
                Kind::Solo => g.to_move() == Player::One,
                Kind::Versus => g.to_move() == ai_seat,
            };
            let mv = if ai_turn {
                let t = Instant::now();
                let m = g.ai_move(rng.next_u64()).expect("AI has a move");
                let e = t.elapsed().as_millis();
                r.max_ms = r.max_ms.max(e);
                r.tot_ms += e;
                r.moves += 1;
                m
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            if std::env::var("AI_TRACE")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                == Some(seed)
            {
                println!("{}\n> {}", g.render(), mv);
            }
            g.play(&mv).unwrap();
            plies += 1;
            assert!(plies < 20000);
        }
        // Coop with random team: also run random-team baseline separately.
        let ai_side = match kind {
            Kind::Coop | Kind::Solo => Player::One,
            Kind::Versus => ai_seat,
        };
        if std::env::var("AI_DEBUG").is_ok() && g.outcome() != Some(Outcome::Win(ai_side)) {
            let t = g.render();
            let lines: Vec<&str> = t.lines().collect();
            println!(
                "  loss seed {seed}: {}",
                lines[lines.len().saturating_sub(4)..].join(" | ")
            );
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == ai_side => r.win += 1,
            Some(Outcome::Win(_)) => r.loss += 1,
            _ => r.draw += 1,
        }
    }
    r
}

/// Random team baseline for coop / solo games (all random).
fn random_team(make: Maker, kind: Kind, games: u64) -> f64 {
    let mut w = 0;
    for seed in 1..=games {
        let mut g = make(seed);
        let mut rng = Rng::new(seed * 17 + 3);
        while g.outcome().is_none() {
            let m = g.legal_moves();
            let mv = m[rng.below(m.len())].clone();
            g.play(&mv).unwrap();
        }
        if g.outcome() == Some(Outcome::Win(Player::One)) {
            w += 1;
        }
    }
    let _ = kind;
    100.0 * w as f64 / games as f64
}

#[test]
#[ignore]
fn ai_strength_report() {
    let games: u64 = std::env::var("AI_GAMES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let only = std::env::var("AI_ONLY").ok();
    println!("game      games  win%   loss%  draw%  rndbase%  avg_ms/mv  max_ms/mv");
    for (name, make, kind) in list() {
        if let Some(o) = &only {
            if !o.split(',').any(|x| x == name) {
                continue;
            }
        }
        let r = play(make, kind, games);
        let base = if kind == Kind::Versus {
            f64::NAN
        } else {
            random_team(make, kind, games)
        };
        let n = games as f64;
        println!(
            "{:9} {:5} {:6.1} {:6.1} {:6.1} {:8.1} {:9.1} {:9}",
            name,
            games,
            100.0 * r.win as f64 / n,
            100.0 * r.loss as f64 / n,
            100.0 * r.draw as f64 / n,
            base,
            r.tot_ms as f64 / r.moves.max(1) as f64,
            r.max_ms
        );
    }
}
