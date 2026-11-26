//! Valul P3: Pinochle, Gin Rummy, Crazy Eights, Pitch, Bid Whist, All Fours, Conquián, Chicken Foot,
//! Bunco, Farkle, Liar's Dice, Crokinole — AI cinstit (nu citește informația ascunsă), terminare, viteză.
//!
//! Testul central: în orice poziție, AI-ul alege **aceeași** mutare dacă informația ascunsă (mâna
//! adversarului, stocul, zarurile din pahar, zgomotul viitor) e înlocuită cu o altă lume compatibilă
//! cu ce vede jucătorul la mutare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{ai, GameObj, Outcome, Player};
use polytabletop_games::all_fours::AllFours;
use polytabletop_games::bid_whist::BidWhist;
use polytabletop_games::bunco::Bunco;
use polytabletop_games::chicken_foot::ChickenFoot;
use polytabletop_games::conquian::Conquian;
use polytabletop_games::crazy_eights::CrazyEights;
use polytabletop_games::crokinole::Crokinole;
use polytabletop_games::farkle::Farkle;
use polytabletop_games::gin_rummy::GinRummy;
use polytabletop_games::liars_dice::LiarsDice;
use polytabletop_games::p3_comun::Honest;
use polytabletop_games::pinochle::Pinochle;
use polytabletop_games::pitch::Pitch;
use std::time::Instant;

/// Nivelul AI și bugetul de timp sunt globale per proces: testele care le ating rulează pe rând.
static AI_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

type Maker = fn(u64) -> Box<dyn GameObj>;

fn makers() -> Vec<(&'static str, Maker)> {
    vec![
        ("pinochle", |s| Box::new(Pinochle::with_seed(s))),
        ("gin_rummy", |s| Box::new(GinRummy::with_seed(s))),
        ("crazy_eights", |s| Box::new(CrazyEights::with_seed(s))),
        ("pitch", |s| Box::new(Pitch::with_seed(s))),
        ("bid_whist", |s| Box::new(BidWhist::with_seed(s))),
        ("all_fours", |s| Box::new(AllFours::with_seed(s))),
        ("conquian", |s| Box::new(Conquian::with_seed(s))),
        ("chicken_foot", |s| Box::new(ChickenFoot::with_seed(s))),
        ("bunco", |s| Box::new(Bunco::with_seed(s))),
        ("farkle", |s| Box::new(Farkle::with_seed(s))),
        ("liars_dice", |s| Box::new(LiarsDice::with_seed(s))),
        ("crokinole", |s| Box::new(Crokinole::with_seed(s))),
    ]
}

#[test]
fn every_move_listed_is_playable_and_unique() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for (name, mk) in makers() {
        let mut g = mk(3);
        let mut rng = Rng::new(5);
        for _ in 0..400 {
            if g.outcome().is_some() {
                break;
            }
            let mut m = g.legal_moves();
            assert!(
                !m.is_empty(),
                "{name}: fără mutări dar jocul nu s-a terminat"
            );
            let mut s = m.clone();
            s.sort();
            s.dedup();
            assert_eq!(s.len(), m.len(), "{name}: mutări duplicate");
            let pick = m.swap_remove(rng.below(m.len()));
            assert!(g.play(&pick).is_ok(), "{name}: {pick}");
        }
    }
}

#[test]
fn random_games_finish() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for (name, mk) in makers() {
        for seed in 1..=3u64 {
            let mut g = mk(seed);
            let mut rng = Rng::new(seed * 7 + 1);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let m = g.legal_moves();
                let p = m[rng.below(m.len())].clone();
                g.play(&p).unwrap();
                n += 1;
            }
            assert!(
                g.outcome().is_some(),
                "{name} seed {seed}: nu s-a terminat în {n} mutări"
            );
        }
    }
}

#[test]
fn deterministic_for_a_seed() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for (name, mk) in makers() {
        let (mut a, mut b) = (mk(11), mk(11));
        let mut r1 = Rng::new(2);
        let mut r2 = Rng::new(2);
        for _ in 0..60 {
            if a.outcome().is_some() {
                break;
            }
            let (m1, m2) = (a.legal_moves(), b.legal_moves());
            assert_eq!(m1, m2, "{name}");
            let (p1, p2) = (
                m1[r1.below(m1.len())].clone(),
                m2[r2.below(m2.len())].clone(),
            );
            a.play(&p1).unwrap();
            b.play(&p2).unwrap();
        }
        assert_eq!(a.render(), b.render(), "{name}");
    }
}

/// AI-ul alege aceeași mutare indiferent de informația ascunsă (altă lume compatibilă cu ce vede).
fn invariance_concrete<G: Honest + Clone + 'static>(name: &str, mk: fn(u64) -> G) {
    ai::set_level(ai::Level::Normal);
    for seed in 1..=3u64 {
        let mut g = mk(seed);
        let mut rng = Rng::new(seed + 40);
        let mut checked = 0;
        for step in 0..250 {
            if polytabletop_core::Game::outcome(&g).is_some() {
                break;
            }
            let legal = polytabletop_core::Game::legal_moves(&g);
            if step % 7 == 3 && legal.len() > 1 && checked < 4 {
                let me = polytabletop_core::Game::to_move(&g);
                let mut w = Rng::new(step as u64 * 31 + 7);
                let mut base = g.clone();
                base.forget_fresh();
                let g2 = base.determinize(me, &mut w);
                let a = GameObj::ai_move(&g, 99);
                let b = GameObj::ai_move(&g2, 99);
                assert_eq!(
                    a, b,
                    "{name} seed {seed} step {step}: AI-ul depinde de informația ascunsă"
                );
                checked += 1;
            }
            let m = legal[rng.below(legal.len())].clone();
            polytabletop_core::Game::apply(&mut g, &m);
        }
        assert!(checked > 0, "{name}: nicio poziție verificată");
    }
}

#[test]
fn ai_ignores_hidden_information() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    invariance_concrete("pinochle", Pinochle::with_seed);
    invariance_concrete("gin_rummy", GinRummy::with_seed);
    invariance_concrete("crazy_eights", CrazyEights::with_seed);
    invariance_concrete("pitch", Pitch::with_seed);
    invariance_concrete("bid_whist", BidWhist::with_seed);
    invariance_concrete("all_fours", AllFours::with_seed);
    invariance_concrete("conquian", Conquian::with_seed);
    invariance_concrete("chicken_foot", ChickenFoot::with_seed);
}

#[test]
fn ai_ignores_future_dice_and_noise() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ai::set_level(ai::Level::Normal);
    // Liar's Dice: zarurile adversarului
    for seed in 1..=6u64 {
        let mut g = LiarsDice::with_seed(seed);
        let mut rng = Rng::new(seed);
        for step in 0..40 {
            if polytabletop_core::Game::outcome(&g).is_some() {
                break;
            }
            let legal = polytabletop_core::Game::legal_moves(&g);
            if legal.len() > 1 {
                let me = polytabletop_core::Game::to_move(&g);
                let g2 = g.resample_hidden(me, step as u64 * 13 + 1);
                assert_eq!(
                    GameObj::ai_move(&g, 5),
                    GameObj::ai_move(&g2, 5),
                    "liars seed {seed} step {step}"
                );
            }
            let m = legal[rng.below(legal.len())];
            polytabletop_core::Game::apply(&mut g, &m);
        }
    }
    // Farkle: aruncările viitoare
    for seed in 1..=6u64 {
        let mut g = Farkle::with_seed(seed);
        let mut rng = Rng::new(seed + 9);
        for step in 0..80 {
            if polytabletop_core::Game::outcome(&g).is_some() {
                break;
            }
            let legal = polytabletop_core::Game::legal_moves(&g);
            if legal.len() > 1 {
                let mut g2 = g.clone();
                g2.reseed(step as u64 * 17 + 3);
                assert_eq!(
                    GameObj::ai_move(&g, 5),
                    GameObj::ai_move(&g2, 5),
                    "farkle seed {seed} step {step}"
                );
            }
            let m = legal[rng.below(legal.len())].clone();
            polytabletop_core::Game::apply(&mut g, &m);
        }
    }
    // Crokinole: zgomotul loviturii
    for seed in 1..=2u64 {
        let mut g = Crokinole::with_seed(seed);
        let mut rng = Rng::new(seed + 9);
        for step in 0..14 {
            if polytabletop_core::Game::outcome(&g).is_some() {
                break;
            }
            let legal = polytabletop_core::Game::legal_moves(&g);
            if legal.len() > 1 {
                let mut g2 = g.clone();
                g2.reseed(step as u64 * 17 + 3);
                assert_eq!(
                    GameObj::ai_move(&g, 5),
                    GameObj::ai_move(&g2, 5),
                    "crokinole seed {seed} step {step}"
                );
            }
            let m = legal[rng.below(legal.len())];
            polytabletop_core::Game::apply(&mut g, &m);
        }
    }
}

#[test]
fn hot_seat_viewer_never_shows_the_other_hand() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for (name, mk) in makers() {
        if matches!(name, "bunco" | "farkle" | "crokinole") {
            continue;
        }
        let mut g = mk(5);
        let mut rng = Rng::new(9);
        for _ in 0..120 {
            if g.outcome().is_some() {
                break;
            }
            let mut a = g.clone_box();
            a.set_viewer(None);
            let mut b = g.clone_box();
            b.set_viewer(Some(g.to_move()));
            let strip = |r: String| {
                r.lines()
                    .filter(|l| !l.starts_with('═'))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            assert_eq!(
                strip(a.render()),
                strip(b.render()),
                "{name}: hot-seat != privitor fix pe cel la mutare"
            );
            let mut c = g.clone_box();
            c.set_viewer(Some(g.to_move().other()));
            let other = c.render();
            // privitorul opus vede mâna lui (linia „Mâna ta (J…”) cu eticheta altui jucător
            let mine = b.render();
            let who = g.to_move().idx() + 1;
            if mine.contains("Mâna ta (J") {
                assert!(mine.contains(&format!("Mâna ta (J{who})")), "{name}");
                assert!(
                    !other.contains(&format!("Mâna ta (J{who})")) || g.outcome().is_some(),
                    "{name}: privitorul opus vede mâna celui la mutare"
                );
            }
            let m = g.legal_moves();
            let p = m[rng.below(m.len())].clone();
            g.play(&p).unwrap();
        }
    }
}

fn ai_vs_random(name: &str, mk: Maker, seeds: u64) -> (u32, u32, u128) {
    ai::set_time_budget_override_ms(Some(10));
    let (mut w, mut l) = (0, 0);
    let mut worst = 0u128;
    for seed in 1..=seeds {
        let mut g = mk(seed);
        let mut rng = Rng::new(seed + 1000);
        let mut n = 0;
        while g.outcome().is_none() && n < 6000 {
            let mv = if g.to_move() == Player::One {
                let t = Instant::now();
                let m = g
                    .ai_move(rng.next_u64())
                    .unwrap_or_else(|| panic!("{name}: AI fără mutare"));
                worst = worst.max(t.elapsed().as_millis());
                m
            } else {
                let m = g.legal_moves();
                m[rng.below(m.len())].clone()
            };
            g.play(&mv)
                .unwrap_or_else(|e| panic!("{name}: {e}\n{}", g.render()));
            n += 1;
        }
        match g.outcome() {
            Some(Outcome::Win(Player::One)) => w += 1,
            Some(Outcome::Win(Player::Two)) => l += 1,
            Some(Outcome::Draw) => {}
            None => panic!("{name}: AI vs random nu s-a terminat"),
        }
    }
    (w, l, worst)
}

#[test]
fn ai_beats_random_on_card_and_dice_games() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ai::set_level(ai::Level::Normal);
    for (name, mk, seeds) in [
        ("pinochle", makers()[0].1, 4u64),
        ("gin_rummy", makers()[1].1, 4),
        ("crazy_eights", makers()[2].1, 6),
        ("pitch", makers()[3].1, 6),
        ("bid_whist", makers()[4].1, 6),
        ("all_fours", makers()[5].1, 6),
        ("conquian", makers()[6].1, 6),
        ("chicken_foot", makers()[7].1, 2),
        ("farkle", makers()[9].1, 6),
        ("liars_dice", makers()[10].1, 8),
    ] {
        let (w, l, worst) = ai_vs_random(name, mk, seeds);
        println!("{name}: AI {w} - {l} aleator; cea mai lentă mutare {worst} ms");
        assert!(w >= l, "{name}: AI {w} - {l} contra aleator");
        assert!(worst < 4000, "{name}: mutare prea lentă ({worst} ms)");
    }
}

#[test]
fn crokinole_ai_plays_a_round_and_is_not_worse_than_random() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ai::set_level(ai::Level::Easy);
    let (w, l, worst) = ai_vs_random("crokinole", makers()[11].1, 2);
    println!("crokinole: AI {w} - {l} aleator; cea mai lentă mutare {worst} ms");
    assert!(w >= l);
    ai::set_level(ai::Level::Normal);
}
