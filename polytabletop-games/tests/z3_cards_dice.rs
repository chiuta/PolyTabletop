#![allow(unused_imports)]
//! Integration checks for popa, pescuit, treizeci1, casino, zarpoker (via `GameObj`).
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Player};
use polytabletop_games::{playable, HOTSEAT_SAFE};

const IDS: &[&str] = &["popa", "pescuit", "treizeci1", "casino", "zarpoker"];

fn make(id: &str) -> Box<dyn GameObj> {
    let p = playable()
        .into_iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("lipsește {id}"));
    (p.make)()
}

#[test]
fn registered_and_hotseat_safe() {
    for id in IDS {
        assert!(HOTSEAT_SAFE.contains(id), "{id}");
        let g = make(id);
        assert!(!g.render().is_empty());
        assert!(!g.legal_moves().is_empty());
    }
}

#[test]
fn default_view_is_player_one_and_hotseat_follows_mover() {
    for id in IDS {
        let mut g = make(id);
        let default = g.render();
        g.set_viewer(Some(Player::One));
        assert_eq!(default, g.render(), "{id}");
        let mut rng = Rng::new(3);
        for step in 0..60 {
            if g.outcome().is_some() {
                break;
            }
            g.set_viewer(None);
            let strip = |t: String| -> String {
                t.lines()
                    .filter(|l| !l.contains("Predă ecranul"))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            let hot = strip(g.render());
            g.set_viewer(Some(g.to_move()));
            assert_eq!(hot, strip(g.render()), "{id}: step {step}");
            let mv = g.legal_moves();
            g.play(&mv[rng.below(mv.len())]).unwrap();
        }
    }
}

#[test]
fn ai_vs_ai_and_vs_random_finish() {
    for id in IDS {
        for seed in 1..4u64 {
            let mut g = make(id);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = if n % 2 == 0 || seed == 1 {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let mv = g.legal_moves();
                    mv[rng.below(mv.len())].clone()
                };
                g.play(&m).unwrap();
                n += 1;
                assert!(n < 20000, "{id}");
            }
        }
    }
}
