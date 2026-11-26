//! Utilitare comune pentru jocurile din valul 17 / Q6.
#![allow(dead_code)]

use polytabletop_core::rng::Rng;

/// Aruncă un zar cu `n` fețe (1..=n).
pub fn die(rng: &mut Rng, n: usize) -> u8 {
    rng.below(n) as u8 + 1
}

/// Amestecă un vector (Fisher–Yates).
pub fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

/// Eticheta unui rând de cărți: `1:2c 3:As`.
pub fn p_label(i: usize) -> String {
    format!("J{}", i + 1)
}
