#![allow(unused_imports)]
//! coloane, bile, patru, cupe, poliedre: playouts, text moves, AI strength.

use polytabletop_core::ai::set_time_budget_override_ms;
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{find, playable};

const IDS: [&str; 5] = ["coloane", "bile", "patru", "cupe", "poliedre"];

#[test]
fn registered_with_help_and_catalog() {
    for id in IDS {
        let p = find(id).unwrap_or_else(|| panic!("{id} missing"));
        assert!(p.rules.len() > 80);
        assert_eq!(p.category, "Abstract");
        assert!(
            polytabletop_games::catalog()
                .iter()
                .any(|e| e.impl_id == id && e.kind == polytabletop_games::Kind::Implemented),
            "{id} not in catalog"
        );
    }
    let all = playable();
    for id in IDS {
        assert_eq!(all.iter().filter(|p| p.id == id).count(), 1);
    }
}

#[test]
fn random_playouts_end_and_texts_unique() {
    for id in IDS {
        for seed in 1..=10u64 {
            let mut g = (find(id).unwrap().make)();
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() {
                let mut m = g.legal_moves();
                assert!(!m.is_empty(), "{id}\n{}", g.render());
                let n = m.len();
                m.sort();
                m.dedup();
                assert_eq!(m.len(), n, "{id} duplicate texts");
                let pick = m[rng.below(m.len())].clone();
                g.play(&pick.to_uppercase()).expect("legal move plays");
                plies += 1;
                assert!(plies < 1500, "{id} too long");
            }
            assert!(g.legal_moves().is_empty());
            assert!(g.play("zzzz").is_err());
        }
    }
}

fn ai_vs_random(id: &str, games: u64) -> (u32, u32) {
    let (mut win, mut done) = (0, 0);
    for k in 0..games {
        let ai_side = if k % 2 == 0 { Player::One } else { Player::Two };
        let mut g = (find(id).unwrap().make)();
        let mut rng = Rng::new(1000 + k);
        while g.outcome().is_none() {
            let m = if g.to_move() == ai_side {
                g.ai_move(k).expect("ai move")
            } else {
                let l = g.legal_moves();
                l[rng.below(l.len())].clone()
            };
            g.play(&m).unwrap();
        }
        done += 1;
        if g.outcome() == Some(Outcome::Win(ai_side)) {
            win += 1;
        }
    }
    (win, done)
}

#[test]
fn ai_beats_random() {
    set_time_budget_override_ms(Some(25));
    for id in IDS {
        let (w, n) = ai_vs_random(id, 20);
        eprintln!("{id}: {w}/{n}");
        assert!(w * 100 >= n * 80, "{id}: only {w}/{n}");
    }
    set_time_budget_override_ms(None);
}
