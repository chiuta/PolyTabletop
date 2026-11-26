//! Valul Q3 (36 de jocuri): mutări unice și jucabile, AI legal și rapid, AI cinstit pentru jocurile cu
//! informație ascunsă, AI mai bun decât jocul aleator pentru jocurile abstracte.

use polytabletop_core::rng::Rng;
use polytabletop_core::{ai, GameObj, Outcome, Player};
use polytabletop_games::brag::Brag;
use polytabletop_games::mau_mau::MauMau;
use polytabletop_games::p3_comun::Honest;
use polytabletop_games::trick32::{Kind, Trick32};
use std::time::Instant;

static AI_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

const Q3_IDS: [&str; 36] = [
    "shashki",
    "shatranj",
    "chu_shogi",
    "o_an_quan",
    "ataxx",
    "tombola",
    "pachinko",
    "uckers",
    "muggins",
    "jacquet",
    "fan_tan",
    "hounds",
    "moultezim",
    "hanoi",
    "crossword",
    "pingpong",
    "hypergammon",
    "truth_dare",
    "ligretto",
    "sardines",
    "sticla",
    "schafkopf",
    "mau_mau",
    "preferans",
    "brag",
    "ulti",
    "scaune_muzicale",
    "mikado",
    "kick_can",
    "mingea",
    "shut_box",
    "chuck",
    "cosmic",
    "chowka_bara",
    "perquackey",
    "bar_dice",
];

fn make(id: &str) -> Box<dyn GameObj> {
    let p = polytabletop_games::playable()
        .into_iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("{id} lipsește din playable()"));
    (p.make)()
}

#[test]
fn all_q3_games_are_registered() {
    let ids: Vec<&str> = polytabletop_games::playable()
        .iter()
        .map(|p| p.id)
        .collect();
    for id in Q3_IDS {
        assert!(ids.contains(&id), "{id}");
    }
}

#[test]
fn moves_unique_playable_and_ai_legal_and_fast() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ai::set_level(ai::Level::Normal);
    for id in Q3_IDS {
        let mut g = make(id);
        let mut rng = Rng::new(17);
        for ply in 0..80 {
            if g.outcome().is_some() {
                break;
            }
            let m = g.legal_moves();
            assert!(
                !m.is_empty(),
                "{id}: fără mutări, dar jocul nu s-a terminat"
            );
            let mut s = m.clone();
            s.sort();
            s.dedup();
            assert_eq!(s.len(), m.len(), "{id}: mutări duplicate");
            if ply % 9 == 4 {
                let t = Instant::now();
                let a = g
                    .ai_move(7)
                    .unwrap_or_else(|| panic!("{id}: AI fără mutare"));
                assert!(m.contains(&a), "{id}: mutarea AI nu e legală: {a}");
                assert_eq!(
                    Some(a.clone()),
                    g.clone_box().ai_move(7),
                    "{id}: AI nedeterminist"
                );
                assert!(
                    t.elapsed().as_secs_f64() < 30.0,
                    "{id}: AI prea lent ({:?})",
                    t.elapsed()
                );
            }
            let pick = m[rng.below(m.len())].clone();
            assert!(g.play(&pick).is_ok(), "{id}: {pick}");
        }
    }
}

fn invariance<G: Honest + Clone + 'static>(name: &str, mk: fn(u64) -> G) {
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
            if step % 5 == 2 && legal.len() > 1 && checked < 4 {
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
    invariance("mau_mau", MauMau::with_seed);
    invariance("brag", Brag::with_seed);
    invariance("schafkopf", |s| Trick32::with_seed(Kind::Schafkopf, s));
    invariance("preferans", |s| Trick32::with_seed(Kind::Preferans, s));
    invariance("ulti", |s| Trick32::with_seed(Kind::Ulti, s));
}

/// AI-ul (jucătorul 1) bate un adversar aleator în cel puțin 3 din 4 partide.
fn beats_random(id: &str) {
    let mut wins = 0;
    for seed in 1..=4u64 {
        let mut g = make(id);
        let mut rng = Rng::new(seed * 13);
        let mut n = 0;
        while g.outcome().is_none() && n < 1500 {
            if g.to_move() == Player::One {
                let m = g.ai_move(seed).expect("AI fără mutare");
                g.play(&m).unwrap();
            } else {
                let m = g.legal_moves();
                let p = m[rng.below(m.len())].clone();
                g.play(&p).unwrap();
            }
            n += 1;
        }
        if g.outcome() == Some(Outcome::Win(Player::One)) {
            wins += 1;
        }
    }
    assert!(
        wins >= 3,
        "{id}: AI-ul a câștigat doar {wins}/4 contra jocului aleator"
    );
}

#[test]
fn ai_beats_random_in_abstract_games() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ai::set_level(ai::Level::Normal);
    for id in ["ataxx", "shashki", "o_an_quan", "kick_can", "hanoi"] {
        beats_random(id);
    }
}

#[test]
fn ai_beats_random_in_word_and_dice_games() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ai::set_level(ai::Level::Normal);
    for id in ["perquackey", "mikado"] {
        beats_random(id);
    }
}

#[test]
fn hot_seat_viewer_hides_other_hand() {
    let _ai = AI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for id in [
        "mau_mau",
        "muggins",
        "brag",
        "schafkopf",
        "preferans",
        "ulti",
        "sardines",
    ] {
        let mut g = make(id);
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
                    .filter(|l| !l.contains("Predă ecranul"))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            assert_eq!(strip(a.render()), strip(b.render()), "{id}");
            let m = g.legal_moves();
            let p = m[rng.below(m.len())].clone();
            g.play(&p).unwrap();
        }
    }
}
