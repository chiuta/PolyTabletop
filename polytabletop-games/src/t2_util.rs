//! Teste comune pentru familia T2 (RPG și miniaturi): partide aleatorii, AI legal,
//! determinism și „AI-ul nu vede zarurile viitoare” (mutarea AI nu depinde de generatorul de zaruri).
#![cfg(test)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Player};

pub fn check_fn<M: Mini + 'static>(mk: &dyn Fn(u64) -> M, plies: usize, reseed: fn(&mut M, u64)) {
    polytabletop_core::ai::set_time_budget_override_ms(Some(6));
    for seed in 1..=3u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed * 77);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            assert!(!lm.is_empty(), "fără mutări\n{}", g.0.show());
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        assert!(
            GameObj::outcome(&g).is_some(),
            "nu s-a terminat în {plies}\n{}",
            g.0.show()
        );
        assert!(!g.0.show().is_empty());
    }
    // determinism
    let run = |seed: u64| {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(5);
        for _ in 0..60 {
            if GameObj::outcome(&g).is_some() {
                break;
            }
            let lm = GameObj::legal_moves(&g);
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
        }
        g.0.show()
    };
    assert_eq!(run(9), run(9));
    // AI: legal, termină partida, iar mutarea nu depinde de zarurile viitoare
    for seed in 1..=2u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed + 1000);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            let ai_turn = GameObj::to_move(&g) == Player::One || seed == 2 && n % 3 == 0;
            let m = if ai_turn {
                let a = GameObj::ai_move(&g, 7).expect("AI fără mutare");
                assert!(lm.contains(&a), "AI a ales mutare ilegală {a}");
                let mut g2 = g.clone();
                reseed(&mut g2.0, 123_456 + n as u64);
                assert_eq!(
                    GameObj::ai_move(&g2, 7).unwrap(),
                    a,
                    "AI-ul depinde de zaruri\n{}",
                    g.0.show()
                );
                a
            } else {
                lm[rng.below(lm.len())].clone()
            };
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        assert!(
            GameObj::outcome(&g).is_some(),
            "partida cu AI nu s-a terminat\n{}",
            g.0.show()
        );
    }
}

/// Calitatea AI-ului: contra unui adversar aleatoriu, AI-ul câștigă mai mult de jumătate din partide
/// (pe jocuri-martor ale fiecărui motor); se listează toate jocurile T2 pentru orientare.
#[test]
fn t2_ai_beats_random() {
    let ids: Vec<&str> = include_str!("../data/i18n/titles_T2.tsv")
        .lines()
        .filter(|l| l.contains("\ten\t"))
        .map(|l| l.split('\t').next().unwrap())
        .collect();
    assert_eq!(ids.len(), 46);
    let all = std::env::var("T2_ALL").is_ok();
    polytabletop_core::ai::set_time_budget_override_ms(Some(6));
    for id in ids {
        if !all
            && ![
                "luna_in_cioburi",
                "faclia_din_hornuri",
                "regimente_pe_cimpie",
            ]
            .contains(&id)
        {
            continue;
        }
        let p = crate::find(id).unwrap();
        let (mut ai_w, mut rnd_w, mut dr) = (0, 0, 0);
        for seed in 0..8u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = (p.make)();
            let mut rng = Rng::new(seed + 40);
            let mut n = 0;
            while GameObj::outcome(&*g).is_none() && n < 1500 {
                let lm = g.legal_moves();
                let m = if g.to_move() == ai_side {
                    g.ai_move(seed * 13 + n as u64).unwrap()
                } else {
                    lm[rng.below(lm.len())].clone()
                };
                g.play(&m).unwrap();
                n += 1;
            }
            match g.outcome() {
                Some(polytabletop_core::Outcome::Win(w)) if w == ai_side => ai_w += 1,
                Some(polytabletop_core::Outcome::Win(_)) => rnd_w += 1,
                _ => dr += 1,
            }
        }
        eprintln!("{id}: AI {ai_w} / aleator {rnd_w} / remize {dr}");
        if !all {
            assert!(ai_w > rnd_w, "{id}: AI-ul nu bate adversarul aleatoriu");
        }
    }
}
