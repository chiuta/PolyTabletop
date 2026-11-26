//! Utilitare de test comune pentru jocurile adăugate în valul 17 (Q4).
#![cfg(test)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};

/// Joacă partide cu mutări aleatoare: textele sunt unice, fiecare mutare listată se poate juca,
/// partida se termină înainte de `cap` mutări. Întoarce rezultatele.
pub fn random_finishes(
    mk: impl Fn(u64) -> Box<dyn GameObj>,
    seeds: u64,
    cap: usize,
) -> Vec<Option<Outcome>> {
    let mut res = Vec::new();
    for seed in 1..=seeds {
        let mut g = mk(seed);
        let mut rng = Rng::new(seed.wrapping_mul(7919));
        let mut n = 0;
        while g.outcome().is_none() && n < cap {
            let mv = g.legal_moves();
            assert!(
                !mv.is_empty(),
                "fără mutări dar jocul nu s-a terminat\n{}",
                g.render()
            );
            let mut s = mv.clone();
            s.sort();
            s.dedup();
            assert_eq!(s.len(), mv.len(), "mutări duplicate\n{}", g.render());
            let m = mv[rng.below(mv.len())].clone();
            g.play(&m).expect("mutare listată dar nejucabilă");
            n += 1;
        }
        assert!(
            g.outcome().is_some(),
            "partida {seed} nu s-a terminat în {cap} mutări\n{}",
            g.render()
        );
        assert!(!g.render().is_empty());
        res.push(g.outcome());
    }
    res
}

/// AI-ul (care joacă Jucătorul `ai`) contra unui adversar aleator: mutările AI sunt legale,
/// iar partida se termină. Întoarce rezultatul.
pub fn ai_vs_random(
    mk: impl Fn(u64) -> Box<dyn GameObj>,
    seed: u64,
    ai: Player,
    cap: usize,
) -> Option<Outcome> {
    polytabletop_core::ai::set_time_budget_override_ms(Some(5));
    let mut g = mk(seed);
    let mut rng = Rng::new(seed ^ 0x51);
    let mut n = 0;
    while g.outcome().is_none() && n < cap {
        let legal = g.legal_moves();
        let m = if g.to_move() == ai {
            let m = g.ai_move(rng.next_u64()).expect("AI fără mutare");
            assert!(legal.iter().any(|x| x.replace(' ', "").to_lowercase() == m.replace(' ', "").to_lowercase()), "AI a ales o mutare ilegală: {m}");
            m
        } else {
            legal[rng.below(legal.len())].clone()
        };
        g.play(&m).unwrap();
        n += 1;
    }
    g.outcome()
}
