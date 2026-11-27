//! Utilitare comune pentru jocurile de copii (lista C4, variante originale).
pub use crate::b6_util::pl;
pub use crate::brand_util::*;
pub use crate::w7_util::Dice;
pub use polytabletop_core::{Outcome, Player};

/// Adevărat cu probabilitatea `p`% (zar re-însămânțat la `clone()`).
pub fn chance(d: &mut Dice, p: i32) -> bool {
    (d.below(100) as i32) < p
}

/// Câștigă cine are mai multe puncte; egal = remiză.
pub fn pts_outcome(p: [i32; 2]) -> Outcome {
    by_score(p[0], p[1])
}

/// Ca `check`, dar acceptă o închidere (o specificație per joc).
#[cfg(test)]
use polytabletop_core::rng::Rng;
#[cfg(test)]
pub fn check_c<M: Mini + 'static>(mk: &dyn Fn(u64) -> M, plies: usize) {
    use polytabletop_core::GameObj;
    polytabletop_core::ai::set_time_budget_override_ms(Some(6));
    for seed in 1..=3u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed * 77);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            assert!(
                !lm.is_empty(),
                "fără mutări dar jocul nu s-a terminat\n{}",
                g.0.show()
            );
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        assert!(
            GameObj::outcome(&g).is_some(),
            "partida nu s-a terminat în {plies} mutări\n{}",
            g.0.show()
        );
        assert!(!g.0.show().is_empty());
    }
    // determinism
    let run = |seed: u64| {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(5);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < 60 {
            let lm = GameObj::legal_moves(&g);
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        g.0.show()
    };
    assert_eq!(run(9), run(9));
    // AI: legal și duce partida la capăt contra unui jucător aleatoriu
    for seed in 1..=2u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed + 1000);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            let m = if GameObj::to_move(&g) == Player::One || seed == 2 && n % 3 == 0 {
                let a = GameObj::ai_move(&g, rng.next_u64()).expect("AI fără mutare");
                assert!(lm.contains(&a), "AI a ales mutare ilegală {a}");
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
