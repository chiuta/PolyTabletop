//! Utilitare comune pentru jocurile cu zaruri (lista C6, variante originale).
//!
//! Zarurile se aruncă doar în mutări forțate `tira`; starea dinaintea aruncării nu conține
//! niciun rezultat viitor care să fie citit de `eval`, deci AI-ul (adâncime fixă ≤ 1) nu poate trișa.
//! Generatorul e un xorshift pe 64 de biți, determinist după sămânță.
pub use crate::brand_util::*;
pub use polytabletop_core::{Outcome, Player};

pub fn seed_state(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (z ^ (z >> 31)) | 1
}

pub fn next(r: &mut u64) -> u64 {
    let mut x = *r;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *r = x;
    x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11
}

/// Zar cu `n` fețe: 0..n.
pub fn die(r: &mut u64, n: u32) -> u8 {
    (next(r) % n as u64) as u8
}

/// Index ponderat.
pub fn weighted(r: &mut u64, w: &[u32]) -> usize {
    let t: u32 = w.iter().sum();
    let mut x = (next(r) % t as u64) as u32;
    for (i, &k) in w.iter().enumerate() {
        if x < k {
            return i;
        }
        x -= k;
    }
    w.len() - 1
}

pub fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

pub fn dstr(d: &[u8]) -> String {
    d.iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Test: AI-ul nu citește generatorul de zaruri (starea viitoare a aruncărilor).
#[cfg(test)]
pub fn no_peek<M: Mini + 'static>(m: &M, flip: impl Fn(&mut M)) {
    let mut h = m.clone();
    flip(&mut h);
    for seed in 1..6 {
        assert_eq!(ai_of(m, seed), ai_of(&h, seed), "AI-ul citește viitorul");
    }
}

/// Avansează jocul cu mutări aleatoare până când `pred` e adevărat (maxim `n` mutări).
#[cfg(test)]
pub fn advance<M: Mini>(m: &mut M, n: usize, pred: impl Fn(&M) -> bool) {
    let mut rng = polytabletop_core::rng::Rng::new(3);
    for _ in 0..n {
        if pred(m) || m.result().is_some() {
            return;
        }
        let lm = m.moves();
        let k = lm[rng.below(lm.len())].clone();
        m.step(&k);
    }
}
