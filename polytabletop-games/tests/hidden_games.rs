//! Hidden-information games (Septica, Macao, Domino): AI sanity and secrecy.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::domino::Domino;
use polytabletop_games::macao::Macao;
use polytabletop_games::septica::Septica;

type Maker = fn(u64) -> Box<dyn GameObj>;

fn makers() -> Vec<(&'static str, Maker)> {
    vec![
        ("septica", |s| Box::new(Septica::with_seed(s))),
        ("macao", |s| Box::new(Macao::with_seed(s))),
        ("domino", |s| Box::new(Domino::with_seed(s))),
    ]
}

/// AI plays One (or Two alternately) against a random mover; returns (ai wins, random wins).
fn ai_vs_random(make: Maker, games: u64) -> (u32, u32) {
    polytabletop_core::ai::set_time_budget_override_ms(Some(10));
    let (mut ai_w, mut rnd_w) = (0, 0);
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
                g.ai_move(rng.next_u64()).expect("AI has a move")
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
            plies += 1;
            assert!(plies < 3000);
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == ai => ai_w += 1,
            Some(Outcome::Win(_)) => rnd_w += 1,
            _ => {}
        }
    }
    (ai_w, rnd_w)
}

#[test]
fn ai_games_terminate_with_legal_moves() {
    for (_, make) in makers() {
        let (a, r) = ai_vs_random(make, 20);
        assert!(a + r <= 20);
    }
}

#[test]
fn ai_is_not_worse_than_random() {
    for (id, make) in makers() {
        let (a, r) = ai_vs_random(make, 120);
        assert!(a >= r, "{id}: AI {a} vs random {r}");
    }
}

#[test]
fn render_never_shows_a_hand_that_is_not_the_viewers() {
    // text of every non-game-over render has exactly one "Mâna Jucătorului 1" line
    for (id, make) in makers() {
        let mut g = make(77);
        let mut rng = Rng::new(5);
        for _ in 0..30 {
            if g.outcome().is_some() {
                break;
            }
            let r = g.render();
            assert!(r.contains("Mâna Jucătorului 1"), "{id}\n{r}");
            assert!(!r.contains("Mâna Jucătorului 2"), "{id}\n{r}");
            let m = g.legal_moves();
            g.play(&m[rng.below(m.len())].clone()).unwrap();
        }
    }
}
