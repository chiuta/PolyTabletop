//! Helperi mici comuni pentru jocurile cu „masa simulată” (copia w5).
//!
//! Convenție: `Player::One` este omul; `Player::Two` este *masa* — ceilalți
//! jucători, simulați de roboți interni cu euristici care citesc doar
//! informația pe care ar avea-o un jucător real (niciodată mâna omului).
//! Mutările masei sunt mutări separate (de obicei unica mutare `polytabletop`),
//! așa că AI-ul generic nu „vede” viitorul prin alegerile ei.

use polytabletop_core::rng::Rng;

/// Alege un indice cu probabilitate proporțională cu `w` (ponderi >= 0).
/// Dacă toate ponderile sunt 0, alege uniform.
pub fn pick_weighted(rng: &mut Rng, w: &[u32]) -> usize {
    let total: u64 = w.iter().map(|&x| x as u64).sum();
    if total == 0 {
        return rng.below(w.len());
    }
    let mut r = (rng.next_u64() % total) as i64;
    for (i, &x) in w.iter().enumerate() {
        r -= x as i64;
        if r < 0 {
            return i;
        }
    }
    w.len() - 1
}

/// Păstrează doar ultimele `n` rânduri din jurnal.
pub fn trim_log(log: &mut Vec<String>, n: usize) {
    if log.len() > n {
        let cut = log.len() - n;
        log.drain(0..cut);
    }
}

/// Amestecă un vector (Fisher–Yates).
pub fn shuffle_vec<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}
