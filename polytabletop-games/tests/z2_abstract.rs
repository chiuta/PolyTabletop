#![allow(unused_imports)]
//! Cross-checks for the four z2 abstract games, through the object-safe interface.
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome};
use polytabletop_games::find;

const IDS: [&str; 4] = ["coridor", "drum", "tripierd", "konane"];

#[test]
fn registered_and_playouts_end() {
    for id in IDS {
        let p = find(id).unwrap_or_else(|| panic!("{id} missing"));
        for seed in 1..=8u64 {
            let mut g = (p.make)();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{id}: no moves yet no outcome");
                let pick = m[rng.below(m.len())].clone();
                // text round trip, also upper-cased
                g.play(&pick.to_uppercase()).unwrap();
                n += 1;
                assert!(n < 400, "{id} too long");
            }
            assert!(!g.render().is_empty());
        }
    }
}

#[test]
fn ai_plays_legal_and_fast_enough() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(30));
    for id in IDS {
        let p = find(id).unwrap();
        let mut g = (p.make)();
        let mut rng = Rng::new(5);
        for _ in 0..24 {
            if g.outcome().is_some() {
                break;
            }
            let m = g.ai_move(rng.next_u64()).expect("ai move");
            g.play(&m).unwrap_or_else(|e| panic!("{id}: {e}"));
        }
    }
    polytabletop_core::ai::set_time_budget_override_ms(None);
}

#[test]
fn ai_beats_random_at_tripierd_and_konane() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(40));
    for id in ["tripierd", "konane"] {
        let p = find(id).unwrap();
        let mut wins = 0;
        let games = 6;
        for k in 0..games {
            let mut g = (p.make)();
            let ai_first = k % 2 == 0;
            let mut rng = Rng::new(100 + k as u64);
            let mut ply = 0;
            while g.outcome().is_none() {
                let ai_turn = (ply % 2 == 0) == ai_first;
                let m = if ai_turn {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())].clone()
                };
                g.play(&m).unwrap();
                ply += 1;
            }
            let ai_player = if ai_first {
                polytabletop_core::Player::One
            } else {
                polytabletop_core::Player::Two
            };
            if g.outcome() == Some(Outcome::Win(ai_player)) {
                wins += 1;
            }
        }
        assert!(wins >= 4, "{id}: AI won only {wins}/{games}");
    }
    polytabletop_core::ai::set_time_budget_override_ms(None);
}
