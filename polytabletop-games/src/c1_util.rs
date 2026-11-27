//! Ajutoare de test pentru jocurile cooperative / de petrecere C1.
#[cfg(test)]
use crate::brand_util::*;
#[cfg(test)]
use polytabletop_core::rng::Rng;
#[cfg(test)]
use polytabletop_core::GameObj;

/// Test de onestitate: la fiecare mutare AI, informația ascunsă a celuilalt jucător este
/// amestecată (`scramble`) și mutarea AI-ului trebuie să rămână aceeași.
#[cfg(test)]
pub fn honest<M: Mini + 'static>(mk: fn(u64) -> M, scramble: fn(&mut M, usize, u64), plies: usize) {
    for seed in 1..=3u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed * 31);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            let who = pi(GameObj::to_move(&g));
            if lm.len() > 1 {
                let a = GameObj::ai_move(&g, 7).expect("AI fără mutare");
                let mut h = g.clone();
                scramble(&mut h.0, 1 - who, seed + n as u64);
                let b = GameObj::ai_move(&h, 7).expect("AI fără mutare");
                assert_eq!(
                    a,
                    b,
                    "AI-ul depinde de informația ascunsă a adversarului\n{}",
                    g.0.show()
                );
            }
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
    }
}

/// Redarea cu privitor: J1 și J2 văd lucruri diferite, iar `None` (hot-seat) = vederea celui la mutare.
#[cfg(test)]
pub fn viewer_ok<M: Mini + 'static>(mk: fn(u64) -> M, plies: usize) {
    let mut g = B(mk(3));
    let mut rng = Rng::new(5);
    let mut differ = false;
    let mut n = 0;
    while GameObj::outcome(&g).is_none() && n < plies {
        let mut v0 = g.clone();
        v0.0.view(None);
        let mut vm = g.clone();
        vm.0.view(Some(GameObj::to_move(&g)));
        assert_eq!(v0.0.show(), vm.0.show(), "None != vederea celui la mutare");
        let mut v1 = g.clone();
        v1.0.view(Some(polytabletop_core::Player::One));
        let mut v2 = g.clone();
        v2.0.view(Some(polytabletop_core::Player::Two));
        if v1.0.show() != v2.0.show() {
            differ = true;
        }
        let lm = GameObj::legal_moves(&g);
        let m = lm[rng.below(lm.len())].clone();
        GameObj::play(&mut g, &m).unwrap();
        n += 1;
    }
    assert!(differ, "vederile lui J1 și J2 nu diferă niciodată");
}
