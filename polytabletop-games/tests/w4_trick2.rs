#![allow(unused_imports)]
//! Inimi, Pică, Prinde, Whist: reguli, secretul mâinii, terminare, AI fără trișat, AI contra aleator.

use polytabletop_core::rng::Rng;
use polytabletop_core::{ai, Game, GameObj, Outcome, Player};
use polytabletop_games::cards::join_cards;
use polytabletop_games::inimi::Inimi;
use polytabletop_games::pica::Pica;
use polytabletop_games::prinde::Prinde;
use polytabletop_games::whist::Whist;

type Maker = fn(u64) -> Box<dyn GameObj>;

fn makers() -> Vec<(&'static str, Maker)> {
    vec![
        ("inimi", |s| Box::new(Inimi::with_seed(s))),
        ("pica", |s| Box::new(Pica::with_seed(s))),
        ("prinde", |s| Box::new(Prinde::with_seed(s))),
        ("whist", |s| Box::new(Whist::with_seed(s))),
    ]
}

#[test]
fn registered_and_hotseat_safe() {
    for (id, _) in makers() {
        assert!(
            polytabletop_games::playable().iter().any(|p| p.id == id),
            "{id}"
        );
        assert!(polytabletop_games::HOTSEAT_SAFE.contains(&id), "{id}");
    }
}

#[test]
fn random_playouts_end() {
    for (id, make) in makers() {
        for seed in 1..=15u64 {
            let mut g = make(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{id} seed {seed}");
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 5000, "{id} seed {seed} nu se termină");
            }
        }
    }
}

macro_rules! no_leak {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            for seed in 1..=8u64 {
                let mut g = <$ty>::with_seed(seed);
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while Game::outcome(&g).is_none() {
                    let me = Game::to_move(&g);
                    let opp = me.other();
                    // render pentru J-ul care mută nu arată mâna adversarului
                    let mut v = g.clone();
                    Game::set_viewer(&mut v, Some(me));
                    let r = Game::render(&v);
                    let oh = join_cards(g.hand_of(opp));
                    if g.hand_of(opp).len() >= 2 {
                        assert!(!r.contains(&oh), "{r}");
                    }
                    assert!(r.contains(&join_cards(g.hand_of(me))) || g.hand_of(me).is_empty());
                    // AI: aceeași mutare oricum ar fi cărțile ascunse
                    let s = rng.next_u64();
                    let m0 = ai::best_move(&g, s);
                    for k in 0..3u64 {
                        let mut h = g.clone();
                        h.scramble_hidden(me, 1000 + k + seed);
                        assert_eq!(m0, ai::best_move(&h, s), "AI depinde de cărți ascunse");
                    }
                    let mv = Game::legal_moves(&g);
                    let pick = if n % 3 == 0 {
                        m0.unwrap()
                    } else {
                        mv[rng.below(mv.len())].clone()
                    };
                    g.apply(&pick);
                    n += 1;
                }
            }
        }
    };
}
no_leak!(inimi_no_leak, Inimi);
no_leak!(pica_no_leak, Pica);
no_leak!(prinde_no_leak, Prinde);
no_leak!(whist_no_leak, Whist);

fn versus_random(make: Maker, games: u64) -> f64 {
    let mut wins = 0.0;
    for seed in 1..=games {
        let ai_seat = if seed % 2 == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = make(seed);
        let mut rng = Rng::new(seed * 31 + 7);
        while g.outcome().is_none() {
            let mv = if g.to_move() == ai_seat {
                g.ai_move(rng.next_u64()).unwrap()
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv).unwrap();
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == ai_seat => wins += 1.0,
            Some(Outcome::Draw) => wins += 0.5,
            _ => {}
        }
    }
    100.0 * wins / games as f64
}

#[test]
fn ai_beats_random() {
    for (id, make) in makers() {
        let w = versus_random(make, 100);
        println!("{id}: AI {w:.0}%");
        assert!(w >= 60.0, "{id}: {w}");
    }
}

#[test]
fn renders_do_not_show_opponent_hand_in_hotseat() {
    for (id, make) in makers() {
        let mut g = make(5);
        g.set_viewer(None);
        let r = g.render();
        assert!(r.contains("Mâna ta"), "{id}");
        assert!(
            !r.contains("Mâna ta (J2)") || g.to_move() == Player::Two,
            "{id}"
        );
    }
}

#[test]
#[ignore]
fn ai_strength_300_games() {
    for (id, make) in makers() {
        println!("{id}: AI {:.1}%", versus_random(make, 300));
    }
}
